use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ShadedMesh<M: Mesh<Requirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> {
    mesh: M,
    shader: S,
}

impl<M: Mesh<Requirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> ShadedMesh<M, S> {
    #[inline(always)]
    pub fn new(mesh: M, shader: S) -> Self {
        Self { mesh, shader }
    }
}

impl<M: Mesh<Requirements<VertexLayout<S::Layout>>>, S: ApplyShaderInstance> Surface
    for ShadedMesh<M, S>
{
    type Layout = S::Layout;

    #[inline]
    fn draw(&self, render_pass: &mut RenderPass<S::Layout>) {
        render_pass.apply_shader(&self.shader);
        render_pass.draw_mesh(&self.mesh);
    }
}
