use crate::{prelude::*, surface::SurfaceInstanced};

#[derive(Debug, Clone)]
pub struct ShadedMesh<M, S> {
    mesh: M,
    shader: S,
}

impl<M: Mesh<VRequirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> ShadedMesh<M, S> {
    #[inline(always)]
    pub fn new(mesh: M, shader: S) -> Self {
        Self { mesh, shader }
    }
}

impl<M: Mesh<VRequirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> Surface
    for ShadedMesh<M, S>
where
    VertexLayout<S::Layout>: NoInstanceRequirements,
{
    type Layout = S::Layout;

    #[inline]
    fn draw<const SA: bool>(&self, render_pass: &mut RenderPass<S::Layout, SA>) {
        render_pass
            .apply_shader_ref(&self.shader)
            .draw_mesh(&self.mesh);
    }
}

impl<M: Mesh<VRequirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> SurfaceInstanced
    for ShadedMesh<M, S>
where
    VertexLayout<S::Layout>: InstanceRequirements,
{
    type Layout = S::Layout;

    #[inline]
    fn draw<const SA: bool>(&self, render_pass: &mut RenderPassInstanced<S::Layout, SA>) {
        render_pass
            .apply_shader_ref(&self.shader)
            .draw_mesh(&self.mesh);
    }
}
