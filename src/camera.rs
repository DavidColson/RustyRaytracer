use rand;
use rand::Rng;

use vec::Vec3;
use ray::Ray;

pub struct Camera
{
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    view_plane_right: Vec3,
    view_plane_up: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(position: Vec3, lookat: Vec3, up: Vec3, fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = fov * 3.14159 / 180.0;
        let height = 2.0 * (theta / 2.0).tan();
        let width = aspect * height;

        let reverse_dir = (position - lookat).norm();
        let view_plane_right = up.cross(reverse_dir).norm();
        let view_plane_up = reverse_dir.cross(view_plane_right);

        // Note focus_dist here, we're setting the plane that we're sending rays through to be the focus plane. Where it is, things will be in focus
        let lower_left_corner = position - (width * focus_dist * 0.5) * view_plane_right - (height * focus_dist * 0.5) * view_plane_up - focus_dist * reverse_dir;
        let horizontal = width * focus_dist * view_plane_right;
        let vertical = height * focus_dist * view_plane_up;
        let origin = position;

        Camera {lower_left_corner, horizontal, vertical, origin,
         view_plane_right, view_plane_up, lens_radius: aperture / 2.0}
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random = self.lens_radius * random_in_unit_disk();
        let offset = self.view_plane_right * random.x + self.view_plane_up * random.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3{
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(100.0, 100.0, 100.0);
    while p.sqr_length() >= 1.0 {
        p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0)
    }
    p
}