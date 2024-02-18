extern crate nalgebra_glm as glm;

use std::rc::Rc;

use glm::clamp;
use image::{GenericImageView, Pixel};

use crate::Framebuffer;

use super::Uniforms;


pub struct Rasterizer {
    pub width: i32,
    pub height: i32,
    screen_positions: Vec<glm::I32Vec2>,
    pub framebuffer: Framebuffer,
}


pub struct RasterizerInput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec2>,
    pub uniforms: super::Uniforms,
    pub textures: Vec<Rc<image::DynamicImage>>,
}



impl Rasterizer {
    pub fn new(
        width: i32,
        height: i32,
    ) -> Rasterizer {
        Rasterizer {
            width,
            height,
            screen_positions: vec![],
            framebuffer: Framebuffer::new(width, height),
        }
    }

    pub fn run(&mut self, input: &RasterizerInput) {
        self.screen_mapping(&input);
        self.rasterize(&input);
    }

    pub fn screen_mapping(&mut self, input: &RasterizerInput) {
        self.screen_positions.resize(input.positions.len(), glm::vec2(0, 0));

        for i in 0..input.positions.len() {
            let image_coords = Self::norm_to_image_coords(&input.positions[i], self.width, self.height);

            self.screen_positions[i] = image_coords;
        }
    }

    pub fn rasterize(&mut self, input: &RasterizerInput) {

        self.framebuffer.pixels.iter_mut().for_each(|e| *e = 0);
        self.framebuffer.depth.iter_mut().for_each(|e| *e = 1.0);

        for i in (0..self.screen_positions.len()).step_by(3) {
            let p1 = self.screen_positions[i];
            let p2 = self.screen_positions[i + 1];
            let p3 = self.screen_positions[i + 2];

            let p1_z = input.positions[i].z;
            let p2_z = input.positions[i + 1].z;
            let p3_z = input.positions[i + 2].z;

            let n1 = input.normals[i];
            let n2 = input.normals[i + 1];
            let n3 = input.normals[i + 2];

            let t1 = input.uv_coords[i];
            let t2 = input.uv_coords[i + 1];
            let t3 = input.uv_coords[i + 2];

            let min_x = p1.x.min(p2.x).min(p3.x).max(0);
            let max_x = p1.x.max(p2.x).max(p3.x).min(self.width);
            let min_y = p1.y.min(p2.y).min(p3.y).max(0);
            let max_y = p1.y.max(p2.y).max(p3.y).min(self.height);

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if Self::is_inside_triangle(glm::vec2(x, y), p1, p2, p3) {
                        let (u, v, w) = Self::to_baricentric_coords(glm::vec2(x, y), p1, p2, p3);
                        let depth = -(u * p1_z + v * p2_z + w * p3_z) / 2.0 + 0.5;
                        let normal = u * n1 + v * n2 + w * n3;
                        let tex_coord = u * t1 + v * t2 + w * t3;
                        if depth < self.framebuffer.get_depth(x, y) {
                            let color = self.fragment_shader(x, y, normal, tex_coord, &input.uniforms, &input.textures);
                            self.framebuffer.set(x, y, glm::vec4(color.x, color.y, color.z, 1.0));
                            self.framebuffer.set_depth(x, y, depth)
                        }
                    }
                }
            }
        }
    }


    fn fragment_shader(&self, x: i32, y:i32, normal: glm::Vec3, tex_coord: glm::Vec2, uniforms: &Uniforms, textures: &Vec<Rc<image::DynamicImage>>) -> glm::Vec3 {
        let diff = glm::dot(&normal, &-uniforms.sun_light_dir).clamp(0.0, 1.0);
        let diffuse = glm::vec3(diff, diff, diff);
        let ambient = glm::vec3(0.1, 0.1, 0.1);

        let (w, h) = textures[0].dimensions();
        let tx = (w as f32 * tex_coord.x as f32).clamp(0.0, (w - 1) as f32);
        let ty = (h as f32 * tex_coord.y as f32).clamp(0.0, (h - 1) as f32);
        let pixel = textures[0].get_pixel(tx as u32, (h -1) - ty as u32).0;
        let albedo = glm::vec3(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0);

        let attenuation = ambient + diffuse;
        let color = glm::matrix_comp_mult(&attenuation, &albedo);

        let inv_gamma = 1.0 / uniforms.gamma;
        let final_color = glm::pow(&color, &glm::vec3(inv_gamma, inv_gamma, inv_gamma));

        return final_color;
    }

    fn to_baricentric_coords(
        point: glm::I32Vec2,
        p1: glm::I32Vec2,
        p2: glm::I32Vec2,
        p3: glm::I32Vec2,
    ) -> (f32, f32, f32) {
        let a = glm::vec3((p3 - p1).x, (p3 - p2).x, (point - p3).x);
        let b = glm::vec3((p3 - p1).y, (p3 - p2).y, (point - p3).y);
        let cross = glm::cross::<_>(&a, &b);

        if cross.z == 0 {
            return (-1.0, -1.0, 1.0);
        }

        let (u, v) = (
            cross.x as f32 / cross.z as f32,
            cross.y as f32 / cross.z as f32,
        );

        return (u, v, 1.0 - u - v);
    }

    fn norm_to_image_coords(from: &glm::Vec3, width: i32, height: i32) -> glm::I32Vec2 {
        let mut out = glm::vec2(0, 0);
        out.x = ((from.x + 0.5) * width as f32) as i32;
        out.y = ((-from.y + 0.5) * height as f32) as i32;

        return out;
    }

    fn is_inside_triangle(
        point: glm::I32Vec2,
        p1: glm::I32Vec2,
        p2: glm::I32Vec2,
        p3: glm::I32Vec2,
    ) -> bool {
        let (u, v, w) = Self::to_baricentric_coords(point, p1, p2, p3);

        return u >= 0.0 && v >= 0.0 && w >= 0.0;
    }
}
