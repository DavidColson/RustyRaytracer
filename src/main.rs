#![warn(bare_trait_objects)]

extern crate image;
extern crate rand;
extern crate rayon;
extern crate atomic_counter;

mod ray;
mod vec;
mod sphere;
mod drawable;
mod world;
mod camera;
mod scatterer;
mod materials;

use atomic_counter::{AtomicCounter, RelaxedCounter};
use rand::Rng;
use std::time::Instant;
use std::sync::Arc;
use rayon::prelude::*;

use materials::Lambertian;
use materials::Metal;
use materials::Dielectric;
use camera::Camera;
use world::World;
use vec::Vec3;
use ray::Ray;
use drawable::Drawable;
use sphere::Sphere;

fn color(ray: &Ray, world: &World, depth: u32, rng: &mut rand::ThreadRng, ray_count: &mut i32) -> Vec3 {
    match world.hit(ray, 0.001, 1000000000000000000.0) {
        Some(hit_res) => {
            if depth < 50 {
                *ray_count += 1;
                match hit_res.material.scatter(ray, &hit_res, rng) {
                    Some((attenuation, scattered)) => return attenuation.comp_mul(color(&scattered, world, depth + 1, rng, ray_count)),
                    None => {
                        return Vec3::new(0.0, 0.0, 0.0)
                    }
                }
            } else {
                return Vec3::new(0.0, 0.0, 0.0)
            }
        },
        None => {
            
            let unit_dir = ray.dir.norm();
            let t = 0.5 * unit_dir.y + 1.0;
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let imgx = 1000.0;
    let imgy = 500.0;
    let rays_per_pix = 100;
    
    let cam_pos = Vec3::new(-1.5, 1.5, 0.75);
    let cam_target = Vec3::new(0.0, 0.0, -1.0);
    let focus_dist = (cam_target - cam_pos).length();

    let cam = Camera::new(
        cam_pos,
        cam_target,
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        imgx / imgy,
        0.2,
        focus_dist,
    );

    let mut objects: Vec<Box<dyn Drawable>>  = Vec::new();
    objects.push(
        Box::new(
            Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))
                )
            )
        );

    objects.push(
        Box::new(
            Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Arc::new(Lambertian::new(Vec3::new(0.6, 0.6, 0.6)))
                )
            )
        );

    objects.push(
        Box::new(
            Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1))
                )
            )
        );
    
    objects.push(
        Box::new(
            Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Arc::new(Dielectric::new(1.5))
                )
            )
        );

    objects.push(
        Box::new(
            Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                -0.47,
                Arc::new(Dielectric::new(1.5))
                )
            )
        );

    let world = World::new(objects);

    let start_time = Instant::now();

    let atomic_ray_counter = RelaxedCounter::new(0);

    let mut pixels = vec![image::Rgb([0, 0, 0]); imgx as usize * imgy as usize];

    pixels.par_iter_mut().enumerate().for_each(|(i, p)| {
        let mut rng = rand::thread_rng();
        let mut rays: i32 = 0;

        let x = i % imgx as usize;
        let y = (i - x) / imgx as usize;

        let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut rs = 0;
        while rs < rays_per_pix {
            let u = (rng.gen_range(0.0, 1.0) + x as f64) / imgx;
            let v = 1.0 - ((rng.gen_range(0.0, 1.0) + y as f64) / imgy);
            let r = cam.get_ray(u, v);
            rays += 1;
            col += color(&r, &world, 0, &mut rng, &mut rays);
            rs += 1;
        }
        col /= rays_per_pix as f64;
        let col_screen = Vec3::new(255.99 * col.x.sqrt(), 255.99 * col.y.sqrt(), 255.99 * col.z.sqrt());
        *p = image::Rgb([col_screen.x as u8, col_screen.y as u8, col_screen.z as u8]);
        atomic_ray_counter.add(rays as usize);
    });

    let img : image::RgbImage = image::ImageBuffer::from_fn(imgx as u32, imgy as u32, |x, y| {
        let idx = (y * imgx as u32) + x;
        pixels[idx as usize]
    });

    let duration = start_time.elapsed().as_secs() as f64 * 1_000_000.0 + start_time.elapsed().subsec_micros() as f64;
    let duration = duration / 1_000_000.0;
    println!("Total Time {}", duration);
    println!("Total Rays {}", (atomic_ray_counter.get() as f64));
    println!("Speed {} Rays per second", (atomic_ray_counter.get() as f64) / duration);
    img.save("image.png").unwrap();
}
