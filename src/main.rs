use ray_trace::vector::{Vector, Vector3, unit_vector};
use ray_trace::ray::Ray;
use ray_trace::ppm::{PPM, RGB};

fn ray_colour(ray: &Ray) -> RGB {
    let unit_direction = unit_vector(ray.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t)*RGB{r: 1.0, g:1.0, b:1.0} + t*RGB{r:0.5, g:0.7, b:1.0}
}

fn main() {

    // IMAGE
    const IMAGE_WIDTH: u32 = 512;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let mut image = PPM::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // CAMERA
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO / viewport_height;
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
