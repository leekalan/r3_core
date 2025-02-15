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
    let layout = Layout::new(
        app.render_context().clone(),
        LayoutConfig {
            format: app.window.format(),
            ..Default::default()
        },
    );

    let mesh = RawMesh::new_uint16(app.render_context(), VERTICES, INDICES);

    let shader = StaticShaderInstance::new(Arc::new(NewShader::new(layout)));

    let mut encoder = app.window.command_encoder();

    encoder.render_pass().apply_shader(&shader).draw_mesh(&mesh);

    encoder.present();
}

struct NewShader {
    pipeline: wgpu::RenderPipeline,
}

impl NewShader {
    fn new(layout: Layout<RGBVertex>) -> Self {
        let module = layout.render_context().create_shader_module(
            Some("Shader"),
            wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        );

        Self {
            pipeline: layout.create_pipeline(&module, ShaderConfig::default()),
        }
    }
}

impl Shader for NewShader {
    type V = RGBVertex;
    type Config = ();

    fn set_shader(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
    }

    fn apply_config(&self, _: &mut wgpu::RenderPass, _: &Self::Config) {}
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
