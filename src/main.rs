use macroquad::{
    color::{self, Color},
    math::vec3,
    models::draw_grid,
    window::{clear_background, next_frame, Conf},
};
use result_error::*;

mod camera;
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
    let mut cam = camera::Camera::new(vec3(-20., 15., 0.), vec3(0., 1., 0.), vec3(0., 1., 0.));
    let mut orb = orb::Orb::new_stationary(vec3(-0., 0., 0.), 1., None, color::BLUE);
    loop {
        clear_background(BLACK_BACKGROUND);

        cam.spawn_camera_space(|| {
            draw_grid(20, 1., color::BLACK, color::WHITE);

            orb.animate();
        });

        next_frame().await
    }
}
