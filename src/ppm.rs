use std::path::Path;
use std::io::Write;
use std::fs::File;

#[derive(Debug)]
pub struct RGB{
    r: u8,
    g: u8,
    b: u8,
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
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB {r, g, b})
            },
            None => None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
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

fn main() {
    let mut image = PPM::new(512, 512);

    for j in (0..image.height).rev() {
        for i in 0..image.width {

            let x = (i as f64 / (image.width - 1) as f64) * 255.999;
            let r = x as u8;

            let y = ((image.height - j) as f64 / (image.height) as f64) * 255.999;
            let g = y as u8;

            let color = RGB{r, g, b: 125};
            image.set_pixel(i, j, color);
        }
    }
    let image = image.write_file("./test.ppm");
    match image {
        Ok(()) => println!("Successfully wrote pppm file."),
        Err(_) => println!("Failed to write ppm file.")
    }
}
