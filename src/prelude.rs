pub use crate::{
    bind::{
        create_bind, dynamic_buffer::DynamicBuffer, storage_buffer::StorageBuffer,
        uniform_buffer::UniformBuffer, Bind, BindLayout,
    },
    handler::{
        app::{App, AppConfig, Framerate},
        window::{Window, WindowCommandEncoder, WindowConfig},
        Callbacks, Handler, OnCloseCallBack, OnEventCallback, OnPollCallback, OnStartCallback,
    },
    layout::{
        compute_shader::{
            ApplyComputeShaderInstance, ComputeShader, ComputeShaderHandle, ComputeShaderInstance,
            DefaultComputeShaderInstance, StaticComputeShaderInstance,
        },
        instances::{
            Instances, SimpleInstances, SimpleInstances2, SimpleInstances3, SimpleInstances4,
            SimpleInstances5,
        },
        shader::Shader,
        vertex::{
            create_vertex_attr, create_vertex_layout, IRequirements, InstanceRequirements,
            NoInstanceRequirements, VRequirements, VertexAttr, VertexAttrMarker,
            VertexBufferLayout, VertexRequirements,
        },
        ComputeLayout, ComputeLayoutConfig, ComputeShaderConfig, CreateComputePipeline,
        CreatePipeline, Layout, LayoutConfig, RawComputeLayout, RawLayout, ShaderConfig,
        SharedComputeData, SharedData, VertexLayout,
    },
    render_context::{
        command_encoder::CommandEncoder,
        compute_pass::ComputePass,
        render_pass::RenderPass,
        RenderContext, RenderContextConfig,
    },
    surface::mesh::{
        index_format, Mesh, SimpleMesh, SimpleMesh0, SimpleMesh2, SimpleMesh3, SimpleMesh4,
        SimpleMesh5,
    },
    texture::{
        RawTexture, RawTextureView, Sampler, Texture, Texture1D, Texture2D, Texture3D,
        TextureConfig,
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
        hdr::{CommandEncoderHdr, Hdr, WindowCommandEncoderHdr},
        post_processing::{PostProc, PostProcBind, PostProcBindLayout},
        tileset::{SimpleTileLayout, TileInstance, TilesetQuad},
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
