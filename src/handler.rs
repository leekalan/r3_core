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

pub trait OnStartCallback<C, S> {
    fn call(self, app: AppConfig<C>, event_loop: &ActiveEventLoop) -> S;
}

impl<F: FnOnce(AppConfig<C>, &ActiveEventLoop) -> S, C, S> OnStartCallback<C, S> for F {
    #[inline]
    fn call(self, app: AppConfig<C>, event_loop: &ActiveEventLoop) -> S {
        self(app, event_loop)
    }
}

impl<C, S: Default> OnStartCallback<C, S> for Void {
    #[inline]
    fn call(self, _: AppConfig<C>, _: &ActiveEventLoop) -> S {
        S::default()
    }
}

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

impl<S> OnEventCallback<S> for Void {
    #[inline]
    fn call(&mut self, _: &mut App<S>, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
}

pub struct HandlerConfig<
    C = Void,
    S = Void,
    OnStart: OnStartCallback<C, S> = Void,
    OnEvent: OnEventCallback<S> = Void,
> {
    state_config: C,
    window_attributes: Option<WindowAttributes>,
    window_config: WindowConfig,
    render_context: Asc<RenderContext>,
    on_start: OnStart,
    on_event: OnEvent,
    _marker: PhantomData<*const S>,
}

impl<C, S, OnStart: OnStartCallback<C, S>, OnEvent: OnEventCallback<S>>
    HandlerConfig<C, S, OnStart, OnEvent>
{
    #[inline]
    pub fn new(
        render_context: Asc<RenderContext>,
        window_config: WindowConfig,
        on_start: OnStart,
        on_event: OnEvent,
    ) -> Self
    where
        C: Default,
    {
        Self {
            state_config: C::default(),
            window_attributes: None,
            window_config,
            render_context,
            on_start,
            on_event,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_state_config(
        render_context: Asc<RenderContext>,
        window_config: WindowConfig,
        state_config: C,
        on_start: OnStart,
        on_event: OnEvent,
    ) -> Self {
        Self {
            state_config,
            window_attributes: None,
            window_config,
            render_context,
            on_start,
            on_event,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn window_attributes(mut self, window_attributes: WindowAttributes) -> Self {
        self.window_attributes = Some(window_attributes);
        self
    }
}

pub enum Handler<
    C = Void,
    S = Void,
    OnStart: OnStartCallback<C, S> = Void,
    OnEvent: OnEventCallback<S> = Void,
> {
    Uninit(Option<HandlerConfig<C, S, OnStart, OnEvent>>),
    Active { app: App<S>, on_event: OnEvent },
}

impl<C, S, OnStart: OnStartCallback<C, S>, OnEvent: OnEventCallback<S>>
    Handler<C, S, OnStart, OnEvent>
{
    #[inline]
    pub fn new(handler_config: HandlerConfig<C, S, OnStart, OnEvent>) -> Self {
        Self::Uninit(Some(handler_config))
    }

    #[inline]
    pub fn init(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).unwrap();
    }
}

impl<C, S, OnStart: OnStartCallback<C, S>, OnEvent: OnEventCallback<S>> ApplicationHandler
    for Handler<C, S, OnStart, OnEvent>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Handler::Uninit(config) = self {
            let config = config.take().unwrap();

            let winit_window = Arc::new(
                event_loop
                    .create_window(config.window_attributes.unwrap_or_default())
                    .unwrap(),
            );

            let render_context = config.render_context;
            let mut window =
                Window::new(winit_window, render_context.clone(), config.window_config);

            let uninit_app = AppConfig {
                render_context: &render_context,
                window: &mut window,
                state_config: config.state_config,
            };

            let state = config.on_start.call(uninit_app, event_loop);

            let app = App::new(render_context, window, state);

            *self = Handler::Active {
                app,
                on_event: config.on_event,
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

        if let Handler::Active { app, on_event } = self {
            on_event.call(app, event_loop, window_id, event);
        }
    }
}
