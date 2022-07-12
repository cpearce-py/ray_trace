// Module for all Vector based things.
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// This is like an ABC
pub trait Vector:
    Add
    + AddAssign
    + Sub
    + SubAssign
    + PartialEq
    + PartialOrd
    + Sized
{
    type Scalar;
    fn zero() -> Self;
    fn magnitude(&self) -> Self::Scalar;
    fn normalized(self) -> Self;
    fn normalize(&mut self);
    fn sqr_magnitude(&self) -> Self::Scalar;
    fn angle(&self, other: &Self) -> Self::Scalar;
    fn clamp_magnitude(self, max_len: Self::Scalar) -> Self;
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn scale(self, other: Self) -> Self;
    fn lerp(self, other: Self, t: Self::Scalar) -> Self;
    fn lerp_unclamped(self, other: Self, t: Self::Scalar) -> Self;
    fn max(self, other: Self) -> Self {
        if self >= other { self } else { other }
    }
    fn min(self, other: Self) -> Self {
        if self <= other { self } else { other }
    }
    fn reflect(self, normal: Self) -> Self;

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn cross(self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.z,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (*self - *other).magnitude()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Vector3{
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Vector for Vector3 {
    type Scalar = f64;

    fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn magnitude(&self) -> Self::Scalar {
        self.sqr_magnitude().sqrt()
    }

    fn normalized(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    fn normalize(&mut self) {
        *self = self.normalized();
    }

    fn sqr_magnitude(&self) -> Self::Scalar {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn angle(&self, other: &Self) -> Self::Scalar {
        let dot = self.dot(other);
        let mag = self.magnitude() * other.magnitude();
        (dot / mag).acos()
    }

    fn clamp_magnitude(self, max_len: Self::Scalar) -> Self{
        if self.magnitude() > max_len {
            self / max_len
        } else {
            self
        }
    }

    #[inline]
    fn dot(&self, other: &Self) -> Self::Scalar {
        self.x * other.x + self.y * other.y
    }

    fn scale(self, other: Self) -> Self {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    fn lerp(self, other: Self, t: Self::Scalar) -> Self {
        if t <= 0.0 {
            self
        } else if t >= 1.0 {
            other
        } else {
            self.lerp_unclamped(other, t)
        }
    }

    fn lerp_unclamped(self, other: Self, t: Self::Scalar) -> Self {
        (1.0 - t) * self + t * other
    }

    fn reflect(self, normal: Self) -> Self {
        -2.0 * self.dot(&normal) * normal + self
    }
}
