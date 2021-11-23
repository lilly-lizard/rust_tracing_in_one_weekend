use glam::Vec3;
use std::sync::Arc;

use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::tools::get_uv_unit_sphere;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let half_b: f64 = oc.dot(ray.unit_direction()) as f64;
        let oc_length = oc.length() as f64;
        let c: f64 = oc_length * oc_length - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - c;

        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();

            let temp_t = (-half_b - root) as f32;
            if t_min < temp_t && temp_t < t_max {
                let intersection: Vec3 = ray.at(temp_t);
                let outward_normal: Vec3 = (intersection - self.center) / Vec3::splat(self.radius as f32);
                let uv = get_uv_unit_sphere(outward_normal); // note that outward_normal same as intersection equivalent on unit sphere
                return Some(HitRecord::new_hit(temp_t, intersection, outward_normal, ray, uv, self.material.clone()));
            }

            let temp_t = (-half_b + root) as f32;
            if t_min < temp_t && temp_t < t_max {
                let intersection: Vec3 = ray.at(temp_t);
                let outward_normal: Vec3 = (intersection - self.center) / Vec3::splat(self.radius as f32);
                let uv = get_uv_unit_sphere(outward_normal);
                return Some(HitRecord::new_hit(temp_t, intersection, outward_normal, ray, uv, self.material.clone()));
            }
        }

        return None;
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(self.center - Vec3::splat(self.radius as f32), self.center + Vec3::splat(self.radius as f32))
    }
}
