use std::f32::consts::PI;

use crate::prelude::{core::*, *};

use cgmath::{Matrix4, Rad};

create_bind::bind!(CameraBind, CameraBindLayout {
    UniformBuffers => {
        uniform: CameraUniform => 0 for VERTEX,
    },
});

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug, Clone)]
pub struct Camera {
    pub projection: Projection,
    projection_matrix: ProjectionMatrix,
    uniform: CameraUniform,
    bind: CameraBind,
}

impl Camera {
    #[inline]
    pub fn new(bind: CameraBind, projection: Projection, transform: Transform) -> Self {
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
    pub fn apply_transform(&mut self, transform: Transform) -> &mut Self {
        self.uniform = self.projection_matrix.apply_transform(&transform);
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

    #[inline(always)]
    pub fn layout(&self) -> &CameraBindLayout {
        self.bind.layout()
    }

    #[inline(always)]
    pub fn write_buffer(&self, render_context: &RenderContext) {
        self.bind.uniform().write(render_context, &self.uniform);
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
            aspect: 1.,
            fovy: Rad(85. / 180. * PI),
            near: 0.1,
            far: 100.,
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

    pub fn apply_transform(self, transform: &Transform) -> CameraUniform {
        CameraUniform {
            matrix: (self.matrix * transform.transform_matrix()).into(),
        }
    }
}

pub type M4 = [[f32; 4]; 4];

#[repr(transparent)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub matrix: M4,
}

impl CameraUniform {
    pub const fn matrix(&self) -> &M4 {
        &self.matrix
    }
}
