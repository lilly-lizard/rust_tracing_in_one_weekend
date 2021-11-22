use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;
use std::sync::Arc;

// contains information about a ray intersection
pub struct HitRecord {
    t: f32,
    point: Vec3,
    normal: Vec3,
    front_face: bool,
    uv: (f32, f32),
    material: Option<Arc<dyn Material>>,
}
impl HitRecord {
    // constructors
    pub fn new_hit(t: f32, point: Vec3, outward_normal: Vec3, ray: &Ray, uv: (f32, f32), material: Arc<dyn Material>) -> Self {
        let front_face: bool = ray.unit_direction().dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        Self {
            t: t,
            point: point,
            normal: normal,
            uv: uv,
            front_face: front_face,
            material: Some(material.clone()),
        }
    }
    pub fn new_hit_empty() -> Self {
        Self {
            t: f32::INFINITY,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            front_face: false,
            uv: (0.0, 0.0),
            material: None,
        }
    }

    // getters
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn point(&self) -> Vec3 {
        self.point
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material(&self) -> &Arc<dyn Material> {
        match &self.material {
            Some(m) => m,
            None => panic!("trying to access uninitialized material in HitRecord"),
        }
    }
    pub fn uv(&self) -> (f32, f32) {
        self.uv
    }

    // modifiers
    pub fn translate(&mut self, translation: Vec3) {
        self.point = self.point + translation;
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.unit_direction().dot(outward_normal) < 0.0;
        self.normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
    }
}
