use crate::prelude::*;

pub mod raw_mesh;

pub trait Surface {
    type Layout: Layout;

    fn draw(&self, render_pass: &mut RenderPass<Self::Layout>);
}
