use r3_core::prelude::{core::*, *};

use cgmath::Vector3;

use wgpu::Extent3d;
use winit::{
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

#[tokio::main]
async fn main() {
    let render_context = RenderContext::new(RenderContextConfig {
        features: Some(wgpu::Features::all_webgpu_mask()),
        ..default()
    })
    .await;

    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut handler = Handler::new(
        render_context,
        WindowConfig {
            clear: Some(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            }),
            ..default()
        },
        on_start,
        on_event,
    );

    handler.init(event_loop);
}

fn on_start(app: AppConfig<Void>, _: &ActiveEventLoop) -> State {
    let (width, height) = app.window.size();

    let camera_controller = GroundedCamera {
        position: Vector3::new(0.0, 0.0, -3.0),
        ..default()
    };

    let camera = Camera::new(
        CameraBind::new(
            app.render_context,
            CameraBindLayout::new(app.render_context),
            UniformBuffer::new(app.render_context),
        ),
        Projection::new(width as _, height as _),
        camera_controller.generate_transform(),
    );

    let layout = NewLayout::new(app.render_context, camera.layout());

    let shader = NewShader::new(app.render_context, &layout);

    let shader_instance = DefaultShaderInstance::new(shader);

    let mesh = Mesh::new(
        RawMesh::new_uint16(app.render_context, VERTICES, INDICES),
        shader_instance,
    );

    let post_processing_layout = PostProcessingLayout::new(app.render_context, width, height);

    let post_processing_shader = DefaultShaderInstance::new(PostProcessingShader::new(
        app.render_context,
        &post_processing_layout,
    ));

    State {
        camera_controller,
        camera,
        _layout: layout,
        mesh,
        post_processing_layout,
        post_processing_shader,
    }
}

fn on_event(app: &mut App<State>, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    let post_proc = &mut app.state.post_processing_layout.post_proc;

    let camera = &mut app.state.camera;
    let camera_controller = &mut app.state.camera_controller;

    camera_controller.yaw += 0.04;

    if let WindowEvent::Resized(new_size) = event {
        camera
            .projection
            .resize(new_size.width as _, new_size.height as _);
        camera.update_projection();

        post_proc.render.resize(
            &app.render_context,
            None,
            Extent3d {
                width: new_size.width,
                height: new_size.height,
                depth_or_array_layers: 1,
            },
        );
        post_proc.refresh(&app.render_context);
    };

    camera.apply_transform(camera_controller.generate_transform());

    camera.write_buffer(&app.render_context);

    let mut encoder = app.window.command_encoder();

    encoder
        .render_pass(None, true)
        .set_shared_data(camera.bind())
        .draw_surface(&app.state.mesh);

    encoder.copy_from_output(post_proc.raw_texture());

    encoder
        .render_pass(None, false)
        .set_shared_data(post_proc)
        .apply_shader(&app.state.post_processing_shader)
        .draw_screen_quad();

    encoder.present();
}

#[derive(Debug)]
struct State {
    camera_controller: GroundedCamera,
    camera: Camera,
    _layout: NewLayout,
    mesh: Mesh<DefaultShaderInstance<NewShader>>,
    post_processing_layout: PostProcessingLayout,
    post_processing_shader: DefaultShaderInstance<PostProcessingShader>,
}

#[repr(transparent)]
#[derive(Debug, Clone)]
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
                    bind_group_layouts: &[camera.wgpu_layout()],
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
#[derive(Debug, Clone)]
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

    #[inline(always)]
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

#[derive(Debug, Clone)]
struct PostProcessingLayout {
    layout: RawLayout<Void>,
    post_proc: PostProc,
}

impl PostProcessingLayout {
    fn new(render_context: &RenderContext, width: u32, height: u32) -> Self {
        let post_proc = PostProc::new(render_context, wgpu::TextureUsages::COPY_DST, width, height);

        let layout = RawLayout::new(
            render_context,
            LayoutConfig {
                bind_group_layouts: &[post_proc.wgpu_layout()],
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
            },
        );

        Self { layout, post_proc }
    }
}

impl Layout for PostProcessingLayout {
    type Vertex = Void;
    type SharedData = PostProc;

    fn raw_layout(&self) -> &RawLayout<Self::Vertex> {
        &self.layout
    }

    fn set_shared_data(render_pass: &mut wgpu::RenderPass, shared_data: &SharedLayoutData<Self>) {
        render_pass.set_bind_group(0, shared_data.bind_group(), &[]);
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
struct PostProcessingShader {
    pipeline: wgpu::RenderPipeline,
}

impl PostProcessingShader {
    fn new(render_context: &RenderContext, layout: &PostProcessingLayout) -> Self {
        let module = render_context.create_shader_module(
            Some("Post Processing"),
            wgpu::ShaderSource::Wgsl(include_str!("post_processing.wgsl").into()),
        );

        Self {
            pipeline: layout.create_pipeline(
                render_context,
                &module,
                ShaderConfig {
                    depth_stencil: Some(None),
                    ..default()
                },
            ),
        }
    }
}

impl Shader for PostProcessingShader {
    type Layout = PostProcessingLayout;

    fn get_pipeline(&self, _: &Self::Settings) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
