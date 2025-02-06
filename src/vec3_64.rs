#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec3_64 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3_64 {
    pub const ZERO: Self = Self {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add<Vec3_64> for Vec3_64 {
    type Output = Self;

    fn add(mut self, rhs: Vec3_64) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl std::ops::Sub<Vec3_64> for Vec3_64 {
    type Output = Self;

    fn sub(mut self, rhs: Vec3_64) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl std::ops::Mul<f64> for Vec3_64 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl std::ops::Div<f64> for Vec3_64 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl std::ops::AddAssign<Vec3_64> for Vec3_64 {
    fn add_assign(&mut self, rhs: Vec3_64) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<Vec3_64> for macroquad::math::Vec3 {
    fn from(value: Vec3_64) -> Self {
        Self::new(value.x as f32, value.y as f32, value.z as f32)
    }
}
