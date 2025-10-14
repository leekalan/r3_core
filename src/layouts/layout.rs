use std::num::NonZeroU32;

use crate::prelude::*;

pub type VertexLayout<L> = <L as Layout>::VertexLayout;
pub type SharedData<'a, L> = <L as Layout>::SharedData<'a>;

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

pub trait Layout {
    type VertexLayout: VertexBufferLayout;
    type SharedData<'a> = Void;

    fn raw_layout(&self) -> &RawLayout<Self::VertexLayout>;

    #[allow(unused)]
    fn set_shared_data(render_pass: &mut wgpu::RenderPass, shared_data: SharedData<Self>) {}
}

pub trait CreatePipeline {
    fn create_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        config: ShaderConfig,
    ) -> wgpu::RenderPipeline;
}
impl<L: Layout> CreatePipeline for L {
    fn create_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        shader_config: ShaderConfig,
    ) -> wgpu::RenderPipeline {
        self.raw_layout()
            .create_pipeline(render_context, module, shader_config)
    }
}

#[derive(Debug, Clone)]
pub struct RawLayout<V: VertexBufferLayout> {
    pipeline_layout: wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    __vertex: PhantomData<V>,
}

#[derive(Debug, Clone)]
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

impl<V: VertexBufferLayout> RawLayout<V> {
    fn layout(&self) -> &wgpu::PipelineLayout {
        &self.pipeline_layout
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    pub fn from_raw(pipeline_layout: wgpu::PipelineLayout, format: wgpu::TextureFormat) -> Self {
        Self {
            pipeline_layout,
            format,
            __vertex: PhantomData,
        }
    }

    pub fn new(render_context: &RenderContext, config: LayoutConfig) -> Self {
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
            __vertex: PhantomData,
        }
    }

    pub fn create_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        shader_config: ShaderConfig,
    ) -> wgpu::RenderPipeline {
        unsafe { render_context.device() }.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: shader_config.label,
            layout: Some(self.layout()),
            vertex: wgpu::VertexState {
                module,
                entry_point: Some(shader_config.vertex_entry.unwrap_or("vs")),
                buffers: V::DESC,
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
            depth_stencil: shader_config
                .depth_stencil
                .unwrap_or(Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                })),
            multisample: shader_config.multisample.unwrap_or(wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }),
            multiview: shader_config.multiview,
            cache: shader_config.cache,
        })
    }
}
