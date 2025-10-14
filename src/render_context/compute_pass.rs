use crate::prelude::*;

#[repr(transparent)]
#[derive(Debug)]
pub struct ComputePass<'r, Layout> {
    pub(crate) compute_pass: wgpu::ComputePass<'r>,
    pub(crate) __layout: PhantomData<Layout>,
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
