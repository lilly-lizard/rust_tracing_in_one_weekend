use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableList};
use crate::material::Material;
use crate::ray::Ray;
use crate::rectangle::AxisRectangle;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Cuboid {
    min: Vec3,
    max: Vec3,
    sides: HittableList,
}
impl Cuboid {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides
            .list
            .push(Box::new(AxisRectangle::new_xy(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone())));
        sides
            .list
            .push(Box::new(AxisRectangle::new_xy(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone())));

        sides
            .list
            .push(Box::new(AxisRectangle::new_xz(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone())));
        sides
            .list
            .push(Box::new(AxisRectangle::new_xz(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone())));

        sides
            .list
            .push(Box::new(AxisRectangle::new_yz(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone())));
        sides
            .list
            .push(Box::new(AxisRectangle::new_yz(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone())));

        Self { min: p0, max: p1, sides: sides }
    }
}
impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}
