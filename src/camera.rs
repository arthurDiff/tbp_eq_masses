use macroquad::{
    camera::{set_camera, set_default_camera, Camera3D},
    math::Vec3,
};

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
        set_camera(&self.0);
        space_fn();
        set_default_camera();
    }
}
