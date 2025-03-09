pub use crate::{
    handler::{
        app::App,
        layout::{
            shader::{ApplyShaderInstance, Shader, ShaderSettings, ShaderHandle, ShaderInstance, StaticShaderInstance},
            Layout, LayoutConfig, LayoutInstance, LayoutVertex, RawLayout, ShaderConfig, Vertex,
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
