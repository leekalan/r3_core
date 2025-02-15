use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

use crate::prelude::*;

pub mod app;
pub mod layout;
pub mod raw_mesh;
pub mod render_context;
pub mod surface;
pub mod window;

pub type OnStartCallback<S> = dyn Fn(&mut App<S>, &ActiveEventLoop);

pub trait OnEventCallback<S> {
    fn call(
        &mut self,
        app: &mut App<S>,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    );
}

impl<F: Fn(&mut App<S>, &ActiveEventLoop, WindowId, WindowEvent), S> OnEventCallback<S> for F {
    #[inline]
    fn call(
        &mut self,
        app: &mut App<S>,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self(app, event_loop, window_id, event);
    }
}

impl<S> OnEventCallback<S> for () {
    #[inline]
    fn call(&mut self, _: &mut App<S>, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
}

pub struct HandlerConfig<S, OnEvent: OnEventCallback<S>> {
    state: S,
    window_attributes: Option<WindowAttributes>,
    window_config: WindowConfig,
    render_context: Arc<RenderContext>,
    on_start: Option<Box<OnStartCallback<S>>>,
    callback: OnEvent,
}

impl<S, OnEvent: OnEventCallback<S>> HandlerConfig<S, OnEvent> {
    pub fn new(
        render_context: Arc<RenderContext>,
        window_config: WindowConfig,
        callback: OnEvent,
    ) -> Self
    where
        S: Default,
    {
        Self {
            state: S::default(),
            window_attributes: None,
            window_config,
            render_context,
            callback,
            on_start: None,
        }
    }

    pub fn with_state(
        render_context: Arc<RenderContext>,
        window_config: WindowConfig,
        state: S,
        callback: OnEvent,
    ) -> Self {
        Self {
            state,
            window_attributes: None,
            window_config,
            render_context,
            callback,
            on_start: None,
        }
    }

    pub fn on_start(mut self, on_start: Box<OnStartCallback<S>>) -> Self {
        self.on_start = Some(on_start);
        self
    }

    pub fn window_attributes(mut self, window_attributes: WindowAttributes) -> Self {
        self.window_attributes = Some(window_attributes);
        self
    }
}

pub enum Handler<S, OnEvent: OnEventCallback<S>> {
    Uninit(Option<HandlerConfig<S, OnEvent>>),
    Active { app: App<S>, callback: OnEvent },
}

impl<S, OnEvent: OnEventCallback<S>> Handler<S, OnEvent> {
    pub fn new(handler_config: HandlerConfig<S, OnEvent>) -> Self {
        Self::Uninit(Some(handler_config))
    }

    pub fn init(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).unwrap();
    }
}

impl<S, OnEvent: OnEventCallback<S>> ApplicationHandler for Handler<S, OnEvent> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Handler::Uninit(config) = self {
            let config = config.take().unwrap();

            let winit_window = Arc::new(
                event_loop
                    .create_window(config.window_attributes.unwrap_or_default())
                    .unwrap(),
            );

            let window = Window::new(winit_window, config.render_context, config.window_config);

            let mut app = App::new(window, config.state);

            if let Some(on_start) = config.on_start {
                on_start(&mut app, event_loop);
            }

            *self = Handler::Active {
                app,
                callback: config.callback,
            };
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        } else if let WindowEvent::Resized(new_size) = event {
            if let Handler::Active { app, .. } = self {
                app.window.resize(new_size);
            }
        }

        if let Handler::Active { app, callback } = self {
            callback.call(app, event_loop, window_id, event);
        }
    }
}
