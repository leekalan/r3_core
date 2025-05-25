use crate::prelude::*;

pub mod mesh;

pub trait Surface
where
    VertexLayout<Self::Layout>: NoInstanceRequirements,
{
    type Layout: Layout;

    fn draw<const SA: bool>(&self, render_pass: &mut RenderPass<Self::Layout, SA>);
}

pub trait SurfaceInstanced
where
    VertexLayout<Self::Layout>: InstanceRequirements,
{
    type Layout: Layout;

    fn draw<const SA: bool>(&self, render_pass: &mut RenderPassInstanced<Self::Layout, SA>);
}
