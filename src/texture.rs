use std::ops::Deref;

use crate::prelude::*;
use image::GenericImageView;

#[derive(Debug, Clone)]
pub struct Texture<DIMENSION: TextureDimension = Texture2D> {
    pub texture: RawTexture<DIMENSION>,
    pub sampler: Sampler,
}

pub mod texture_dimension {
    pub trait TextureDimension: Default {
        const DIMENSION: wgpu::TextureDimension;
        const VIEW_DIMENSION: wgpu::TextureViewDimension;
    }
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Texture1D;
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Texture2D;
    #[derive(Default, Debug, Clone, Copy)]
    pub struct Texture3D;

    impl TextureDimension for Texture1D {
        const DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D1;
        const VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D1;
    }
    impl TextureDimension for Texture2D {
        const DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D2;
        const VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;
    }
    impl TextureDimension for Texture3D {
        const DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D3;
        const VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D3;
    }
}
use texture_dimension::TextureDimension;
pub use texture_dimension::{Texture1D, Texture2D, Texture3D};

impl<DIMENSION: TextureDimension> Texture<DIMENSION> {
    /// # Safety
    /// This function is unsafe because the sampler may conflict with the texture
    #[inline(always)]
    pub unsafe fn from_raw(texture: RawTexture<DIMENSION>, sampler: Sampler) -> Self {
        Self { texture, sampler }
    }

    /// # Safety
    /// This function is unsafe because it has no checks for anything
    pub unsafe fn from_bytes(
        render_context: &RenderContext,
        bytes: &[u8],
        sampler_cfg: SamplerConfig,
    ) -> Self {
        let img = image::load_from_memory(bytes).unwrap();
        Self::from_image(render_context, &img, sampler_cfg)
    }

    #[inline(always)]
    pub fn new(
        render_context: &RenderContext,
        size: wgpu::Extent3d,
        texture_cfg: TextureConfig<DIMENSION>,
        sampler_cfg: &SamplerConfig,
    ) -> Self {
        let texture = RawTexture::new(render_context, size, texture_cfg);

        let sampler = Sampler::new(render_context, sampler_cfg);

        Self { texture, sampler }
    }

    pub fn from_image(
        render_context: &RenderContext,
        img: &image::DynamicImage,
        sampler_cfg: SamplerConfig,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = Self::new(
            render_context,
            size,
            TextureConfig::<DIMENSION>::default(),
            &sampler_cfg,
        );

        let queue = unsafe { render_context.queue() };
        queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                aspect: wgpu::TextureAspect::All,
                texture: unsafe { texture.texture.inner() },
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        texture
    }
}

impl Texture<Texture2D> {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        render_context: &RenderContext,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let cfg = TextureConfig::<Texture2D> {
            label: Some("Depth Texture"),
            usage: Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            ),
            format: Some(Self::DEPTH_FORMAT),
            ..Default::default()
        };

        let sampler_config = SamplerConfig::Create(wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self::new(render_context, size, cfg, &sampler_config)
    }
}

#[derive(Debug, Clone)]
pub struct RawTexture<DIMENSION: TextureDimension = Texture2D> {
    texture: wgpu::Texture,
    view: RawTextureView<DIMENSION>,
    view_formats: Box<[wgpu::TextureFormat]>,
    __dimension: PhantomData<DIMENSION>,
}

#[derive(Default, Debug, Clone)]
pub struct TextureConfig<'a, DIMENSION: TextureDimension = Texture2D> {
    pub label: Option<&'a str>,
    pub mip_level_count: Option<u32>,
    pub sample_count: Option<u32>,
    pub __dimension: PhantomData<DIMENSION>,
    pub format: Option<wgpu::TextureFormat>,
    pub usage: Option<wgpu::TextureUsages>,
    pub view_formats: Option<Box<[wgpu::TextureFormat]>>,
}

impl<DIMENSION: TextureDimension> RawTexture<DIMENSION> {
    pub fn new(
        render_context: &RenderContext,
        size: wgpu::Extent3d,
        cfg: TextureConfig<DIMENSION>,
    ) -> Self {
        let view_formats = cfg.view_formats.unwrap_or(vec![].into_boxed_slice());

        let texture_desc = wgpu::TextureDescriptor {
            label: cfg.label,
            size,
            mip_level_count: cfg.mip_level_count.unwrap_or(1),
            sample_count: cfg.sample_count.unwrap_or(1),
            dimension: DIMENSION::DIMENSION,
            format: cfg.format.unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb),
            usage: cfg.usage.unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING),
            view_formats: &view_formats,
        };

        let texture = unsafe { render_context.device().create_texture(&texture_desc) };

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: cfg.label,
            format: cfg.format,
            dimension: Some(DIMENSION::VIEW_DIMENSION),
            usage: Some(cfg.usage.unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING)),
            mip_level_count: Some(cfg.mip_level_count.unwrap_or(1)),
            ..default()
        });
        let view = unsafe { RawTextureView::new(view) };

        Self {
            texture,
            view,
            view_formats,
            __dimension: PhantomData,
        }
    }

    pub fn resize(
        &mut self,
        render_context: &RenderContext,
        label: Option<&str>,
        size: wgpu::Extent3d,
    ) {
        self.texture = unsafe {
            render_context
                .device()
                .create_texture(&wgpu::TextureDescriptor {
                    label,
                    size,
                    mip_level_count: self.texture.mip_level_count(),
                    sample_count: self.texture.sample_count(),
                    dimension: DIMENSION::DIMENSION,
                    format: self.texture.format(),
                    usage: self.texture.usage(),
                    view_formats: &self.view_formats,
                })
        };

        let texture_view: wgpu::TextureView =
            self.texture.create_view(&wgpu::TextureViewDescriptor {
                label,
                format: Some(self.texture.format()),
                dimension: Some(DIMENSION::VIEW_DIMENSION),
                usage: Some(self.texture.usage()),
                mip_level_count: Some(self.texture.mip_level_count()),
                ..default()
            });
        self.view = unsafe { RawTextureView::new(texture_view) };
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Texture`
    pub const unsafe fn inner(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::TextureView`
    pub unsafe fn view(&self) -> &RawTextureView<DIMENSION> {
        &self.view
    }

    /// # Safety
    /// This function is unsafe because it has no checks for anything
    pub unsafe fn copy_to(&self, dst: &Self, encoder: &mut CommandEncoder) {
        encoder.encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: &dst.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: self.texture.size().width,
                height: self.texture.size().height,
                depth_or_array_layers: 1,
            },
        );
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RawTextureView<DIMENSION: TextureDimension = Texture2D> {
    view: wgpu::TextureView,
    __dimension: PhantomData<DIMENSION>,
}
impl<DIMENSION: TextureDimension> RawTextureView<DIMENSION> {
    /// # Safety
    /// Does not check the validity of DIMENSION
    pub const unsafe fn new(texture: wgpu::TextureView) -> Self {
        Self {
            view: texture,
            __dimension: PhantomData,
        }
    }

    pub const fn inner(&self) -> &wgpu::TextureView {
        &self.view
    }
}
impl Deref for RawTextureView<Texture2D> {
    type Target = wgpu::TextureView;
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Sampler {
    sampler: wgpu::Sampler,
}

#[derive(Debug, Clone)]
pub enum SamplerConfig<'a> {
    Create(wgpu::SamplerDescriptor<'a>),
    Clone(&'a wgpu::Sampler),
}
impl SamplerConfig<'_> {
    pub fn unwrap(&self, render_context: &RenderContext) -> wgpu::Sampler {
        match self {
            SamplerConfig::Create(cfg) => unsafe { render_context.device().create_sampler(cfg) },
            SamplerConfig::Clone(sampler) => (*sampler).clone(),
        }
    }
}
impl Default for SamplerConfig<'static> {
    fn default() -> Self {
        Self::Create(wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
    }
}

impl Sampler {
    pub fn new(render_context: &RenderContext, cfg: &SamplerConfig) -> Self {
        Self {
            sampler: cfg.unwrap(render_context),
        }
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Sampler`
    pub const unsafe fn inner(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
