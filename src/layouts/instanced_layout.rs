use crate::prelude::*;

pub type SharedDataI<'a, L> = <L as InstancedLayout>::SharedDataI<'a>;
pub type InstanceData<'a, L> = <L as InstancedLayout>::InstanceData<'a>;

pub trait InstancedLayout {
    type VertexLayoutI: VertexBufferLayout + InstanceRequirements;
    type SharedDataI<'a> = Void;
    type InstanceData<'a> = Void;

    fn raw_layout_instanced(&self) -> &RawLayout<Self::VertexLayoutI>;

    #[allow(unused)]
    fn set_shared_data_instanced(
        render_pass: &mut wgpu::RenderPass,
        shared_data: SharedDataI<Self>,
    ) {
    }

    fn set_instances(render_pass: &mut wgpu::RenderPass, requirements: InstanceData<Self>) -> u32;
}

impl<L: InstancedLayout> Layout for L
where
    IRequirements<L::VertexLayoutI>: 'static,
{
    type VertexLayout = L::VertexLayoutI;
    type SharedData<'a> = (SharedDataI<'a, L>, InstanceData<'a, L>);

    fn raw_layout(&self) -> &RawLayout<Self::VertexLayout> {
        self.raw_layout_instanced()
    }

    fn set_shared_data(
        render_pass: &mut wgpu::RenderPass,
        (shared_data, requirements): SharedData<Self>,
    ) {
        L::set_shared_data_instanced(render_pass, shared_data);
        L::set_instances(render_pass, requirements);
    }
}
