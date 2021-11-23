use dyn_clone::DynClone;
use glam::Vec3;

use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::tools::{random_in_unit_sphere, random_unit_vector, reflect, refract, schlick};

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material: DynClone + Send {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay>; // TODO only ray direction needed...
}
dyn_clone::clone_trait_object!(Material);

// material types

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo: albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let scatter_direction: Vec3 = hit_record.normal() + random_unit_vector();
        Some(ScatteredRay {
            ray: Ray::new(hit_record.point(), scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected: Vec3 = reflect(ray.unit_direction(), hit_record.normal());

        if reflected.dot(hit_record.normal()) > 0.0 {
            Some(ScatteredRay {
                ray: Ray::new(
                    hit_record.point(),
                    reflected + self.fuzz * random_in_unit_sphere(),
                ),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refractive_index: f32,
}
impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric {
            refractive_index: refractive_index,
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let refractive_index = match hit_record.front_face() {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };
        let cos_theta = hit_record.normal().dot(-ray.unit_direction()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if refractive_index * sin_theta > 1.0 {
            // reflect
            let reflected = reflect(ray.unit_direction(), hit_record.normal());
            return Some(ScatteredRay {
                ray: Ray::new(hit_record.point(), reflected),
                attenuation: Vec3::splat(1.0),
            });
        }

        let reflect_prob = schlick(cos_theta, refractive_index);
        if rand::random::<f32>() < reflect_prob {
            // reflect
            let reflected = reflect(ray.unit_direction(), hit_record.normal());
            return Some(ScatteredRay {
                ray: Ray::new(hit_record.point(), reflected),
                attenuation: Vec3::splat(1.0),
            });
        }

        // refract
        let refracted = refract(ray.unit_direction(), hit_record.normal(), refractive_index);
        Some(ScatteredRay {
            ray: Ray::new(hit_record.point(), refracted),
            attenuation: Vec3::splat(1.0),
        })
    }
}
