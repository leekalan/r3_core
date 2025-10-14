use std::fmt::Debug;

use wgpu::SurfaceTexture;

use crate::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct WindowConfig {
    pub present_mode: Option<wgpu::PresentMode>,
    pub desired_maximum_frame_latency: Option<u32>,
    pub window_attributes: Option<winit::window::WindowAttributes>,

    pub clear: Option<wgpu::Color>,
}

#[derive(Debug)]
pub struct Window {
    pub window: Arc<winit::window::Window>,

    pub render_context: RenderContext,

    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,

    pub depth_texture: Texture<Texture2D>,

    pub clear: Option<wgpu::Color>,
}

impl Window {
    pub fn new(
        window: Arc<winit::window::Window>,
        render_context: RenderContext,
        config: &WindowConfig,
    ) -> Self {
        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
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

        let depth_texture = Texture::create_depth_texture(&render_context, &surface_config);

        Self {
            window,
            render_context,
            surface_config,
            surface,
            clear: config.clear,
            depth_texture,
        }
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.surface_config.width, self.surface_config.height)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let device = unsafe { self.render_context.device() };

        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(device, &self.surface_config);
        }

        self.depth_texture =
            Texture::create_depth_texture(&self.render_context, &self.surface_config);
    }

    #[inline]
    pub fn format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    #[inline]
    pub fn output(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn command_encoder(&self) -> WindowCommandEncoder<'_> {
        let output = self.surface.get_current_texture().unwrap();

        let view = unsafe {
            RawTextureView::new(
                output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
            )
        };

        WindowCommandEncoder {
            command_encoder: self.render_context.command_encoder(),
            output,
            view,
            clear: self.clear,
            depth_view: unsafe { self.depth_texture.texture.view().inner() }.clone(),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }
}

#[derive(Debug)]
pub struct WindowCommandEncoder<'r> {
    command_encoder: CommandEncoder<'r>,
    output: wgpu::SurfaceTexture,
    view: RawTextureView<Texture2D>,
    clear: Option<wgpu::Color>,
    depth_view: wgpu::TextureView,
    depth_ops: Option<wgpu::Operations<f32>>,
    stencil_ops: Option<wgpu::Operations<u32>>,
}

impl<'a> WindowCommandEncoder<'a> {
    #[inline(always)]
    pub fn command_encoder(&self) -> &CommandEncoder<'_> {
        &self.command_encoder
    }

    #[inline(always)]
    pub fn command_encoder_mut(&mut self) -> &mut CommandEncoder<'a> {
        &mut self.command_encoder
    }

    #[inline(always)]
    pub fn copy_from_output(&mut self, dst: &RawTexture<Texture2D>) {
        let src = &self.output.texture;

        self.command_encoder.encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: src,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: unsafe { dst.inner() },
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: src.size().width,
                height: src.size().height,
                depth_or_array_layers: 1,
            },
        );
    }

    #[inline(always)]
    pub fn set_clear(&mut self, clear: Option<wgpu::Color>) -> &mut Self {
        self.clear = clear;
        self
    }

    #[inline(always)]
    pub fn set_depth_ops(&mut self, depth_ops: Option<wgpu::Operations<f32>>) -> &mut Self {
        self.depth_ops = depth_ops;
        self
    }

    #[inline(always)]
    pub fn set_stencil_ops(&mut self, stencil_ops: Option<wgpu::Operations<u32>>) -> &mut Self {
        self.stencil_ops = stencil_ops;
        self
    }

    #[inline]
    pub fn render_pass(
        &mut self,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: bool,
    ) -> RenderPass<'_> {
        self.command_encoder.render_pass(
            &self.view,
            Some(load.unwrap_or(wgpu::LoadOp::Clear(
                self.clear.unwrap_or(wgpu::Color::TRANSPARENT),
            ))),
            if depth_stencil_attachment {
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: self.depth_ops,
                    stencil_ops: self.stencil_ops,
                })
            } else {
                None
            },
        )
    }

    pub fn render_pass_with(
        &'_ mut self,
        view: &RawTextureView<Texture2D>,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: bool,
    ) -> RenderPass<'_> {
        self.command_encoder.render_pass(
            view,
            Some(load.unwrap_or(wgpu::LoadOp::Clear(
                self.clear.unwrap_or(wgpu::Color::TRANSPARENT),
            ))),
            if depth_stencil_attachment {
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: self.depth_ops,
                    stencil_ops: self.stencil_ops,
                })
            } else {
                None
            },
        )
    }

    pub fn submit(self) -> SurfaceTexture {
        self.command_encoder.submit();
        self.output
    }

    #[inline]
    pub fn present(self) {
        self.command_encoder.submit();
        self.output.present();
    }
}
