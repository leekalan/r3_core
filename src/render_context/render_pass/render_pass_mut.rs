use crate::prelude::*;

#[derive(Debug)]
pub struct RenderPassMut<
    'm,
    'r,
    Layout = Void,
    Shader = Void,
    const SHADER_SETTINGS: bool = false,
    Instance: Copy = Void,
> {
    pub(crate) render_pass: &'m mut wgpu::RenderPass<'r>,
    pub(crate) __layout: PhantomData<Layout>,
    pub(crate) __shader_attached: PhantomData<Shader>,
    pub(crate) instance: Instance,
}

impl<'m, 'r, L, S, const SA: bool, I: Copy> RenderPassMut<'m, 'r, L, S, SA, I> {
    /// # Safety
    /// This function is unsafe because it allows the caller
    /// to mutate the inner `wgpu::RenderPass`
    #[inline(always)]
    pub unsafe fn inner(&mut self) -> &mut wgpu::RenderPass<'r> {
        self.render_pass
    }

    #[inline(always)]
    pub fn as_mut(&mut self) -> RenderPassMut<'_, 'r, L, S, SA, I> {
        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }
}

impl<'m, 'r> RenderPassMut<'m, 'r, Void, Void, false, Void> {
    #[inline]
    pub fn set_shared_data_ref<NL: Layout>(
        mut self,
        shared_data: &SharedData<NL>,
    ) -> RenderPassMut<'m, 'r, NL, Void, false, Void> {
        NL::set_shared_data(unsafe { self.inner() }, shared_data);

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    #[inline]
    pub fn create_shared_data<NL: Layout>(mut self) -> RenderPassMut<'m, 'r, NL, Void, false, Void>
    where
        SharedData<NL>: Default,
    {
        NL::set_shared_data(unsafe { self.inner() }, &default());

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }
}

impl<'m, 'r, L: Layout, I: Copy> RenderPassMut<'m, 'r, L, Void, false, I> {
    #[inline]
    pub fn override_shared_data(mut self, shared_data: &SharedData<L>) -> Self {
        L::set_shared_data(unsafe { self.inner() }, shared_data);
        self
    }
}

impl<'m, 'r, L: Layout, I: Copy> RenderPassMut<'m, 'r, L, Void, false, I> {
    #[inline]
    pub fn apply_shader<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
    ) -> RenderPassMut<'m, 'r, L, NS, false, I> {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    #[inline]
    pub fn apply_shader_with<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
        settings: &NS::Settings,
    ) -> RenderPassMut<'m, 'r, L, NS, true, I> {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
        NS::apply_settings(unsafe { self.inner() }, settings);

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    #[inline]
    pub fn apply_shader_with_default<NS: Shader<Layout = L>>(
        mut self,
        shader: &NS,
    ) -> RenderPassMut<'m, 'r, L, NS, true, I>
    where
        NS::Settings: Default,
    {
        unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
        NS::apply_settings(unsafe { self.inner() }, &default());

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    pub fn draw_screen_quad(mut self)
    where
        L::VertexLayout: VertexRequirements<Requirements = ()>,
    {
        unsafe { self.inner() }.draw(0..3, 0..1);
    }
}

impl<'m, 'r, L: Layout, S: Shader, const SA: bool, I: Copy> RenderPassMut<'m, 'r, L, S, SA, I> {
    pub fn apply_settings(
        mut self,
        settings: &S::Settings,
    ) -> RenderPassMut<'m, 'r, L, S, true, I> {
        S::apply_settings(unsafe { self.inner() }, settings);

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }

    pub fn default_settings(mut self) -> RenderPassMut<'m, 'r, L, S, true, I>
    where
        S::Settings: Default,
    {
        S::apply_settings(unsafe { self.inner() }, &default());

        RenderPassMut {
            render_pass: self.render_pass,
            __layout: PhantomData,
            __shader_attached: PhantomData,
            instance: self.instance,
        }
    }
}

impl<'m, 'r, L: Layout, S: Shader> RenderPassMut<'m, 'r, L, S, true, Void> {
    #[inline]
    pub fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(mut self, mesh: &M) {
        unsafe { mesh.draw(self.inner()) };
    }
}

impl<'m, 'r, L: Layout, S: Shader> RenderPassMut<'m, 'r, L, S, true, super::Instanced> {
    #[inline]
    pub fn draw_mesh_instanced<M: Mesh<VRequirements<L::VertexLayout>>>(self, mesh: &M) {
        unsafe { mesh.draw_instanced(self.render_pass, 0..self.instance.size) };
    }
}
