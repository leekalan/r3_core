use wgpu::util::DeviceExt;

use crate::prelude::*;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct UniformBuffer<T: 'static + Copy + bytemuck::Pod + bytemuck::Zeroable> {
    buffer: wgpu::Buffer,
    _marker: PhantomData<&'static mut T>,
}

impl<T: 'static + Copy + bytemuck::Pod + bytemuck::Zeroable> UniformBuffer<T> {
    #[inline]
    pub fn new(render_context: &RenderContext) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<T>() as u64,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn new_init(render_context: &RenderContext, value: T) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[value]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_usage(
        render_context: &RenderContext,
        usage: wgpu::BufferUsages,
        mapped_at_creation: bool,
    ) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage,
            size: size_of::<T>() as u64,
            mapped_at_creation,
        });

        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn init_with_usage(
        render_context: &RenderContext,
        usage: wgpu::BufferUsages,
        value: T,
    ) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[value]),
            usage,
        });

        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Buffer`
    #[inline(always)]
    pub unsafe fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline]
    pub fn write(&self, render_context: &RenderContext, value: T) {
        let queue = unsafe { render_context.queue() };

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[value]));
    }
}
