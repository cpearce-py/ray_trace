use rand::*;

const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random() -> f64 {
    rand::thread_rng().gen()
}

pub fn clamp<T>(x: T, min: T, max: T) -> T 
where T: PartialEq + PartialOrd
{
    if x < min { min } 
    else if x > max { max } 
    else { x }
}
