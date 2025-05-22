use std::ops::Range;

use crate::{prelude::*, surface::mesh::Mesh};

#[derive(Default, Debug, Clone)]
pub struct RenderContextConfig {
    pub backends: Option<wgpu::Backends>,
    pub power_preference: Option<wgpu::PowerPreference>,
    pub features: Option<wgpu::Features>,
    pub limits: Option<wgpu::Limits>,
}

#[derive(Debug, Clone)]
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

    pub async fn new(config: RenderContextConfig) -> Self {
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
            .request_device(&wgpu::DeviceDescriptor {
                required_features: config.features.unwrap_or(wgpu::Features::empty()),
                required_limits: config.limits.unwrap_or_default(),
                label: Some("Device"),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        Self {
            instance,
            adapter,
            device,
            queue,
        }
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

#[derive(Debug)]
pub struct CommandEncoder<'r> {
    pub encoder: wgpu::CommandEncoder,
    pub render_context: &'r RenderContext,
}

impl CommandEncoder<'_> {
    pub fn render_pass(
        &mut self,
        view: &RawTextureView<Texture2D>,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
    ) -> RenderPass<Void> {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load.unwrap_or(wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment,
            ..Default::default()
        });

        RenderPass {
            render_pass,
            __layout: PhantomData,
        }
    }

    pub fn compute_pass(&mut self) -> ComputePass<Void> {
        ComputePass {
            compute_pass: self
                .encoder
                .begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass"),
                    ..Default::default()
                }),
            __layout: PhantomData,
        }
    }

    #[inline]
    pub fn submit(self) {
        self.render_context
            .queue
            .submit(Some(self.encoder.finish()));
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct RenderPass<'r, Layout> {
    render_pass: wgpu::RenderPass<'r>,
    __layout: PhantomData<Layout>,
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
            __layout: PhantomData,
        }
    }

    #[inline(always)]
    pub fn wipe(self) -> RenderPass<'r, Void> {
        unsafe { self.coerce() }
    }

    #[inline]
    pub fn set_shared_data<NewLayout: Layout>(
        mut self,
        shared_data: SharedData<NewLayout>,
    ) -> RenderPass<'r, NewLayout> {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, shared_data);

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn create_shared_data<NewLayout: Layout>(mut self) -> RenderPass<'r, NewLayout>
    where
        for<'a> SharedData<'a, NewLayout>: Default,
    {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, Default::default());

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn draw_screen_quad(&mut self) -> &mut Self {
        unsafe { self.inner() }.draw(0..3, 0..1);

        self
    }
}

impl<L: Layout> RenderPass<'_, L> {
    #[inline]
    pub fn apply_shader(&mut self, handle: &ShaderHandle<L>) -> &mut Self {
        let inner = unsafe { self.inner() };

        handle.apply_shader(inner);

        self
    }

    #[inline(always)]
    pub fn draw_surface<S: Surface<Layout = L>>(&mut self, surface: &S) {
        surface.draw(self);
    }

    #[inline]
    pub fn draw_mesh<M: Mesh<Requirements<L::VertexLayout>>>(&mut self, mesh: &M) -> &mut Self {
        unsafe { mesh.draw(self.inner()) };
        self
    }

    #[inline]
    pub fn draw_instanced_mesh<M: Mesh<Requirements<L::VertexLayout>>>(
        &mut self,
        mesh: &M,
        instances: Range<u32>,
    ) -> &mut Self {
        unsafe { mesh.draw_instanced(self.inner(), instances) };
        self
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ComputePass<'r, Layout> {
    compute_pass: wgpu::ComputePass<'r>,
    __layout: PhantomData<Layout>,
}

impl<'r, L> ComputePass<'r, L> {
    /// # Safety
    /// This function is unsafe because it allows the caller
    /// to mutate the inner `wgpu::RenderPass`
    #[inline]
    pub unsafe fn inner(&mut self) -> &mut wgpu::ComputePass<'r> {
        &mut self.compute_pass
    }

    /// # Safety
    /// This function is unsafe because it coerces the vertex type
    #[inline]
    pub unsafe fn coerce<NewLayout>(self) -> ComputePass<'r, NewLayout> {
        ComputePass {
            compute_pass: self.compute_pass,
            __layout: PhantomData,
        }
    }

    #[inline]
    pub fn wipe(self) -> ComputePass<'r, Void> {
        unsafe { self.coerce() }
    }

    #[inline]
    pub fn set_shared_data<NewLayout: ComputeLayout>(
        mut self,
        shared_data: SharedComputeData<NewLayout>,
    ) -> ComputePass<'r, NewLayout> {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, shared_data);

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn create_shared_data<NewLayout: ComputeLayout>(mut self) -> ComputePass<'r, NewLayout>
    where
        for<'a> SharedComputeData<'a, NewLayout>: Default,
    {
        let inner = unsafe { self.inner() };

        NewLayout::set_shared_data(inner, Default::default());

        unsafe { self.coerce() }
    }
}

impl<L: ComputeLayout> ComputePass<'_, L> {
    #[inline]
    pub fn apply_compute_shader(&mut self, handle: &ComputeShaderHandle<L>) {
        let inner = unsafe { self.inner() };

        handle.apply_compute_shader(inner);
    }

    #[inline]
    pub fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) {
        unsafe { self.inner() }.dispatch_workgroups(x, y, z);
    }
}
