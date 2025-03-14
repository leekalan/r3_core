#![allow(private_bounds)]
use crate::prelude::*;
use cgmath::{Matrix4, One, Quaternion, Rad, Vector3, Zero};
use std::f32::consts::PI;

create_bind::bind!(CameraBind, CameraBindLayout {
    uniform: CameraUniform => 0 for VERTEX
});

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Clone, Debug)]
pub struct Camera {
    pub transform: CameraTransform,
    pub projection: Projection,
    projection_matrix: ProjectionMatrix,
    uniform: CameraUniform,
    bind: CameraBind,
}

impl Camera {
    #[inline]
    pub fn new(bind: CameraBind, projection: Projection, transform: CameraTransform) -> Self {
        let projection_matrix = projection.proj_matrix();

        Self {
            uniform: projection_matrix.apply_transform(&transform),
            projection_matrix,
            transform,
            projection,
            bind,
        }
    }

    #[inline]
    pub fn update_projection(&mut self) -> &mut Self {
        self.projection_matrix = self.projection.proj_matrix();
        self.uniform = self.projection_matrix.apply_transform(&self.transform);
        self
    }

    #[inline]
    pub fn update_transform(&mut self) -> &mut Self {
        self.uniform = self.projection_matrix.apply_transform(&self.transform);
        self
    }

    #[inline(always)]
    pub const fn projection_matrix(&self) -> ProjectionMatrix {
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

    pub const fn layout(&self) -> &CameraBindLayout {
        self.bind.layout()
    }

    pub fn write_buffer(&self, render_context: &RenderContext) {
        self.bind.uniform().write(render_context, self.uniform);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CameraTransform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: f32,
}

impl CameraTransform {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_scale(self.scale)
    }
}

impl Default for CameraTransform {
    #[inline]
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Quaternion::one(),
            scale: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Projection {
    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub near: f32,
    pub far: f32,
}

impl Projection {
    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            aspect: width / height,
            ..default()
        }
    }

    #[inline]
    pub const fn resize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    #[inline]
    pub fn proj_matrix(&self) -> ProjectionMatrix {
        ProjectionMatrix {
            matrix: OPENGL_TO_WGPU_MATRIX
                * cgmath::perspective(self.fovy, self.aspect, self.near, self.far),
        }
    }
}

impl Default for Projection {
    #[inline]
    fn default() -> Self {
        Self {
            aspect: 1.0,
            fovy: Rad(85f32 / 180f32 * PI),
            near: 0.1,
            far: 100.0,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct ProjectionMatrix {
    matrix: Matrix4<f32>,
}

impl ProjectionMatrix {
    pub const fn matrix(self) -> Matrix4<f32> {
        self.matrix
    }

    pub fn apply_transform(self, transform: &CameraTransform) -> CameraUniform {
        CameraUniform {
            matrix: (self.matrix * transform.transform_matrix()).into(),
        }
    }
}

pub type M4 = [[f32; 4]; 4];

#[repr(transparent)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    matrix: M4,
}

impl CameraUniform {
    pub const fn matrix(&self) -> &M4 {
        &self.matrix
    }
}
