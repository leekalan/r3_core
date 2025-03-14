use crate::prelude::*;

#[derive(Default)]
pub struct RenderContextConfig {
    pub backends: Option<wgpu::Backends>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub features: Option<wgpu::Features>,
    pub limits: Option<wgpu::Limits>,
}

pub struct RenderContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RenderContext {
    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn device(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub const unsafe fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub async fn new(config: RenderContextConfig) -> Asc<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: config.backends.unwrap_or(wgpu::Backends::PRIMARY),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: config
                    .power_preference
                    .unwrap_or(wgpu::PowerPreference::HighPerformance),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: config.features.unwrap_or(wgpu::Features::empty()),
                    required_limits: config.limits.unwrap_or_default(),
                    label: Some("Device"),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        Asc::new(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    #[inline]
    pub fn create_shader_module(
        &self,
        label: Option<&str>,
        source: wgpu::ShaderSource,
    ) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor { label, source })
    }

    pub fn command_encoder(&self) -> CommandEncoder {
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        CommandEncoder {
            encoder,
            render_context: self,
        }
    }
}

pub struct CommandEncoder<'r> {
    pub encoder: wgpu::CommandEncoder,
    pub render_context: &'r RenderContext,
}

impl CommandEncoder<'_> {
    pub fn render_pass(
        &mut self,
        view: &wgpu::TextureView,
        clear: Option<wgpu::Color>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
    ) -> RenderPass<Void> {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear.unwrap_or(wgpu::Color::TRANSPARENT)),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment,
            ..Default::default()
        });

        RenderPass {
            render_pass,
            __vertex: PhantomData,
        }
    }

    #[inline]
    pub fn submit(self) {
        self.render_context
            .queue
            .submit(Some(self.encoder.finish()));
    }
}

pub struct RenderPass<'r, Layout> {
    render_pass: wgpu::RenderPass<'r>,
    __vertex: PhantomData<Layout>,
}

impl<'r, L> RenderPass<'r, L> {
    /// # Safety
    /// This function is unsafe because it allows the caller
    /// to mutate the inner `wgpu::RenderPass`
    #[inline]
    pub unsafe fn inner(&mut self) -> &mut wgpu::RenderPass<'r> {
        &mut self.render_pass
    }

    /// # Safety
    /// This function is unsafe because it coerces the vertex type
    #[inline]
    pub unsafe fn coerce<NewLayout>(self) -> RenderPass<'r, NewLayout> {
        RenderPass {
            render_pass: self.render_pass,
            __vertex: PhantomData,
        }
    }

    #[inline]
    pub fn wipe(self) -> RenderPass<'r, Void> {
        unsafe { self.coerce() }
    }

    #[inline]
    pub fn set_shared_data<NewLayout: Layout>(
        mut self,
        shared_data: &SharedLayoutData<NewLayout>,
    ) -> RenderPass<'r, NewLayout> {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, shared_data);

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn create_shared_data<NewLayout: Layout>(mut self) -> RenderPass<'r, NewLayout>
    where
        SharedLayoutData<NewLayout>: Default,
    {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, &Default::default());

        unsafe { self.coerce() }
    }
}

impl<L: Layout> RenderPass<'_, L> {
    #[inline]
    pub fn apply_shader(&mut self, handle: &ShaderHandle<L>) {
        let inner = unsafe { self.inner() };

        handle.apply_shader(inner);
    }

    #[inline]
    pub fn draw_surface<S: Surface<Layout = L>>(&mut self, surface: &S) {
        surface.draw(self);
    }

    #[inline]
    pub fn draw_mesh<I: index_format::IndexFormat>(
        &mut self,
        mesh: &RawMesh<L::Vertex, I>,
    ) -> &mut Self {
        mesh.draw(unsafe { self.inner() });
        self
    }
}
