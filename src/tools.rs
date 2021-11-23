use glam::Vec3;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

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

    let a: f32 = rng.sample(Uniform::new(0.0, 2.0 * PI));
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
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

pub fn random_cosine_direction() -> Vec3 {
	let r1 = rand::random::<f32>();
	let r2 = rand::random::<f32>();
	let phi = 2.0 * PI * r1;
	let x = phi.cos() * r2.sqrt();
	let y = phi.sin() * r2.sqrt();
	let z = (1.0 - r2).sqrt();
	Vec3::new(x, y, z)
}

pub fn onb_build_from_w(n: &Vec3) -> [Vec3; 3] {
	let w = n.normalize();
	let a = if w.x().abs() > 0.9 {
		Vec3::new(0.0, 1.0, 0.0)
	} else {
		Vec3::new(1.0, 0.0, 0.0)
	};
	let v = w.cross(a).normalize();
	let u = w.cross(v);
	[u, v, w]
}

pub fn onb_local(obn: [Vec3; 3], a: Vec3) -> Vec3 {
	a.x() * obn[0] + a.y() * obn[1] + a.z() * obn[2]
}