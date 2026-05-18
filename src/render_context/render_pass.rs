use reqs::feedable::{feedable, Fed, Req, Unfed};

use crate::{layouts::layout_binds::LayoutFeedableBind, prelude::*};

// pub mod render_pass_mut;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoInstance;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instanced {
    pub size: u32,
}

#[derive(Debug)]
pub struct RenderPass<'r, Layout = Void, LayoutBinds = Void, Shader = Void, Instance: Copy = Void> {
    pub(crate) render_pass: wgpu::RenderPass<'r>,
    pub(crate) __layout: PhantomData<Layout>,
    pub(crate) layout_binds: LayoutBinds,
    pub(crate) __shader: PhantomData<Shader>,
    pub(crate) instance: Instance,
}

impl<'r, L, B, S, I: Copy> RenderPass<'r, L, B, S, I> {
    /// # Safety
    /// This function is unsafe because it allows the caller
    /// to mutate the inner `wgpu::RenderPass`
    #[inline(always)]
    pub unsafe fn inner(&mut self) -> &mut wgpu::RenderPass<'r> {
        &mut self.render_pass
    }

    // pub fn as_mut<'m>(&'m mut self) -> RenderPassMut<'m, 'r, L, S, SA, I> {
    //     RenderPassMut {
    //         render_pass: &mut self.render_pass,
    //         __layout: PhantomData,
    //         __shader_attached: PhantomData,
    //         instance: self.instance,
    //     }
    // }

    #[inline(always)]
    pub fn wipe(self) -> RenderPass<'r, Void, Void, Void, Void> {
        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            layout_binds: Void,
            __shader: PhantomData,
            instance: Void,
        }
    }

    pub fn use_layout<NL: Layout>(
        self,
    ) -> RenderPass<'r, NL, EmptyBinds<NL::LayoutBinds>, Void, NoInstance> {
        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            layout_binds: LayoutBinds::<NL>::empty_binds(),
            __shader: PhantomData,
            instance: NoInstance,
        }
    }

    // #[inline]
    // pub fn set_shared_data<NL: Layout>(
    //     mut self,
    //     shared_data: &SharedData<NL>,
    // ) -> RenderPass<'r, NL, Void, false, NoInstance> {
    //     NL::set_shared_data(unsafe { self.inner() }, shared_data);

    //     RenderPass {
    //         render_pass: self.render_pass,
    //         __layout: PhantomData,
    //         __shader_attached: PhantomData,
    //         instance: NoInstance,
    //     }
    // }

    // #[inline]
    // pub fn set_shared_data_i<NL: InstancedLayout>(
    //     mut self,
    //     shared_data: &SharedDataI<NL>,
    // ) -> RenderPass<'r, NL, Void, false, Void> {
    //     NL::set_shared_data_instanced(unsafe { self.inner() }, shared_data);

    //     RenderPass {
    //         render_pass: self.render_pass,
    //         __layout: PhantomData,
    //         __shader_attached: PhantomData,
    //         instance: Void,
    //     }
    // }

    // #[inline(always)]
    // pub fn create_shared_data<NL: Layout>(self) -> RenderPass<'r, NL, Void, false, NoInstance>
    // where
    //     SharedData<NL>: Default,
    // {
    //     self.set_shared_data(&default())
    // }

    // pub fn create_shared_data_i<NL: InstancedLayout>(self) -> RenderPass<'r, NL, Void, false, Void>
    // where
    //     SharedDataI<NL>: Default,
    // {
    //     self.set_shared_data_i(&default())
    // }
}

feedable!(RequiresRPBind => FeedableRPBind
    in 'a for 'r where 'r: 'a
);

impl<'r, L, B: LayoutFeedableBind<'r, F>, S, I: Copy, F>
    FeedableRPBind<'r, RenderPass<'r, L, F, S, I>> for RenderPass<'r, L, B, S, I>
{
    type Food<'a>
        = B::Food<'a>
    where
        'r: 'a;

    fn feed<'a>(mut self, req: Self::Food<'a>) -> RenderPass<'r, L, F, S, I>
    where
        'r: 'a,
    {
        let lb = self
            .layout_binds
            .feed(&mut self.render_pass, B::covariant(req));

        RenderPass {
            render_pass: self.render_pass,
            __layout: PhantomData,
            layout_binds: lb,
            __shader: PhantomData,
            instance: self.instance,
        }
    }
}

// impl<'r, L: Layout, S, const SA: bool, I: Copy> RenderPass<'r, L, S, SA, I> {
//     #[inline]
//     pub fn apply_shader<NS: Shader<Layout = L>>(
//         mut self,
//         shader: &NS,
//     ) -> RenderPass<'r, L, NS, false, I> {
//         unsafe { self.inner() }.set_pipeline(shader.get_pipeline());

//         unsafe { self.coerce() }
//     }

//     #[inline]
//     pub fn apply_shader_with<NS: Shader<Layout = L>>(
//         mut self,
//         shader: &NS,
//         settings: &NS::Settings,
//     ) -> RenderPass<'r, L, NS, true, I> {
//         unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
//         NS::apply_settings(unsafe { self.inner() }, settings);

//         unsafe { self.coerce() }
//     }

//     #[inline]
//     pub fn apply_shader_with_default<NS: Shader<Layout = L>>(
//         mut self,
//         shader: &NS,
//     ) -> RenderPass<'r, L, NS, true, I>
//     where
//         NS::Settings: Default,
//     {
//         unsafe { self.inner() }.set_pipeline(shader.get_pipeline());
//         NS::apply_settings(unsafe { self.inner() }, &default());

//         unsafe { self.coerce() }
//     }

//     // pub fn set_instance_requirements(
//     //     mut self,
//     //     requirements: &InstanceData<L>,
//     // ) -> RenderPass<'r, L, S, SA, Instanced>
//     // where
//     //     L: InstancedLayout,
//     // {
//     //     let size = L::set_instances(unsafe { self.inner() }, requirements);

//     //     RenderPass {
//     //         render_pass: self.render_pass,
//     //         __layout: PhantomData,
//     //         __shader_attached: PhantomData,
//     //         instance: Instanced { size },
//     //     }
//     // }

//     pub fn draw_screen_quad(&mut self)
//     where
//         L::VertexLayout: VertexRequirements<Requirements = ()>,
//     {
//         unsafe { self.inner() }.draw(0..3, 0..1);
//     }
// }

// impl<'r, L: Layout, S: Shader, const SA: bool, I: Copy> RenderPass<'r, L, S, SA, I> {
//     #[inline]
//     pub fn apply_settings(mut self, settings: &S::Settings) -> RenderPass<'r, L, S, true, I> {
//         S::apply_settings(unsafe { self.inner() }, settings);

//         unsafe { self.coerce() }
//     }

//     pub fn default_settings(mut self) -> RenderPass<'r, L, S, true, I>
//     where
//         S::Settings: Default,
//     {
//         S::apply_settings(unsafe { self.inner() }, &default());

//         unsafe { self.coerce() }
//     }
// }

// impl<'r, L: Layout, S: Shader> RenderPass<'r, L, S, true, NoInstance> {
//     #[inline]
//     pub fn draw_mesh<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) {
//         unsafe { mesh.draw(self.inner()) };
//     }
// }

// impl<'r, L: Layout, S: Shader> RenderPass<'r, L, S, true, Instanced> {
//     #[inline]
//     pub fn draw_mesh_instanced<M: Mesh<VRequirements<L::VertexLayout>>>(&mut self, mesh: &M) {
//         unsafe { mesh.draw_instanced(&mut self.render_pass, 0..self.instance.size) };
//     }
// }
