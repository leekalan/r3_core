use std::{rc::Rc, sync::Arc};

use crate::prelude::*;

pub trait Shader {
    type Layout: Layout;
    type Settings = Void;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline;

    #[allow(unused)]
    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {}
}

impl<S: Shader> Shader for Rc<S> {
    type Layout = S::Layout;
    type Settings = S::Settings;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        self.as_ref().get_pipeline()
    }

    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {
        S::apply_settings(render_pass, settings)
    }
}

impl<S: Shader> Shader for Arc<S> {
    type Layout = S::Layout;
    type Settings = S::Settings;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        self.as_ref().get_pipeline()
    }

    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {
        S::apply_settings(render_pass, settings)
    }
}

impl<S: Shader> Shader for Box<S> {
    type Layout = S::Layout;
    type Settings = S::Settings;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        self.as_ref().get_pipeline()
    }

    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {
        S::apply_settings(render_pass, settings)
    }
}

impl<S: Shader> Shader for Sc<S> {
    type Layout = S::Layout;
    type Settings = S::Settings;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        self.as_ref().get_pipeline()
    }

    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {
        S::apply_settings(render_pass, settings)
    }
}

impl<S: Shader> Shader for Asc<S> {
    type Layout = S::Layout;
    type Settings = S::Settings;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {
        self.as_ref().get_pipeline()
    }

    fn apply_settings(render_pass: &mut wgpu::RenderPass, settings: &Self::Settings) {
        S::apply_settings(render_pass, settings)
    }
}
