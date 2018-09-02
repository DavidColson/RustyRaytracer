use std::sync::Arc;

use vec::Vec3;
use ray::Ray;
use scatterer::Scatterer;

pub struct HitResult {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Scatterer>
}

pub trait Drawable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}