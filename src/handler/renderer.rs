use std::{ops::Range, sync::Arc};

use wgpu::BufferSlice;
use winit::window::Window;

#[derive(Default)]
pub struct WindowRendererConfig {
    pub backends: Option<wgpu::Backends>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub features: Option<wgpu::Features>,
    pub limits: Option<wgpu::Limits>,
    pub present_mode: Option<wgpu::PresentMode>,
    pub desired_maximum_frame_latency: Option<u32>,

    pub clear: Option<wgpu::Color>,
}

pub struct WindowRenderer {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub clear: Option<wgpu::Color>,
}

impl WindowRenderer {
    pub async fn new(window: Arc<Window>, config: WindowRendererConfig) -> Self {
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
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: config.features.unwrap_or(wgpu::Features::empty()),
                    required_limits: config.limits.unwrap_or_default(),
                    label: Some("Device"),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: config.present_mode.unwrap_or_default(),
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: config.desired_maximum_frame_latency.unwrap_or(2),
        };

        let surface = instance.create_surface(window).unwrap();

        surface.configure(&device, &surface_config);

        Self {
            instance,
            adapter,
            surface_config,
            surface,
            device,
            queue,
            clear: config.clear,
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn command_encoder(&self) -> CommandEncoder {
        let output = self.surface.get_current_texture().unwrap();

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        CommandEncoder {
            encoder,
            output,
            renderer: self,
        }
    }
}

pub struct CommandEncoder<'w> {
    encoder: wgpu::CommandEncoder,
    output: wgpu::SurfaceTexture,
    renderer: &'w WindowRenderer,
}

impl CommandEncoder<'_> {
    pub fn render_pass(&mut self) -> RenderPass {
        let view = &self
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.renderer.clear.unwrap_or(wgpu::Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 1.0,
                    })),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        RenderPass { render_pass }
    }

    #[inline]
    pub fn submit_and_present(self) {
        self.renderer.queue.submit(Some(self.encoder.finish()));
        self.output.present();
    }
}

pub struct RenderPass<'r> {
    render_pass: wgpu::RenderPass<'r>,
}

impl RenderPass<'_> {
    #[inline]
    pub fn set_pipeline(&mut self, pipeline: &wgpu::RenderPipeline) -> &mut Self {
        self.render_pass.set_pipeline(pipeline);

        self
    }

    #[inline]
    pub fn set_vertex_buffer(&mut self, binding: u32, buffer: BufferSlice) -> &mut Self {
        self.render_pass.set_vertex_buffer(binding, buffer);

        self
    }

    #[inline]
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) -> &mut Self {
        self.render_pass.draw(vertices, instances);

        self
    }
}
