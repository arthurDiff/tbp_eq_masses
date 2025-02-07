use macroquad::{
    color::Color,
    math::vec3,
    miniquad::conf::Icon,
    models::draw_grid,
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};
use result_error::*;
use vec3_64::Vec3_64;

mod camera;
mod orb;
mod result_error;
mod store;
mod vec3_64;

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

const ORB_POS_OFFSET_1: f64 = 0.24308753;
const ORB_POS_OFFSET_2: f64 = 0.97000436;
const ORB_VEL_OFFSET_1: f64 = 0.43236573;
const ORB_VEL_OFFSET_2: f64 = 0.466203685;

// https://arxiv.org/abs/math/0011268
#[macroquad::main(window_config)]
async fn main() -> Result<()> {
    let mut store = store::Store::new();
    let mut cam = camera::Camera::new(vec3(-4.5, 1., 0.), vec3(0., 0., 0.), vec3(0., 1., 0.));
    let (mut orb1, mut orb2, mut orb3) = (
        orb::Orb::new(
            Vec3_64::new(0., -ORB_POS_OFFSET_1, ORB_POS_OFFSET_2),
            Vec3_64::new(0., ORB_VEL_OFFSET_1, ORB_VEL_OFFSET_2),
            0.15,
            1.,
            1.,
            None,
        ),
        orb::Orb::new(
            Vec3_64::new(0., ORB_POS_OFFSET_1, -ORB_POS_OFFSET_2),
            Vec3_64::new(0., ORB_VEL_OFFSET_1, ORB_VEL_OFFSET_2),
            0.15,
            1.,
            1.,
            None,
        ),
        orb::Orb::new(
            Vec3_64::new(0., 0., 0.),
            Vec3_64::new(0., -ORB_VEL_OFFSET_1 * 2., -ORB_VEL_OFFSET_2 * 2.),
            0.15,
            1.,
            1.,
            None,
        ),
    );

    loop {
        clear_background(BLACK_BACKGROUND);
        let delta = get_frame_time();
        let (orb1_i, orb2_i, orb3_i) = (orb1.info(), orb2.info(), orb3.info());
        store.draw((&orb1_i, &orb2_i, &orb3_i));

        cam.spawn_camera_space(|| async {
            draw_grid(50, 1., GRID_COLOR, GRID_COLOR);

            futures::future::join3(
                orb1.animate((&orb2_i, &orb3_i), delta),
                orb2.animate((&orb1_i, &orb3_i), delta),
                orb3.animate((&orb1_i, &orb2_i), delta),
            )
            .await;
        })
        .await;

        next_frame().await
    }
}
