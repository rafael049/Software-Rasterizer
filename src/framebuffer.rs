extern crate nalgebra_glm as glm;

use glm::U1;
use image::{ImageBuffer, Rgba};

pub struct Framebuffer {
    pub pixels: Vec<u8>,
    pub depth: Vec<f32>,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
        Framebuffer {
            pixels: vec![0; (width * height * 4) as usize],
            depth: vec![1.0; (width * height * 1) as usize],
            width: width,
            height: height,
            channels: 4,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: glm::Vec4) {
        if x < 0 || y < 0 {
            return;
        }

        self.pixels[(y * (self.width * self.channels) + x * self.channels + 0) as usize] =
            (value.x * 255.0) as u8;
        self.pixels[(y * (self.width * self.channels) + x * self.channels + 1) as usize] =
            (value.y * 255.0) as u8;
        self.pixels[(y * (self.width * self.channels) + x * self.channels + 2) as usize] =
            (value.z * 255.0) as u8;
        self.pixels[(y * (self.width * self.channels) + x * self.channels + 3) as usize] =
            (value.w * 255.0) as u8;
    }

    pub fn set_depth(&mut self, x: i32, y: i32, value: f32){
        if x < 0 || y < 0 {
            return;
        }

        self.depth[(y * (self.width * 1) + x * 1) as usize] = value;
            
    }

    pub fn get(&self, x: i32, y: i32) -> glm::Vec4 {
        let index = ((y * self.width + x) * self.channels) as usize;

        glm::vec4(
            self.pixels[index + 0] as f32 / 255.0,
            self.pixels[index + 1] as f32 / 255.0,
            self.pixels[index + 2] as f32 / 255.0,
            self.pixels[index + 3] as f32 / 255.0,
        )
    }

    pub fn get_depth(&self, x: i32, y: i32) -> f32 {
        let index = ((y * self.width + x) * 1) as usize;

        return self.depth[index];
    }

    pub fn line(&mut self, from: glm::I32Vec2, to: glm::I32Vec2, color: glm::Vec4) {
        let mut steep = false;

        let mut x0 = from.x;
        let mut x1 = to.x;
        let mut y0 = from.y;
        let mut y1 = to.y;

        if (x0 - x1).abs() < (y0 - y1).abs() {
            (x0, y0) = (y0, x0);
            (x1, y1) = (y1, x1);
            steep = true;
        }
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }
        for x in (x0)..(x1) {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = ((y0 as f32) * (1.0 - t) + (y1 as f32) * t) as i32;

            if steep {
                self.set(y, x, color);
            } else {
                self.set(x, y, color);
            }
        }
    }


    pub fn write_to_file(&self, filename: &str) {
        let image_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.width as u32,
            self.height as u32,
            self.pixels.as_slice(),
        )
        .expect("Failed to create ImageBuffer");

        // Save the image as a PNG file
        image::save_buffer(
            filename,
            &image_buffer,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgba8,
        )
        .expect("Failed to save PNG file");
    }

}

