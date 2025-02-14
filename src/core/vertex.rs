use crate::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimpleVertex {
    pub position: [f32; 3],
}

impl Vertex for SimpleVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTR: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTR,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for ColoredVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTR: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTR,
        }
    }
}

pub struct TextureVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex for TextureVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTR: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTR,
        }
    }
}
