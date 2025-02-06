use macroquad::color::Color;

use crate::vec3_64::Vec3_64;

const DEFAULT_ORB_COLOR: Color = Color::new(0.85, 0.86, 0.86, 1.);

//Orb Collision Point
const ORB_MIN_DIST: f64 = 0.0001;

#[derive(Debug)]
pub(crate) struct Orb {
    pos: Vec3_64,
    velocity: Vec3_64,
    radius: f32,
    mass: f64,
    gravity: f64,
    color: Color,
}

pub(crate) struct OrbInfo {
    pos: Vec3_64,
    mass: f64,
}

impl Orb {
    pub fn new(
        pos: Vec3_64,
        velocity: Vec3_64,
        radius: f32,
        mass: f64,
        gravity: f64,
        color: Option<Color>,
    ) -> Self {
        Self {
            pos,
            velocity,
            radius,
            mass,
            gravity,
            color: color.unwrap_or(DEFAULT_ORB_COLOR),
        }
    }

    pub fn info(&self) -> OrbInfo {
        OrbInfo {
            pos: self.pos,
            mass: self.mass,
        }
    }

    pub async fn animate(&mut self, other_orbs: (&OrbInfo, &OrbInfo), delta_time: f32) {
        let net_force =
            self.gravitational_force(other_orbs.0) + self.gravitational_force(other_orbs.1);

        // calculate acceleration (a = m / F | F = ma)
        let acceleration = net_force / self.mass;

        // update velocity (v = v0 + a * t)
        self.velocity += acceleration * delta_time as f64;

        // update position (p = p0 + v * t)
        self.pos += self.velocity * delta_time as f64;

        // draw newly positioned orb
        macroquad::models::draw_sphere(self.pos.into(), self.radius, None, self.color);
    }

    /// F = G * (m1 * m2) / r**2
    fn gravitational_force(&self, orb: &OrbInfo) -> Vec3_64 {
        let dist = orb.pos - self.pos;
        let dist_sq = dist.x * dist.x + dist.y * dist.y + dist.z * dist.z;

        // Avoid direct overlap
        if dist_sq < ORB_MIN_DIST {
            return Vec3_64::ZERO;
        }

        let force_mag = (self.gravity * self.mass * orb.mass) / dist_sq;
        // normalize distance and multiply by force magnitude
        (dist / dist_sq.sqrt()) * force_mag
    }
}
