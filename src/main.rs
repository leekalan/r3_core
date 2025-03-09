use std::sync::Arc;

use r3_core::prelude::{core::*, *};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

#[tokio::main]
async fn main() {
    let render_context = RenderContext::new(RenderContextConfig::default()).await;

    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let window_config = WindowConfig {
        clear: Some(wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        }),
        ..Default::default()
    };

    let mut handler = Handler::new(
        HandlerConfig::new(render_context, window_config, ()).on_start(Box::new(on_start)),
    );

    handler.init(event_loop);
}

fn on_start(app: &mut App<()>, _: &ActiveEventLoop) {
    let render_context = app.render_context();

    let layout = NewLayout::new(render_context);

    let shader = Arc::new(NewShader::new(render_context, layout));

    let shader_instance = Arc::new(StaticShaderInstance::new(shader));

    let mesh = Mesh::new(
        RawMesh::new_uint16(render_context, VERTICES, INDICES),
        shader_instance,
    );

    let mut encoder = app.window.command_encoder();

    encoder.render_pass().draw_surface(mesh);

    encoder.present();
}

#[repr(transparent)]
struct NewLayout {
    layout: RawLayout<RGBVertex>,
}

impl NewLayout {
    #[inline]
    fn new(render_context: &RenderContext) -> Self {
        Self {
            layout: RawLayout::new(render_context, LayoutConfig::default()),
        }
    }
}

impl Layout for NewLayout {
    type Vertex = RGBVertex;

    #[inline]
    fn raw_layout(&self) -> &RawLayout<Self::Vertex> {
        &self.layout
    }

    #[inline]
    fn set_instance(_: &mut wgpu::RenderPass, _: &LayoutInstance<Self>) {}
}

#[repr(transparent)]
struct NewShader {
    pipeline: wgpu::RenderPipeline,
}

impl NewShader {
    #[inline]
    fn new(render_context: &RenderContext, layout: NewLayout) -> Self {
        let module = render_context.create_shader_module(
            Some("Shader"),
            wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        );

        Self {
            pipeline: layout.raw_layout().create_pipeline(
                render_context,
                &module,
                ShaderConfig::default(),
            ),
        }
    }
}

impl Shader for NewShader {
    type Layout = NewLayout;

    #[inline]
    fn pipeline(&self, _: &Self::Settings) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

const VERTICES: &[RGBVertex] = &[
    RGBVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // A
    RGBVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [1.0, 1.0, 0.0],
    }, // B
    RGBVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // C
    RGBVertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // D
    RGBVertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [1.0, 0.0, 1.0],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
