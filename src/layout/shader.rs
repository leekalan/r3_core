use std::{
    rc::Rc,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::prelude::*;

pub trait Shader {
    type Layout: Layout;
    type Settings = Void;

    fn get_pipeline(&self, settings: &Self::Settings) -> &wgpu::RenderPipeline;

    #[allow(unused)]
    fn apply_settings(&self, render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {}
}

impl<S: Shader> Shader for &S {
    type Layout = S::Layout;

    type Settings = S::Settings;

    #[inline(always)]
    fn get_pipeline(&self, settings: &Self::Settings) -> &wgpu::RenderPipeline {
        (**self).get_pipeline(settings)
    }
}

#[derive(Debug)]
pub struct ShaderInstance<S: Shader> {
    shader: S,
    settings: RwLock<S::Settings>,
}

impl<S: Shader> ShaderInstance<S> {
    #[inline]
    pub fn new(shader: S) -> Self
    where
        S::Settings: Default,
    {
        Self {
            shader,
            settings: RwLock::new(S::Settings::default()),
        }
    }

    #[inline]
    pub fn new_with(shader: S, settings: S::Settings) -> Self {
        Self {
            shader,
            settings: RwLock::new(settings),
        }
    }

    #[inline]
    pub fn shader(&self) -> &S {
        &self.shader
    }

    #[inline]
    pub fn settings(&self) -> RwLockReadGuard<'_, S::Settings> {
        self.settings.read().unwrap()
    }

    #[inline]
    pub fn settings_mut(&self) -> RwLockWriteGuard<'_, S::Settings> {
        self.settings.write().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct StaticShaderInstance<S: Shader> {
    shader: S,
    settings: S::Settings,
}

impl<S: Shader> StaticShaderInstance<S> {
    #[inline]
    pub fn new(shader: S) -> Self
    where
        S::Settings: Default,
    {
        Self {
            shader,
            settings: S::Settings::default(),
        }
    }

    #[inline]
    pub fn new_with(shader: S, settings: S::Settings) -> Self {
        Self { shader, settings }
    }

    #[inline(always)]
    pub fn shader(&self) -> &S {
        &self.shader
    }

    #[inline(always)]
    pub fn settings(&self) -> &S::Settings {
        &self.settings
    }

    #[inline(always)]
    pub fn settings_mut(&mut self) -> &mut S::Settings {
        &mut self.settings
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DefaultShaderInstance<S: Shader> {
    shader: S,
}

impl<S: Shader> DefaultShaderInstance<S>
where
    S::Settings: Default,
{
    #[inline]
    pub fn new(shader: S) -> Self {
        Self { shader }
    }

    pub fn new_ref(shader: &S) -> &Self {
        unsafe { &*(shader as *const _ as *const Self) }
    }

    #[inline]
    pub fn shader(&self) -> &S {
        &self.shader
    }
}

pub trait ApplyShaderInstance {
    type Layout: Layout;

    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass);

    fn change_settings(&self, render_pass: &mut wgpu::RenderPass);
}

impl<S: Shader> ApplyShaderInstance for ShaderInstance<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        let settings = &*self.settings();
        render_pass.set_pipeline(self.shader.get_pipeline(settings));
        self.shader.apply_settings(render_pass, settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.apply_settings(render_pass, &*self.settings());
    }
}

impl<S: Shader> ApplyShaderInstance for StaticShaderInstance<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.shader.get_pipeline(&self.settings));
        self.shader.apply_settings(render_pass, &self.settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.apply_settings(render_pass, &self.settings);
    }
}

impl<S: Shader> ApplyShaderInstance for DefaultShaderInstance<S>
where
    S::Settings: Default,
{
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        let settings = &S::Settings::default();
        render_pass.set_pipeline(self.shader.get_pipeline(settings));
        self.shader.apply_settings(render_pass, settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader
            .apply_settings(render_pass, &S::Settings::default());
    }
}

impl<S: ApplyShaderInstance> ApplyShaderInstance for Box<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().apply_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyShaderInstance> ApplyShaderInstance for Rc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().apply_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyShaderInstance> ApplyShaderInstance for Arc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().apply_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyShaderInstance> ApplyShaderInstance for Sc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().apply_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyShaderInstance> ApplyShaderInstance for Asc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().apply_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::RenderPass) {
        self.as_ref().change_settings(render_pass);
    }
}

pub type ShaderHandle<'s, L> = dyn ApplyShaderInstance<Layout = L> + 's;
