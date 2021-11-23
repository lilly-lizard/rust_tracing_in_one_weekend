use dyn_clone::DynClone;

use crate::hit_record::HitRecord;
use crate::ray::Ray;

// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7

// hittable trait for hittable object generalization
pub trait Hittable: DynClone + Send {
    // returns true if the ray intersects the hittable object
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRecord;
}
dyn_clone::clone_trait_object!(Hittable);

// describe a list of hittable objects
#[derive(Clone)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut closest_hit: HitRecord = HitRecord::new_no_hit();

        for hittable_object in self.list.iter() {
            let hit = hittable_object.hit(ray, t_min, t_max);

            if !hit.no_hit() && hit.t() < closest_hit.t() {
                closest_hit = hit;
            }
        }

        return closest_hit;
    }
}
