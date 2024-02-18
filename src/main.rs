use std::fs::File;
use std::io::BufReader;

use std::io::Cursor;
use image::io::Reader as ImageReader;

use framebuffer::Framebuffer;
use obj::TexturedVertex;
use obj::{self, load_obj, Obj, Vertex};
use sdl2::{pixels::Color, render::TextureCreator, video, event::Event, keyboard::Keycode};

use crate::pipeline::Uniforms;

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

fn array2_to_vec2(from: &[f32; 2]) -> glm::Vec2 {
    return glm::vec2(from[0], from[1]);
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

fn main() {
    println!("Renderer started!");

    let (width, height) = (1920i32, 1080i32);

    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem
        .window("Rasterizer", width as u32, height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut texture =
        texture_creator.create_texture(None, sdl2::render::TextureAccess::Streaming, width as u32, height as u32).unwrap();

    println!("Resolution: {}x{}", width, height);

    let filename = "teste.obj";

    println!("Reading file {filename}.");
    let input = BufReader::new(File::open(filename).unwrap());
    let mesh: Obj<TexturedVertex, i32> = obj::load_obj(input).unwrap();

    println!("File {filename} succesfuly read.");

    let vertices = mesh.vertices;
    let indices = mesh.indices;

    let mut positions = vec![];
    let mut normals = vec![];
    let mut tex_coords = vec![];

    println!("Getting vertex data from obj.");
    for i in (0..indices.len()).step_by(3) {
        positions.push(array3_to_vec3(&vertices[indices[i] as usize].position));
        positions.push(array3_to_vec3(&vertices[indices[i + 1] as usize].position));
        positions.push(array3_to_vec3(&vertices[indices[i + 2] as usize].position));

        normals.push(array3_to_vec3(&vertices[indices[i] as usize].normal));
        normals.push(array3_to_vec3(&vertices[indices[i + 1] as usize].normal));
        normals.push(array3_to_vec3(&vertices[indices[i + 2] as usize].normal));

        tex_coords.push(array3_to_vec3(&vertices[indices[i] as usize].texture).xy());
        tex_coords.push(array3_to_vec3(&vertices[indices[i+1] as usize].texture).xy());
        tex_coords.push(array3_to_vec3(&vertices[indices[i+2] as usize].texture).xy());
    }

    println!("Loading Textures");

    let img = ImageReader::open("Suzanne.png").unwrap();

    let texture1 = std::rc::Rc::new(img.decode().unwrap());

    let textures = vec![texture1];

    let mut rasterizer = pipeline::Pipeline::new(width, height);

    let mut event_pump = sdl2_context.event_pump().unwrap();

    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let uniforms = Uniforms {
            time: sdl2_context.timer().unwrap().ticks() as f32,
            sun_light_dir: glm::vec3(1.0, -1.0, -1.0).normalize(),
            model_mat: glm::identity(),
            gamma: 2.2,
        };

        rasterizer.run(&positions, &normals, &tex_coords, &textures, uniforms);

        texture.update(None, &rasterizer.rasterizer.framebuffer.pixels, (width*4i32) as usize).unwrap();

        canvas.set_draw_color(Color::RGB(25, 25, 128));
        canvas.clear();

        canvas.copy(&texture, None, None).unwrap();
        
        canvas.present();
    }

    //println!("Writing output file.");
    //rasterizer.rasterizer.framebuffer.write_to_file("test.png");
    //println!("Output put file written.");
}
