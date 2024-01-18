use self::rasterizer::RasterizerInput;
use self::geometry::GeometryInput;
use self::geometry::GeometryOutput;

extern crate nalgebra_glm as glm;

mod rasterizer;
mod geometry;

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

    pub fn run(&mut self, positions: &Vec<glm::Vec3>, normals: &Vec<glm::Vec3>) {
        let geometry_input = GeometryInput {
            positions: positions.clone(),
            normals: normals.clone(),
            uv_coords: vec![],
        };

        let geometry_output = self.geometry.run(geometry_input);

        let input = RasterizerInput {
            positions: geometry_output.positions,
            normals: geometry_output.normals,
            uv_coords: vec![],
        };

        self.rasterizer.run(&input);
    }
}
