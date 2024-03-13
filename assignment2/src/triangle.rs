use crate::color::Color;
use nalgebra_glm::{Vec2, Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Triangle {
    /// the original coordinates of the triangle, v0, v1, v2 in
    // counterclockwise order
    pub v: Vec<Vec3>,
    /// color at each vertex
    pub color: Vec<Color>,
    /// texture u,v
    pub tex_coords: Vec<Vec2>,
    /// normal vector for each vertex
    pub normal: Vec<Vec3>,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            v: vec![Vec3::zeros(); 3],
            color: vec![Color::BLACK; 3],
            tex_coords: vec![Vec2::zeros(); 3],
            normal: vec![Vec3::zeros(); 3],
        }
    }
}

impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn a(&self) -> Vec3 {
        self.v[0]
    }

    pub fn b(&self) -> Vec3 {
        self.v[1]
    }
    pub fn c(&self) -> Vec3 {
        self.v[2]
    }

    pub fn set_vertex(&mut self, index: usize, vertex: Vec3) {
        self.v[index] = vertex;
    }

    pub fn set_normal(&mut self, index: usize, normal: Vec3) {
        self.normal[index] = normal;
    }

    pub fn set_tex_coords(&mut self, index: usize, s: f32, t: f32) {
        self.tex_coords[index] = Vec2::new(s, t);
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        self.color[index] = color;
    }

    pub fn color(&self) -> Color {
        self.color[0]
    }

    pub fn color_by_barycentric(&self, alpha: f32, beta: f32, gamma: f32) -> Color {
        self.color[0] * alpha + self.color[1] * beta + self.color[2] * gamma
    }

    pub fn to_vector4(&self) -> [Vec4; 3] {
        [
            Vec4::new(self.v[0].x, self.v[0].y, self.v[0].z, 1.0),
            Vec4::new(self.v[1].x, self.v[1].y, self.v[1].z, 1.0),
            Vec4::new(self.v[2].x, self.v[2].y, self.v[2].z, 1.0),
        ]
    }
}
