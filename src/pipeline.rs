use std::rc::Rc;

use self::rasterizer::RasterizerInput;
use self::geometry::GeometryInput;

extern crate nalgebra_glm as glm;

mod rasterizer;
mod geometry;

#[derive(Clone)]
pub struct Uniforms {
    pub time: f32,
    pub sun_light_dir: glm::Vec3,
    pub model_mat: glm::Mat4,
    pub gamma : f32,
}

pub struct Pipeline {
    pub rasterizer: rasterizer::Rasterizer,
    pub geometry: geometry::Geometry,
}

impl Pipeline {
    pub fn new(width: i32, height: i32) -> Pipeline{
        Pipeline {
            rasterizer: rasterizer::Rasterizer::new(width, height),
            geometry: geometry::Geometry {  }
        }
    }

    pub fn run(&mut self, positions: &Vec<glm::Vec3>, normals: &Vec<glm::Vec3>, tex_coords: &Vec<glm::Vec2>, textures: &Vec<Rc<image::DynamicImage>>,uniforms: Uniforms) {
        let geometry_input = GeometryInput {
            positions: positions.clone(),
            normals: normals.clone(),
            uv_coords: tex_coords.clone(),
            uniforms: uniforms.clone(),
        };

        let geometry_output = self.geometry.run(geometry_input);

        let input = RasterizerInput {
            positions: geometry_output.positions,
            normals: geometry_output.normals,
            uv_coords: geometry_output.uv_coords,
            uniforms: uniforms.clone(),
            textures: textures.clone(),
        };

        self.rasterizer.run(&input);
    }
}
