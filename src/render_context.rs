use std::{mem::transmute, ops::Range};

use crate::{
    layout::vertex::NoInstanceRequirements,
    prelude::*,
    surface::{mesh::Mesh, SurfaceInstanced},
};

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
                ..default()
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

    pub fn command_encoder(&'_ self) -> CommandEncoder<'_> {
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
        &'_ mut self,
        view: &RawTextureView<Texture2D>,
        load: Option<wgpu::LoadOp<wgpu::Color>>,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
    ) -> RenderPass<'_, Void> {
        let render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load.unwrap_or(wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment,
            ..Default::default()
        });

        RenderPass {
            render_pass,
            __layout: PhantomData,
        }
    }

    pub fn compute_pass(&mut self) -> ComputePass<'_, Void> {
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
pub struct RenderPass<'r, Layout, const SHADER_ATTACHED: bool = false> {
    render_pass: wgpu::RenderPass<'r>,
    __layout: PhantomData<Layout>,
}

impl<'r, L, const SA: bool> RenderPass<'r, L, SA> {
    /// # Safety
    /// This function is unsafe because it allows the caller
    /// to mutate the inner `wgpu::RenderPass`
    #[inline(always)]
    pub unsafe fn inner(&mut self) -> &mut wgpu::RenderPass<'r> {
        &mut self.render_pass
    }

    /// # Safety
    /// This function is unsafe because it coerces the layout
    #[inline(always)]
    pub unsafe fn coerce<NewLayout, const NEW_SA: bool>(self) -> RenderPass<'r, NewLayout, NEW_SA> {
        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
        }
    }

    /// # Safety
    /// This function is unsafe because it coerces the layout
    #[inline(always)]
    pub unsafe fn coerce_ref<NewLayout, const NEW_SA: bool>(
        &mut self,
    ) -> &mut RenderPass<'r, NewLayout, NEW_SA> {
        transmute::<&mut RenderPass<'r, L, SA>, &mut RenderPass<'r, NewLayout, NEW_SA>>(self)
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
}

impl<'r, L: Layout, const SA: bool> RenderPass<'r, L, SA> {
    #[inline]
    pub fn apply_shader(mut self, handle: &ShaderHandle<L>) -> RenderPass<'r, L, true> {
        handle.apply_shader(unsafe { self.inner() });

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn apply_shader_ref(&mut self, handle: &ShaderHandle<L>) -> &mut RenderPass<'r, L, true> {
        handle.apply_shader(unsafe { self.inner() });

        unsafe { self.coerce_ref() }
    }

    pub fn set_instances<I: Instances<IRequirements<L::VertexLayout>>>(
        mut self,
        instances: &I,
    ) -> RenderPassInstanced<'r, L, SA>
    where
        L::VertexLayout: InstanceRequirements,
    {
        unsafe { instances.set_vertex_buffers(self.inner()) };

        RenderPassInstanced {
            render_pass: self,
            instances: instances.range(),
        }
    }

    #[inline]
    pub fn draw_surface<S: Surface<Layout = L>>(&mut self, surface: &S) -> &mut Self
    where
        L::VertexLayout: NoInstanceRequirements,
    {
        surface.draw(self);
        self
    }

    /// # Safety
    /// This function is unsafe because it is impossible to check
    /// if the `instances` range is valid or even excpected
    #[inline]
    pub unsafe fn draw_instanced_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(
        &mut self,
        mesh: &M,
        instances: Range<u32>,
    ) -> &mut Self {
        unsafe { mesh.draw_instanced(self.inner(), instances) };
        self
    }

    #[inline]
    pub fn draw_screen_quad(&mut self) -> &mut Self
    where
        L::VertexLayout: VertexRequirements<Requirements = ()>,
    {
        unsafe { self.inner() }.draw(0..3, 0..1);

        self
    }
}

impl<'r, L: Layout> RenderPass<'r, L, false> {
    /// # Safety
    /// This function is unsafe because it is impossible to check
    /// if a shader has been applied
    #[inline]
    pub unsafe fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(
        &mut self,
        mesh: &M,
    ) -> &mut Self
    where
        L::VertexLayout: NoInstanceRequirements,
    {
        unsafe { mesh.draw(self.inner()) };
        self
    }
}

impl<'r, L: Layout> RenderPass<'r, L, true> {
    #[inline]
    pub fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) -> &mut Self
    where
        L::VertexLayout: NoInstanceRequirements,
    {
        unsafe { mesh.draw(self.inner()) };
        self
    }
}

#[derive(Debug)]
pub struct RenderPassInstanced<'r, L, const SA: bool = false> {
    render_pass: RenderPass<'r, L, SA>,
    instances: Range<u32>,
}

impl<'r, L, const SA: bool> RenderPassInstanced<'r, L, SA> {
    /// # Safety
    /// This function is unsafe because it coerces the layout
    #[inline]
    pub unsafe fn coerce<NewLayout, const NEW_SA: bool>(
        self,
    ) -> RenderPassInstanced<'r, NewLayout, NEW_SA> {
        RenderPassInstanced {
            render_pass: unsafe { self.render_pass.coerce() },
            instances: self.instances.clone(),
        }
    }

    /// # Safety
    /// This function is unsafe because it coerces the layout
    #[inline]
    pub unsafe fn coerce_ref<NewLayout, const NEW_SA: bool>(
        &mut self,
    ) -> &mut RenderPassInstanced<'r, NewLayout, NEW_SA> {
        unsafe {
            transmute::<
                &mut RenderPassInstanced<'r, L, SA>,
                &mut RenderPassInstanced<'r, NewLayout, NEW_SA>,
            >(self)
        }
    }

    #[inline(always)]
    pub fn wipe_instances(self) -> RenderPass<'r, L, SA> {
        self.render_pass
    }
}

impl<'r, L: Layout, const SA: bool> RenderPassInstanced<'r, L, SA>
where
    L::VertexLayout: InstanceRequirements,
{
    #[inline]
    pub fn apply_shader(mut self, handle: &ShaderHandle<L>) -> RenderPassInstanced<'r, L, true> {
        handle.apply_shader(unsafe { self.render_pass.inner() });

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn apply_shader_ref(
        &mut self,
        handle: &ShaderHandle<L>,
    ) -> &mut RenderPassInstanced<'r, L, true> {
        handle.apply_shader(unsafe { self.render_pass.inner() });

        unsafe { self.coerce_ref() }
    }

    #[inline]
    pub fn set_shared_data<NewLayout: Layout>(
        self,
        shared_data: SharedData<NewLayout>,
    ) -> RenderPassInstanced<'r, NewLayout>
    where
        VertexLayout<NewLayout>:
            InstanceRequirements<Requirements = IRequirements<L::VertexLayout>>,
    {
        RenderPassInstanced {
            render_pass: self.render_pass.set_shared_data(shared_data),
            instances: self.instances,
        }
    }

    #[inline]
    pub fn create_shared_data<NewLayout: Layout>(self) -> RenderPassInstanced<'r, NewLayout>
    where
        VertexLayout<NewLayout>:
            InstanceRequirements<Requirements = IRequirements<L::VertexLayout>>,
        for<'a> SharedData<'a, NewLayout>: Default,
    {
        RenderPassInstanced {
            render_pass: self.render_pass.create_shared_data(),
            instances: self.instances,
        }
    }

    #[inline]
    pub fn draw_surface<S: SurfaceInstanced<Layout = L>>(&mut self, surface: &S) -> &mut Self {
        surface.draw(self);
        self
    }
}

impl<'r, L: Layout> RenderPassInstanced<'r, L, false>
where
    L::VertexLayout: InstanceRequirements,
{
    /// # Safety
    /// This function is unsafe because it is impossible to check
    /// if a shader has been applied
    #[inline]
    pub unsafe fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(
        &mut self,
        mesh: &M,
    ) -> &mut Self {
        unsafe { mesh.draw_instanced(self.render_pass.inner(), self.instances.clone()) };
        self
    }
}

impl<'r, L: Layout> RenderPassInstanced<'r, L, true>
where
    L::VertexLayout: InstanceRequirements,
{
    #[inline]
    pub fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) -> &mut Self {
        unsafe { mesh.draw_instanced(self.render_pass.inner(), self.instances.clone()) };
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
    pub fn apply_compute_shader(&mut self, handle: &ComputeShaderHandle<L>) -> &mut Self {
        let inner = unsafe { self.inner() };

        handle.apply_compute_shader(inner);

        self
    }

    #[inline]
    pub fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32) -> &mut Self {
        unsafe { self.inner() }.dispatch_workgroups(x, y, z);
        self
    }
}
