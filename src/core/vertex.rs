use crate::prelude::*;

impl VertexBufferLayout for Void {
    const DESC: &'static [wgpu::VertexBufferLayout<'static>] = &[];
}

impl VertexRequirements for Void {
    type Requirements = ();
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PosVertex {
    pub position: [f32; 3],
}

create_vertex_attr::attr!(PosVertex => [
    0 => Float32x3,
]);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PosVertex2d {
    pub position: [f32; 2],
}

create_vertex_attr::attr!(PosVertex2d => [
    0 => Float32x2,
]);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RGBVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

create_vertex_attr::attr!(RGBVertex => [
    0 => Float32x3,
    1 => Float32x3,
]);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RBGAVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

create_vertex_attr::attr!(RBGAVertex => [
    0 => Float32x3,
    1 => Float32x4,
]);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UVVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

create_vertex_attr::attr!(UVVertex => [
    0 => Float32x3,
    1 => Float32x2,
]);
