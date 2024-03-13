use assignment2::color::Color;
use assignment2::triangle::Triangle;
use bitflags::bitflags;
use nalgebra_glm::{vec4, Mat4, UVec2, Vec3, Vec4};
use std::collections::HashMap;

pub struct Rasterizer {
    width: u32,
    height: u32,
    frame_buf: Vec<Color>,
    /// 都是正数，并且越大表示离视点越远
    depth_buf: Vec<f32>,
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    pos_buf: HashMap<u32, Vec<Vec3>>,
    ind_buf: HashMap<u32, Vec<Vec3>>,
    col_buf: HashMap<u32, Vec<Color>>,
    next_id: u32,
    clear_color: Color,
}

pub enum Primitive {
    Line,
    Triangle,
}

bitflags! {
    pub struct Buffers: u32 {
        const COLOR = 0b00000001;
        const DEPTH = 0b00000010;
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct PosBufId(u32);

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct IndBufId(u32);

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct ColBufId(u32);

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        let frame_buf = vec![Color::BLACK; (width * height) as usize];
        let depth_buf = vec![f32::MAX; (width * height) as usize];
        let model = Mat4::identity();
        let view = Mat4::identity();
        let projection = Mat4::identity();
        let pos_buf = HashMap::new();
        let ind_buf = HashMap::new();
        let col_buf = HashMap::new();
        let next_id = 0;
        Self {
            width,
            height,
            frame_buf,
            depth_buf,
            model,
            view,
            projection,
            pos_buf,
            ind_buf,
            col_buf,
            next_id,
            clear_color: Color::BLACK,
        }
    }

    pub fn load_positions(&mut self, positions: Vec<Vec3>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: Vec<Vec3>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: Vec<Color>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors);
        ColBufId(id)
    }

    fn inside_triangle(&self, x: u32, y: u32, v: [Vec4; 3]) -> bool {
        // check if the point (x, y) is inside the triangle represented by _v[0], _v[1], _v[2]
        let (alpha, beta, gamma) = self.compute_barycentric2d(x, y, v);
        alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0
    }

    /// 计算点相对于三角形顶点的重心坐标。
    ///
    /// # 参数
    ///
    /// * `x` - 点的 x 坐标。
    /// * `y` - 点的 y 坐标。
    /// * `v` - 三角形顶点的 `Vec4` 数组，表示三个顶点。
    ///
    /// # 返回值
    ///
    /// 返回一个元组 `(alpha, beta, gamma)`，代表点的重心坐标。
    fn compute_barycentric2d(&self, x: u32, y: u32, v: [Vec4; 3]) -> (f32, f32, f32) {
        let x = x as f32;
        let y = y as f32;

        let x0 = v[0].x;
        let y0 = v[0].y;
        let x1 = v[1].x;
        let y1 = v[1].y;
        let x2 = v[2].x;
        let y2 = v[2].y;

        // 使用三角形的顶点坐标和点 (x, y) 的坐标来计算重心坐标 alpha, beta, gamma。
        // alpha = ((y1 - y2) * (x - x2) + (x2 - x1) * (y - y2)) / 分母
        // beta = ((y2 - y0) * (x - x2) + (x0 - x2) * (y - y2)) / 分母
        // gamma = 1 - alpha - beta
        // 其中，分母 = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2)，为常数项，用于归一化。

        let denom = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2);
        let alpha = ((y1 - y2) * (x - x2) + (x2 - x1) * (y - y2)) / denom;
        let beta = ((y2 - y0) * (x - x2) + (x0 - x2) * (y - y2)) / denom;
        let gamma = 1.0 - alpha - beta;

        (alpha, beta, gamma)
    }

    pub fn set_model(&mut self, model: Mat4) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color, depth: f32) {
        if x < self.width && y < self.height {
            let index = self.get_index(x, y);
            if depth < self.depth_buf[index] {
                self.frame_buf[index] = color;
                self.depth_buf[index] = depth;
            }
        }
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(self.clear_color);
        }
        if buffers.contains(Buffers::DEPTH) {
            self.depth_buf.fill(f32::MAX);
        }
    }

    fn vec3_to_vec4(v: Vec3) -> Vec4 {
        vec4(v.x, v.y, v.z, 1.0)
    }

    pub fn draw(
        &mut self,
        pos_buffer: PosBufId,
        ind_buffer: IndBufId,
        col_buffer: ColBufId,
        primitive: Primitive,
    ) {
        let pos = self.pos_buf.get(&pos_buffer.0).unwrap().clone();
        let ind = self.ind_buf.get(&ind_buffer.0).unwrap().clone();
        let col = self.col_buf.get(&col_buffer.0).unwrap().clone();

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;
        let width = self.width as f32;
        let height = self.height as f32;
        match primitive {
            Primitive::Triangle => {
                for i in ind {
                    let mut t = Triangle::new();
                    let mut v = [
                        mvp * Self::vec3_to_vec4(pos[i[0] as usize]),
                        mvp * Self::vec3_to_vec4(pos[i[1] as usize]),
                        mvp * Self::vec3_to_vec4(pos[i[2] as usize]),
                    ];
                    for vec in &mut v {
                        vec.x /= vec.w;
                        vec.y /= vec.w;
                        vec.z /= vec.w;
                    }

                    for vert in &mut v {
                        vert.x = (vert.x + 1.0) * 0.5 * width;
                        vert.y = (vert.y + 1.0) * 0.5 * height;
                        vert.z = vert.z * f1 + f2;
                    }
                    for (i, vertex) in v.iter().enumerate() {
                        t.set_vertex(i, Vec3::new(vertex.x, vertex.y, vertex.z));
                    }

                    let col_x = col[i[0] as usize];
                    let col_y = col[i[1] as usize];
                    let col_z = col[i[2] as usize];

                    t.set_color(0, col_x);
                    t.set_color(1, col_y);
                    t.set_color(2, col_z);

                    self.rasterize_triangle(&t);
                }
            }
            _ => {
                eprintln!("Drawing primitives other than triangle is not implemented yet");
            }
        }
    }

    pub fn framebuffer(&self) -> &[Color] {
        &self.frame_buf
    }

    pub fn framebuffer_u32(&self) -> &[u32] {
        bytemuck::cast_slice(&self.frame_buf)
    }

    pub fn framebuffer_u8(&self) -> &[u8] {
        bytemuck::cast_slice(&self.frame_buf)
    }

    /// bresenhams line algorithm
    pub fn draw_line(&mut self, begin: Vec3, end: Vec3, color: Color) {
        let mut x0 = begin.x as i32;
        let mut y0 = begin.y as i32;
        let mut x1 = end.x as i32;
        let mut y1 = end.y as i32;
        let mut steep = false;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
            steep = true;
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;
        for x in x0..x1 {
            if steep {
                self.set_pixel(y as u32, x as u32, color, 0.0);
            } else {
                self.set_pixel(x as u32, y as u32, color, 0.0);
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }

    fn rasterize_triangle(&mut self, t: &Triangle) {
        let v = t.to_vector4();
        // Find out the bounding box of current triangle.
        // iterate through the pixel and find if the current pixel is inside the triangle
        let bbox_min = UVec2::new(
            v.iter()
                .map(|vertex| vertex.x)
                .reduce(f32::min)
                .unwrap()
                .floor() as u32,
            v.iter()
                .map(|vertex| vertex.y)
                .reduce(f32::min)
                .unwrap()
                .floor() as u32,
        );
        let bbox_max = UVec2::new(
            v.iter()
                .map(|vertex| vertex.x)
                .reduce(f32::max)
                .unwrap()
                .ceil() as u32,
            v.iter()
                .map(|vertex| vertex.y)
                .reduce(f32::max)
                .unwrap()
                .ceil() as u32,
        );
        for x in bbox_min.x..=bbox_max.x {
            for y in bbox_min.y..=bbox_max.y {
                let inside = self.inside_triangle(x, y, v);
                if inside {
                    // to get the interpolated z value
                    let (alpha, beta, gamma) = self.compute_barycentric2d(x, y, v);
                    let w_reciprocal = 1.0 / (alpha / v[0].w + beta / v[1].w + gamma / v[2].w);
                    let z_interpolated =
                        alpha * v[0].z / v[0].w + beta * v[1].z / v[1].w + gamma * v[2].z / v[2].w;
                    let z_interpolated = z_interpolated * w_reciprocal;

                    // set the current pixel (use the set_pixel function) to the color of the triangle (use getColor function) if it should be painted.
                    let color = t.color();
                    self.set_pixel(x, y, color, z_interpolated);
                }
            }
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        // flip y
        ((self.height - 1 - y) * self.width + x) as usize
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn save_framebuffer_to_png(&self, file_path: &str) -> image::ImageResult<()> {
        let width = self.width;
        let height = self.height;

        let buffer = self.framebuffer_u8();
        image::save_buffer(
            file_path,
            buffer,
            width,
            height,
            image::ExtendedColorType::Rgba8,
        )?;
        Ok(())
    }
}
