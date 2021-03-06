use ray_trace::vector::{Vector, Vector3, Point3D};
use ray_trace::ray::Ray;
use ray_trace::ppm::{PPM, RGB};
use ray_trace::sphere::*;
use ray_trace::hittable::*;
use ray_trace::camera::*;
use ray_trace::utils::random;


#[inline]
pub fn vec_to_rgb(vec: Vector3) -> RGB {
    RGB{r:vec.x, g:vec.y, b:vec.z}
}


fn ray_colour(r: &Ray, world: &HittableList, depth: u32) -> RGB {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return RGB{r:0.0, g:0.0, b:0.0};
    }

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vector3::random_in_unit_sphere();
        let new_ray = Ray{origin:rec.p, dir:target - rec.p};
        return 0.5 * (ray_colour(&new_ray, &world, depth-1));
    }

    let unit_direction = r.dir.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t) * RGB{r:1.0, g:1.0, b:1.0} + t*RGB{r:0.5, g:0.7, b:1.0}
}


fn main() {
   
    // IMAGE
    const SAMPLES_PER_PIXEL: u32 = 100;
    const IMAGE_WIDTH: u32 = 400;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const MAX_DEPTH: u32 = 50;
    let mut image = PPM::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // WORLD
    let mut world = HittableList::new();
    let center1 = Point3D{x:0.0, y:0.0, z:-1.0};
    let sphere1 = Sphere{center: center1, radius:0.5};
    let center2 = Point3D{x:0.0, y:-100.5, z:-1.0};
    let sphere2 = Sphere{center: center2, radius:100.0};
    world.push(sphere1);
    world.push(sphere2);

    // CAMERA
    let camera = Camera::new();
    
    //RENDER
    for j in (0..IMAGE_HEIGHT).rev() {
        println!("\rScanlines remaining: {}", &j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = RGB::new();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random())/(IMAGE_WIDTH-1) as f64;
                let v = ((IMAGE_HEIGHT-j) as f64 + random()) / (IMAGE_HEIGHT-1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_colour(&r, &world, MAX_DEPTH);
            }
            image.set_pixel(i, j, &mut pixel_color, SAMPLES_PER_PIXEL);
        }
    }   

    let image = image.write_file("./test.ppm");
    match image {
        Ok(()) => println!("Successfully wrote ppm file."),
        Err(_) => println!("Failed to write ppm file.")
    }
}
