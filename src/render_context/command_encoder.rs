use crate::prelude::*;

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
            __shader_attached: PhantomData,
            instance: Void,
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
