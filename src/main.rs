use std::f32::consts::PI;

use r3_core::prelude::{core::*, *};

use cgmath::{Quaternion, Rad, Rotation3, Vector3};

use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

#[tokio::main]
async fn main() {
    let render_context = RenderContext::new(RenderContextConfig::default()).await;

    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let window_config = WindowConfig {
        clear: Some(wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        }),
        ..default()
    };

    let mut handler = Handler::new(HandlerConfig::new(
        render_context,
        window_config,
        on_start,
        on_event,
    ));

    handler.init(event_loop);
}

fn on_start(app: AppConfig<Void>, _: &ActiveEventLoop) -> State {
    let (width, height) = app.window.size();

    let camera = Camera::new(
        CameraBind::new(
            app.render_context,
            CameraBindLayout::new(app.render_context),
            UniformBuffer::new(app.render_context),
        ),
        Projection::new(width as _, height as _),
        CameraTransform {
            position: Vector3::new(0.0, 0.0, -5.0),
            rotation: Quaternion::from_angle_y(Rad(PI / 6.0)),
            ..default()
        },
    );

    let layout = NewLayout::new(app.render_context, camera.layout());

    let shader = NewShader::new(app.render_context, &layout);

    let shader_instance = DefaultShaderInstance::new(shader);

    let mesh = Mesh::new(
        RawMesh::new_uint16(app.render_context, VERTICES, INDICES),
        shader_instance,
    );

    State {
        camera,
        _layout: layout,
        mesh,
        rot: 0.0,
    }
}

fn on_event(app: &mut App<State>, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    app.state.rot += 0.04;

    let camera = &mut app.state.camera;
    camera.transform.rotation = Quaternion::from_angle_y(Rad(app.state.rot));

    if let WindowEvent::Resized(new_size) = event {
        camera
            .projection
            .resize(new_size.width as _, new_size.height as _);
        camera.update_projection()
    } else {
        camera.update_transform()
    }
    .write_buffer(&app.render_context);

    let mut encoder = app.window.command_encoder();

    encoder
        .render_pass()
        .set_shared_data(camera.bind())
        .draw_surface(&app.state.mesh);

    encoder.present();
}

struct State {
    camera: Camera,
    _layout: NewLayout,
    mesh: Mesh<DefaultShaderInstance<NewShader>>,
    rot: f32,
}

#[repr(transparent)]
struct NewLayout {
    layout: RawLayout<RGBVertex>,
}

impl NewLayout {
    #[inline]
    fn new(render_context: &RenderContext, camera: &CameraBindLayout) -> Self {
        Self {
            layout: RawLayout::new(
                render_context,
                LayoutConfig {
                    bind_group_layouts: &[camera.layout()],
                    ..default()
                },
            ),
        }
    }
}

impl Layout for NewLayout {
    type Vertex = RGBVertex;
    type SharedData = CameraBind;

    #[inline(always)]
    fn raw_layout(&self) -> &RawLayout<Self::Vertex> {
        &self.layout
    }

    #[inline(always)]
    fn set_shared_data(render_pass: &mut wgpu::RenderPass, shared_data: &SharedLayoutData<Self>) {
        render_pass.set_bind_group(0, shared_data.bind_group(), &[]);
    }
}

#[repr(transparent)]
struct NewShader {
    pipeline: wgpu::RenderPipeline,
}

impl NewShader {
    #[inline]
    fn new(render_context: &RenderContext, layout: &NewLayout) -> Self {
        let module = render_context.create_shader_module(
            Some("Shader"),
            wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        );

        Self {
            pipeline: layout.create_pipeline(render_context, &module, ShaderConfig::default()),
        }
    }
}

impl Shader for NewShader {
    type Layout = NewLayout;

    #[inline]
    fn get_pipeline(&self, _: &Self::Settings) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

const VERTICES: &[RGBVertex] = &[
    RGBVertex {
        position: [-0.0868241, 0.49240386, -2.0],
        color: [1.0, 0.0, 0.0],
    }, // A
    RGBVertex {
        position: [-0.49513406, 0.06958647, -2.0],
        color: [1.0, 1.0, 0.0],
    }, // B
    RGBVertex {
        position: [-0.21918549, -0.44939706, -2.0],
        color: [0.0, 1.0, 0.0],
    }, // C
    RGBVertex {
        position: [0.35966998, -0.3473291, -2.0],
        color: [0.0, 0.0, 1.0],
    }, // D
    RGBVertex {
        position: [0.44147372, 0.2347359, -2.0],
        color: [1.0, 0.0, 1.0],
    }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
