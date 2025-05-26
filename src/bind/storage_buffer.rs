use std::{mem, num::NonZeroU64, slice};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct StorageBuffer<T: 'static> {
    buffer: wgpu::Buffer,
    size: NonZeroU64,
    _marker: PhantomData<T>,
}

impl<T: 'static> StorageBuffer<T> {
    #[inline]
    pub fn new(render_context: &RenderContext, size: NonZeroU64) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            size: mem::size_of::<T>() as u64 * size.get(),
            mapped_at_creation: false,
        });

        Self {
            size,
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn new_init(render_context: &RenderContext, value: &[T]) -> Self
    where
        T: bytemuck::NoUninit,
    {
        let size = NonZeroU64::new(value.len() as u64).expect("storage buffer size cannot be zero");

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            size: mem::size_of::<T>() as u64 * size.get(),
            mapped_at_creation: true,
        });

        buffer
            .slice(..std::mem::size_of_val(value) as u64)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(value));

        buffer.unmap();

        Self {
            size,
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_usage(
        render_context: &RenderContext,
        size: NonZeroU64,
        usage: wgpu::BufferUsages,
        mapped_at_creation: bool,
    ) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | usage,
            size: mem::size_of::<T>() as u64 * size.get(),
            mapped_at_creation,
        });

        Self {
            size,
            buffer,
            _marker: PhantomData,
        }
    }

    pub fn with_usage_init(
        render_context: &RenderContext,
        value: &[T],
        usage: wgpu::BufferUsages,
    ) -> Self
    where
        T: bytemuck::NoUninit,
    {
        let size = NonZeroU64::new(value.len() as u64).expect("storage buffer size cannot be zero");

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | usage,
            size: mem::size_of::<T>() as u64 * size.get(),
            mapped_at_creation: true,
        });

        buffer
            .slice(..std::mem::size_of_val(value) as u64)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(value));

        buffer.unmap();

        Self {
            size,
            buffer,
            _marker: PhantomData,
        }
    }

    /// # Safety
    /// This function is unsafe because the buffer may not be mapped
    #[inline(always)]
    pub unsafe fn unmap(&mut self) {
        self.buffer.unmap();
    }

    #[inline(always)]
    pub fn size(&self) -> NonZeroU64 {
        self.size
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Buffer`
    #[inline(always)]
    pub unsafe fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline(always)]
    pub fn write(&self, render_context: &RenderContext, data: &[T])
    where
        T: bytemuck::NoUninit,
    {
        self.write_at_offset(render_context, data, 0);
    }

    #[inline]
    pub fn write_at_offset(&self, render_context: &RenderContext, data: &[T], offset: u64)
    where
        T: bytemuck::NoUninit,
    {
        if offset + data.len() as u64 > self.size.get() {
            panic!(
                "offset ({}) + data len ({}) larger than buffer size ({})",
                offset,
                data.len(),
                self.size.get()
            );
        }

        unsafe { self.write_at_offset_unchecked(render_context, data, offset) };
    }

    /// # Safety
    /// This may make an unsafe memory access
    #[inline]
    pub unsafe fn write_at_offset_unchecked(
        &self,
        render_context: &RenderContext,
        data: &[T],
        offset: u64,
    ) where
        T: bytemuck::NoUninit,
    {
        let queue = unsafe { render_context.queue() };

        queue.write_buffer(
            &self.buffer,
            offset * mem::size_of::<T>() as u64,
            bytemuck::cast_slice(data),
        );
    }

    #[inline]
    pub fn set_at_offset(&self, render_context: &RenderContext, data: &T, offset: u64)
    where
        T: bytemuck::NoUninit,
    {
        if offset >= self.size.get() {
            panic!(
                "offset ({}) larger than or equal to buffer size ({})",
                offset,
                self.size.get()
            );
        }

        unsafe { self.set_at_offset_unchecked(render_context, data, offset) };
    }

    /// # Safety
    /// This may make an unsafe memory access
    #[inline]
    pub unsafe fn set_at_offset_unchecked(
        &self,
        render_context: &RenderContext,
        data: &T,
        offset: u64,
    ) where
        T: bytemuck::NoUninit,
    {
        let queue = unsafe { render_context.queue() };

        let slice = unsafe { slice::from_raw_parts(data, 1) };

        queue.write_buffer(
            &self.buffer,
            offset * mem::size_of::<T>() as u64,
            bytemuck::cast_slice(slice),
        );
    }
}
