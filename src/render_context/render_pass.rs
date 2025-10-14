use crate::prelude::*;

pub mod render_pass_mut;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instanced {
    pub size: u32,
}

#[derive(Debug)]
pub struct RenderPass<
    'r,
    Layout = Void,
    Shader = Void,
    const SHADER_SETTINGS: bool = false,
    Instance: Copy = Void,
> {
    pub(crate) render_pass: wgpu::RenderPass<'r>,
    pub(crate) __layout: PhantomData<Layout>,
    pub(crate) __shader_attached: PhantomData<Shader>,
    pub(crate) instance: Instance,
}

impl<'r, L, S, const SA: bool, I: Copy> RenderPass<'r, L, S, SA, I> {
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
    pub unsafe fn coerce<NL, NS, const NSA: bool>(self) -> RenderPass<'r, NL, NS, NSA, I> {
        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    pub fn as_mut<'m>(&'m mut self) -> RenderPassMut<'m, 'r, L, S, SA, I> {
        RenderPassMut {
            render_pass: &mut self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    #[inline(always)]
    pub fn wipe(self) -> RenderPass<'r, Void, Void, SA, Void> {
        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: Void,
        }
    }

    #[inline]
    pub fn set_shared_data<NL: Layout>(
        mut self,
        shared_data: SharedData<NL>,
    ) -> RenderPass<'r, NL, Void, false, Void> {
        NL::set_shared_data(unsafe { self.inner() }, shared_data);

        unsafe { self.wipe().coerce() }
    }

    #[inline]
    pub fn create_shared_data<NL: Layout>(mut self) -> RenderPass<'r, NL, Void, false, Void>
    where
        for<'k> SharedData<'k, NL>: Default,
    {
        NL::set_shared_data(unsafe { self.inner() }, default());

        unsafe { self.wipe().coerce() }
    }
}

impl<'r, L: Layout, S, const SA: bool, I: Copy> RenderPass<'r, L, S, SA, I> {
    #[inline]
    pub fn apply_shader<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
    ) -> RenderPass<'r, L, NS, false, I> {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn apply_shader_with<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
        settings: &NS::Settings,
    ) -> RenderPass<'r, L, NS, true, I> {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
        NS::apply_settings(unsafe { self.inner() }, settings);

        unsafe { self.coerce() }
    }

    #[inline]
    pub fn apply_shader_with_default<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
    ) -> RenderPass<'r, L, NS, true, I>
    where
        NS::Settings: Default,
    {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
        NS::apply_settings(unsafe { self.inner() }, &default());

        unsafe { self.coerce() }
    }

    pub fn set_instance_requirements(
        mut self,
        requirements: InstanceData<L>,
    ) -> RenderPass<'r, L, S, SA, Instanced>
    where
        L: InstancedLayout,
    {
        let size = L::set_instances(unsafe { self.inner() }, requirements);

        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: Instanced { size },
        }
    }

    pub fn draw_screen_quad(&mut self)
    where
        L::VertexLayout: VertexRequirements<Requirements = ()>,
    {
        unsafe { self.inner() }.draw(0..3, 0..1);
    }
}

impl<'r, L: Layout, S: Shader, const SA: bool, I: Copy> RenderPass<'r, L, S, SA, I> {
    #[inline]
    pub fn apply_settings(mut self, settings: &S::Settings) -> RenderPass<'r, L, S, true, I> {
        S::apply_settings(unsafe { self.inner() }, settings);

        unsafe { self.coerce() }
    }

    pub fn default_settings(mut self) -> RenderPass<'r, L, S, true, I>
    where
        S::Settings: Default,
    {
        S::apply_settings(unsafe { self.inner() }, &default());

        unsafe { self.coerce() }
    }
}

impl<'r, L: Layout, S: Shader> RenderPass<'r, L, S, true, Void> {
    #[inline]
    pub fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) {
        unsafe { mesh.draw(self.inner()) };
    }
}

impl<'r, L: Layout, S: Shader> RenderPass<'r, L, S, true, Instanced> {
    #[inline]
    pub fn draw_mesh_instanced<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) {
        unsafe { mesh.draw_instanced(&mut self.render_pass, 0..self.instance.size) };
    }
}
