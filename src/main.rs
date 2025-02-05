use macroquad::{
    color::Color,
    math::vec3,
    miniquad::conf::Icon,
    models::draw_grid,
    window::{clear_background, next_frame, Conf},
};
use result_error::*;

mod camera;
mod orb;
mod result_error;

const ICON: Icon = Icon {
    // 16 * 16 * 4
    small: [0; 1024],
    // 32 * 32 * 4
    medium: [0; 4096],
    // 64 * 64 * 4
    big: [0; 16384],
};
const BLACK_BACKGROUND: Color = Color::new(0.05, 0.05, 0.05, 1.);
const GRID_COLOR: Color = Color::new(0.95, 0.95, 0.95, 0.1);

fn window_config() -> Conf {
    Conf {
        window_title: "tbp_eq_masses".to_owned(),
        icon: Some(ICON),
        ..Default::default()
    }
}
#[macroquad::main(window_config)]
async fn main() -> Result<()> {
    let mut cam = camera::Camera::new(vec3(-25., 30., 0.), vec3(0., 0., 0.), vec3(0., 1., 0.));
    let (mut orb1, mut orb2, mut orb3) = (
        orb::Orb::new_stationary(vec3(5., 0., 0.), 1., 1., None, None),
        orb::Orb::new_stationary(vec3(0., 0., 5.), 1., 1., None, None),
        orb::Orb::new_stationary(vec3(-2.5, 0., -2.5), 1., 1., None, None),
    );

    loop {
        clear_background(BLACK_BACKGROUND);

        let (orb1_i, orb2_i, orb3_i) = (orb1.info(), orb2.info(), orb3.info());

        cam.spawn_camera_space(|| async {
            draw_grid(50, 1., GRID_COLOR, GRID_COLOR);

            futures::future::join3(
                orb1.animate((&orb2_i, &orb3_i)),
                orb2.animate((&orb1_i, &orb3_i)),
                orb3.animate((&orb1_i, &orb2_i)),
            )
            .await;
        })
        .await;

        next_frame().await
    }
}
