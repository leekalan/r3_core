use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
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
    ) -> EventResult;
}
pub enum EventResult {
    Redraw,
    Pause,
    Quit,
}

impl<F: Fn(&mut App<S>, &ActiveEventLoop, WindowId, WindowEvent) -> EventResult, S>
    OnEventCallback<S> for F
{
    #[inline(always)]
    fn call(
        &mut self,
        app: &mut App<S>,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) -> EventResult {
        self(app, event_loop, window_id, event)
    }
}

impl<S> OnEventCallback<S> for Void {
    #[inline(always)]
    fn call(
        &mut self,
        _: &mut App<S>,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent,
    ) -> EventResult {
        EventResult::Pause
    }
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
pub enum Handler<
    C = Void,
    S = Void,
    OnStart: OnStartCallback<C, S> = Void,
    OnEvent: OnEventCallback<S> = Void,
    OnClose: OnCloseCallBack<S> = Void,
> {
    Uninit {
        render_context: Asc<RenderContext>,
        window_config: WindowConfig,
        state: Option<C>,
        on_start: Option<OnStart>,
        on_event: Option<OnEvent>,
        on_close: Option<OnClose>,
    },
    Active {
        app: App<S>,
        on_event: OnEvent,
        on_close: Option<OnClose>,
    },
}

impl<
        C,
        S,
        OnStart: OnStartCallback<C, S>,
        OnEvent: OnEventCallback<S>,
        OnClose: OnCloseCallBack<S>,
    > Handler<C, S, OnStart, OnEvent, OnClose>
{
    #[inline(always)]
    pub fn new(
        render_context: Asc<RenderContext>,
        window_config: WindowConfig,
        on_start: OnStart,
        on_event: OnEvent,
        on_close: OnClose,
    ) -> Self
    where
        C: Default,
    {
        Self::Uninit {
            render_context,
            window_config,
            state: Some(default()),
            on_start: Some(on_start),
            on_event: Some(on_event),
            on_close: Some(on_close),
        }
    }

    #[inline(always)]
    pub fn new_with(
        render_context: Asc<RenderContext>,
        window_config: WindowConfig,
        state: C,
        on_start: OnStart,
        on_event: OnEvent,
        on_close: OnClose,
    ) -> Self {
        Self::Uninit {
            render_context,
            window_config,
            state: Some(state),
            on_start: Some(on_start),
            on_event: Some(on_event),
            on_close: Some(on_close),
        }
    }

    #[inline(always)]
    pub fn init(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(self).unwrap();
    }
}

impl<
        C,
        S,
        OnStart: OnStartCallback<C, S>,
        OnEvent: OnEventCallback<S>,
        OnClose: OnCloseCallBack<S>,
    > ApplicationHandler for Handler<C, S, OnStart, OnEvent, OnClose>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Handler::Uninit {
            render_context,
            window_config,
            state,
            on_start,
            on_event,
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
        let Handler::Active {
            app,
            on_event,
            on_close,
        } = self
        else {
            event_loop.exit();
            return;
        };

        if let WindowEvent::CloseRequested = event {
            unsafe { on_close.take().unwrap_unchecked() }.call(app, event_loop, window_id);
            event_loop.exit();
            return;
        } else if let WindowEvent::Resized(new_size) = event {
            app.window.resize(new_size);
        }

        let result = on_event.call(app, event_loop, window_id, event);
        match result {
            EventResult::Redraw => app.winit_window().request_redraw(),
            EventResult::Quit => {
                unsafe { on_close.take().unwrap_unchecked() }.call(app, event_loop, window_id);
                event_loop.exit()
            }
            EventResult::Pause => (),
        }
    }
}
