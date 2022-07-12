// Module for all Vector based things.

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    fn length_square(&self) -> f64 {
       &self.x * &self.x + &self.y * &self.y + &self.z * &self.z  
    }
}

//fn main() {
//    let mut vec = Vec3{x: 1.1, y: 2.2, z: 3.3};
//    println!("{}", vec.length());
//}
