use std::sync::Arc;

use crate::prelude::*;

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

    pub depth_texture: RawTexture,

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

        let surface = unsafe { render_context.instance() }
            .create_surface(window.clone())
            .unwrap();

        surface.configure(unsafe { render_context.device() }, &surface_config);

        let depth_texture = RawTexture::create_depth_texture(
            unsafe { render_context.device() },
            &surface_config,
            "Depth Texture",
        );

        Self {
            window,
            render_context,
            surface_config,
            surface,
            clear: config.clear,
            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let device = unsafe { self.render_context.device() };

        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(device, &self.surface_config);
        }

        self.depth_texture =
            RawTexture::create_depth_texture(device, &self.surface_config, "Depth Texture");
    }

    #[inline]
    pub fn format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
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
            depth_view: self.depth_texture.view.clone(),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }
}

pub struct WindowCommandEncoder<'r> {
    command_encoder: CommandEncoder<'r>,
    output: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    clear: Option<wgpu::Color>,
    depth_view: wgpu::TextureView,
    depth_ops: Option<wgpu::Operations<f32>>,
    stencil_ops: Option<wgpu::Operations<u32>>,
}

impl WindowCommandEncoder<'_> {
    pub fn set_clear(&mut self, clear: Option<wgpu::Color>) -> &mut Self {
        self.clear = clear;
        self
    }

    pub fn set_depth_ops(&mut self, depth_ops: Option<wgpu::Operations<f32>>) -> &mut Self {
        self.depth_ops = depth_ops;
        self
    }

    pub fn set_stencil_ops(&mut self, stencil_ops: Option<wgpu::Operations<u32>>) -> &mut Self {
        self.stencil_ops = stencil_ops;
        self
    }

    #[inline]
    pub fn render_pass(&mut self) -> RenderPass<Void> {
        self.command_encoder.render_pass(
            &self.view,
            self.clear,
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: self.depth_ops,
                stencil_ops: self.stencil_ops,
            }),
        )
    }

    #[inline]
    pub fn present(self) {
        self.command_encoder.submit();
        self.output.present();
    }
}
