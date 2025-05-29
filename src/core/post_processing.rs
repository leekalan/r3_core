use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PostProc {
    bind: PostProcBind,
}

create_bind::bind!(PostProcBind, PostProcBindLayout {
    Textures => {
        render: Texture2D => 0 for FRAGMENT use {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
        },
    },
    Samplers => {
        sampler => 1 for FRAGMENT,
    },
});

impl PostProc {
    #[inline]
    pub fn new(
        render_context: &RenderContext,
        usage: wgpu::TextureUsages,
        width: u32,
        height: u32,
    ) -> Self {
        let layout = PostProcBindLayout::new(render_context);

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
                            | wgpu::TextureUsages::TEXTURE_BINDING
                            | usage,
                    ),
                    ..default()
                },
            ),
            Sampler::new(render_context, &wgpu::SamplerDescriptor::default()),
        );

        Self {
            bind: PostProcBind::new(render_context, layout, texture.texture, texture.sampler),
        }
    }

    pub fn new_with(render_context: &RenderContext, texture: Texture) -> Self {
        let layout = PostProcBindLayout::new(render_context);
        Self {
            bind: PostProcBind::new(render_context, layout, texture.texture, texture.sampler),
        }
    }

    #[inline(always)]
    pub fn bind(&self) -> &PostProcBind {
        &self.bind
    }

    #[inline(always)]
    pub fn raw_texture(&self) -> &RawTexture {
        self.bind.render()
    }

    #[inline(always)]
    pub fn sampler(&self) -> &Sampler {
        self.bind.sampler()
    }

    pub fn clone_texture(&self) -> Texture {
        Texture::new(self.raw_texture().clone(), self.sampler().clone())
    }

    pub fn resize(&mut self, render_context: &RenderContext, width: u32, height: u32) {
        self.bind.render.resize(
            render_context,
            None,
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}

impl Deref for PostProc {
    type Target = PostProcBind;

    fn deref(&self) -> &Self::Target {
        &self.bind
    }
}

impl DerefMut for PostProc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bind
    }
}
