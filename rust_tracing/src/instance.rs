use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}
impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            object: object.clone(),
            offset: offset,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.unit_direction());
        let hit_record = self.object.hit(&moved_ray, t_min, t_max);
        match hit_record {
            None => None,
            Some(mut hit) => {
                hit.translate(self.offset);
                hit.set_face_normal(&moved_ray, hit.normal());
                Some(hit)
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        let aabb = self.object.bounding_box();
        AABB::new(aabb.min() + self.offset, aabb.max() + self.offset)
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    aabb: AABB,
}
impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle_d: f32) -> Self {
        let obj_aabb = object.bounding_box();
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let obj_max = obj_aabb.max();
                    let obj_min = obj_aabb.min();
                    let x = i as f32 * obj_max.x() + (1 - i) as f32 * obj_min.x();
                    let y = j as f32 * obj_max.y() + (1 - j) as f32 * obj_min.y();
                    let z = k as f32 * obj_max.z() + (1 - k) as f32 * obj_min.z();
                    let compare = Vec3::new(x, y, z);

                    for c in 0..3 {
                        min[c] = min[c].min(compare[c]);
                        max[c] = max[c].max(compare[c]);
                    }
                }
            }
        }

        let angle_r = angle_d.to_radians();
        Self {
            object: object.clone(),
            sin_theta: angle_r.sin(),
            cos_theta: angle_r.cos(),
            aabb: AABB::new(min, max),
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = ray.origin();
        let direction = ray.unit_direction();

        let mut o = origin;
        let mut d = direction;

        o[0] = self.cos_theta * origin[0] - self.sin_theta * origin[2];
        o[2] = self.sin_theta * origin[0] + self.cos_theta * origin[2];

        d[0] = self.cos_theta * direction[0] - self.sin_theta * direction[2];
        d[2] = self.sin_theta * direction[0] + self.cos_theta * direction[2];

        let rotated_ray = Ray::new(o, d);
        let hit_record = self.object.hit(&rotated_ray, t_min, t_max);
        match hit_record {
            None => None,
            Some(mut hit) => {
                let point = hit.point();
                let normal = hit.normal();

                let mut p = point;
                let mut n = normal;

                p[0] = self.sin_theta * point[2] + self.cos_theta * point[0];
                p[2] = self.cos_theta * point[2] - self.sin_theta * point[0];

                n[0] = self.sin_theta * normal[2] + self.cos_theta * normal[0];
                n[2] = self.cos_theta * normal[2] - self.sin_theta * normal[0];

                hit.translate(p - point); // point = p
                hit.set_face_normal(&rotated_ray, n);

                Some(hit)
            }
        }
    }
    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}
