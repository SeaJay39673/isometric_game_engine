use crate::graphics::TextureHandle;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    pub model: [[f32; 4]; 4],
    pub base_frame: TextureHandle,
    pub frame_count: u32,
    pub frame_time_ms: u32,
    pub color: [u8; 4],
    pub _padding: [u32; 3],
}
