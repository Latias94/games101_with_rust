use crate::triangle::Triangle;
use bitflags::bitflags;
use nalgebra_glm::{vec4, Mat4, Vec3, Vec4};
use std::collections::HashMap;

pub struct Rasterizer {
    width: u32,
    height: u32,
    frame_buf: Vec<u32>, // for convenience, we use Vec4 instead of Vec3
    depth_buf: Vec<f32>,
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    pos_buf: HashMap<u32, Vec<Vec3>>,
    ind_buf: HashMap<u32, Vec<Vec3>>,
    next_id: u32,
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
pub struct PosBufId {
    pos_id: u32,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct IndBufId {
    ind_id: u32,
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        let frame_buf = vec![0u32; (width * height) as usize];
        let depth_buf = vec![f32::MAX; (width * height) as usize];
        let model = Mat4::identity();
        let view = Mat4::identity();
        let projection = Mat4::identity();
        let pos_buf = HashMap::new();
        let ind_buf = HashMap::new();
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
            next_id,
        }
    }

    pub fn load_positions(&mut self, positions: Vec<Vec3>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions);
        PosBufId { pos_id: id }
    }

    pub fn load_indices(&mut self, indices: Vec<Vec3>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices);
        IndBufId { ind_id: id }
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

    fn color_to_u32(color: Vec3) -> u32 {
        let r = (color.x) as u32;
        let g = (color.y) as u32;
        let b = (color.z) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Vec3, depth: f32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if depth < self.depth_buf[index] {
                self.frame_buf[index] = Self::color_to_u32(color);
                self.depth_buf[index] = depth;
            }
        }
    }

    pub fn clear(&mut self, buffers: Buffers) {
        if buffers.contains(Buffers::COLOR) {
            self.frame_buf.fill(0);
        }
        if buffers.contains(Buffers::DEPTH) {
            self.depth_buf.fill(f32::MAX);
        }
    }

    fn vec3_to_vec4(v: Vec3) -> Vec4 {
        vec4(v.x, v.y, v.z, 1.0)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, primitive: Primitive) {
        let pos = self.pos_buf.get(&pos_buffer.pos_id).unwrap().clone();
        let ind = self.ind_buf.get(&ind_buffer.ind_id).unwrap().clone();
        let f1 = (100.0 - 0.1) / 2.0;
        let f2 = (100.0 + 0.1) / 2.0;

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
                    t.set_color(0, 255f32, 0f32, 0f32);
                    t.set_color(1, 0f32, 255f32, 0f32);
                    t.set_color(2, 0f32, 0f32, 255f32);
                    self.rasterize_wireframe(&t);
                }
            }
            _ => {
                eprintln!("Drawing primitives other than triangle is not implemented yet");
            }
        }
    }

    pub fn framebuffer(&self) -> &[u32] {
        &self.frame_buf
    }

    // bresenhams line algorithm
    fn draw_line(&mut self, begin: Vec3, end: Vec3, color: Vec3) {
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

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        let v0 = t.v[0];
        let v1 = t.v[1];
        let v2 = t.v[2];
        self.draw_line(v0, v1, t.color[0]);
        self.draw_line(v1, v2, t.color[1]);
        self.draw_line(v2, v0, t.color[2]);
    }

    #[allow(dead_code)]
    fn get_index(&self, x: u32, y: u32) -> u32 {
        (self.height - y) * self.width + x
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
