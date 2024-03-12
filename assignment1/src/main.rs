use assignment1::rasterizer::{Buffers, Primitive, Rasterizer};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{vec3, Mat4, Vec3};
use std::env;

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

const TITLE: &str = "Assignment 1";

fn main() {
    let mut angle = 0f32;
    let mut command_line = false;
    let mut filename = "output.png";
    let argv: Vec<String> = env::args().collect();
    if argv.len() >= 3 {
        command_line = true;
        angle = argv[2].parse().unwrap();
        if argv.len() == 4 {
            filename = &argv[3];
        }
    }

    let mut rasterizer = Rasterizer::new(WIDTH as u32, HEIGHT as u32);
    let eye_pos = vec3(0.0, 0.0, 5.0);
    let pos = [
        vec3(2.0, 0.0, -2.0),
        vec3(0.0, 2.0, -2.0),
        vec3(-2.0, 0.0, -2.0),
    ]
    .to_vec();
    let ind = [vec3(0.0, 1.0, 2.0)].to_vec();

    let pos_id = rasterizer.load_positions(pos);
    let ind_id = rasterizer.load_indices(ind);

    // render to file
    if command_line {
        rasterizer.clear(Buffers::all());
        rasterizer.set_model(get_model_matrix(angle));
        rasterizer.set_view(get_view_matrix(eye_pos));
        rasterizer.set_projection(get_projection_matrix(
            45.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            50.0,
        ));
        rasterizer.draw(pos_id, ind_id, Primitive::Triangle);
        rasterizer.save_framebuffer_to_png(filename).unwrap();
        return;
    }

    // render to window
    let mut window = Window::new(
        format!("{} - ESC to exit", TITLE).as_str(),
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    // let mut buffer = vec![0u32; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        rasterizer.clear(Buffers::all());
        rasterizer.set_model(get_model_matrix(angle));
        rasterizer.set_view(get_view_matrix(eye_pos));
        rasterizer.set_projection(get_projection_matrix(
            45.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            50.0,
        ));
        rasterizer.draw(pos_id, ind_id, Primitive::Triangle);
        let buffer = rasterizer
            .framebuffer()
            .iter()
            .map(|c| c.argb())
            .collect::<Vec<u32>>();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if window.is_key_down(Key::A) {
            angle += 0.1;
        } else if window.is_key_down(Key::D) {
            angle -= 0.1;
        }
    }
}

fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    let mut view = Mat4::identity();
    let translate = Mat4::new_translation(&-eye_pos);
    view = translate * view;
    view
}

fn get_model_matrix(rotation_angle: f32) -> Mat4 {
    let mut model = Mat4::identity();
    // TODO: Implement this function
    // Create the model matrix for rotating the triangle around the Z axis.
    // Then return it.
    todo!();
    model
}

fn get_projection_matrix(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    // TODO: Implement this function
    // Create the model matrix for rotating the triangle around the Z axis.
    // Then return it.
    todo!()
}
