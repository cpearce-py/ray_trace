use crate::vector::*;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere{
            center: Point3D{x: 0.0, y:0.0, z:0.0},
            radius: 0.5,
        }
    }

    pub fn offset(&mut self, axis: &str, value: f64) -> &Self {
        if axis == "x" {
            self.center.x += value;
        } else if axis == "y" {
            self.center.y += value;
        } else if axis == "z" {
            self.center.z += value;
        } else {
            println!("Cannot convert for {}", axis);
        }
        self
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {

        let oc = r.origin - self.center;
        let a = r.dir.sqr_magnitude();
        let b = oc.dot(&r.dir);
        let c = oc.sqr_magnitude() - (self.radius*self.radius);
        let discriminant = (b*b) - (a*c);
        if discriminant < 0.0 { return false};
        let sqrtd = discriminant.sqrt();

        // Find nearest root
        let mut root = (-b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd ) / a;
            if  root < t_min || t_max < root  {return false};
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }

}



