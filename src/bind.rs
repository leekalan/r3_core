pub mod buffer;

pub trait Bind {
    fn bind_group(&self) -> &wgpu::BindGroup;
}

pub trait BindLayout {
    fn wgpu_layout(&self) -> &wgpu::BindGroupLayout;
}

pub mod create_bind {
    #[allow(unused)]
    #[macro_export]
    macro_rules! unwrap_or_default {
        ($val:expr, $default:expr) => {
            $val
        };
        (, $default:expr) => {
            $default
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! bind {
        ($bind:ident, $bind_layout:ident {
            $(Buffers => {
                $($buffer:ident: $ty:ty => $binding:literal for $visibility:ident,)*
            },)?
            $(Textures => {
                $($texture:ident: $DIMENSION:ty $(| for $t_count:literal)? => $t_binding:literal for $t_visibility:ident,)*
            },)?
            $(Samplers => {
                $($sampler:ident $(for $s_count:literal)? => $s_binding:literal for $s_visibility:ident,)*
            },)?
        }) => {
            #[allow(unused)]
            #[repr(transparent)]
            #[derive(Debug, Clone)]
            pub struct $bind_layout {
                layout: wgpu::BindGroupLayout,
            }

            #[allow(unused)]
            impl $bind_layout {
                #[inline]
                pub fn new(
                    render_context: &RenderContext,
                ) -> Self {
                    let device = unsafe { render_context.device() };

                    Self {
                        layout: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            entries: &[$($(
                                wgpu::BindGroupLayoutEntry {
                                    binding: $binding,
                                    visibility: wgpu::ShaderStages::$visibility,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Uniform,
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    count: None,
                                },
                            )*)?
                            $($(
                                wgpu::BindGroupLayoutEntry {
                                    binding: $t_binding,
                                    visibility: wgpu::ShaderStages::$t_visibility,
                                    ty: wgpu::BindingType::Texture {
                                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                        view_dimension: wgpu::TextureViewDimension::D2,
                                        multisampled: false,
                                    },
                                    count: create_bind::unwrap_or_default!(
                                        $(Some(std::num::NonZero::new($t_count).unwrap()))?,
                                        None
                                    ),
                                },
                            )*)?
                            $($(
                                wgpu::BindGroupLayoutEntry {
                                    binding: $s_binding,
                                    visibility: wgpu::ShaderStages::$s_visibility,
                                    ty: wgpu::BindingType::Sampler(
                                        wgpu::SamplerBindingType::Filtering
                                    ),
                                    count: create_bind::unwrap_or_default!(
                                        $(Some(std::num::NonZero::new($s_count).unwrap()))?,
                                        None
                                    ),
                                },
                            )*)?
                            ],
                            label: None,
                        }),
                    }
                }
            }

            impl BindLayout for $bind_layout {
                #[inline(always)]
                fn wgpu_layout(&self) -> &wgpu::BindGroupLayout {
                    &self.layout
                }
            }

            #[allow(unused)]
            #[derive(Debug, Clone)]
            pub struct $bind {
                bind_group: wgpu::BindGroup,
                layout: $bind_layout,
                $($(
                    pub $buffer: UniformBuffer<$ty>,
                )*)?
                $($(
                    pub $texture: RawTexture<$DIMENSION>,
                )*)?
                $($(
                    pub $sampler: Sampler,
                )*)?
            }

            impl std::ops::Deref for $bind {
                type Target = $bind_layout;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    &self.layout
                }
            }

            #[allow(unused)]
            impl $bind {
                #[inline]
                pub fn new(
                    render_context: &RenderContext,
                    layout: $bind_layout,
                    $($(
                        $buffer: UniformBuffer<$ty>,
                    )*)?
                    $($(
                        $texture: RawTexture<$DIMENSION>,
                    )*)?
                    $($(
                        $sampler: Sampler,
                    )*)?
                ) -> Self {
                    let device = unsafe { render_context.device() };

                    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &layout.wgpu_layout(),
                        entries: &[
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $binding,
                                    resource: unsafe { $buffer.buffer() }.as_entire_binding(),
                                }
                            ,)*)?
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $t_binding,
                                    resource: wgpu::BindingResource::TextureView(
                                        unsafe { &$texture.view() }
                                    ),
                                }
                            ,)*)?
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $s_binding,
                                    resource: wgpu::BindingResource::Sampler(
                                        unsafe { &$sampler.inner() }
                                    ),
                                }
                            ,)*)?
                        ],
                        label: None,
                    });

                    Self {
                        bind_group,
                        layout,
                        $($(
                            $buffer,
                        )*)?
                        $($(
                            $texture,
                        )*)?
                        $($(
                            $sampler,
                        )*)?
                    }
                }

                #[inline]
                pub fn refresh(&mut self, render_context: &RenderContext) {
                    let device = unsafe { render_context.device() };

                    let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &self.layout.wgpu_layout(),
                        entries: &[
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $binding,
                                    resource: unsafe { self.$buffer.buffer() }.as_entire_binding(),
                                }
                            ,)*)?
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $t_binding,
                                    resource: wgpu::BindingResource::TextureView(
                                        unsafe { &self.$texture.view() }
                                    ),
                                }
                            ,)*)?
                            $($(
                                wgpu::BindGroupEntry {
                                    binding: $s_binding,
                                    resource: wgpu::BindingResource::Sampler(
                                        unsafe { &self.$sampler.inner() }
                                    ),
                                }
                            ,)*)?
                        ],
                        label: None,
                    });

                    self.bind_group = new_bind_group;
                }

                #[inline]
                pub const fn layout(&self) -> &$bind_layout {
                    &self.layout
                }

                $($(
                    #[inline(always)]
                    pub const fn $buffer(&self) -> &UniformBuffer<$ty> {
                        &self.$buffer
                    }
                )*)?

                $($(
                    #[inline(always)]
                    pub const fn $texture(&self) -> &RawTexture<$DIMENSION> {
                        &self.$texture
                    }
                )*)?

                $($(
                    #[inline(always)]
                    pub const fn $sampler(&self) -> &Sampler {
                        &self.$sampler
                    }
                )*)?
            }

            impl Bind for $bind {
                #[inline(always)]
                fn bind_group(&self) -> &wgpu::BindGroup {
                    &self.bind_group
                }
            }
        };
    }

    pub use bind;
    pub use unwrap_or_default;
}

#[cfg(test)]
mod bind_tests {
    use crate::prelude::*;

    create_bind::bind!(SizeBind, SizeBindLayout {
        Buffers => {
            width: f32 => 0 for VERTEX,
            height: f32 => 1 for VERTEX,
        },
    });

    #[tokio::test]
    async fn test_bind() {
        let render_context = RenderContext::new(RenderContextConfig::default()).await;

        let bind_layout = SizeBindLayout::new(&render_context);

        let bind = SizeBind::new(
            &render_context,
            bind_layout,
            UniformBuffer::new_init(&render_context, 0.0),
            UniformBuffer::new_init(&render_context, 0.0),
        );

        drop(bind);
    }
}

#[cfg(test)]
mod cloning_tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn check_buffer_cloning() {
        let render_context = RenderContext::new(RenderContextConfig::default()).await;

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<f32>() as u64,
            mapped_at_creation: false,
        });
        let buffer2 = buffer.clone();
        let buffer3 = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<f32>() as u64,
            mapped_at_creation: false,
        });

        assert_eq!(buffer, buffer2);
        assert_ne!(buffer, buffer3);
        assert_ne!(buffer2, buffer3);
    }

    #[tokio::test]
    async fn check_layout_cloning() {
        let render_context = RenderContext::new(RenderContextConfig::default()).await;

        let device = unsafe { render_context.device() };

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });
        let layout2 = layout.clone();
        let layout3 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        assert_eq!(layout, layout2);
        assert_ne!(layout, layout3);
        assert_ne!(layout2, layout3);
    }

    #[tokio::test]
    async fn check_bind_group_cloning() {
        let render_context = RenderContext::new(RenderContextConfig::default()).await;

        let device = unsafe { render_context.device() };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<f32>() as u64,
            mapped_at_creation: false,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: None,
        });
        let bind_group2 = bind_group.clone();
        let bind_group3 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
            label: None,
        });

        assert_eq!(bind_group, bind_group2);
        assert_ne!(bind_group, bind_group3);
        assert_ne!(bind_group2, bind_group3);
    }
}
