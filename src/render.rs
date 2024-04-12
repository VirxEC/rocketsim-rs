use crate::math::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0., 0.);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl Color {
    pub const BLACK: Self = Self::rgb(0., 0., 0.);
    pub const WHITE: Self = Self::rgb(1., 1., 1.);
    pub const RED: Self = Self::rgb(1., 0., 0.);
    pub const GREEN: Self = Self::rgb(0., 1., 0.);
    pub const BLUE: Self = Self::rgb(0., 0., 1.);

    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1. }
    }

    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug)]
pub enum Render {
    Line2D { start: Vec2, end: Vec2, color: Color },
    Line { start: Vec3, end: Vec3, color: Color },
    LineStrip { positions: Vec<Vec3>, color: Color },
}

#[derive(Clone, Debug)]
pub enum RenderMessage {
    AddRender(i32, Vec<Render>),
    RemoveRender(i32),
}
