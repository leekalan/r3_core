use std::{num::NonZero, sync::Arc};

use r3_core::prelude::{core::*, *};
use wgpu::util::DeviceExt;
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

const VERTICES: &[ColoredVertex] = &[
    ColoredVertex {
        position: [0.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // A
    ColoredVertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [1.0, 1.0, 0.0],
    }, // B
    ColoredVertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // C
    ColoredVertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // D
    ColoredVertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [1.0, 0.0, 1.0],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

fn on_start(app: &mut App<()>, _: &ActiveEventLoop) {
    let layout = Layout::new(app.render_context().clone(), LayoutConfig::default());

    let shader = StaticShaderInstance::new(Arc::new(NewShader::new(layout)), ()).handle();

    let vertex_buffer =
        app.render_context()
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

    const NEW_VERT: &[ColoredVertex] = &[ColoredVertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [1.0, 0.0, 0.0],
    }];

    let mut write = app
        .render_context()
        .queue
        .write_buffer_with(
            &vertex_buffer,
            0,
            NonZero::new(size_of::<ColoredVertex>() as u64).unwrap(),
        )
        .unwrap();

    write.copy_from_slice(bytemuck::cast_slice(NEW_VERT));

    drop(write);

    let index_buffer =
        app.render_context()
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

    let mut encoder = app.window.command_encoder();

    let mut render_pass = encoder.render_pass();

    render_pass.set_shader(&shader);

    let inner = render_pass.inner();
    inner.set_vertex_buffer(0, vertex_buffer.slice(..));
    inner.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

    render_pass
        .inner()
        .draw_indexed(0..INDICES.len() as u32, 0, 0..1);
    drop(render_pass);

    encoder.present();
}

struct NewShader {
    pipeline: wgpu::RenderPipeline,
}

impl NewShader {
    fn new(layout: Layout<ColoredVertex>) -> Self {
        let shader =
            layout
                .render_context()
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Shader"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
                });

        let pipeline = layout.render_context().device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(layout.layout()),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[ColoredVertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: layout.format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: RawTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            },
        );

        Self { pipeline }
    }
}

impl Shader for NewShader {
    type V = ColoredVertex;
    type Config = ();

    fn set_shader(&self, render_pass: &mut RenderPass) {
        render_pass.inner().set_pipeline(&self.pipeline);
    }

    fn apply_config(&self, _: &mut RenderPass, _: &Self::Config) {}
}
