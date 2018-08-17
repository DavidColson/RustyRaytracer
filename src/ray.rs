use vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray{origin, dir}
    }

    #[inline(always)]
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}