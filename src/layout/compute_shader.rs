use std::{
    rc::Rc,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::prelude::*;

pub trait ComputeShader {
    type Layout: ComputeLayout;
    type Settings = Void;

    fn get_compute_pipeline(&self, settings: &Self::Settings) -> &wgpu::ComputePipeline;

    #[allow(unused)]
    fn apply_settings(&self, render_pass: &mut wgpu::ComputePass, settings: &Self::Settings) {}
}

impl<S: ComputeShader> ComputeShader for &S {
    type Layout = S::Layout;

    type Settings = S::Settings;

    #[inline(always)]
    fn get_compute_pipeline(&self, settings: &Self::Settings) -> &wgpu::ComputePipeline {
        (**self).get_compute_pipeline(settings)
    }
}

pub struct ComputeShaderInstance<S: ComputeShader> {
    shader: S,
    settings: RwLock<S::Settings>,
}

impl<S: ComputeShader> ComputeShaderInstance<S> {
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
    pub fn settings(&self) -> RwLockReadGuard<S::Settings> {
        self.settings.read().unwrap()
    }

    #[inline]
    pub fn settings_mut(&self) -> RwLockWriteGuard<S::Settings> {
        self.settings.write().unwrap()
    }
}

pub struct StaticComputeShaderInstance<S: ComputeShader> {
    shader: S,
    settings: S::Settings,
}

impl<S: ComputeShader> StaticComputeShaderInstance<S> {
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
pub struct DefaultComputeShaderInstance<S: ComputeShader> {
    shader: S,
}

impl<S: ComputeShader> DefaultComputeShaderInstance<S>
where
    S::Settings: Default,
{
    #[inline]
    pub fn new(shader: S) -> Self {
        Self { shader }
    }

    #[inline]
    pub fn from_ref(shader: &S) -> &Self {
        unsafe { &*(shader as *const _ as *const Self) }
    }

    #[inline]
    pub fn shader(&self) -> &S {
        &self.shader
    }
}

pub trait ApplyComputeShaderInstance {
    type Layout: ComputeLayout;

    fn apply_compute_shader(&self, compute_pass: &mut wgpu::ComputePass);

    fn change_settings(&self, compute_pass: &mut wgpu::ComputePass);
}

impl<S: ComputeShader> ApplyComputeShaderInstance for ComputeShaderInstance<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        let settings = &*self.settings();
        render_pass.set_pipeline(self.shader.get_compute_pipeline(settings));
        self.shader.apply_settings(render_pass, settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.shader.apply_settings(render_pass, &*self.settings());
    }
}

impl<S: ComputeShader> ApplyComputeShaderInstance for StaticComputeShaderInstance<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        render_pass.set_pipeline(self.shader.get_compute_pipeline(&self.settings));
        self.shader.apply_settings(render_pass, &self.settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.shader.apply_settings(render_pass, &self.settings);
    }
}

impl<S: ComputeShader> ApplyComputeShaderInstance for DefaultComputeShaderInstance<S>
where
    S::Settings: Default,
{
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        let settings = &S::Settings::default();
        render_pass.set_pipeline(self.shader.get_compute_pipeline(settings));
        self.shader.apply_settings(render_pass, settings);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.shader
            .apply_settings(render_pass, &S::Settings::default());
    }
}

impl<S: ApplyComputeShaderInstance> ApplyComputeShaderInstance for Box<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().apply_compute_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyComputeShaderInstance> ApplyComputeShaderInstance for Rc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().apply_compute_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyComputeShaderInstance> ApplyComputeShaderInstance for Arc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().apply_compute_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyComputeShaderInstance> ApplyComputeShaderInstance for Sc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().apply_compute_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().change_settings(render_pass);
    }
}

impl<S: ApplyComputeShaderInstance> ApplyComputeShaderInstance for Asc<S> {
    type Layout = S::Layout;

    #[inline]
    fn apply_compute_shader(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().apply_compute_shader(render_pass);
    }

    #[inline]
    fn change_settings(&self, render_pass: &mut wgpu::ComputePass) {
        self.as_ref().change_settings(render_pass);
    }
}

pub type ComputeShaderHandle<'s, L> = dyn ApplyComputeShaderInstance<Layout = L> + 's;
