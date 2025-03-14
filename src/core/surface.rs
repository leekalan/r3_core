use crate::prelude::*;

pub struct Mesh<S: ApplyShaderInstance, I: index_format::IndexFormat = index_format::Uint16> {
    raw_mesh: RawMesh<LayoutVertex<S::Layout>, I>,
    shader: S,
}

impl<S: ApplyShaderInstance, I: index_format::IndexFormat> Mesh<S, I> {
    pub fn new(raw_mesh: RawMesh<LayoutVertex<S::Layout>, I>, shader: S) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<S: ApplyShaderInstance, I: index_format::IndexFormat> Surface for Mesh<S, I> {
    type Layout = S::Layout;

    fn draw(&self, render_pass: &mut RenderPass<S::Layout>) {
        render_pass.apply_shader(&self.shader);
        render_pass.draw_mesh(&self.raw_mesh);
    }
}

pub struct AscMesh<S: ApplyShaderInstance, I: index_format::IndexFormat = index_format::Uint16> {
    raw_mesh: Asc<RawMesh<LayoutVertex<S::Layout>, I>>,
    shader: S,
}

impl<S: ApplyShaderInstance, I: index_format::IndexFormat> AscMesh<S, I> {
    pub fn new(raw_mesh: Asc<RawMesh<LayoutVertex<S::Layout>, I>>, shader: S) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<S: ApplyShaderInstance, I: index_format::IndexFormat> Surface for AscMesh<S, I> {
    type Layout = S::Layout;

    fn draw(&self, render_pass: &mut RenderPass<S::Layout>) {
        render_pass.apply_shader(&self.shader);
        render_pass.draw_mesh(&self.raw_mesh);
    }
}

pub struct ExtendedSurface<S: Surface, I: index_format::IndexFormat = index_format::Uint16> {
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

    fn draw(&self, render_pass: &mut RenderPass<S::Layout>) {
        render_pass.draw_surface(&self.surface);
        render_pass.draw_mesh(&self.raw_mesh);
    }
}
