use std::sync::Arc;

use drawable::Drawable;
use drawable::HitResult;
use ray::Ray;
use vec::Vec3;
use materials::NullMaterial;
use scatterer::Scatterer;

pub struct World {
    objects: Vec<Box<dyn Drawable>>,
    background_mat: Arc<dyn Scatterer>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Drawable>>) -> World {
        World{objects, background_mat: Arc::new(NullMaterial::new())}
    }
}

impl Drawable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut result: HitResult = HitResult{
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::clone(&self.background_mat),
            };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                None => (),
                Some(hit_res) => {
                    hit_anything = true;
                    closest_so_far = hit_res.t;
                    result = hit_res;
                }
            }
        }

        if hit_anything {
            Some(result)
        }
        else {
            None
        }
    }
}