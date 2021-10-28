#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}

/// Non-static implementations
impl Vertex {
    /// Construct a new vertex.
    pub fn new(position: [f32; 3], colour: [f32; 3]) -> Self {
        Self {
            position,
            colour,
        }
    }
}

/// Static implementations
impl Vertex {
    /// Descibes how a buffer is layed out in memory.
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        // We could technically use the macro (vertex_attr_array) to clean this code up a bit, but
        // then our lifetimes would have to be static. Plus, this makes things clearer and easier
        // to debug in the future.

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

/// Implementation to enable bytemuck.
unsafe impl bytemuck::Zeroable for Vertex {}

/// Implementation to enable bytemuck.
unsafe impl bytemuck::Pod for Vertex {}

