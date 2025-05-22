use std::fmt::Debug;

// use crate::prelude::*;

pub trait VertexAttr<const OFFSET: u32 = 0>: Debug + Clone {
    const ATTR: &'static [wgpu::VertexAttribute];
    const SIZE: u32;
}

pub trait VertexBufferLayout: VertexRequirements + Debug + Clone {
    const DESC: &'static [wgpu::VertexBufferLayout<'static>];
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct VertexAttrMarker<V: VertexAttr, const OFFSET: u32> {
    attr: V,
}

pub trait VertexRequirements: Debug + Clone {
    type Requirements: Debug + Clone;
}
pub type Requirements<V> = <V as VertexRequirements>::Requirements;

impl<V: VertexAttr> VertexBufferLayout for V {
    const DESC: &'static [wgpu::VertexBufferLayout<'static>] =
        &[create_vertex_layout::raw_layout_inner!(V => Vertex for 0)];
}

impl<V: VertexAttr> VertexRequirements for V {
    type Requirements = VertexAttrMarker<V, 0>;
}

pub mod create_vertex_attr {
    #[allow(unused)]
    #[macro_export]
    macro_rules! arg_len {
        ([] for $count:expr) => { $count };
        ([$_first:ident] for $count:expr) => {
            $count + 1
        };
        ([$_first:ident $(, $_marker:ident)*] for $count:expr) => {
            create_vertex_attr::arg_len!([$($_marker),*] for ($count + 1))
        };

        ([] => $count:expr) => { $count };
        ([$_first:ty => $count:expr]) => {
            $count + 1
        };
        ([$_first:ty $(, $_marker:ty)*] => $count:expr) => {
            create_vertex_attr::arg_len!([$($_marker),*] => ($count + 1))
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! attr {
        ($T:ty => [$($i:expr => $type:ident),*$(,)?]) => {
            impl<const OFFSET: u32> VertexAttr<OFFSET> for $T {
                const ATTR: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![$(OFFSET + $i => $type),*];
                const SIZE: u32 = create_vertex_attr::arg_len!([$($type),*] for 0);
            }
        };
    }

    pub use arg_len;
    pub use attr;
}

pub mod create_vertex_layout {
    #[allow(unused)]
    #[macro_export]
    macro_rules! ignore_buffer {
        ($buffer:ident) => {
            wgpu::Buffer
        };
        ($buffer:ty) => {
            wgpu::Buffer
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! raw_layout_inner {
        ($V:ty => $step_mode:ident for $offset:expr) => {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<$V>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::$step_mode,
                attributes: <$V as $crate::prelude::VertexAttr<$offset>>::ATTR,
            }
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! vertex_req_inner {
        ($L:ty: {} => ($($A:ty),*) for $count:expr) => {
            impl VertexRequirements for $L {
                #[allow(unused_parens)]
                type Requirements = ($($A),*);
                type VertexBuffers = ($(create_vertex_layout::ignore_buffer!($A)),*);
            }
        };

        ($L:ty: {$V:ty => Vertex $(,$Vs:ty => Instance)*} => ($($A:ty),*) for $count:expr) => {
            impl VertexRequirements for $L {
                #[allow(unused_parens)]
                type Requirements = ($($A,)*VertexAttrMarker<$V,$count>);
            }
        };
        ($L:ty: {$V:ty => Vertex $(,$Vs:ty => $step_mode:ident)*} => ($($A:ty),*) for $count:expr) => {
            create_vertex_layout::vertex_req_inner!($L: {$($Vs => $step_mode),*} => ($($A,)* VertexAttrMarker<$V,$count>) for { ($count + 1) });
        };
        ($L:ty: {$V:ty => Instance $(,$Vs:ty => $step_mode:ident)*} => ($($A:ty),*) for $count:expr) => {
            create_vertex_layout::vertex_req_inner!($L: {$($Vs => $step_mode),*} => ($($A),*) for { ($count + 1) });
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! raw_layout {
        ($vis:vis struct $L:ident {
            $($V:ty => $step_mode:ident for $offset:expr),*$(,)?
        }) => {
            #[derive(Debug, Clone, Copy)]
            $vis struct $L;

            impl $L {
                const fn desc() -> &'static [wgpu::VertexBufferLayout<'static>] {
                    &[$(create_vertex_layout::raw_layout_inner!($V => $step_mode for $offset)),*]
                }
            }

            impl VertexBufferLayout for $L {
                const DESC: &'static [wgpu::VertexBufferLayout<'static>] = Self::desc();
            }
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! layout_inner {
        ({ $V:ty => $step_mode:ident } => [$($xs:expr),*] for $offset:expr) => {
            [$($xs,)* wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<$V>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::$step_mode,
                attributes: <$V as VertexAttr<$offset>>::ATTR
            }]
        };

        ({ $V:ty => $step_mode:ident $(,$Vs:ty => $step_mode_s:ident)+} => [$($xs:expr),*] for $offset:expr) => {
            create_vertex_layout::layout_inner!(
                {$($Vs => $step_mode_s),*} => [$($xs,)* (wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<$V>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::$step_mode,
                    attributes: <$V as VertexAttr<$offset>>::ATTR
                })] for { ($offset + <$V as VertexAttr>::SIZE) }
            )
        };
    }

    #[allow(unused)]
    #[macro_export]
    macro_rules! layout {
        ($vis:vis $L:ident {
            $($V:ty => $step_mode:ident),*$(,)?
        }) => {
            #[derive(Debug, Clone, Copy)]
            $vis struct $L;

            impl $L {
                const fn desc() -> &'static [wgpu::VertexBufferLayout<'static>] {
                    const ARR: [wgpu::VertexBufferLayout<'static>; create_vertex_attr::arg_len!([$($step_mode),*] for 0)] =
                        create_vertex_layout::layout_inner!({ $($V => $step_mode),* } => [] for 0);

                    &ARR
                }
            }

            impl VertexBufferLayout for $L {
                const DESC: &'static [wgpu::VertexBufferLayout<'static>] = Self::desc();
            }

            create_vertex_layout::vertex_req_inner!($L: { $($V => $step_mode),* } => () for 0);
        }
    }

    pub use ignore_buffer;
    pub use layout;
    pub use layout_inner;
    pub use raw_layout;
    pub use raw_layout_inner;
    pub use vertex_req_inner;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Vertex {
        pub pos: [f32; 3],
        pub color: [f32; 4],
    }

    #[derive(Debug, Clone)]
    pub struct VertexExtra {
        pub vibe: f32,
    }

    #[derive(Debug, Clone)]
    pub struct Instance {
        pub pos: [f32; 2],
        pub rot: f32,
    }

    create_vertex_attr::attr!(Vertex => [
        0 => Float32x3,
        1 => Float32x4,
    ]);

    create_vertex_attr::attr!(VertexExtra => [
        0 => Float32,
    ]);

    create_vertex_attr::attr!(Instance => [
        0 => Float32x2,
        1 => Float32,
    ]);

    create_vertex_layout::raw_layout!(struct RawVertexLayout {
        Vertex => Vertex for 0,
        Instance => Instance for 2,
        VertexExtra => Vertex for 4,
    });

    impl VertexRequirements for RawVertexLayout {
        type Requirements = (
            VertexAttrMarker<Vertex, 0>,
            VertexAttrMarker<VertexExtra, 2>,
        );
    }

    create_vertex_layout::layout!(VertexLayout {
        Vertex => Vertex,
        Instance => Instance,
        VertexExtra => Vertex,
    });

    #[test]
    fn check_layout() {
        assert_eq!(RawVertexLayout::DESC, VertexLayout::DESC);
    }

    // Compile time checks that it all generates correctly
    fn _req(req: <RawVertexLayout as VertexRequirements>::Requirements) {
        let _: <VertexLayout as VertexRequirements>::Requirements = req;
    }
}
