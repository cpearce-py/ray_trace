use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::ops::{Add, Mul, MulAssign};


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct RGB{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}


impl RGB {
    pub fn new() -> Self {
        RGB{ r:1.0, g:1.0, b:1.0}
    }
}

impl Add for RGB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RGB {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl std::ops::AddAssign for RGB {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        RGB {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<RGB> for f64 {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> RGB {
        RGB {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl MulAssign<f64> for RGB {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}


pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl PPM {
    pub fn new(width: u32, height: u32) -> PPM {
        let size = 3 * width * height;
        let buffer = vec![0; size as usize];
        PPM { height, width, data: buffer }
    }

    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x *3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = (self.data[offset] / 255) as f64;
                let g = (self.data[offset + 1] / 255) as f64;
                let b = (self.data[offset + 2] / 255) as f64;
                Some(RGB {r, g, b})
            },
            None => None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &mut RGB, samples: u32) -> bool {
        let scale = 1.0 / samples as f64;
        color.r *= scale;
        color.g *= scale;
        color.b *= scale;
        //*color *= scale;
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = (255.99 * color.r) as u8;
                self.data[offset + 1] = (255.99 * color.g) as u8;
                self.data[offset + 2] = (255.99 * color.b) as u8;
                true
            },
            None => false
        }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}
