use std::ops::Mul;

use nalgebra_glm::proj;


pub struct Geometry {
}

pub struct GeometryInput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec3>,
}

pub struct GeometryOutput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec3>,
}

impl Geometry {

    pub fn run(&mut self, input: GeometryInput) -> GeometryOutput{
        self.vertex_shader(input)
    }

    pub fn vertex_shader(&mut self, mut input: GeometryInput) -> GeometryOutput{

        let view = glm::look_at(&glm::vec3(0.0, 0.0, -1.0), &glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 1.0, 0.0));

        let projection = glm::perspective(1280.0/720.0, 45.0, 0.0, 10.0);

        for position in &mut input.positions {

            let ex_position = glm::vec4(position.x, position.y, position.z, 1.0);

            *position = (projection * view * ex_position).xyz();
        }

        return GeometryOutput {
            positions: input.positions,
            normals: input.normals,
            uv_coords: input.uv_coords,
        };
    }
}