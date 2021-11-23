mod camera;
mod hit_record;
mod hittable;
mod material;
mod ray;
mod sphere;
mod tools;

use glam::Vec3;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use camera::Camera;
use hit_record::HitRecord;
use hittable::*;
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;

// ash, sdl2, microprofile

// config
const THREAD_COUNT: usize = 12;
const WIDTH: usize = 600;
const HEIGHT: usize = 400;
const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_BOUNCES: i32 = 50;
const RAY_OFFSET: f64 = 0.001;

#[derive(Clone)]
struct ThreadReturnData {
    thread_id: usize,
    color_data: Vec<Vec3>,
}

fn rand_color() -> Vec3 {
    Vec3::new(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    )
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    // ground
    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    )));

    // small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rand_color() * rand_color();
                    let sphere_material = Lambertian::new(albedo);
                    world.list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rand_color() * 0.5 + Vec3::splat(0.5);
                    let fuzz = rand::random::<f32>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else {
                    // dielectric
                    let sphere_material = Dielectric::new(1.5);
                    world.list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                }
            }
        }
    }

    // big spheres
    let material_1 = Dielectric::new(1.5);
    world.list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material_1),
    )));

    let material_2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material_2),
    )));

    let material_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material_3),
    )));

    world
}

fn ray_color(ray: Ray, world: &HittableList, recursive_depth: i32) -> Vec3 {
    if recursive_depth <= 0 {
        return Vec3::zero(); // bounce limit -> no light generated
    }

    let world_hit: HitRecord = world.hit(ray, RAY_OFFSET, f64::INFINITY);

    // hit
    if !world_hit.no_hit() {
        let scatter_result = world_hit.material().scatter(&ray, &world_hit);

        match scatter_result {
            Some(scatter) => {
                return scatter.attenuation * ray_color(scatter.ray, &world, recursive_depth - 1)
            }
            None => return Vec3::zero(), // no scattered/reflected ray -> fully absorbed
        }
    }

    // no hit
    let t: f32 = 0.5 * (ray.unit_direction().y() + 1.0);
    return Vec3::splat(1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    println!("ray tracing in one weekend");
    let start_time = std::time::Instant::now();

    // open image file
    let mut file: File = match File::create("image.ppm") {
        Err(why) => panic!("couldn't create image file: {}", why),
        Ok(file) => file,
    };
    match write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT) {
        Err(why) => panic!("couldn't write to image: {}", why),
        Ok(_) => (),
    };

    // create world
    let world: HittableList = random_scene();

    // camera
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

    let camera: Camera = Camera::new(
        look_from,
        look_at,
        up,
        vfov,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    // start render threads
    let (tx_return, rx_return): (Sender<ThreadReturnData>, Receiver<ThreadReturnData>) =
        mpsc::channel();
    for t in 0..THREAD_COUNT {
        let tx_return_clone = mpsc::Sender::clone(&tx_return);
        let world_clone = world.clone();
        let camera_clone = camera.clone();

        thread::spawn(move || {
            let thread_id = t;
            let world = world_clone;
            let camera = camera_clone;
            let pixel_count = WIDTH * HEIGHT;

            let mut pixel_index = t;
            let mut return_data = ThreadReturnData {
                thread_id: thread_id,
                color_data: Vec::new(),
            };
            loop {
                if pixel_index >= pixel_count {
                    break;
				}

				let x = pixel_index % WIDTH;
				let y = pixel_index / WIDTH;
				if y % 10 == 0 && x == 0 {
					println!("scanlines remaining: {}", HEIGHT - y);
				}

                // render
                let mut color: Vec3 = Vec3::zero();
                for _s in 0..SAMPLES_PER_PIXEL {
                    let u: f32 = (x as f32 + rand::random::<f32>()) / (WIDTH - 1) as f32;
                    let v: f32 = (y as f32 + rand::random::<f32>()) / (HEIGHT - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    color += ray_color(ray, &world, MAX_BOUNCES);
                }

                return_data.color_data.push(color);
                pixel_index = pixel_index + THREAD_COUNT;
            }
            tx_return_clone.send(return_data).unwrap();
        });
    }

    // wait for threads to complete rendering respective pixels
    let mut pixels = vec![Vec3::splat(-1.0); WIDTH * HEIGHT];
    for _t in 0..THREAD_COUNT {
        let return_data: ThreadReturnData = rx_return.recv().unwrap();
        for i in 0..return_data.color_data.len() {
            pixels[return_data.thread_id + i * THREAD_COUNT] = return_data.color_data[i];
        }
    }

    // write to file
    println!("writing to file");
    for i in 0..(WIDTH * HEIGHT) {
        write_pixel(&mut file, pixels[i], SAMPLES_PER_PIXEL);
    }

    let duration = start_time.elapsed();
    println!(
        "image render + write took {}.{}s to complete",
        duration.as_secs(),
        duration.subsec_millis()
    );
}

fn write_pixel(file: &mut File, color: Vec3, samples_per_pixel: i32) -> () {
    let r = (color.x() / (samples_per_pixel as f32)).sqrt();
    let g = (color.y() / (samples_per_pixel as f32)).sqrt();
    let b = (color.z() / (samples_per_pixel as f32)).sqrt();

    match write!(
        file,
        "{} {} {} ",
        (255.999 * tools::clamp(r, 0.0, 0.999)) as i32,
        (255.999 * tools::clamp(g, 0.0, 0.999)) as i32,
        (255.999 * tools::clamp(b, 0.0, 0.999)) as i32
    ) {
        Err(why) => panic!("couldn't write to image: {}", why),
        Ok(_) => (),
    }
}
