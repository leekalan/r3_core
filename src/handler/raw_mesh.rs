use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::prelude::*;

pub mod index_format {
    pub trait IndexFormat {
        const FORMAT: wgpu::IndexFormat;
    }

    pub struct Uint16;

    impl IndexFormat for Uint16 {
        const FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
    }

    pub struct Uint32;

    impl IndexFormat for Uint32 {
        const FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
    }
}

pub struct RawMesh<V: Vertex, I: index_format::IndexFormat> {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    __index_format: PhantomData<I>,
    __vertex: PhantomData<V>,
}

impl<V: Vertex, I: index_format::IndexFormat> RawMesh<V, I> {
    /// # Safety
    /// This function is unsafe because the caller must ensure
    /// that the generic `V` matches with `vertex_buffer`
    #[inline]
    pub const unsafe fn from_raw(
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        index_count: u32,
    ) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            index_count,
            __index_format: PhantomData,
            __vertex: PhantomData,
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), I::FORMAT);

        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

impl<V: Vertex + bytemuck::NoUninit> RawMesh<V, index_format::Uint16> {
    pub fn new_uint16(render_context: &RenderContext, vertices: &[V], indices: &[u16]) -> Self {
        let device = unsafe { render_context.device() };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        unsafe { Self::from_raw(vertex_buffer, index_buffer, indices.len() as u32) }
    }
}

impl<V: Vertex + bytemuck::NoUninit> RawMesh<V, index_format::Uint32> {
    pub fn new_uint32(render_context: &RenderContext, vertices: &[V], indices: &[u32]) -> Self {
        let device = unsafe { render_context.device() };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        unsafe { Self::from_raw(vertex_buffer, index_buffer, indices.len() as u32) }
    }
}
