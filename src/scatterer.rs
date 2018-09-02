extern crate rand;

use ray::Ray;
use vec::Vec3;
use drawable::HitResult;

pub trait Scatterer: Send + Sync {
    fn scatter(&self, in_ray: &Ray, hit: &HitResult, rng: &mut rand::ThreadRng) -> Option<(Vec3, Ray)>;
}