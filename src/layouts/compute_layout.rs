use crate::prelude::*;

pub type SharedComputeData<'a, L> = <L as ComputeLayout>::SharedData<'a>;

pub trait ComputeLayout {
    type SharedData<'a> = Void;

    fn raw_layout(&self) -> &RawComputeLayout;

    #[allow(unused)]
    fn set_shared_data(compute_pass: &mut wgpu::ComputePass, shared_data: SharedComputeData<Self>) {
    }
}

pub trait CreateComputePipeline: ComputeLayout {
    fn create_compute_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        compute_shader_config: ComputeShaderConfig,
    ) -> wgpu::ComputePipeline;
}
impl<L: ComputeLayout> CreateComputePipeline for L {
    fn create_compute_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        compute_shader_config: ComputeShaderConfig,
    ) -> wgpu::ComputePipeline {
        self.raw_layout()
            .create_compute_pipeline(render_context, module, compute_shader_config)
    }
}

#[derive(Debug, Clone)]
pub struct RawComputeLayout {
    pipeline_layout: wgpu::PipelineLayout,
}

#[derive(Default, Debug, Clone)]
pub struct ComputeLayoutConfig<'a> {
    pub bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
}

impl RawComputeLayout {
    fn layout(&self) -> &wgpu::PipelineLayout {
        &self.pipeline_layout
    }

    pub fn from_raw(pipeline_layout: wgpu::PipelineLayout) -> Self {
        Self { pipeline_layout }
    }

    pub fn new(render_context: &RenderContext, config: ComputeLayoutConfig) -> Self {
        let pipeline_layout = unsafe { render_context.device() }.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: config.bind_group_layouts,
                push_constant_ranges: &[],
            },
        );

        Self { pipeline_layout }
    }

    pub fn create_compute_pipeline(
        &self,
        render_context: &RenderContext,
        module: &wgpu::ShaderModule,
        compute_shader_config: ComputeShaderConfig,
    ) -> wgpu::ComputePipeline {
        unsafe { render_context.device() }.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: compute_shader_config.label,
                layout: Some(self.layout()),
                module,
                entry_point: Some(compute_shader_config.entry.unwrap_or("cs")),
                compilation_options: compute_shader_config
                    .compilation_options
                    .unwrap_or_default(),
                cache: compute_shader_config.cache,
            },
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct ComputeShaderConfig<'a> {
    pub label: Option<&'a str>,
    pub entry: Option<&'a str>,
    pub compilation_options: Option<wgpu::PipelineCompilationOptions<'a>>,
    pub cache: Option<&'a wgpu::PipelineCache>,
}
