extern crate nalgebra_glm as glm;

use crate::Framebuffer;

pub struct Rasterizer {
    pub width: i32,
    pub height: i32,
    screen_positions: Vec<glm::I32Vec2>,
    pub framebuffer: Framebuffer,
}

pub struct RasterizerInput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec3>,
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
        for position in &input.positions {
            let image_coords = Self::norm_to_image_coords(&position, self.width, self.height);

            self.screen_positions.push(image_coords);
        }
    }

    pub fn rasterize(&mut self, input: &RasterizerInput) {
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
                        if depth < self.framebuffer.get_depth(x, y) {
                            self.framebuffer.set(x, y, glm::vec4(normal.x, normal.y, normal.z, 1.0));
                            self.framebuffer.set_depth(x, y, depth)
                        }
                    }
                }
            }
        }
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
