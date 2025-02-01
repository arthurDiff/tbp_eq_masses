use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    color::{self, Color},
    math::vec3,
    models::draw_grid,
    window::{clear_background, next_frame, Conf},
};
use result_error::*;

mod orb;
mod result_error;

const BLACK_BACKGROUND: Color = Color::new(0.05, 0.05, 0.05, 1.);

fn window_config() -> Conf {
    Conf {
        window_title: "tbp_eq_masses".to_owned(),
        ..Default::default()
    }
}
#[macroquad::main(window_config)]
async fn main() -> Result<()> {
    let orb = orb::Orb::new_stationary(vec3(-0., 0., 0.), 1., None, color::BLUE);
    loop {
        clear_background(BLACK_BACKGROUND);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., color::BLACK, color::WHITE);

        orb.animate();

        set_default_camera();

        next_frame().await
    }
}
