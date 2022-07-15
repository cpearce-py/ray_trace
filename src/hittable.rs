use crate::vector::{Point3D, Vector3, Vector};
use crate::ray::*;


#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3D,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    
    pub const fn new() -> Self {
        let p = Point3D{x:0.0, y:0.0, z:0.0};
        let normal = Vector3{x:0.0, y:0.0, z:0.0};
        let t = 0.0;
        let front_face = false;
        HitRecord { p, normal, t, front_face }
    }
    
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable:
{
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    // Box is to allow allocation on the heap
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList
{

    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    pub fn push<S: Hittable + 'static>(&mut self, obj: S) -> &mut Self {
        self.objects.push(Box::new(obj));
        self
    } 

    pub fn pop(&mut self) -> Option<Box<dyn Hittable + 'static>> {
        self.objects.pop()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.objects.clear();
        self
    }

    pub fn hit(&mut self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut obj = &self.objects[0];
//        for obj in self.objects.iter() {
//            dbg!(obj);
//        }
        hit_anything
    }
}
