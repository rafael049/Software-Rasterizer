use std::fs::File;
use std::io::BufReader;

use image::Image;
use obj::{self, Obj, Vertex, load_obj};

extern crate nalgebra_glm as glm;

mod image;

fn norm_to_image_coords(image: &Image, from: &glm::Vec3) -> glm::I32Vec2 {
    let mut out = glm::vec2(0, 0);
    out.x = ((from.x + 0.5) * image.width as f32) as i32;
    out.y = ((-from.y + 0.5) * image.height as f32) as i32;

    return out;
}

fn array3_to_vec3(from: &[f32;3]) -> glm::Vec3 {
    return glm::vec3(from[0], from[1], from[2]);
}

fn main() {
    let (width, height) = (1280, 720);

    let mut image = Image::new(width, height);

    let input = BufReader::new(File::open("teste.obj").unwrap());
    let mesh: Obj<Vertex, i32> = obj::load_obj(input).unwrap();

    let vertices = mesh.vertices;
    let indices = mesh.indices;

    for i in (0..indices.len()).step_by(3) {
        let a = array3_to_vec3(&vertices[indices[i] as usize].position);
        let b = array3_to_vec3(&vertices[indices[i+1] as usize].position);
        let c = array3_to_vec3(&vertices[indices[i+2] as usize].position);

        let norm = array3_to_vec3(&vertices[indices[i] as usize].normal);
        let light_dir = glm::vec3(-1.0, -1.0, 1.0).normalize();

        let attenuation = norm.dot(&light_dir).max(0.0) + 0.5;
        let difuse = glm::vec3(0.3, 0.2, 0.7);
        let final_color = difuse * attenuation;

        image.fill_triangle(
            norm_to_image_coords(&image, &a),
            norm_to_image_coords(&image, &b),
            norm_to_image_coords(&image, &c),
            glm::vec4(final_color.x, final_color.y, final_color.z, 1.0));
    }

    image.write_to_file("test.png");
}
