use std::{fmt::Debug, marker::PhantomData, ops::Range};

use wgpu::util::DeviceExt;

use crate::prelude::*;

pub mod index_format {
    use std::fmt::Debug;

    pub trait IndexFormat: Debug {
        const FORMAT: wgpu::IndexFormat;
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Uint16;

    impl IndexFormat for Uint16 {
        const FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
    }

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Uint32;

    impl IndexFormat for Uint32 {
        const FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
    }
}

pub trait Mesh<Requirements>: Debug {
    /// # Safety
    /// This function is unsafe because the caller must ensure
    /// that the render pass meets the requirements
    unsafe fn draw(&self, render_pass: &mut wgpu::RenderPass);
    /// # Safety
    /// This function is unsafe because the caller must ensure
    /// that the render pass meets the requirements
    unsafe fn draw_instanced(&self, render_pass: &mut wgpu::RenderPass, instances: Range<u32>);
}

macro_rules! simple_mesh_impl {
    ($Mesh:ident => $(($buffer:ident: <$As:ident, $Ns:ident>),)*) => {
        #[derive(Debug, Clone)]
        pub struct $Mesh<$($As: VertexAttr + bytemuck::NoUninit,)* I: index_format::IndexFormat> {
            $($buffer: wgpu::Buffer,)*
            index_buffer: wgpu::Buffer,
            index_count: u32,
            __index_format: PhantomData<I>,
            #[allow(unused_parens)]
            __vertices: PhantomData<($($As),*)>,
        }

        impl<
            $($As: VertexAttr + bytemuck::NoUninit,)*
            I: index_format::IndexFormat
        > $Mesh<$($As,)* I> {
            /// # Safety
            /// This function is unsafe because the caller must ensure
            /// that the generic `V` matches with `vertex_buffer`
            #[inline(always)]
            pub const unsafe fn from_raw(
                $($buffer: wgpu::Buffer,)*
                index_buffer: wgpu::Buffer,
                index_count: u32,
            ) -> Self {
                Self {
                    $($buffer,)*
                    index_buffer,
                    index_count,
                    __index_format: PhantomData,
                    __vertices: PhantomData,
                }
            }

            pub fn new_uint16(render_context: &RenderContext, $($buffer: &[$As],)* indices: &[u16]) -> Self {
                let device = unsafe { render_context.device() };

                $(
                    let $buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice($buffer),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                )*

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                unsafe { Self::from_raw($($buffer,)* index_buffer, indices.len() as u32) }
            }

            pub fn new_uint32(render_context: &RenderContext, $($buffer: &[$As],)* indices: &[u32]) -> Self {
                let device = unsafe { render_context.device() };

                $(
                    let $buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice($buffer),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                )*

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                unsafe { Self::from_raw($($buffer,)* index_buffer, indices.len() as u32) }
            }
        }

        #[allow(unused_parens)]
        impl<
            $($As: VertexAttr + bytemuck::NoUninit, const $Ns: u32,)*
            I: index_format::IndexFormat
        > Mesh<($(VertexAttrMarker<$As, $Ns>),*)> for $Mesh<$($As,)* I> {
            unsafe fn draw(&self, render_pass: &mut wgpu::RenderPass) {
                $(
                    render_pass.set_vertex_buffer($Ns, self.$buffer.slice(..));
                )*

                render_pass.set_index_buffer(self.index_buffer.slice(..), I::FORMAT);

                render_pass.draw_indexed(0..self.index_count, 0, 0..1);
            }

            unsafe fn draw_instanced(&self, render_pass: &mut wgpu::RenderPass, instances: Range<u32>) {
                $(
                    render_pass.set_vertex_buffer($Ns, self.$buffer.slice(..));
                )*

                render_pass.set_index_buffer(self.index_buffer.slice(..), I::FORMAT);

                render_pass.draw_indexed(0..self.index_count, 0, instances);
            }
        }
    };
}

simple_mesh_impl! { SimpleMesh0 => }
simple_mesh_impl! { SimpleMesh =>
    (buffer: <A, N>),
}
simple_mesh_impl! { SimpleMesh2 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>),
}
simple_mesh_impl! { SimpleMesh3 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
}
simple_mesh_impl! { SimpleMesh4 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
    (buffer4: <A4, N4>),
}
simple_mesh_impl! { SimpleMesh5 =>
    (buffer1: <A1, N1>), (buffer2: <A2, N2>), (buffer3: <A3, N3>),
    (buffer4: <A4, N4>), (buffer5: <A5, N5>),
}
