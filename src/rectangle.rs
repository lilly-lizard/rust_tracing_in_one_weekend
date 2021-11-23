use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
pub struct AxisRectangle {
    c0: [f32; 2], // bottom corner
    c1: [f32; 2], // top corner
    k: f32,
    axis_u: usize,      // plane axis u
    axis_v: usize,      // plane axis v
    axis_normal: usize, // normal to plane
    material: Arc<dyn Material>,
    aabb_bottom: Vec3,
    aabb_top: Vec3,
}
impl AxisRectangle {
    pub fn new_xy(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        AxisRectangle::new(
            [x0, y0],
            [x1, y1],
            k,
            Axis::X,
            Axis::Y,
            Axis::Z,
            material.clone(),
            Vec3::new(x0, y0, k - 0.0001),
            Vec3::new(x1, y1, k + 0.0001),
        )
    }
    pub fn new_xz(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        AxisRectangle::new(
            [x0, z0],
            [x1, z1],
            k,
            Axis::X,
            Axis::Z,
            Axis::Y,
            material.clone(),
            Vec3::new(x0, k - 0.0001, z0),
            Vec3::new(x1, k + 0.0001, z1),
        )
    }
    pub fn new_yz(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<dyn Material>) -> Self {
        AxisRectangle::new(
            [y0, z0],
            [y1, z1],
            k,
            Axis::Y,
            Axis::Z,
            Axis::X,
            material.clone(),
            Vec3::new(k - 0.0001, y0, z0),
            Vec3::new(k + 0.0001, y1, z1),
        )
    }

    fn new(c0: [f32; 2], c1: [f32; 2], k: f32, axis_u: Axis, axis_v: Axis, axis_normal: Axis, material: Arc<dyn Material>, aabb_bottom: Vec3, aabb_top: Vec3) -> Self {
        Self {
            c0: c0,
            c1: c1,
            k: k,
            axis_u: axis_u as usize,
            axis_v: axis_v as usize,
            axis_normal: axis_normal as usize,
            material: material.clone(),
            aabb_bottom: aabb_bottom,
            aabb_top: aabb_top,
        }
    }
}
impl Hittable for AxisRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin()[self.axis_normal]) / ray.unit_direction()[self.axis_normal];
        if t < t_min || t_max < t {
            return None;
        }

        let p0 = ray.origin()[self.axis_u] + t * ray.unit_direction()[self.axis_u];
        let p1 = ray.origin()[self.axis_v] + t * ray.unit_direction()[self.axis_v];
        if p0 < self.c0[0] || self.c1[0] < p0 || p1 < self.c0[1] || self.c1[1] < p1 {
            return None;
        }
        let u = (p0 - self.c0[0]) / (self.c1[0] - self.c0[0]);
        let v = (p1 - self.c0[1]) / (self.c1[1] - self.c0[1]);

        let mut outward_normal = Vec3::zero();
        outward_normal[self.axis_normal] = 1.0;
        Some(HitRecord::new_hit(t, ray.at(t), outward_normal, ray, (u, v), self.material.clone()))
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(self.aabb_bottom, self.aabb_top)
    }
}
