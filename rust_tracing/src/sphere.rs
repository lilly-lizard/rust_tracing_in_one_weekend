use glam::Vec3;

use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc: Vec3 = ray.origin() - self.center;
        let half_b: f64 = oc.dot(ray.unit_direction()) as f64;
        let oc_length = oc.length() as f64;
        let c: f64 = oc_length * oc_length - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - c;

        if discriminant > 0.0 {
            let root: f64 = discriminant.sqrt();

            let temp_t = -half_b - root;
            if t_min < temp_t && temp_t < t_max {
                let intersection: Vec3 = ray.at(temp_t);
                let outward_normal: Vec3 =
                    (intersection - self.center) / Vec3::splat(self.radius as f32);
                return HitRecord::new_hit(
                    temp_t,
                    intersection,
                    outward_normal,
                    ray,
                    self.material.clone(),
                );
            }

            let temp_t: f64 = -half_b + root;
            if t_min < temp_t && temp_t < t_max {
                let intersection: Vec3 = ray.at(temp_t);
                let outward_normal: Vec3 =
                    (intersection - self.center) / Vec3::splat(self.radius as f32);
                return HitRecord::new_hit(
                    temp_t,
                    intersection,
                    outward_normal,
                    ray,
                    self.material.clone(),
                );
            }
        }

        return HitRecord::new_no_hit();
    }
}
