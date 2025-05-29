use crate::prelude::*;

use super::post_processing::PostProc;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Hdr {
    post_proc: PostProc,
}

impl Hdr {
    const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

    pub fn new(render_context: &RenderContext, width: u32, height: u32) -> Self {
        let texture = Texture::new(
            RawTexture::new(
                render_context,
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                &TextureConfig {
                    usages: Some(
                        wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::TEXTURE_BINDING,
                    ),
                    format: Some(Self::TEXTURE_FORMAT),
                    ..default()
                },
            ),
            Sampler::new(
                render_context,
                &wgpu::SamplerDescriptor {
                    mag_filter: wgpu::FilterMode::Nearest,
                    ..default()
                },
            ),
        );

        Self {
            post_proc: PostProc::new_with(render_context, texture),
        }
    }

    #[inline(always)]
    pub const fn post_proc(&self) -> &PostProc {
        &self.post_proc
    }

    #[inline(always)]
    pub fn resize(&mut self, render_context: &RenderContext, width: u32, height: u32) {
        self.post_proc.resize(render_context, width, height);
    }

    #[inline(always)]
    pub fn texture(&self) -> &RawTexture {
        self.post_proc.raw_texture()
    }
}

pub trait CommandEncoderHdr {
    fn hdr_render_pass(
        &mut self,
        hdr: &Hdr,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'_>>,
    );
}

impl CommandEncoderHdr for CommandEncoder<'_> {
    #[inline(always)]
    fn hdr_render_pass(
        &mut self,
        hdr: &Hdr,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'_>>,
    ) {
        self.render_pass(
            unsafe { hdr.texture().view() },
            load,
            depth_stencil_attachment,
        );
    }
}

pub trait WindowCommandEncoderHdr {
    fn hdr_render_pass(&mut self, hdr: &Hdr, depth_stencil_attachment: bool);
}

impl WindowCommandEncoderHdr for WindowCommandEncoder<'_> {
    #[inline(always)]
    fn hdr_render_pass(&mut self, hdr: &Hdr, depth_stencil_attachment: bool) {
        self.render_pass_with(
            unsafe { hdr.texture().view() },
            None,
            depth_stencil_attachment,
        );
    }
}
