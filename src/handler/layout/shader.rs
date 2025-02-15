use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::prelude::*;

pub trait Shader {
    type V: Vertex;
    type Config;

    fn set_shader(&self, render_pass: &mut wgpu::RenderPass);
    fn apply_config(&self, render_pass: &mut wgpu::RenderPass, config: &Self::Config);
}

pub struct ShaderInstance<S: Shader> {
    shader: Arc<S>,
    config: RwLock<S::Config>,
}

impl<S: Shader> ShaderInstance<S> {
    #[inline]
    pub fn new(shader: Arc<S>) -> Self
    where
        S::Config: Default,
    {
        Self {
            shader,
            config: RwLock::new(S::Config::default()),
        }
    }

    #[inline]
    pub fn new_with(shader: Arc<S>, config: S::Config) -> Self {
        Self {
            shader,
            config: RwLock::new(config),
        }
    }

    #[inline]
    pub fn shader(&self) -> &Arc<S> {
        &self.shader
    }

    #[inline]
    pub fn config(&self) -> RwLockReadGuard<S::Config> {
        self.config.read().unwrap()
    }

    #[inline]
    pub fn config_mut(&self) -> RwLockWriteGuard<S::Config> {
        self.config.write().unwrap()
    }

    #[inline]
    pub fn handle(self: &Arc<Self>) -> Arc<ShaderHandle<S::V>>
    where
        S: 'static,
    {
        self.clone()
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> Arc<ShaderHandle<S::V>>
    where
        S: 'static,
    {
        self
    }
}

pub struct StaticShaderInstance<S: Shader> {
    shader: Arc<S>,
    config: S::Config,
}

impl<S: Shader> StaticShaderInstance<S> {
    #[inline]
    pub fn new(shader: Arc<S>) -> Self
    where
        S::Config: Default,
    {
        Self {
            shader,
            config: S::Config::default(),
        }
    }

    #[inline]
    pub fn new_with(shader: Arc<S>, config: S::Config) -> Self {
        Self { shader, config }
    }

    #[inline]
    pub fn shader(&self) -> &Arc<S> {
        &self.shader
    }

    #[inline]
    pub fn config(&self) -> &S::Config {
        &self.config
    }

    #[inline]
    pub fn config_mut(&mut self) -> &mut S::Config {
        &mut self.config
    }

    #[inline]
    pub fn handle(self: &Arc<Self>) -> Arc<ShaderHandle<S::V>>
    where
        S: 'static,
    {
        self.clone()
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> Arc<ShaderHandle<S::V>>
    where
        S: 'static,
    {
        self
    }
}

pub trait ApplyShaderInstance {
    type V: Vertex;

    fn set_shader(&self, render_pass: &mut wgpu::RenderPass);
    fn apply_config(&self, render_pass: &mut wgpu::RenderPass);
}

impl<S: Shader> ApplyShaderInstance for ShaderInstance<S> {
    type V = S::V;

    #[inline]
    fn set_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.set_shader(render_pass);
    }

    #[inline]
    fn apply_config(&self, render_pass: &mut wgpu::RenderPass) {
        let config = self.config.read().unwrap();
        self.shader.apply_config(render_pass, &config);
    }
}

impl<S: Shader> ApplyShaderInstance for StaticShaderInstance<S> {
    type V = S::V;

    #[inline]
    fn set_shader(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.set_shader(render_pass);
    }

    #[inline]
    fn apply_config(&self, render_pass: &mut wgpu::RenderPass) {
        self.shader.apply_config(render_pass, &self.config);
    }
}

pub type ShaderHandle<V> = dyn ApplyShaderInstance<V = V>;
