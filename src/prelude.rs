pub use crate::{
    handler::{
        app::App,
        layout::{
            shader::{
                ApplyShaderInstance, Shader, ShaderHandle, ShaderInstance, StaticShaderInstance,
            },
            Layout, LayoutConfig, ShaderConfig, Vertex,
        },
        raw_mesh::{index_format, RawMesh},
        render_context::{CommandEncoder, RenderContext, RenderContextConfig, RenderPass},
        surface::Surface,
        window::{Window, WindowCommandEncoder, WindowConfig},
        Handler, HandlerConfig,
    },
    texture::RawTexture,
};

pub mod core {
    pub use crate::core::{
        surface::{ArcMesh, ExtendedSurface, Mesh, SurfaceExt},
        vertex::{PosVertex, RBGAVertex, RGBVertex, UVVertex},
    };
}
