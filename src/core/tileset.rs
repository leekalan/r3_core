use crate::prelude::{core::*, *};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileInstance {
    position: [f32; 2],
    tile_id: u32,
}

create_vertex_layout::layout!(pub SimpleTileLayout {
    PosVertex2d => Vertex,
    TileInstance => Instance,
});

const QUAD_VERTICES: &[PosVertex2d] = &[
    PosVertex2d { position: [0., 0.] },
    PosVertex2d { position: [1., 0.] },
    PosVertex2d { position: [1., 1.] },
    PosVertex2d { position: [0., 1.] },
];

const QUAD_INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct TilesetQuad(Asc<SimpleMesh<PosVertex2d, index_format::Uint16>>);

impl TilesetQuad {
    #[inline]
    pub fn new(render_context: &RenderContext) -> Self {
        let mesh = SimpleMesh::new_uint16(render_context, QUAD_VERTICES, QUAD_INDICES);

        Self(Asc::new(mesh))
    }

    #[inline(always)]
    pub fn inner(&self) -> &SimpleMesh<PosVertex2d, index_format::Uint16> {
        &self.0
    }
}

create_vertex_attr::attr!(TileInstance => [
    0 => Float32x2,
    1 => Uint32,
]);
