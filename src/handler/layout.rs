use std::{marker::PhantomData, sync::Arc};

use crate::prelude::RenderContext;

pub mod shader;

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub struct Layout<V: Vertex> {
    pipeline_layout: wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    render_context: Arc<RenderContext>,
    __vertex: PhantomData<V>,
}

pub struct LayoutConfig<'a> {
    bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    format: wgpu::TextureFormat,
}

impl Default for LayoutConfig<'_> {
    fn default() -> Self {
        Self {
            bind_group_layouts: &[],
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
}

impl<V: Vertex> Layout<V> {
    pub fn layout(&self) -> &wgpu::PipelineLayout {
        &self.pipeline_layout
    }

    pub fn render_context(&self) -> &Arc<RenderContext> {
        &self.render_context
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn from_raw(
        pipeline_layout: wgpu::PipelineLayout,
        format: wgpu::TextureFormat,
        render_context: Arc<RenderContext>,
    ) -> Self {
        Self {
            pipeline_layout,
            format,
            render_context,
            __vertex: PhantomData,
        }
    }

    pub fn new(render_context: Arc<RenderContext>, config: LayoutConfig) -> Self {
        let pipeline_layout =
            render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: config.bind_group_layouts,
                    push_constant_ranges: &[],
                });

        Self {
            pipeline_layout,
            format: config.format,
            render_context,
            __vertex: PhantomData,
        }
    }
}
