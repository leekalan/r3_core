use std::{marker::PhantomData, num::NonZeroU32, sync::Arc};

use crate::prelude::*;

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
    pub bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    pub format: wgpu::TextureFormat,
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
        let pipeline_layout = unsafe { render_context.device() }.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: config.bind_group_layouts,
                push_constant_ranges: &[],
            },
        );

        Self {
            pipeline_layout,
            format: config.format,
            render_context,
            __vertex: PhantomData,
        }
    }

    pub fn create_pipeline(
        &self,
        module: &wgpu::ShaderModule,
        shader_config: ShaderConfig,
    ) -> wgpu::RenderPipeline {
        unsafe { self.render_context.device() }.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: shader_config.label,
                layout: Some(self.layout()),
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some(shader_config.vertex_entry.unwrap_or("vs")),
                    buffers: &[V::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some(shader_config.fragment_entry.unwrap_or("fs")),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: shader_config.primitive.unwrap_or(wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                }),
                depth_stencil: shader_config.depth_stencil.unwrap_or(Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    },
                )),
                multisample: shader_config.multisample.unwrap_or(wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                }),
                multiview: shader_config.multiview,
                cache: shader_config.cache,
            },
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct ShaderConfig<'a> {
    pub label: Option<&'a str>,
    pub primitive: Option<wgpu::PrimitiveState>,
    pub depth_stencil: Option<Option<wgpu::DepthStencilState>>,
    pub multisample: Option<wgpu::MultisampleState>,
    pub multiview: Option<NonZeroU32>,
    pub cache: Option<&'a wgpu::PipelineCache>,
    pub vertex_entry: Option<&'a str>,
    pub fragment_entry: Option<&'a str>,
}
