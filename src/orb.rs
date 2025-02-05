use macroquad::{color::Color, math::Vec3, texture::Texture2D};

const DEFAULT_ORB_COLOR: Color = Color::new(0.85, 0.86, 0.86, 1.);

#[derive(Debug)]
pub(crate) struct Orb {
    position: Vec3,
    radius: f32,
    mass: f32,
    direction: Vec3,
    acceleration: f32,
    texture: Option<Texture2D>,
    color: Color,
}

#[allow(dead_code)]
pub(crate) struct OrbInfo {
    position: Vec3,
    mass: f32,
}

impl Orb {
    pub fn new(
        position: Vec3,
        radius: f32,
        mass: f32,
        direction: Vec3,
        acceleration: f32,
        texture: Option<Texture2D>,
        color: Color,
    ) -> Self {
        Self {
            position,
            radius,
            mass,
            direction,
            acceleration,
            texture,
            color,
        }
    }

    pub fn new_stationary(
        position: Vec3,
        radius: f32,
        mass: f32,
        texture: Option<Texture2D>,
        color: Option<Color>,
    ) -> Self {
        Self::new(
            position,
            radius,
            mass,
            Vec3::new(0., 0., 0.),
            0.,
            texture,
            color.unwrap_or(DEFAULT_ORB_COLOR),
        )
    }

    pub fn info(&self) -> OrbInfo {
        OrbInfo {
            position: self.position,
            mass: self.mass,
        }
    }

    #[allow(unused_variables)]
    pub async fn animate(&mut self, other_orbs: (&OrbInfo, &OrbInfo)) {
        self.move_orb();
        self.draw();
    }

    fn move_orb(&mut self) {
        self.position += self.direction.x * self.acceleration;
    }

    fn draw(&self) {
        macroquad::models::draw_sphere(
            self.position,
            self.radius,
            self.texture.as_ref(),
            self.color,
        );
    }
}
