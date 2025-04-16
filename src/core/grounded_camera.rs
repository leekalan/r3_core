use cgmath::{Quaternion, Rad, Rotation3, Vector3};

use crate::prelude::core::*;

#[derive(Debug, Clone)]
pub struct GroundedCamera {
    pub position: Vector3<f32>,
    pub up: Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Default for GroundedCamera {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl GroundedCamera {
    #[inline]
    pub const fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0,
        }
    }

    #[inline]
    pub fn generate_transform(&self) -> Transform {
        Transform {
            position: self.position,
            rotation: Quaternion::from_axis_angle(self.up, Rad(self.yaw))
                * Quaternion::from_angle_z(Rad(self.roll))
                * Quaternion::from_angle_x(Rad(self.pitch)),
            scale: 1.0,
        }
    }
}
