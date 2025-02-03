use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    input::{is_mouse_button_down, mouse_delta_position, MouseButton},
    math::{Vec2, Vec3},
};

pub(crate) struct Camera(pub Camera3D);

const MOUSE_SENSITIVITY: f32 = 30.;

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
        self.handle_input();
        set_camera(&self.0);
        space_fn();
        set_default_camera();
    }
    fn handle_input(&mut self) {
        if !is_mouse_button_down(MouseButton::Left) {
            return;
        }
        let Vec2 {
            x: delta_x,
            y: delta_y,
        } = mouse_delta_position();

        let (temp_x, new_z) = (
            self.0.position.x * delta_x.cos() + self.0.position.z * delta_x.sin(),
            -self.0.position.x * delta_x.sin() + self.0.position.z * delta_x.cos(),
        );
        let (new_x, new_y) = (
            temp_x * delta_y.cos() + self.0.position.y * delta_y.sin(),
            -temp_x * delta_y.sin() + self.0.position.y * delta_y.cos(),
        );

        self.0.position.x = new_x;
        self.0.position.y = new_y;
        self.0.position.z = new_z;
    }
}
