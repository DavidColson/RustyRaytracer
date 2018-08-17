use std::option::Option;

use drawable::Drawable;
use drawable::HitResult;
use ray::Ray;
use vec::Vec3;
use scatterer::Scatterer;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Scatterer>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Scatterer>) -> Sphere {
        Sphere{center, radius, material}
    }
}

impl Drawable for Sphere {
     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let oc = ray.origin - (*self).center;
        let a = ray.dir.dot(ray.dir);
        let b = oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discrim = b * b - a *c;
        if discrim > 0.0 {
            let disc_sqrt = discrim.sqrt();
            let temp = (-b - disc_sqrt) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let norm = (p - self.center) / self.radius;
                return Some(HitResult{t: temp, p: p, normal: norm, material: &self.material})
            }
            let temp = (-b + disc_sqrt) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let norm = (p - self.center) / self.radius;
                return Some(HitResult{t: temp, p: p, normal: norm, material: &self.material})
            }
        }
        None
     }
}