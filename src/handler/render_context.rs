use std::{ops::Range, sync::Arc};

use wgpu::BufferSlice;

#[derive(Default)]
pub struct RenderContextConfig {
    pub backends: Option<wgpu::Backends>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub features: Option<wgpu::Features>,
    pub limits: Option<wgpu::Limits>,
}

pub struct RenderContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl RenderContext {
    pub async fn new(config: RenderContextConfig) -> Arc<Self> {
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

        Arc::new(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn command_encoder(&self) -> CommandEncoder {
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

pub struct CommandEncoder<'r> {
    pub encoder: wgpu::CommandEncoder,
    pub render_context: &'r RenderContext,
}

impl CommandEncoder<'_> {
    pub fn render_pass(
        &mut self,
        view: &wgpu::TextureView,
        clear: Option<wgpu::Color>,
    ) -> RenderPass {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear.unwrap_or(wgpu::Color {
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
    pub fn submit(self) {
        self.render_context
            .queue
            .submit(Some(self.encoder.finish()));
    }
}

#[repr(transparent)]
pub struct RenderPass<'r> {
    pub render_pass: wgpu::RenderPass<'r>,
}

impl<'r> RenderPass<'r> {
    #[inline]
    pub fn inner(&mut self) -> &'r mut wgpu::RenderPass {
        &mut self.render_pass
    }

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
    pub fn set_index_buffer(
        &mut self,
        buffer: BufferSlice,
        index_format: wgpu::IndexFormat,
    ) -> &mut Self {
        self.render_pass.set_index_buffer(buffer, index_format);
        self
    }

    #[inline]
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) -> &mut Self {
        self.render_pass.draw(vertices, instances);
        self
    }

    #[inline]
    pub fn draw_indexed(
        &mut self,
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    ) -> &mut Self {
        self.render_pass
            .draw_indexed(indices, base_vertex, instances);
        self
    }
}
