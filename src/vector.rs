use std::ops::{Div, Sub};
use crate::matrix::det2x2;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub fn dot(v0: &Vec3, v1: &Vec3) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
}

pub fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
    Vec3 {
        x: det2x2(&v0.y, &v1.y, &v0.z, &v1.z),
        y: det2x2(&v0.z, &v1.z, &v0.x, &v1.x),
        z: det2x2(&v0.x, &v1.x, &v0.y, &v1.y)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    #[allow(unused)]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z}
    }

    pub fn length(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    #[allow(unused)]
    pub fn dot(self, rhs: &Vec3) -> f32 {
        dot(&self,rhs)
    }

    #[allow(unused)]
    pub fn cross(self, rhs: &Vec3) -> Vec3 {
        cross(&self,rhs)
    }

    #[allow(unused)]
    pub fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}