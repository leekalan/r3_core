use crate::prelude::*;

pub type SharedDataI<L> = <L as InstancedLayout>::SharedDataI;
pub type InstanceData<L> = <L as InstancedLayout>::InstanceData;

pub trait InstancedLayout {
    type VertexLayoutI: VertexBufferLayout + InstanceRequirements;
    type SharedDataI = Void;
    type InstanceData = Void;

    fn raw_layout_instanced(&self) -> &RawLayout<Self::VertexLayoutI>;

    #[allow(unused)]
    fn set_shared_data_instanced(
        render_pass: &mut wgpu::RenderPass,
        shared_data: &SharedDataI<Self>,
    ) {
    }

    fn set_instances(render_pass: &mut wgpu::RenderPass, requirements: &InstanceData<Self>) -> u32;
}

impl<L: InstancedLayout> Layout for L
where
    IRequirements<L::VertexLayoutI>: 'static,
{
    type VertexLayout = L::VertexLayoutI;
    type SharedData = (SharedDataI<L>, InstanceData<L>);

    fn raw_layout(&self) -> &RawLayout<Self::VertexLayout> {
        self.raw_layout_instanced()
    }

    fn set_shared_data(
        render_pass: &mut wgpu::RenderPass,
        (shared_data, requirements): &SharedData<Self>,
    ) {
        L::set_shared_data_instanced(render_pass, shared_data);
        L::set_instances(render_pass, requirements);
    }
}
