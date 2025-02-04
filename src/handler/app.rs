use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use super::renderer::WindowRenderer;

pub struct App<S> {
    pub window: Arc<Window>,
    pub renderer: WindowRenderer,
    pub state: S,
}

impl<S> App<S> {
    #[inline]
    pub fn new(window: Arc<Window>, renderer: WindowRenderer, state: S) -> Self {
        Self {
            window,
            renderer,
            state,
        }
    }

    #[inline]
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }
}
