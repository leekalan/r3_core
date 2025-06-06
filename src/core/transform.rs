use cgmath::{Matrix4, Quaternion, Rad, Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,
}

impl Default for Transform {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    #[inline]
    pub const fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: 1.0,
        }
    }

    #[inline]
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_scale(self.scale)
            * Matrix4::from(self.rotation)
            * Matrix4::from_translation(self.position)
    }
}

pub struct Transform2d {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: f32,
}

impl Default for Transform2d {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Transform2d {
    #[inline]
    pub const fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
        }
    }

    #[inline]
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_scale(self.scale)
            * Matrix4::from_angle_z(Rad(self.rotation))
            * Matrix4::from_translation(Vector3::new(self.position.x, self.position.y, 0.0))
    }
}
