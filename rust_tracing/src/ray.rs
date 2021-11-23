use glam::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    unit_direction: Vec3, // guaranteed to be normalized
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let dir_normalized = direction.normalize();
        Self {
            origin: origin,
            unit_direction: dir_normalized,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn unit_direction(&self) -> Vec3 {
        self.unit_direction
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.unit_direction * t as f32
    }
}

pub struct ScatteredRay {
    pub ray: Ray,
	pub albedo: Vec3,
	pub pdf: f32,
}
