pub use crate::{
    handler::{
        app::App,
        layout::{
            shader::{
                ApplyShaderInstance, Shader, ShaderHandle, ShaderInstance, StaticShaderInstance,
            },
            Layout, LayoutConfig, ShaderConfig, Vertex,
        },
        render_context::{CommandEncoder, RenderContext, RenderContextConfig, RenderPass},
        window::{Window, WindowCommandEncoder, WindowConfig},
        Handler, HandlerConfig,
    },
    texture::RawTexture,
};

pub mod core {
    pub use crate::core::{
        mesh::{index_format, RawMesh},
        vertex::{PosVertex, RBGAVertex, RGBVertex, UVVertex},
    };
}
