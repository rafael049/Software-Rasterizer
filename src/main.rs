use std::fs::File;
use std::io::BufReader;

use framebuffer::Framebuffer;
use obj::{self, load_obj, Obj, Vertex};

extern crate nalgebra_glm as glm;
extern crate sdl2;

mod framebuffer;
mod pipeline;
mod vertex;

fn norm_to_image_coords(image: &Framebuffer, from: &glm::Vec3) -> glm::I32Vec2 {
    let mut out = glm::vec2(0, 0);
    out.x = ((from.x + 0.5) * image.width as f32) as i32;
    out.y = ((-from.y + 0.5) * image.height as f32) as i32;

    return out;
}

fn array3_to_vec3(from: &[f32; 3]) -> glm::Vec3 {
    return glm::vec3(from[0], from[1], from[2]);
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

fn is_inside_triangle(
    point: glm::I32Vec2,
    p1: glm::I32Vec2,
    p2: glm::I32Vec2,
    p3: glm::I32Vec2,
) -> bool {
    let (u, v, w) = to_baricentric_coords(point, p1, p2, p3);

    return u >= 0.0 && v >= 0.0 && w >= 0.0;
}

fn triangle(
    image: &mut Framebuffer,
    norm_p1: glm::Vec3,
    norm_p2: glm::Vec3,
    norm_p3: glm::Vec3,
    color: glm::Vec4,
) {
    let p1 = norm_to_image_coords(&image, &norm_p1);
    let p2 = norm_to_image_coords(&image, &norm_p2);
    let p3 = norm_to_image_coords(&image, &norm_p3);

    image.line(p1, p2, color);
    image.line(p2, p3, color);
    image.line(p3, p1, color);
}

pub fn fill_triangle(
    image: &mut Framebuffer,
    norm_p1: vertex::Vertex,
    norm_p2: vertex::Vertex,
    norm_p3: vertex::Vertex,
    color: glm::Vec4,
) {
    let p1 = norm_to_image_coords(&image, &norm_p1.position);
    let p2 = norm_to_image_coords(&image, &norm_p2.position);
    let p3 = norm_to_image_coords(&image, &norm_p3.position);
    let min_x = p1.x.min(p2.x).min(p3.x).max(0);
    let max_x = p1.x.max(p2.x).max(p3.x).min(image.width);
    let min_y = p1.y.min(p2.y).min(p3.y).max(0);
    let max_y = p1.y.max(p2.y).max(p3.y).min(image.height);

    for x in min_x..max_x {
        for y in min_y..max_y {
            if is_inside_triangle(glm::vec2(x, y), p1, p2, p3) {
                let (u, v, w) = to_baricentric_coords(glm::vec2(x, y), p1, p2, p3);
                let depth = -(u * (norm_p1.position.z)
                    + v * (norm_p2.position.z)
                    + w * (norm_p3.position.z))
                    / 2.0
                    + 0.5;
                if depth < image.get_depth(x, y) {
                    image.set(x, y, color);
                    image.set_depth(x, y, depth)
                }
            }
        }
    }
}
fn main() {
    println!("Renderer started!");

    let (width, height) = (1280, 720);

    println!("Resolution: {}x{}", width, height);

    let filename = "teste.obj";

    println!("Reading file {filename}.");
    let input = BufReader::new(File::open(filename).unwrap());
    let mesh: Obj<Vertex, i32> = obj::load_obj(input).unwrap();

    println!("File {filename} succesfuly read.");

    let vertices = mesh.vertices;
    let indices = mesh.indices;

    let mut positions = vec![];
    let mut normals = vec![];

    println!("Getting vertex data from obj.");
    for i in (0..indices.len()).step_by(3) {
        positions.push(array3_to_vec3(&vertices[indices[i] as usize].position));
        positions.push(array3_to_vec3(&vertices[indices[i + 1] as usize].position));
        positions.push(array3_to_vec3(&vertices[indices[i + 2] as usize].position));

        normals.push(array3_to_vec3(&vertices[indices[i] as usize].normal));
        normals.push(array3_to_vec3(&vertices[indices[i + 1] as usize].normal));
        normals.push(array3_to_vec3(&vertices[indices[i + 2] as usize].normal));
    }

    let mut rasterizer = pipeline::Pipeline::new(width, height);

    println!("Render pass started.");
    rasterizer.run(&positions, &normals);
    println!("Render pass ended.");

    println!("Writing output file.");
    rasterizer.rasterizer.framebuffer.write_to_file("test.png");
    println!("Output put file written.");
}
