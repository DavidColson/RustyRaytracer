extern crate rand;

use rand::Rng;

use scatterer::Scatterer;
use vec::Vec3;
use ray::Ray;
use drawable::HitResult;


// ------------------
// Dialetric (Reflective + Refractive)
// ------------------

pub struct Dielectric {
    ior: f64
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric{ior}
    }
}

impl Scatterer for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit: &HitResult, rng: &mut rand::ThreadRng) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(0.95, 0.95, 0.95);

        let incident_norm = in_ray.dir.dot(hit.normal);
        let outward_norm = if incident_norm > 0.0 {-hit.normal} else {hit.normal};
        let ni_over_nt = if incident_norm > 0.0 {self.ior} else {1.0 / self.ior};
        let cosine = if incident_norm > 0.0 {
            self.ior * in_ray.dir.dot(hit.normal) / in_ray.dir.length()
        } else {
           -in_ray.dir.dot(hit.normal) / in_ray.dir.length()
        };

        // If refraction is possible, calculate probability that they ray reflects instead
        let refracted = refract(in_ray.dir, outward_norm, ni_over_nt);
        let reflect_prob = match refracted {
            Some(_) => schlick(cosine, self.ior),
            None => 1.0,
        };

        if rng.gen_range(0.0, 1.0) < reflect_prob  {
            Some((attenuation, Ray::new(hit.p, reflect(in_ray.dir, hit.normal)))) // reflect the ray
        }
        else {
            Some((attenuation, Ray::new(hit.p, refracted.unwrap()))) // refract the ray
        }
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let dt: f64 = v.norm().dot(n);
    let discrim: f64 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt* dt);
    if discrim > 0.0 {
        Some(ni_over_nt*(v.norm() - n * dt) - n * discrim.sqrt())
    }
    else{
        None
    }
}

fn schlick(cosine: f64, ior: f64) -> f64 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

// ------------------
// Metal (Reflective)
// ------------------

pub struct Metal {
    albedo: Vec3,
    roughness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f64) -> Metal {
        Metal{albedo, roughness}
    }
}

impl Scatterer for Metal {
    fn scatter(&self, in_ray: &Ray, hit: &HitResult, rng: &mut rand::ThreadRng) -> Option<(Vec3, Ray)> {
        let scattered = Ray::new(hit.p, reflect(in_ray.dir.norm(), hit.normal) + self.roughness * random_in_unit_sphere(rng));
        if scattered.dir.dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }
}

#[inline]
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}


// -----------------------------
// Lambertian (Diffuse material)
// -----------------------------

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{albedo}
    }
}

impl Scatterer for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitResult, rng: &mut rand::ThreadRng) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere(rng);
        Some((self.albedo, Ray::new(hit.p, (target) - hit.p)))
    }
}

#[inline]
fn random_in_unit_sphere(rng: &mut rand::ThreadRng) -> Vec3 {
    let mut p = Vec3::new(100.0, 100.0, 100.0);
    while p.sqr_length() >= 1.0 {
        p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0))
    }
    p
}

// -----------------------------------------------------------------
// Null Material (does nothing, not the same as absorbing everything)
// ------------------------------------------------------------------

pub struct NullMaterial {
}

impl NullMaterial {
    pub fn new() -> NullMaterial {
        NullMaterial{}
    }
}

impl Scatterer for NullMaterial {
    fn scatter(&self, _: &Ray, _: &HitResult, _: &mut rand::ThreadRng) -> Option<(Vec3, Ray)> {
        None
    }
}