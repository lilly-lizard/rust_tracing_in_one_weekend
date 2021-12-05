#![allow(dead_code)]

mod bvh;
mod camera;
mod constant_medium;
mod cuboid;
mod hit_record;
mod hittable;
mod instance;
mod material;
mod perlin;
mod ray;
mod rectangle;
mod sphere;
mod texture;
mod tools;

use glam::Vec3;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use camera::Camera;
use constant_medium::ConstantMedium;
use cuboid::Cuboid;
use hittable::{Hittable, HittableList};
use instance::{RotateY, Translate};
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use ray::Ray;
use rectangle::AxisRectangle;
use sphere::Sphere;
use texture::{CheckerTexture, NoiseTexture};

// ash, sdl2, microprofile

// config
const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const SAMPLES_PER_PIXEL: i32 = 2000;
const MAX_BOUNCES: i32 = 50;
const RAY_OFFSET: f32 = 0.001;

type Background = Vec3;

#[derive(Clone)]
struct ThreadReturnData {
    thread_id: usize,
    color_data: Vec<Vec3>,
}

fn cornell_box() -> (HittableList, Camera, Background) {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new_from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_from_color(Vec3::new(15.0, 15.0, 15.0)));

    world.list.push(Box::new(AxisRectangle::new_yz(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.list.push(Box::new(AxisRectangle::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.list.push(Box::new(AxisRectangle::new_xz(113.0, 443.0, 127.0, 432.0, 554.0, light)));
    world
        .list
        .push(Box::new(AxisRectangle::new_xz(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    world
        .list
        .push(Box::new(AxisRectangle::new_xz(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world
        .list
        .push(Box::new(AxisRectangle::new_xy(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    // TODO if HittableList can use box with DynClone, can everything else too? (instead of Arc)

    let box1 = Arc::new(Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Arc::new(RotateY::new(box1.clone(), 15.0));
    let box1 = Translate::new(box1.clone(), Vec3::new(265.0, 0.0, 295.0));
    world
        .list
        .push(Box::new(ConstantMedium::new_from_color(Arc::new(box1), 0.01, Vec3::splat(0.0))));
    //world.list.push(Box::new(box1));

    let box2 = Arc::new(Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Arc::new(RotateY::new(box2.clone(), -18.0));
    let box2 = Translate::new(box2.clone(), Vec3::new(130.0, 0.0, 65.0));
    world
        .list
        .push(Box::new(ConstantMedium::new_from_color(Arc::new(box2), 0.01, Vec3::splat(1.0))));
    //world.list.push(Box::new(box2));

    // camera
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let focus_distance = 10.0;
    let aperture = 0.0;

    let camera: Camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        focus_distance,
    );

    let background = Vec3::new(0.0, 0.0, 0.0);
    (world, camera, background)
}

fn random_scene() -> (HittableList, Camera, Background) {
    let mut world = HittableList::new();

    // ground
    let ground_material = Lambertian::new(Box::new(CheckerTexture::new_from_colors(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
        10.0,
    )));
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(ground_material))));

    // small spheres
    fn rand_color() -> Vec3 {
        Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>())
    }
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(), 0.2, b as f32 + 0.9 * rand::random::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rand_color() * rand_color();
                    let sphere_material = Lambertian::new_from_color(albedo);
                    world.list.push(Box::new(Sphere::new(center, 0.2, Arc::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rand_color() * 0.5 + Vec3::splat(0.5);
                    let fuzz = rand::random::<f32>() * 0.5;
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.list.push(Box::new(Sphere::new(center, 0.2, Arc::new(sphere_material))));
                } else {
                    // dielectric
                    let sphere_material = Dielectric::new(1.5);
                    world.list.push(Box::new(Sphere::new(center, 0.2, Arc::new(sphere_material))));
                }
            }
        }
    }

    // big spheres
    let material_1 = Dielectric::new(1.5);
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(material_1))));

    let material_2 = Lambertian::new_from_color(Vec3::new(0.4, 0.2, 0.1));
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(material_2))));

    let material_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world
        .list
        .push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(material_3))));

    // camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;

    let camera: Camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        focus_distance,
    );

    let background = Vec3::new(0.7, 0.8, 1.0);
    (world, camera, background)
}

fn ray_color(ray: &Ray, world: &HittableList, background: &Background, recursive_depth: i32) -> Vec3 {
    if recursive_depth <= 0 {
        return Vec3::zero(); // bounce limit -> no light generated
    }

    let world_hit = world.hit(ray, RAY_OFFSET, f32::INFINITY);

    // hit
    if let Some(hit_res) = world_hit {
        let scatter_result = hit_res.material().scatter(&ray.unit_direction(), &hit_res);
        let emitted = hit_res.material().emitted(hit_res.uv(), &hit_res.point());

        match scatter_result {
            Some(scatter) => {
                let color = ray_color(&scatter.ray, &world, background, recursive_depth - 1);
                return emitted + scatter.attenuation * color;
            }
            None => return emitted, // no scattered/reflected ray -> fully absorbed, only return emitted component
        }
    }

    // no hit
    return *background;
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

    // create world and camera
    let (world, camera, background) = cornell_box();

    let thread_count = num_cpus::get();

    // start render threads
    let (tx_return, rx_return): (Sender<ThreadReturnData>, Receiver<ThreadReturnData>) = mpsc::channel();
    for t in 0..thread_count {
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
                    color += ray_color(&ray, &world, &background, MAX_BOUNCES);
                }

                return_data.color_data.push(color);
                pixel_index = pixel_index + thread_count;
            }
            tx_return_clone.send(return_data).unwrap();
        });
    }

    // wait for threads to complete rendering respective pixels
    let mut pixels = vec![Vec3::splat(-1.0); WIDTH * HEIGHT];
    for _t in 0..thread_count {
        let return_data: ThreadReturnData = rx_return.recv().unwrap();
        for i in 0..return_data.color_data.len() {
            pixels[return_data.thread_id + i * thread_count] = return_data.color_data[i];
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
