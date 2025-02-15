use std::sync::Arc;

use crate::prelude::{index_format::IndexFormat, *};

pub struct Mesh<V: Vertex, I: IndexFormat> {
    raw_mesh: RawMesh<V, I>,
    shader: Arc<ShaderHandle<V>>,
}

impl<V: Vertex, I: IndexFormat> Mesh<V, I> {
    pub fn new(raw_mesh: RawMesh<V, I>, shader: Arc<ShaderHandle<V>>) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<V: Vertex, I: IndexFormat> Surface<V> for Mesh<V, I> {
    fn draw<'r>(&self, render_pass: RenderPass<'r, ()>) -> RenderPass<'r, V> {
        let mut rp = render_pass.apply_shader(&*self.shader);
        rp.draw_mesh(&self.raw_mesh);
        rp
    }
}

pub struct ArcMesh<V: Vertex, I: IndexFormat> {
    raw_mesh: Arc<RawMesh<V, I>>,
    shader: Arc<ShaderHandle<V>>,
}

impl<V: Vertex, I: IndexFormat> ArcMesh<V, I> {
    pub fn new(raw_mesh: Arc<RawMesh<V, I>>, shader: Arc<ShaderHandle<V>>) -> Self {
        Self { raw_mesh, shader }
    }
}

impl<V: Vertex, I: IndexFormat> Surface<V> for ArcMesh<V, I> {
    fn draw<'r>(&self, render_pass: RenderPass<'r, ()>) -> RenderPass<'r, V> {
        let mut rp = render_pass.apply_shader(&*self.shader);
        rp.draw_mesh(&*self.raw_mesh);
        rp
    }
}

pub struct ExtendedSurface<S: Surface<V>, V: Vertex, I: IndexFormat> {
    surface: S,
    raw_mesh: RawMesh<V, I>,
}

impl<S: Surface<V>, V: Vertex, I: IndexFormat> ExtendedSurface<S, V, I> {
    pub fn new(surface: S, raw_mesh: RawMesh<V, I>) -> Self {
        Self { surface, raw_mesh }
    }
}

pub trait SurfaceExt<V: Vertex>: Sized + Surface<V> {
    fn extended<I: IndexFormat>(self, raw_mesh: RawMesh<V, I>) -> ExtendedSurface<Self, V, I>;
}

impl<S: Surface<V>, V: Vertex> SurfaceExt<V> for S {
    fn extended<I: IndexFormat>(self, raw_mesh: RawMesh<V, I>) -> ExtendedSurface<S, V, I> {
        ExtendedSurface::new(self, raw_mesh)
    }
}

impl<S: Surface<V>, V: Vertex, I: IndexFormat> Surface<V> for ExtendedSurface<S, V, I> {
    fn draw<'r>(&self, render_pass: RenderPass<'r, ()>) -> RenderPass<'r, V> {
        let mut surface = self.surface.draw(render_pass);
        surface.draw_mesh(&self.raw_mesh);
        surface
    }
}
