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
            render_context,
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureConfig {
                usage: Some(
                    wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                ),
                format: Some(Self::TEXTURE_FORMAT),
                ..default()
            },
            &SamplerConfig::Create(wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Nearest,
                ..default()
            }),
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
