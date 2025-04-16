use crate::prelude::*;

#[derive(Debug)]
pub struct AppConfig<'r, 'w, C> {
    pub render_context: &'r RenderContext,
    pub window: &'w mut Window,
    pub state: C,
}

#[derive(Debug)]
pub struct App<S> {
    pub render_context: Asc<RenderContext>,
    pub state: S,
    pub window: Window,
}

impl<S> App<S> {
    #[inline]
    pub fn new(render_context: Asc<RenderContext>, window: Window, state: S) -> Self {
        Self {
            render_context,
            window,
            state,
        }
    }

    #[inline(always)]
    pub fn command_encoder(&self) -> WindowCommandEncoder {
        self.window.command_encoder()
    }
}
