use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::prelude::*;

pub mod app;
pub mod window;

pub trait OnStartCallback<C, S> {
    fn call(self, app: AppConfig<C>, event_loop: &ActiveEventLoop) -> S;
}

impl<F: FnOnce(AppConfig<C>, &ActiveEventLoop) -> S, C, S> OnStartCallback<C, S> for F {
    #[inline(always)]
    fn call(self, app: AppConfig<C>, event_loop: &ActiveEventLoop) -> S {
        self(app, event_loop)
    }
}

impl<C, S: Default> OnStartCallback<C, S> for Void {
    #[inline(always)]
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
    #[inline(always)]
    fn call(
        &mut self,
        app: &mut App<S>,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self(app, event_loop, window_id, event)
    }
}

impl<S> OnEventCallback<S> for Void {
    #[inline(always)]
    fn call(&mut self, _: &mut App<S>, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}
}

pub trait OnPollCallback<S> {
    fn call(&mut self, app: &mut App<S>, event_loop: &ActiveEventLoop);
}

impl<F: Fn(&mut App<S>, &ActiveEventLoop), S> OnPollCallback<S> for F {
    #[inline(always)]
    fn call(&mut self, app: &mut App<S>, event_loop: &ActiveEventLoop) {
        self(app, event_loop)
    }
}

impl<S> OnPollCallback<S> for Void {
    #[inline(always)]
    fn call(&mut self, _: &mut App<S>, _: &ActiveEventLoop) {}
}

pub trait OnDrawCallback<S> {
    fn call(&mut self, app: &mut App<S>, event_loop: &ActiveEventLoop, window_id: WindowId);
}

impl<F: Fn(&mut App<S>, &ActiveEventLoop, WindowId), S> OnDrawCallback<S> for F {
    #[inline(always)]
    fn call(&mut self, app: &mut App<S>, event_loop: &ActiveEventLoop, window_id: WindowId) {
        self(app, event_loop, window_id)
    }
}

impl<S> OnDrawCallback<S> for Void {
    #[inline(always)]
    fn call(&mut self, _: &mut App<S>, _: &ActiveEventLoop, _: WindowId) {}
}

pub trait OnCloseCallBack<S> {
    fn call(self, app: &mut App<S>, event_loop: &ActiveEventLoop, window_id: WindowId);
}

impl<F: FnOnce(&mut App<S>, &ActiveEventLoop, WindowId), S> OnCloseCallBack<S> for F {
    #[inline(always)]
    fn call(self, app: &mut App<S>, event_loop: &ActiveEventLoop, window_id: WindowId) {
        self(app, event_loop, window_id)
    }
}

impl<S> OnCloseCallBack<S> for Void {
    #[inline(always)]
    fn call(self, _: &mut App<S>, _: &ActiveEventLoop, _: WindowId) {}
}

#[derive(Debug)]
pub struct Callbacks<OnStart, OnEvent, OnPoll, OnDraw, OnClose> {
    on_start: Option<OnStart>,
    on_event: Option<OnEvent>,
    on_poll: Option<OnPoll>,
    on_draw: Option<OnDraw>,
    on_close: Option<OnClose>,
}

impl<OnStart, OnEvent, OnPoll, OnDraw, OnClose>
    Callbacks<OnStart, OnEvent, OnPoll, OnDraw, OnClose>
{
    #[inline(always)]
    pub fn new(
        on_start: Option<OnStart>,
        on_event: Option<OnEvent>,
        on_poll: Option<OnPoll>,
        on_draw: Option<OnDraw>,
        on_close: Option<OnClose>,
    ) -> Self {
        Self {
            on_start,
            on_event,
            on_poll,
            on_draw,
            on_close,
        }
    }
}

#[derive(Debug)]
pub enum Handler<
    C = Void,
    S = Void,
    OnStart: OnStartCallback<C, S> = Void,
    OnEvent: OnEventCallback<S> = Void,
    OnPoll: OnPollCallback<S> = Void,
    OnDraw: OnDrawCallback<S> = Void,
    OnClose: OnCloseCallBack<S> = Void,
> {
    Uninit {
        render_context: RenderContext,
        window_config: WindowConfig,
        state: Option<C>,
        on_start: Option<OnStart>,
        on_event: Option<OnEvent>,
        on_poll: Option<OnPoll>,
        on_draw: Option<OnDraw>,
        on_close: Option<OnClose>,
    },
    Active {
        app: App<S>,
        on_poll: OnPoll,
        on_event: OnEvent,
        on_draw: OnDraw,
        on_close: Option<OnClose>,
    },
}

impl<
        C,
        S,
        OnStart: OnStartCallback<C, S>,
        OnEvent: OnEventCallback<S>,
        OnPoll: OnPollCallback<S>,
        OnDraw: OnDrawCallback<S>,
        OnClose: OnCloseCallBack<S>,
    > Handler<C, S, OnStart, OnEvent, OnPoll, OnDraw, OnClose>
{
    #[inline(always)]
    pub fn new(
        render_context: RenderContext,
        window_config: WindowConfig,
        callbacks: Callbacks<OnStart, OnEvent, OnPoll, OnDraw, OnClose>,
    ) -> Self
    where
        C: Default,
    {
        Self::Uninit {
            render_context,
            window_config,
            state: Some(default()),
            on_start: callbacks.on_start,
            on_event: callbacks.on_event,
            on_poll: callbacks.on_poll,
            on_draw: callbacks.on_draw,
            on_close: callbacks.on_close,
        }
    }

    #[inline(always)]
    pub fn new_with(
        render_context: RenderContext,
        window_config: WindowConfig,
        state: C,
        callbacks: Callbacks<OnStart, OnEvent, OnPoll, OnDraw, OnClose>,
    ) -> Self {
        Self::Uninit {
            render_context,
            window_config,
            state: Some(state),
            on_start: callbacks.on_start,
            on_event: callbacks.on_event,
            on_poll: callbacks.on_poll,
            on_draw: callbacks.on_draw,
            on_close: callbacks.on_close,
        }
    }

    #[inline(always)]
    pub fn init(&mut self, event_loop: EventLoop<()>) {
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap();
    }
}

impl<
        C,
        S,
        OnStart: OnStartCallback<C, S>,
        OnEvent: OnEventCallback<S>,
        OnPoll: OnPollCallback<S>,
        OnDraw: OnDrawCallback<S>,
        OnClose: OnCloseCallBack<S>,
    > ApplicationHandler for Handler<C, S, OnStart, OnEvent, OnPoll, OnDraw, OnClose>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Handler::Uninit {
            render_context,
            window_config,
            state,
            on_start,
            on_event,
            on_poll,
            on_draw,
            on_close,
        } = self
        {
            let winit_window = Arc::new(
                event_loop
                    .create_window(window_config.window_attributes.take().unwrap_or_default())
                    .unwrap(),
            );

            let mut window = Window::new(winit_window, render_context.clone(), window_config);

            let uninit_app = AppConfig {
                render_context,
                window: &mut window,
                state: state.take().unwrap(),
            };

            let state = on_start.take().unwrap().call(uninit_app, event_loop);

            let app = App::new(render_context.clone(), window, state);

            *self = Handler::Active {
                app,
                on_event: unsafe { on_event.take().unwrap_unchecked() },
                on_poll: unsafe { on_poll.take().unwrap_unchecked() },
                on_draw: unsafe { on_draw.take().unwrap_unchecked() },
                on_close: on_close.take(),
            };
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        event_loop.set_control_flow(ControlFlow::Poll);

        let Handler::Active {
            app,
            on_event,
            on_poll: _,
            on_draw,
            on_close,
        } = self
        else {
            event_loop.exit();
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                unsafe { on_close.take().unwrap_unchecked() }.call(app, event_loop, window_id);
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                app.window.resize(new_size);
                app.winit_window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                on_draw.call(app, event_loop, window_id);
            }
            _ => {}
        }

        on_event.call(app, event_loop, window_id, event);
    }

    /// This is where the core gameloop is ran inbetween frames
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Handler::Active {
            app,
            on_event: _,
            on_poll,
            on_draw: _,
            on_close: _,
        } = self
        else {
            event_loop.exit();
            return;
        };

        on_poll.call(app, event_loop);

        app.winit_window().request_redraw();
    }
}
