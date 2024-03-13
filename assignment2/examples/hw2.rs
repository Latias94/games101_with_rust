use assignment2::color::Color;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{vec3, Mat4, TVec3, Vec3};
use rasterizer::{Buffers, ColBufId, IndBufId, PosBufId, Primitive, Rasterizer};
use std::env;

mod rasterizer;

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
        vec3(3.5, -1.0, -5.0),
        vec3(2.5, 1.5, -5.0),
        vec3(-1.0, 0.5, -5.0),
    ]
    .to_vec();

    let ind = [vec3(0.0, 1.0, 2.0), vec3(3.0, 4.0, 5.0)].to_vec();

    let cols = [
        Color::new_rgb(217, 238, 185),
        Color::new_rgb(217, 238, 185),
        Color::new_rgb(217, 238, 185),
        Color::new_rgb(185, 217, 238),
        Color::new_rgb(185, 217, 238),
        Color::new_rgb(185, 217, 238),
    ]
    .to_vec();

    let pos_id = rasterizer.load_positions(pos);
    let ind_id = rasterizer.load_indices(ind);
    let col_id = rasterizer.load_colors(cols);

    // render to file
    if command_line {
        draw(&mut rasterizer, angle, eye_pos, pos_id, ind_id, col_id);
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
        draw(&mut rasterizer, angle, eye_pos, pos_id, ind_id, col_id);
        let buffer = rasterizer
            .framebuffer()
            .iter()
            .map(|c| c.argb())
            .collect::<Vec<u32>>();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if window.is_key_down(Key::A) {
            angle += 0.5;
        } else if window.is_key_down(Key::D) {
            angle -= 0.5;
        }
    }
}

fn draw(
    rasterizer: &mut Rasterizer,
    angle: f32,
    eye_pos: TVec3<f32>,
    pos_id: PosBufId,
    ind_id: IndBufId,
    col_id: ColBufId,
) {
    rasterizer.clear(Buffers::all());
    rasterizer.set_model(get_model_matrix(angle));
    rasterizer.set_view(get_view_matrix(eye_pos));
    rasterizer.set_projection(get_projection_matrix(
        45.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        50.0,
    ));
    // test (0, 0)
    // rasterizer.draw_line(vec3(0.0, 0.0, 0.0), vec3(100.0, 0.0, 0.0), Color::GREEN);
    rasterizer.draw(pos_id, ind_id, col_id, Primitive::Triangle);
}

fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    let mut view = Mat4::identity();
    view[(0, 3)] = -eye_pos.x;
    view[(1, 3)] = -eye_pos.y;
    view[(2, 3)] = -eye_pos.z;

    // let translate = Mat4::new_translation(&-eye_pos);
    // let view = translate * view;
    view
}

#[allow(dead_code)]
fn get_model_matrix(rotation_angle: f32) -> Mat4 {
    // Rz(θ) = | cosθ -sinθ  0 0 |
    //         | sinθ  cosθ  0 0 |
    //         |  0      0   1 0 |
    //         |  0      0   0 1 |
    let radian = rotation_angle.to_radians();
    let c = radian.cos();
    let s = radian.sin();
    #[rustfmt::skip]
        let model = Mat4::new(
        c,    -s,    0.0, 0.0,
        s,     c,    0.0, 0.0,
        0.0,   0.0,  1.0, 0.0,
        0.0,   0.0,  0.0, 1.0,
    );

    // let rotation_axis = Vec3::z();
    // let model = nalgebra_glm::rotate(&model, rotation_angle, &rotation_axis);
    model
}

fn get_projection_matrix(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let fov_half = eye_fov / 2.0;
    let tan_half_fov = (fov_half.to_radians()).tan();

    let a = 1.0 / (tan_half_fov * aspect_ratio); // 影响x轴上的缩放
    let b = 1.0 / tan_half_fov; // 影响y轴上的缩放
    let c = -(z_far + z_near) / (z_far - z_near); // 计算z轴上的深度缩放和位移
    let d = -2.0 * z_far * z_near / (z_far - z_near); // 远近裁剪面之间的关系

    #[rustfmt::skip]
        let projection= Mat4::new(
        a,    0.0,  0.0,  0.0,
        0.0,  b,    0.0,  0.0,
        0.0,  0.0,  c,    d,
        0.0,  0.0, -1.0,  0.0,
    );
    // let projection = Mat4::new_perspective(aspect_ratio, eye_fov, z_near, z_far);
    projection
}

/// 根据旋转轴和旋转角度计算旋转矩阵
///
/// # 参数
///
/// * `axis` - 旋转轴，一个三维向量
/// * `angle` - 旋转角度，以弧度为单位
///
/// # 返回值
///
/// 返回绕给定轴旋转指定角度的4x4旋转矩阵
///
/// # 罗德里格斯旋转公式
///
/// 给定单位旋转轴 v = (v_x, v_y, v_z) 和旋转角度 θ，
/// 旋转矩阵 R 可以通过罗德里格斯公式计算得到：
///
/// R = I + sin(θ)K + (1 - cos(θ))K^2
///
/// 其中，I 是单位矩阵，K 是根据旋转轴 v 构造的斜对称矩阵：
///
/// K = |  0   -v_z  v_y |
///     | v_z   0   -v_x |
///     |-v_y  v_x   0   |
///
/// 注意：此函数假设旋转轴通过原点。
fn get_model_matrix_by_any_axis(angle: f32, axis: Vec3) -> Mat4 {
    let norm_axis = nalgebra_glm::normalize(&axis);
    let radian = angle.to_radians();
    let sin_angle = radian.sin();
    let cos_angle = radian.cos();
    let one_minus_cos = 1.0 - cos_angle;

    let x = norm_axis.x;
    let y = norm_axis.y;
    let z = norm_axis.z;

    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;

    let xs = x * sin_angle;
    let ys = y * sin_angle;
    let zs = z * sin_angle;

    let omc_xx = xx * one_minus_cos;
    let omc_yy = yy * one_minus_cos;
    let omc_zz = zz * one_minus_cos;
    let omc_xy = xy * one_minus_cos;
    let omc_xz = xz * one_minus_cos;
    let omc_yz = yz * one_minus_cos;

    #[rustfmt::skip]
        let model= Mat4::new(
        omc_xx + cos_angle, omc_xy - zs,         omc_xz + ys,         0.0,
        omc_xy + zs,        omc_yy + cos_angle,  omc_yz - xs,         0.0,
        omc_xz - ys,        omc_yz + xs,         omc_zz + cos_angle,  0.0,
        0.0,                0.0,                 0.0,                 1.0,
    );
    model
}
