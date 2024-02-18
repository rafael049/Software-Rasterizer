use std::ops::Mul;

use nalgebra_glm::proj;

use super::Uniforms;


pub struct Geometry {
}

pub struct GeometryInput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec2>,
    pub uniforms: Uniforms,
}

pub struct GeometryOutput {
    pub positions: Vec<glm::Vec3>,
    pub normals: Vec<glm::Vec3>,
    pub uv_coords: Vec<glm::Vec2>,
    pub frag_pos: Vec<glm::Vec3>,
}

impl Geometry {

    pub fn run(&mut self, input: GeometryInput) -> GeometryOutput{
        self.vertex_shader(input)
    }

    pub fn vertex_shader(&mut self, mut input: GeometryInput) -> GeometryOutput{

        let r = 2.0;
        let x = r*(input.uniforms.time / 1000.0).cos();
        let y = 0.0;
        let z = r*(input.uniforms.time / 1000.0).sin();

        let view = glm::look_at(&glm::vec3(x, y, z), &glm::vec3(0.0, 0.0, 0.0), &glm::vec3(0.0, 1.0, 0.0));

        // TODO: why does the far and near planes must be negatives?
        let projection = glm::perspective_zo(1280.0/720.0, 90.0f32.to_radians(), -0.1, -10.0);
        let model = input.uniforms.model_mat;

        let mut frags_pos = vec![];

        for position in &mut input.positions {

            let position_xyzw = projection * view * model * glm::vec4(position.x, position.y, position.z, 1.0);

            *position = position_xyzw.xyz() / position_xyzw.w;

            let frag_pos_xyzw = model * glm::vec4(position.x, position.y, position.z, 1.0);
            let frag_pos = frag_pos_xyzw.xyz() / frag_pos_xyzw.w;

            frags_pos.push(frag_pos);
        }

        return GeometryOutput {
            positions: input.positions,
            normals: input.normals,
            uv_coords: input.uv_coords,
            frag_pos:  frags_pos,
        };
    }
}