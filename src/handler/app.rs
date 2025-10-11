use std::time::Duration;

use crate::prelude::*;

#[derive(Debug)]
pub struct AppConfig<'r, 'w, C> {
    pub render_context: &'r RenderContext,
    pub window: &'w mut Window,
    pub state: C,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framerate {
    #[default]
    Unlimited,
    Limited(Duration),
}

#[derive(Debug)]
pub struct App<S> {
    pub render_context: RenderContext,
    pub state: S,
    pub framerate: Framerate,
    pub window: Window,
}

impl<S> App<S> {
    #[inline]
    pub fn new(
        render_context: RenderContext,
        window: Window,
        state: S,
        framerate: Framerate,
    ) -> Self {
        Self {
            render_context,
            state,
            framerate,
            window,
        }
    }

    #[inline(always)]
    pub fn command_encoder(&self) -> WindowCommandEncoder<'_> {
        self.window.command_encoder()
    }

    pub fn winit_window(&self) -> &winit::window::Window {
        &self.window.window
    }
}
