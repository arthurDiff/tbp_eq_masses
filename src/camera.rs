use std::future::Future;

use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    input::{is_mouse_button_down, mouse_delta_position, mouse_wheel, MouseButton},
    math::{Vec2, Vec3},
};

const ROTATION_SENSITIVITY: f32 = 5.;

struct Matrix3D([[f32; 3]; 3]);

pub(crate) struct Camera(pub Camera3D);

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        Self(Camera3D {
            position,
            target,
            up,
            ..Default::default()
        })
    }

    pub async fn spawn_camera_space<F, Fut>(&mut self, controllable: bool, space_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        if controllable {
            self.handle_rotation();
            self.handle_zoom();
        }
        set_camera(&self.0);
        space_fn().await;
        set_default_camera();
    }

    // Rodrigues' rotation formula
    fn handle_rotation(&mut self) {
        if !is_mouse_button_down(MouseButton::Left) {
            return;
        }
        let Vec2 {
            x: delta_x,
            y: delta_y,
        } = mouse_delta_position();

        let (delta_x, delta_y) = (
            delta_x * ROTATION_SENSITIVITY,
            delta_y * ROTATION_SENSITIVITY,
        );

        let relative_pos = self.0.position - self.0.target;
        let forward = relative_pos.normalize_or_zero();
        let right = self.0.up.cross(forward).normalize_or_zero();
        let real_up = forward.cross(right).normalize_or_zero();

        // rotation around y axis
        let (cos_yt, sin_yt) = (delta_x.cos(), delta_x.sin());
        let pos_after_y = Vec3::new(
            cos_yt * relative_pos.x + sin_yt * relative_pos.z,
            relative_pos.y,
            -sin_yt * relative_pos.x + cos_yt * relative_pos.z,
        );

        // new local
        let forward = pos_after_y.normalize();
        let right = self.0.up.cross(forward).normalize();

        // rotation around x axis
        let (cos_xt, sin_xt) = (delta_y.cos(), delta_y.sin());
        let x_matrix = Matrix3D([[1., 0., 0.], [0., cos_xt, -sin_xt], [0., sin_xt, cos_xt]]);

        // local_to_world
        let loc_to_w = Matrix3D([
            [right.x, real_up.x, forward.x],
            [right.y, real_up.y, forward.y],
            [right.z, real_up.z, forward.z],
        ]);

        // transform to world
        let local_pos = loc_to_w.transpose().multiply_vec3(pos_after_y);

        // Apply x rot in local scope
        let rot_local = x_matrix.multiply_vec3(local_pos);

        // target + transformed back relative pos
        self.0.position = self.0.target + loc_to_w.multiply_vec3(rot_local);
    }

    fn handle_zoom(&mut self) {
        // (side wheel, middle mouse button)
        let (_, wheel_y) = mouse_wheel();
        if wheel_y == 0. {
            return;
        }
        self.0.position *= if wheel_y > 0. { 0.95 } else { 1.05 };
    }
}

impl Matrix3D {
    fn transpose(&self) -> Self {
        Self([
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
        ])
    }

    fn multiply_vec3(&self, pos: Vec3) -> Vec3 {
        Vec3::new(
            self[0][0] * pos.x + self[0][1] * pos.y + self[0][2] * pos.z,
            self[1][0] * pos.x + self[1][1] * pos.y + self[1][2] * pos.z,
            self[2][0] * pos.x + self[2][1] * pos.y + self[2][2] * pos.z,
        )
    }
}

impl std::ops::Deref for Matrix3D {
    type Target = [[f32; 3]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
