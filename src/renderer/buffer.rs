use wgpu::Buffer;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;

use super::Renderer;

impl Renderer {
    /// Creates a buffer based on a BufferInitDescriptor. Just a wrapper towards the actual wgpu
    /// function because that requires the device, so if we move this functionality into the
    /// renderer, it
    ///
    /// 1. Makes the code structure cleaner, and makes maintenence a quasi-breeze
    /// 2. Don't have to deal with a super complicated system to pass the renderer/device around.
    pub fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> Buffer {
        self.device.create_buffer_init(desc)
    }
}

