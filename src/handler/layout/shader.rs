use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;

pub trait ShaderSettings<L: Layout> {
    fn layout_instance(&self) -> &LayoutInstance<L>;
}

impl<L: Layout> ShaderSettings<L> for L::Instance {
    fn layout_instance(&self) -> &Self {
        self
    }
}

pub trait Shader {
    type Layout: Layout;
    type Settings: ShaderSettings<Self::Layout> = LayoutInstance<Self::Layout>;

    fn pipeline(&self, settings: &Self::Settings) -> &wgpu::RenderPipeline;
}

pub struct ShaderInstance<S: Shader> {
    shader: Arc<S>,
    settings: RwLock<S::Settings>,
}

impl<S: Shader> ShaderInstance<S> {
    #[inline]
    pub fn new(shader: Arc<S>) -> Self
    where
        S::Settings: Default,
    {
        Self {
            shader,
            settings: RwLock::new(S::Settings::default()),
        }
    }

    #[inline]
    pub fn new_with(shader: Arc<S>, settings: S::Settings) -> Self {
        Self {
            shader,
            settings: RwLock::new(settings),
        }
    }

    #[inline]
    pub fn shader(&self) -> &Arc<S> {
        &self.shader
    }

    #[inline]
    pub fn settings(&self) -> RwLockReadGuard<S::Settings> {
        self.settings.read().unwrap()
    }

    #[inline]
    pub fn settings_mut(&self) -> RwLockWriteGuard<S::Settings> {
        self.settings.write().unwrap()
    }

    #[inline]
    pub fn handle(self: &Arc<Self>) -> Arc<ShaderHandle<S::Layout>>
    where
        S: 'static,
    {
        self.clone()
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> Arc<ShaderHandle<S::Layout>>
    where
        S: 'static,
    {
        self
    }
}

pub struct StaticShaderInstance<S: Shader> {
    shader: Arc<S>,
    settings: S::Settings,
}

impl<S: Shader> StaticShaderInstance<S> {
    #[inline]
    pub fn new(shader: Arc<S>) -> Self
    where
        S::Settings: Default,
    {
        Self {
            shader,
            settings: S::Settings::default(),
        }
    }

    #[inline]
    pub fn new_with(shader: Arc<S>, settings: S::Settings) -> Self {
        Self { shader, settings }
    }

    #[inline]
    pub fn shader(&self) -> &Arc<S> {
        &self.shader
    }

    #[inline]
    pub fn settings(&self) -> &S::Settings {
        &self.settings
    }

    #[inline]
    pub fn settings_mut(&mut self) -> &mut S::Settings {
        &mut self.settings
    }

    #[inline]
    pub fn handle(self: &Arc<Self>) -> Arc<ShaderHandle<S::Layout>>
    where
        S: 'static,
    {
        self.clone()
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> Arc<ShaderHandle<S::Layout>>
    where
        S: 'static,
    {
        self
    }
}

pub trait ApplyShaderInstance {
    type Layout: Layout;

    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass);

    fn apply_settings(&self, render_pass: &mut wgpu::RenderPass);
}

impl<S: Shader> ApplyShaderInstance for ShaderInstance<S> {
    type Layout = S::Layout;

    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        let settings = &*self.settings();
        render_pass.set_pipeline(self.shader.pipeline(settings));
        Self::Layout::set_instance(render_pass, settings.layout_instance());
    }

    #[inline]
    fn apply_settings(&self, render_pass: &mut wgpu::RenderPass) {
        Self::Layout::set_instance(render_pass, (*self.settings()).layout_instance());
    }
}

impl<S: Shader> ApplyShaderInstance for StaticShaderInstance<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.shader.pipeline(&self.settings));
        Self::Layout::set_instance(render_pass, self.settings.layout_instance());
    }

    #[inline]
    fn apply_settings(&self, render_pass: &mut wgpu::RenderPass) {
        Self::Layout::set_instance(render_pass, self.settings().layout_instance());
    }
}

pub type ShaderHandle<L> = dyn ApplyShaderInstance<Layout = L>;
