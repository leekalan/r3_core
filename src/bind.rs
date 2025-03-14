pub mod buffer;

pub trait Bind {
    fn bind_group(&self) -> &wgpu::BindGroup;
}

pub trait BindLayout {
    fn layout(&self) -> &wgpu::BindGroupLayout;
}

pub mod create_bind {
    #[allow(unused)]
    #[macro_export]
    macro_rules! bind {
        ($bind:ident, $bind_layout:ident { $($buffer:ident: $ty:ty => $binding:literal for $visibility:ident),* } ) => {
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
                            entries: &[$(
                                wgpu::BindGroupLayoutEntry {
                                    binding: $binding,
                                    visibility: wgpu::ShaderStages::$visibility,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Uniform,
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    count: None,
                                }
                            ),*],
                            label: None,
                        }),
                    }
                }
            }

            impl BindLayout for $bind_layout {
                #[inline(always)]
                fn layout(&self) -> &wgpu::BindGroupLayout {
                    &self.layout
                }
            }

            #[allow(unused)]
            #[derive(Debug, Clone)]
            pub struct $bind {
                bind_group: wgpu::BindGroup,
                layout: $bind_layout,
                $(
                    $buffer: UniformBuffer<$ty>,
                )*
            }

            #[allow(unused)]
            impl $bind {
                #[inline]
                pub fn new(
                    render_context: &RenderContext,
                    layout: $bind_layout,
                    $(
                        $buffer: UniformBuffer<$ty>,
                    )*
                ) -> Self {
                    let device = unsafe { render_context.device() };

                    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                        layout: &layout.layout(),
                        entries: &[$(
                            wgpu::BindGroupEntry {
                                binding: $binding,
                                resource: unsafe { $buffer.buffer() }.as_entire_binding(),
                            }
                        ),*],
                        label: None,
                    });

                    Self {
                        bind_group,
                        layout,
                        $(
                            $buffer,
                        )*
                    }
                }

                #[inline]
                pub const fn layout(&self) -> &$bind_layout {
                    &self.layout
                }

                $(
                    #[inline]
                    pub const fn $buffer(&self) -> &UniformBuffer<$ty> {
                        &self.$buffer
                    }
                )*
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
}

#[cfg(test)]
mod bind_tests {
    use crate::prelude::*;

    create_bind::bind!(SizeBind, SizeBindLayout {
        width: f32 => 0 for VERTEX,
        height: f32 => 1 for VERTEX
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
