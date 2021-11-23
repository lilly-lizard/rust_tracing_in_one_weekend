use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use glam::Vec3;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}
impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min: min, max: max }
    }
    pub fn new_surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Vec3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = Vec3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );
        AABB::new(small, big)
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn is_hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.unit_direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = match t0 > tmin {
                true => t0,
                false => tmin,
            };
            let tmax = match t1 < tmax {
                true => t1,
                false => tmax,
            };
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}

#[derive(Clone)]
pub struct BVHNode {
    aabb: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}
impl BVHNode {
    pub fn new(objects: &HittableList, start: usize, end: usize) -> Self {
        assert!(start <= end && end < objects.list.len());

        let axis: usize = thread_rng().gen_range(0, 3);
        let num_objects = end - start + 1;

        let mut object_left: Box<dyn Hittable> = objects.list[start].clone();
        let mut object_right: Box<dyn Hittable> = objects.list[start].clone();
        if num_objects == 2 {
            object_right = objects.list[end].clone();
        } else if num_objects > 2 {
            let mut sorted_objects = objects.list.clone();
            sorted_objects.sort_by(|a, b| box_compare(&a, &b, axis));
            let mid = start + num_objects / 2; // bias towards upper

            object_left = Box::new(BVHNode::new(objects, start, mid - 1));
            object_right = Box::new(BVHNode::new(objects, mid, end));
        }

        let box_left = object_left.bounding_box();
        let box_right = object_right.bounding_box();
        Self {
            aabb: AABB::new_surrounding_box(&box_left, &box_right),
            left: Arc::from(object_left),
            right: Arc::from(object_right),
        }
    }
}
impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.is_hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);
        if let Some(_) = hit_left {
            if let Some(_) = hit_right {
                return Some(HitRecord::new_hit_empty());
            }
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}

// axis parameter: 0 = x, 1 = y, 2 = z
fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box();
    let box_b = b.bounding_box();
    box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
}
