use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3;

// contains information about a ray intersection

pub struct HitRecord {
    no_hit: bool,
    t: f64,
    point: Vec3,
    normal: Vec3,
    front_face: bool,
    material: Option<Box<dyn Material>>,
}
impl HitRecord {
    pub fn new_hit(
        t: f64,
        point: Vec3,
        outward_normal: Vec3,
        ray: Ray,
        material: Box<dyn Material>,
    ) -> Self {
        let front_face: bool = ray.unit_direction().dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        Self {
            no_hit: false,
            t: t,
            point: point,
            normal: normal,
            front_face: front_face,
            material: Some(material),
        }
    }

    pub fn new_no_hit() -> Self {
        Self {
            no_hit: true,
            t: f64::MAX,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            front_face: false,
            material: None,
        }
    }

    pub fn no_hit(&self) -> bool {
        self.no_hit
    }
    pub fn t(&self) -> f64 {
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
    pub fn material(&self) -> &Box<dyn Material> {
        match &self.material {
            Some(m) => m,
            None => panic!("trying to access uninitialized material in HitRecord"),
        }
    }
}
