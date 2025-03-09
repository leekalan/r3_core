use crate::prelude::*;

pub trait Surface {
    type Layout: Layout;

    fn draw<'r>(&self, render_pass: RenderPass<'r, Void>) -> RenderPass<'r, Self::Layout>;
}
