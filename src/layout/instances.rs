use std::{fmt::Debug, ops::Range};

use wgpu::util::DeviceExt;

use crate::prelude::*;

pub trait Instances<Requirements>: Debug {
    /// # Safety
    /// This function is unsafe because the caller must ensure
    /// that the render pass meets the requirements
    unsafe fn set_vertex_buffers(&self, render_pass: &mut wgpu::RenderPass);

    fn range(&self) -> Range<u32>;
}

macro_rules! get_len {
    ($buffer_first:ident) => { $buffer_first.len() };
    ($buffer_first:ident, $buffer_other:ident $(,$buffers:ident)*) => {
        {
            assert_eq!($buffer_first.len(), $buffer_other.len());
            get_len!($buffer_first $(,$buffers)*)
        }
    };
}

macro_rules! simple_instances_impl {
    ($Instances:ident => $(($buffer:ident: <$As:ident, $Ns:ident>),)*) => {
        #[derive(Debug, Clone)]
        pub struct $Instances<$($As: VertexAttr + bytemuck::NoUninit),*> {
            $($buffer: wgpu::Buffer,)*
            instance_count: u32,
            #[allow(unused_parens)]
            __instances: PhantomData<($($As),*)>,
        }

        impl<
            $($As: VertexAttr + bytemuck::NoUninit),*
        > $Instances<$($As),*> {
            /// # Safety
            /// This function is unsafe because the caller must ensure
            /// that the generic `V` matches with `vertex_buffer`
            #[inline(always)]
            pub const unsafe fn from_raw(
                $($buffer: wgpu::Buffer,)*
                instance_count: u32
            ) -> Self {
                Self {
                    $($buffer,)*
                    instance_count,
                    __instances: PhantomData
                }
            }

            pub fn new(render_context: &RenderContext, $($buffer: &[$As]),*) -> Self {
                let device = unsafe { render_context.device() };

                let instance_count = get_len!($($buffer),*) as u32;

                $(
                    let $buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice($buffer),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                )*

                Self {
                    $($buffer,)*
                    instance_count,
                    __instances: PhantomData,
                }
            }
        }

        #[allow(unused_parens)]
        impl<
            $($As: VertexAttr + bytemuck::NoUninit, const $Ns: u32),*
        > Instances<($(VertexAttrMarker<$As, $Ns>),*)> for $Instances<$($As),*> {
            unsafe fn set_vertex_buffers(&self, render_pass: &mut wgpu::RenderPass) {
                $(
                    render_pass.set_vertex_buffer($Ns, self.$buffer.slice(..));
                )*
            }

            fn range(&self) -> Range<u32> {
                0..self.instance_count
            }
        }
    }
}

simple_instances_impl! { SimpleInstances =>
    (buffer: <A, N>),
}
simple_instances_impl! { SimpleInstances2 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>),
}
simple_instances_impl! { SimpleInstances3 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
}
simple_instances_impl! { SimpleInstances4 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
    (buffer4: <A4, N4>),
}
simple_instances_impl! { SimpleInstances5 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
    (buffer4: <A4, N4>), (buffer5: <A5, N5>),
}
