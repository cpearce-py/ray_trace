use ray_trace::vector::{Vector, Vector3, Point3D};
use ray_trace::ray::Ray;
use ray_trace::ppm::{PPM, RGB};
use ray_trace::sphere::*;
use ray_trace::hittable::*;

const PI: f64 = f64::consts::PI;

fn hit_sphere(center: &Point3D, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.dir.sqr_magnitude();
    let b = oc.dot(&r.dir);
    let c = oc.sqr_magnitude() - (radius*radius);
    let discriminant = (b*b) - (a*c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / a;
    }

}

fn ray_colour<Hittable>(ray: &Ray, world: Hittable) -> RGB {
    let mut rec = HitRecord::new();
    if world.hit(r, 0, f64::INFINITY, rec) {
        return 0.5 * rec.normal + color(1, 1, 1);
    }
    let unit_direction = ray.dir.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t) * RGB{r:1.0, g:1.0, b:1.0} + t*RGB{r:0.5, g:0.7, b:1.0}
}
//fn ray_colour(ray: &Ray) -> RGB {
//    let sphere_center = Point3D{x:0.0, y:0.0, z:-1.0};
//    let mut t = hit_sphere(&sphere_center, 0.5, ray);
//    if t > 0.0 {
//        let n = (ray.at(t) - Vector3{x:0.0, y:0.0, z:-1.0}).normalized();
//        let r = n.x + 1.0;
//        let g = n.y + 1.0;
//        let b = n.z + 1.0;
//        let color = RGB{r, g, b};
//        return color * 0.5;
//    }
//    let unit_direction = ray.dir.normalized();
//    t = 0.5 * (unit_direction.y + 1.0);
//    (1.0-t)*RGB{r: 1.0, g:1.0, b:1.0} + t*RGB{r:0.5, g:0.7, b:1.0}
//}

fn main() {

    // IMAGE
    const IMAGE_WIDTH: u32 = 1920;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let mut image = PPM::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // WORLD
    let mut world = HittableList::new();
    let sphere1 = Sphere::new();
    world.push(sphere1);

    // CAMERA
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3{x: 0.0, y: 0.0, z:0.0};
    let horizontal = Vector3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3{x:0.0, y: viewport_height, z:0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3{x:0.0, y:0.0, z:focal_length};
    
    //RENDER
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = (IMAGE_HEIGHT-j) as f64 / (IMAGE_HEIGHT-1) as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_colour(&r);
            image.set_pixel(i, j, pixel_color);
        }
    }   

    let image = image.write_file("./test.ppm");
    match image {
        Ok(()) => println!("Successfully wrote ppm file."),
        Err(_) => println!("Failed to write ppm file.")
    }
}
