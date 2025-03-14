use crate::prelude::*;

pub trait Surface {
    type Layout: Layout;

    fn draw(&self, render_pass: &mut RenderPass<Self::Layout>);
}
