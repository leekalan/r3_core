use crate::prelude::RenderContext;

use super::window::{Window, WindowCommandEncoder};

pub struct App<S> {
    pub window: Window,
    pub state: S,
}

impl<S> App<S> {
    #[inline]
    pub fn new(window: Window, state: S) -> Self {
        Self { window, state }
    }

    pub fn render_context(&self) -> &RenderContext {
        &self.window.render_context
    }

    pub fn command_encoder(&self) -> WindowCommandEncoder {
        self.window.command_encoder()
    }
}
