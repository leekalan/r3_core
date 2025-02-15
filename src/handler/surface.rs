use crate::prelude::*;

pub trait Surface<V: Vertex> {
    fn draw<'r>(&self, render_pass: RenderPass<'r, ()>) -> RenderPass<'r, V>;
}
