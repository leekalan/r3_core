use std::sync::Arc;

use crate::prelude::*;

pub struct Mesh<L: Layout, I: index_format::IndexFormat> {
    raw_mesh: RawMesh<L::Vertex, I>,
    shader: Arc<ShaderHandle<L>>,
}

impl<L: Layout, I: index_format::IndexFormat> Mesh<L, I> {
    pub fn new(raw_mesh: RawMesh<L::Vertex, I>, shader: Arc<ShaderHandle<L>>) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<L: Layout, I: index_format::IndexFormat> Surface for Mesh<L, I> {
    type Layout = L;

    fn draw<'r>(&self, render_pass: RenderPass<'r, Void>) -> RenderPass<'r, L> {
        let mut rp = render_pass.apply_shader(&*self.shader);
        rp.draw_mesh(&self.raw_mesh);
        rp
    }
}

pub struct ArcMesh<L: Layout, I: index_format::IndexFormat> {
    raw_mesh: Arc<RawMesh<L::Vertex, I>>,
    shader: Arc<ShaderHandle<L>>,
}

impl<L: Layout, I: index_format::IndexFormat> ArcMesh<L, I> {
    pub fn new(raw_mesh: Arc<RawMesh<L::Vertex, I>>, shader: Arc<ShaderHandle<L>>) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<L: Layout, I: index_format::IndexFormat> Surface for ArcMesh<L, I> {
    type Layout = L;

    fn draw<'r>(&self, render_pass: RenderPass<'r, Void>) -> RenderPass<'r, L> {
        let mut rp = render_pass.apply_shader(&*self.shader);
        rp.draw_mesh(&*self.raw_mesh);
        rp
    }
}

pub struct ExtendedSurface<S: Surface, I: index_format::IndexFormat> {
    surface: S,
    raw_mesh: RawMesh<LayoutVertex<S::Layout>, I>,
}

impl<S: Surface, I: index_format::IndexFormat> ExtendedSurface<S, I> {
    pub fn new(surface: S, raw_mesh: RawMesh<LayoutVertex<S::Layout>, I>) -> Self {
        Self { surface, raw_mesh }
    }
}

pub trait SurfaceExt: Sized + Surface {
    fn extended<I: index_format::IndexFormat>(
        self,
        raw_mesh: RawMesh<LayoutVertex<Self::Layout>, I>,
    ) -> ExtendedSurface<Self, I>;
}

impl<S: Surface> SurfaceExt for S {
    fn extended<I: index_format::IndexFormat>(
        self,
        raw_mesh: RawMesh<LayoutVertex<S::Layout>, I>,
    ) -> ExtendedSurface<S, I> {
        ExtendedSurface::new(self, raw_mesh)
    }
}

impl<S: Surface, I: index_format::IndexFormat> Surface for ExtendedSurface<S, I> {
    type Layout = S::Layout;

    fn draw<'r>(&self, render_pass: RenderPass<'r, Void>) -> RenderPass<'r, S::Layout> {
        let mut surface = self.surface.draw(render_pass);
        surface.draw_mesh(&self.raw_mesh);
        surface
    }
}
