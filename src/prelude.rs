pub use crate::{
    bind::{buffer::UniformBuffer, create_bind, Bind, BindLayout},
    handler::{
        app::{App, AppConfig},
        window::{Window, WindowCommandEncoder, WindowConfig},
        Handler,
    },
    layout::{
        compute_shader::{
            ApplyComputeShaderInstance, ComputeShader, ComputeShaderHandle, ComputeShaderInstance,
            DefaultComputeShaderInstance, StaticComputeShaderInstance,
        },
        shader::{
            ApplyShaderInstance, DefaultShaderInstance, Shader, ShaderHandle, ShaderInstance,
            StaticShaderInstance,
        },
        ComputeLayout, ComputeLayoutConfig, CreateComputePipeline, CreatePipeline, Layout,
        LayoutConfig, LayoutVertex, RawLayout, ShaderConfig, SharedComputeLayoutData,
        SharedLayoutData, Vertex,
    },
    render_context::{CommandEncoder, RenderContext, RenderContextConfig, RenderPass},
    surface::{
        raw_mesh::{index_format, RawMesh},
        Surface,
    },
    texture::{
        RawTexture, RawTextureView, Sampler, SamplerConfig, Texture, Texture1D, Texture2D,
        Texture3D, TextureConfig,
    },
};

pub use std::{
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, RwLock},
};

#[inline(always)]
pub fn default<T: Default>() -> T {
    Default::default()
}

pub use strong_count::prelude::*;

pub mod core {
    pub use crate::core::{
        camera::{Camera, CameraBind, CameraBindLayout, CameraUniform, Projection},
        grounded_camera::GroundedCamera,
        post_processing::{PostProc, PostProcBind, PostProcBindLayout},
        surface::{AscMesh, ExtendedSurface, Mesh, SurfaceExt},
        transform::Transform,
        vertex::{PosVertex, RBGAVertex, RGBVertex, UVVertex},
    };
}

use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Invalid {}

impl Debug for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<invalid>")
    }
}

impl Display for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<invalid>")
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Void;

impl Debug for Void {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<null>")
    }
}

impl Display for Void {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<null>")
    }
}
