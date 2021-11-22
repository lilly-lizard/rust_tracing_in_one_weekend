use glam::Vec3;
use std::sync::Arc;

use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::{Ray, ScatteredRay};
use crate::texture::{SolidColor, Texture};
use crate::tools::random_in_unit_sphere;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}
impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f32, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(texture.clone())),
        }
    }
    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f32, color: Vec3) -> Self {
        Self {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_from_color(color)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec1 = match self.boundary.hit(ray, f32::NEG_INFINITY, f32::INFINITY) {
            None => return None,
            Some(hit) => hit,
        };
        let rec2 = match self.boundary.hit(ray, rec1.t() + 0.0001, f32::INFINITY) {
            None => return None,
            Some(hit) => hit,
        };

        let mut rec1_t = rec1.t();
        if rec1_t < t_min {
            rec1_t = t_min;
        }
        let mut rec2_t = rec2.t();
        if t_max < rec2_t {
            rec2_t = t_max;
        }

        if rec2_t <= rec1_t {
            return None;
        }

        if rec1_t < 0.0 {
            rec1_t = 0.0;
        }
        let distance_inside_boundary = rec2_t - rec1_t;
        let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();
        if distance_inside_boundary < hit_distance {
            return None;
        }

        let t = rec1_t + hit_distance;
        Some(HitRecord::new_hit(
            t,
            ray.at(t),
            Vec3::zero(),
            ray,
            (0.0, 0.0),
            self.phase_function.clone(),
        ))
    }
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

#[derive(Clone)]
struct Isotropic {
    albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo: albedo.clone() }
    }
    pub fn new_from_color(color: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(color)),
        }
    }
}
impl Material for Isotropic {
    fn scatter(&self, _ray_direction: &Vec3, hit_record: &HitRecord) -> Option<ScatteredRay> {
        Some(ScatteredRay {
            ray: Ray::new(hit_record.point(), random_in_unit_sphere()),
            attenuation: self.albedo.color(hit_record.uv(), &hit_record.point()),
        })
    }
    fn emitted(&self, _uv: (f32, f32), _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
