use crate::prelude::*;

pub mod mesh;

pub trait Surface {
    type Layout: Layout;

    fn draw(&self, render_pass: &mut RenderPass<Self::Layout>);
}
