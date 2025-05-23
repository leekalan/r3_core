pub use crate::{
    bind::{
        create_bind, dynamic_buffer::DynamicBuffer, uniform_buffer::UniformBuffer, Bind, BindLayout,
    },
    handler::{
        app::{App, AppConfig},
        window::{Window, WindowCommandEncoder, WindowConfig},
        EventResult, Handler, OnCloseCallBack, OnEventCallback, OnStartCallback,
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
        vertex::{
            create_vertex_attr, create_vertex_layout, Requirements, VertexAttr, VertexAttrMarker,
            VertexBufferLayout, VertexRequirements,
        },
        ComputeLayout, ComputeLayoutConfig, CreateComputePipeline, CreatePipeline, Layout,
        LayoutConfig, RawLayout, ShaderConfig, SharedComputeData, SharedData, VertexLayout,
    },
    render_context::{CommandEncoder, RenderContext, RenderContextConfig, RenderPass},
    surface::{
        mesh::{
            index_format, Mesh, SimpleMesh, SimpleMesh0, SimpleMesh2, SimpleMesh3, SimpleMesh4,
            SimpleMesh5,
        },
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
        camera_2d::{Camera2d, Projection2d},
        grounded_camera::GroundedCamera,
        post_processing::{PostProc, PostProcBind, PostProcBindLayout},
        surface::ShadedMesh,
        tileset::{TilesetMesh, TilesetQuad},
        transform::{Transform, Transform2d},
        vertex::{PosVertex, PosVertex2d, RBGAVertex, RGBVertex, UVVertex},
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
