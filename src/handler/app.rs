use std::sync::Arc;

use crate::prelude::*;

pub struct App<S> {
    pub window: Window,
    pub render_context: Arc<RenderContext>,
    pub state: S,
}

impl<S> App<S> {
    #[inline]
    pub fn new(window: Window, state: S) -> Self {
        Self {
            render_context: window.render_context.clone(),
            window,
            state,
        }
    }

    pub fn render_context(&self) -> &Arc<RenderContext> {
        &self.render_context
    }

    pub fn command_encoder(&self) -> WindowCommandEncoder {
        self.window.command_encoder()
    }
}
