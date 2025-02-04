use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    input::{is_mouse_button_down, mouse_delta_position, mouse_wheel, MouseButton},
    math::{Vec2, Vec3},
};

const ROTATION_SENSITIVITY: f32 = 5.;

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

    pub fn spawn_camera_space<T>(&mut self, space_fn: T)
    where
        T: FnOnce(),
    {
        self.handle_rotation();
        self.handle_zoom();
        set_camera(&self.0);
        space_fn();
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

        let forward = (self.0.target - self.0.position).normalize_or_zero();
        let right = self.0.up.cross(forward).normalize_or_zero();

        // rotation around y axis
        let forward = Self::rotate_by_axis(forward, self.0.up.normalize_or_zero(), delta_x);

        // rotation around x axis
        let forward = Self::rotate_by_axis(forward, right, delta_y);

        let center_mag = (self.0.target - self.0.position).magnitude();

        self.0.position.x = self.0.target.x + forward.x * center_mag;
        self.0.position.y = self.0.target.y + forward.y * center_mag;
        self.0.position.z = self.0.target.z + forward.z * center_mag;
    }

    fn handle_zoom(&mut self) {
        // (side wheel, middle mouse button)
        let (_, wheel_y) = mouse_wheel();
        if wheel_y == 0. {
            return;
        }
        self.0.position *= if wheel_y > 0. { 0.95 } else { 1.05 };
    }

    fn rotate_by_axis(v: Vec3, axis: Vec3, theta: f32) -> Vec3 {
        let (cos_t, sin_t) = (theta.cos(), theta.sin());
        let (axis_cross, axis_dot) = (axis.cross(v), axis.dot(v));
        Vec3::new(
            v.x * cos_t + axis_cross.x * sin_t * axis.x * axis_dot * (1. - theta),
            v.y * cos_t + axis_cross.y * sin_t * axis.y * axis_dot * (1. - theta),
            v.z * cos_t + axis_cross.z * sin_t * axis.z * axis_dot * (1. - theta),
        )
    }
}

trait VectorMath<T> {
    fn magnitude(&self) -> f32;
}

impl VectorMath<Vec3> for Vec3 {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}
