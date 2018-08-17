use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;
use std::ops::DivAssign;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    #[inline(always)]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn sqr_length(&self) -> f64{
        self.dot(*self)
    }

    #[inline(always)]
    pub fn length(&self) -> f64{
        self.sqr_length().sqrt()
    }

    #[inline(always)]
    pub fn norm(&self) -> Vec3 {
        let len = self.length();
        Vec3 {x: self.x / len, y: self.y / len, z: self.z / len}
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z - self.z * other.y),
            y: -(self.x * other.z - self.z * other.x), 
            z: (self.x * other.y - self.y * other.x),
            }
    }

    #[inline(always)]
    pub fn comp_mul(&self, other: Vec3) -> Vec3 {
        Vec3 {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vec3 {

    #[inline(always)]
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {

    #[inline(always)]
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 { x: self * vec.x, y: self * vec.y, z: self * vec.z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, scalar: f64) -> Vec3 {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, vec: Vec3) -> Vec3 {
        Vec3 { x: self / vec.x, y: self / vec.y, z: self / vec.z }
    }
}

impl DivAssign<f64> for Vec3 {

    #[inline(always)]
    fn div_assign(&mut self, scalar: f64) {
        *self = Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar };
    }
}