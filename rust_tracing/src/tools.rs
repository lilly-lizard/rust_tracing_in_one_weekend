use glam::Vec3;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}

pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();

    let a: f32 = rng.sample(Uniform::new(0.0, 2.0 * std::f32::consts::PI));
    let z: f32 = rng.sample(Uniform::new(-1.0, 1.0));
    let r: f32 = (1.0 - z * z).sqrt();

    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(-1.0, 1.0);

    loop {
        let point = Vec3::new(rng.sample(range), rng.sample(range), rng.sample(range));
        if point.length() >= 1.0 {
            continue;
        }
        return point;
    }
}

fn _random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(thread_rng().gen_range(-1.0, 1.0), thread_rng().gen_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn refract(incident_ray: Vec3, normal: Vec3, refractive_index: f32) -> Vec3 {
    let cos_theta = normal.dot(-incident_ray);
    let r_out_perpendicular = refractive_index * (incident_ray + cos_theta * normal);
    let r_out_parallel = normal * -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt();
    r_out_perpendicular + r_out_parallel
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn get_uv_unit_sphere(point: Vec3) -> (f32, f32) {
    let phi = point.z().atan2(point.x());
    let theta = point.y().asin();
    let u = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
    let v = (theta + std::f32::consts::PI / 2.0) / std::f32::consts::PI;
    (u, v)
}
