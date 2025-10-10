use std::{mem, num::NonZeroU64, slice};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct DynamicBuffer<T: 'static> {
    buffer: wgpu::Buffer,
    size: u64,
    _marker: PhantomData<T>,
}

impl<T: 'static> DynamicBuffer<T> {
    #[inline]
    pub fn new(render_context: &RenderContext, max_size: NonZeroU64) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            size: mem::size_of::<T>() as u64 * max_size.get(),
            mapped_at_creation: false,
        });

        Self {
            size: 0,
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn new_init(
        render_context: &RenderContext,
        value: &[T],
        max_size: Option<NonZeroU64>,
    ) -> Self
    where
        T: bytemuck::NoUninit,
    {
        let size = match max_size {
            Some(size) => size.get(),
            None => value.len() as u64,
        };

        assert!(value.len() as u64 <= size);

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            size: mem::size_of::<T>() as u64 * size,
            mapped_at_creation: true,
        });

        buffer
            .slice(..std::mem::size_of_val(value) as u64)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(value));

        buffer.unmap();

        Self {
            size: value.len() as u64,
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_usage(
        render_context: &RenderContext,
        max_size: NonZeroU64,
        usage: wgpu::BufferUsages,
        mapped_at_creation: bool,
    ) -> Self {
        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | usage,
            size: mem::size_of::<T>() as u64 * max_size.get(),
            mapped_at_creation,
        });

        Self {
            size: 0,
            buffer,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_usage_init(
        render_context: &RenderContext,
        value: &[T],
        max_size: Option<NonZeroU64>,
        usage: wgpu::BufferUsages,
    ) -> Self
    where
        T: bytemuck::NoUninit,
    {
        let size = match max_size {
            Some(size) => size.get(),
            None => value.len() as u64,
        };

        assert!(value.len() as u64 <= size);

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::STORAGE | usage,
            size: mem::size_of::<T>() as u64 * size,
            mapped_at_creation: true,
        });

        buffer
            .slice(..std::mem::size_of_val(value) as u64)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(value));

        buffer.unmap();

        Self {
            size: value.len() as u64,
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
    pub const fn size(&self) -> u64 {
        self.size
    }

    #[inline(always)]
    pub fn max_size(&self) -> u64 {
        self.buffer.size() / mem::size_of::<T>() as u64
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Buffer`
    #[inline(always)]
    pub unsafe fn wgpu_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Buffer` as a slice
    #[inline(always)]
    pub unsafe fn wgpu_slice(&self) -> wgpu::BufferSlice<'_> {
        self.buffer.slice(..mem::size_of::<T>() as u64 * self.size)
    }

    #[inline(always)]
    pub fn write(&mut self, render_context: &RenderContext, data: &[T])
    where
        T: bytemuck::NoUninit,
    {
        self.write_at_offset(render_context, data, 0);
    }

    #[inline]
    pub fn write_at_offset(&mut self, render_context: &RenderContext, data: &[T], offset: u64)
    where
        T: bytemuck::NoUninit,
    {
        if offset + data.len() as u64 > self.max_size() {
            panic!(
                "offset ({}) + data len ({}) larger than max size ({})",
                offset,
                data.len(),
                self.max_size()
            );
        }

        unsafe { self.write_at_offset_unchecked(render_context, data, offset) };
    }

    /// # Safety
    /// This may make an unsafe memory access
    #[inline]
    pub unsafe fn write_at_offset_unchecked(
        &mut self,
        render_context: &RenderContext,
        data: &[T],
        offset: u64,
    ) where
        T: bytemuck::NoUninit,
    {
        let queue = unsafe { render_context.queue() };

        self.size = offset + data.len() as u64;

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
        if offset >= self.size() {
            panic!(
                "offset ({}) larger than or equal to current size ({})",
                offset,
                self.size()
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
