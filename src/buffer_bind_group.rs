use wgpu::util::DeviceExt;

pub struct BufferBindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

pub struct BufferBuilder<'d, Data> {
    device: &'d wgpu::Device,
    data: Data,
}

impl<'d> BufferBuilder<'d, ()> {
    pub fn new(device: &'d wgpu::Device) -> Self {
        Self { device, data: () }
    }

    pub fn buffer(self, desc: &wgpu::BufferDescriptor) -> BufferBuilder<'d, wgpu::Buffer> {
        let buffer = self.device.create_buffer(desc);

        BufferBuilder {
            device: self.device,
            data: buffer,
        }
    }

    pub fn buffer_init(
        self,
        desc: &wgpu::util::BufferInitDescriptor,
    ) -> BufferBuilder<'d, wgpu::Buffer> {
        let buffer = self.device.create_buffer_init(desc);

        BufferBuilder {
            device: self.device,
            data: buffer,
        }
    }
}

impl BufferBuilder<'_, wgpu::Buffer> {
    pub fn bind_group(self, desc: &wgpu::BindGroupDescriptor) -> BufferBindGroup {
        let bind_group = self.device.create_bind_group(desc);

        BufferBindGroup {
            buffer: self.data,
            bind_group,
        }
    }
}
