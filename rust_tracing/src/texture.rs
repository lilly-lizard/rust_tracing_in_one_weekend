use crate::perlin::Perlin;
use dyn_clone::DynClone;
use glam::Vec3;

pub trait Texture: DynClone + Send + Sync {
    fn color(&self, uv: (f32, f32), p: &Vec3) -> Vec3;
}
dyn_clone::clone_trait_object!(Texture);

#[derive(Clone)]
pub struct SolidColor {
    color: Vec3,
}
impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color: color }
    }
}
impl Texture for SolidColor {
    fn color(&self, _uv: (f32, f32), _p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    square_size: f32,
}
impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, square_size: f32) -> Self {
        Self {
            odd: odd,
            even: even,
            square_size: square_size,
        }
    }
    pub fn new_from_colors(odd: Vec3, even: Vec3, square_size: f32) -> Self {
        Self {
            odd: Box::new(SolidColor::new(odd)),
            even: Box::new(SolidColor::new(even)),
            square_size: square_size,
        }
    }
}
impl Texture for CheckerTexture {
    fn color(&self, uv: (f32, f32), p: &Vec3) -> Vec3 {
        let sines_product = (self.square_size * p.x()).sin() * (self.square_size * p.y()).sin() * (self.square_size * p.z()).sin();
        if sines_product < 0.0 {
            return self.odd.color(uv, p);
        } else {
            return self.even.color(uv, p);
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}
impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale: scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn color(&self, _uv: (f32, f32), p: &Vec3) -> Vec3 {
        //Vec3::splat(0.5 * (1.0 + self.noise.noise(p / self.scale)))
        //Vec3::splat(self.noise.turbulence(p / self.scale))
        Vec3::splat(0.5 * (1.0 + (p.z() / self.scale + 10.0 * self.noise.turbulence(p)).sin()))
        // marble
    }
}
