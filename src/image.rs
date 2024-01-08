extern crate nalgebra_glm as glm;

use glm::U1;
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::Write;

pub struct Image {
    pub pixels: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        Image {
            pixels: vec![0; (width * height * 4) as usize],
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

    pub fn get(&self, x: i32, y: i32) -> glm::Vec4 {
        let index = ((y * self.width + x) * self.channels) as usize;

        glm::vec4(
            self.pixels[index + 0] as f32 / 255.0,
            self.pixels[index + 1] as f32 / 255.0,
            self.pixels[index + 2] as f32 / 255.0,
            self.pixels[index + 3] as f32 / 255.0,
        )
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

    pub fn triangle(
        &mut self,
        p1: glm::I32Vec2,
        p2: glm::I32Vec2,
        p3: glm::I32Vec2,
        color: glm::Vec4,
    ) {
        self.line(p1, p2, color);
        self.line(p2, p3, color);
        self.line(p3, p1, color);
    }

    pub fn fill_triangle(
        &mut self,
        p1: glm::I32Vec2,
        p2: glm::I32Vec2,
        p3: glm::I32Vec2,
        color: glm::Vec4,
    ) {
        let min_x = p1.x.min(p2.x).min(p3.x).max(0);
        let max_x = p1.x.max(p2.x).max(p3.x).min(self.width);
        let min_y = p1.y.min(p2.y).min(p3.y).max(0);
        let max_y = p1.y.max(p2.y).max(p3.y).min(self.height);

        for x in min_x..max_x {
            for y in min_y..max_y {
                if is_inside_triangle(glm::vec2(x, y), p1, p2, p3) {
                    self.set(x, y, color);
                }
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

fn to_baricentric_coords(point: glm::I32Vec2, p1: glm::I32Vec2, p2: glm::I32Vec2, p3: glm::I32Vec2) -> (f32, f32, f32) {
    let a = glm::vec3((p1-p2).x, (p1-p3).x, (point - p1).x);
    let b = glm::vec3((p1-p2).y, (p1-p3).y, (point - p1).y);
    let cross = glm::cross::<_,U1>(&a, &b);

    if cross.z == 0 {
        return (-1.0, -1.0, 1.0);
    }

    let (u, v) = (cross.x as f32 / cross.z as f32, cross.y as f32 / cross.z as f32);

    return (u , v, 1.0 - u - v);
}

fn is_inside_triangle(point: glm::I32Vec2, p1: glm::I32Vec2, p2: glm::I32Vec2, p3: glm::I32Vec2) -> bool {
    let (u, v, w) = to_baricentric_coords(point, p1, p2, p3); 

    return u >= 0.0 && v >= 0.0 && w >= 0.0;
}
