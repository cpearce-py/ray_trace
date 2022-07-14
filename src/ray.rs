// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::vector::Point3D;
use crate::Vector3;


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Point3D, dir: Vector3) -> Self {
        Self {origin, dir}
    }
    pub fn at(self, t: f64) -> Vector3 {
        self.origin + t*self.dir
    }
}
