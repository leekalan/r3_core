use crate::prelude::*;

pub mod command_encoder;
pub mod compute_pass;
pub mod render_pass;

#[derive(Default, Debug, Clone)]
pub struct RenderContextConfig {
    pub backends: Option<wgpu::Backends>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub features: Option<wgpu::Features>,
    pub experimental_features: wgpu::ExperimentalFeatures,
    pub limits: Option<wgpu::Limits>,
}

#[derive(Debug, Clone)]
pub struct RenderContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RenderContext {
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn device(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub async fn new(config: RenderContextConfig) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: config.backends.unwrap_or(wgpu::Backends::PRIMARY),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: config
                    .power_preference
                    .unwrap_or(wgpu::PowerPreference::HighPerformance),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: config.features.unwrap_or(wgpu::Features::empty()),
                required_limits: config.limits.unwrap_or_default(),
                label: Some("Device"),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
                experimental_features: config.experimental_features,
            })
            .await
            .unwrap();

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }

    #[inline]
    pub fn create_shader_module(
        &self,
        label: Option<&str>,
        source: wgpu::ShaderSource,
    ) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor { label, source })
    }

    pub fn command_encoder(&'_ self) -> CommandEncoder<'_> {
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        CommandEncoder {
            encoder,
            render_context: self,
        }
    }
}
