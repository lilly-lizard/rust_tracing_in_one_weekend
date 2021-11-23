use dyn_clone::DynClone;

use crate::bvh::AABB;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7

// hittable trait for hittable object generalization
pub trait Hittable: DynClone + Send + Sync {
    // returns true if the ray intersects the hittable object
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
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
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;

        for object in self.list.iter() {
            let hit = object.hit(ray, t_min, t_max);

            if let Some(hit_res) = &hit {
                match &closest_hit {
                    Some(closest_hit_res) => {
                        if hit_res.t() < closest_hit_res.t() {
                            closest_hit = hit;
                        }
                    }
                    None => {
                        // first hit found
                        closest_hit = hit;
                    }
                }
            }
        }

        return closest_hit;
    }

    fn bounding_box(&self) -> AABB {
        if self.list.is_empty() {
            panic!("[HittableList::bounding_box] trying to process empty hittable list");
        }

        let mut bounding_box = self.list[0].bounding_box();
        let mut first_box = true;
        for object in self.list.iter() {
            if first_box {
                first_box = false;
                continue;
            }
            bounding_box = AABB::new_surrounding_box(&bounding_box, &object.bounding_box());
        }
        bounding_box
    }
}
