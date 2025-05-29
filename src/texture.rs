use std::ops::Deref;

use crate::prelude::*;

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
    #[inline(always)]
    pub fn new(texture: RawTexture<DIMENSION>, sampler: Sampler) -> Self {
        Self { texture, sampler }
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
            usages: Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            ),
            format: Some(Self::DEPTH_FORMAT),
            ..Default::default()
        };

        let sampler_config = wgpu::SamplerDescriptor {
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
        };

        let raw_texture = RawTexture::new(render_context, size, &cfg);
        let sampler = Sampler::new(render_context, &sampler_config);

        Self::new(raw_texture, sampler)
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
    pub usages: Option<wgpu::TextureUsages>,
    pub view_formats: Option<Box<[wgpu::TextureFormat]>>,
}

impl<DIMENSION: TextureDimension> RawTexture<DIMENSION> {
    pub fn new(
        render_context: &RenderContext,
        size: wgpu::Extent3d,
        cfg: &TextureConfig<DIMENSION>,
    ) -> Self {
        let view_formats = cfg
            .view_formats
            .clone()
            .unwrap_or(vec![].into_boxed_slice());

        let texture_desc = wgpu::TextureDescriptor {
            label: cfg.label,
            size,
            mip_level_count: cfg.mip_level_count.unwrap_or(1),
            sample_count: cfg.sample_count.unwrap_or(1),
            dimension: DIMENSION::DIMENSION,
            format: cfg.format.unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb),
            usage: cfg
                .usages
                .unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
            view_formats: &view_formats,
        };

        let texture = unsafe { render_context.device().create_texture(&texture_desc) };

        let view =
            texture.create_view(&wgpu::TextureViewDescriptor {
                label: cfg.label,
                format: cfg.format,
                dimension: Some(DIMENSION::VIEW_DIMENSION),
                usage: Some(cfg.usages.unwrap_or(
                    wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                )),
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

    #[inline(always)]
    pub fn from_data(
        render_context: &RenderContext,
        data: &[u8],
        size: wgpu::Extent3d,
        bytes_per_pixel: Option<u8>,
        cfg: &TextureConfig<DIMENSION>,
    ) -> Self {
        let texture = Self::new(render_context, size, cfg);

        texture.write_data(render_context, data, size, bytes_per_pixel);

        texture
    }

    pub fn write_data(
        &self,
        render_context: &RenderContext,
        data: &[u8],
        size: wgpu::Extent3d,
        bytes_per_pixel: Option<u8>,
    ) {
        let queue = unsafe { render_context.queue() };

        queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                aspect: wgpu::TextureAspect::All,
                texture: unsafe { self.inner() },
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_pixel.unwrap_or(4) as u32 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );
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

impl Sampler {
    /// #[inline(always)]
    pub fn new(render_context: &RenderContext, cfg: &wgpu::SamplerDescriptor) -> Self {
        Self {
            sampler: unsafe { render_context.device().create_sampler(cfg) },
        }
    }

    /// # Safety
    /// This function is unsafe because it returns the inner `wgpu::Sampler`
    /// #[inline(always)]
    pub const unsafe fn inner(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
