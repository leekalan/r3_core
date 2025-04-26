use cgmath::Matrix4;

use crate::prelude::{core::*, *};

pub struct Camera2d {
    pub projection: Projection2d,
    projection_matrix: Projection2dMatrix,
    uniform: CameraUniform,
    bind: CameraBind,
}

impl Camera2d {
    #[inline]
    pub fn new(bind: CameraBind, projection: Projection2d, transform: Transform2d) -> Self {
        let projection_matrix = projection.proj_matrix();

        Self {
            uniform: projection_matrix.apply_transform(&transform),
            projection_matrix,
            projection,
            bind,
        }
    }

    #[inline]
    pub fn update_projection(&mut self) -> &mut Self {
        self.projection_matrix = self.projection.proj_matrix();
        self
    }

    #[inline]
    pub fn apply_transform(&mut self, transform: Transform2d) -> &mut Self {
        self.uniform = self.projection_matrix.apply_transform(&transform);
        self
    }

    #[inline(always)]
    pub const fn projection_matrix(&self) -> Projection2dMatrix {
        self.projection_matrix
    }

    #[inline(always)]
    pub const fn uniform(&self) -> CameraUniform {
        self.uniform
    }

    #[inline(always)]
    pub const fn bind(&self) -> &CameraBind {
        &self.bind
    }

    #[inline(always)]
    pub const fn layout(&self) -> &CameraBindLayout {
        self.bind.layout()
    }

    #[inline(always)]
    pub fn write_buffer(&self, render_context: &RenderContext) {
        self.bind.uniform().write(render_context, self.uniform);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Projection2d {
    pub aspect: f32,
}

impl Projection2d {
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            aspect: width / height,
        }
    }

    #[inline]
    pub const fn resize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    #[inline]
    pub fn proj_matrix(&self) -> Projection2dMatrix {
        Projection2dMatrix {
            matrix: Matrix4::from_nonuniform_scale(1.0 / self.aspect, 1.0, 1.0),
        }
    }
}

impl Default for Projection2d {
    #[inline]
    fn default() -> Self {
        Self { aspect: 1. }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Projection2dMatrix {
    matrix: Matrix4<f32>,
}

impl Projection2dMatrix {
    pub const fn matrix(self) -> Matrix4<f32> {
        self.matrix
    }

    pub fn apply_transform(self, transform: &Transform2d) -> CameraUniform {
        CameraUniform {
            matrix: (self.matrix * transform.transform_matrix()).into(),
        }
    }
}
