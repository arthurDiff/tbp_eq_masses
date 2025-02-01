use macroquad::{color::Color, math::Vec3, texture::Texture2D};

#[derive(Debug)]
pub(crate) struct Orb {
    position: Vec3,
    radius: f32,
    direction: Vec3,
    acceleration: f32,
    mass: f32,
    texture: Option<Texture2D>,
    color: Color,
}

impl Orb {
    pub fn new(
        position: Vec3,
        radius: f32,
        direction: Vec3,
        acceleration: f32,
        mass: f32,
        texture: Option<Texture2D>,
        color: Color,
    ) -> Self {
        Self {
            position,
            radius,
            direction,
            acceleration,
            mass,
            texture,
            color,
        }
    }

    pub fn new_stationary(
        position: Vec3,
        radius: f32,
        texture: Option<Texture2D>,
        color: Color,
    ) -> Self {
        Self::new(
            position,
            radius,
            Vec3::new(0., 0., 0.),
            0.,
            0.,
            texture,
            color,
        )
    }

    pub fn animate(&self) {
        macroquad::models::draw_sphere(
            self.position,
            self.radius,
            self.texture.as_ref(),
            self.color,
        );
    }
}
