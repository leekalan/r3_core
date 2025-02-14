use std::sync::{Arc, Mutex};

use crate::prelude::RenderPass;

use super::Vertex;

pub trait Shader {
    type V: Vertex;
    type Config;

    fn set_shader(&self, render_pass: &mut RenderPass);
    fn apply_config(&self, render_pass: &mut RenderPass, config: &Self::Config);
}

pub struct ShaderInstance<S: Shader> {
    shader: Arc<S>,
    config: Mutex<S::Config>,
}

impl<S: Shader> ShaderInstance<S> {
    #[inline]
    pub fn new(shader: Arc<S>, config: S::Config) -> Arc<Self> {
        Arc::new(Self {
            shader,
            config: Mutex::new(config),
        })
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> ShaderHandle<S::V>
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
    pub fn new(shader: Arc<S>, config: S::Config) -> Arc<Self> {
        Arc::new(Self { shader, config })
    }

    #[inline]
    pub fn into_handle(self: Arc<Self>) -> ShaderHandle<S::V>
    where
        S: 'static,
    {
        self
    }
}

pub trait ApplyShaderInstance {
    type V: Vertex;

    fn set_shader(&self, render_pass: &mut RenderPass);
    fn apply_config(&self, render_pass: &mut RenderPass);
}

impl<S: Shader> ApplyShaderInstance for ShaderInstance<S> {
    type V = S::V;

    #[inline]
    fn set_shader(&self, render_pass: &mut RenderPass) {
        self.shader.set_shader(render_pass);
    }

    #[inline]
    fn apply_config(&self, render_pass: &mut RenderPass) {
        let config = self.config.lock().unwrap();
        self.shader.apply_config(render_pass, &config);
    }
}

impl<S: Shader> ApplyShaderInstance for StaticShaderInstance<S> {
    type V = S::V;

    #[inline]
    fn set_shader(&self, render_pass: &mut RenderPass) {
        self.shader.set_shader(render_pass);
    }

    #[inline]
    fn apply_config(&self, render_pass: &mut RenderPass) {
        self.shader.apply_config(render_pass, &self.config);
    }
}

pub type ShaderHandle<V> = Arc<dyn ApplyShaderInstance<V = V>>;
