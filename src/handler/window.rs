use std::sync::Arc;

use super::render_context::{CommandEncoder, RenderContext, RenderPass};

#[derive(Default)]
pub struct WindowConfig {
    pub present_mode: Option<wgpu::PresentMode>,
    pub desired_maximum_frame_latency: Option<u32>,

    pub clear: Option<wgpu::Color>,
}

pub struct Window {
    pub window: Arc<winit::window::Window>,

    pub render_context: Arc<RenderContext>,

    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,

    pub clear: Option<wgpu::Color>,
}

impl Window {
    pub fn new(
        window: Arc<winit::window::Window>,
        render_context: Arc<RenderContext>,
        config: WindowConfig,
    ) -> Self {
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

        let surface = render_context
            .instance
            .create_surface(window.clone())
            .unwrap();

        surface.configure(&render_context.device, &surface_config);

        Self {
            window,
            render_context,
            surface_config,
            surface,
            clear: config.clear,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface
                .configure(&self.render_context.device, &self.surface_config);
        }
    }

    #[inline]
    pub fn output(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn command_encoder(&self) -> WindowCommandEncoder {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        WindowCommandEncoder {
            command_encoder: self.render_context.command_encoder(),
            output,
            view,
            clear: self.clear,
        }
    }
}

pub struct WindowCommandEncoder<'r> {
    command_encoder: CommandEncoder<'r>,
    output: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    clear: Option<wgpu::Color>,
}

impl WindowCommandEncoder<'_> {
    #[inline]
    pub fn render_pass(&mut self) -> RenderPass {
        self.command_encoder.render_pass(&self.view, self.clear)
    }

    #[inline]
    pub fn present(self) {
        self.command_encoder.submit();
        self.output.present();
    }
}
