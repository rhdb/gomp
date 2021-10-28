use super::Component;
use super::Entity;

use wgpu::Buffer;

pub struct Mesh {
    parent: Box<dyn Entity>,
    vertex_buffer: Buffer,
}

impl Component for Mesh {
    /// Creates a new mesh.
    fn from(parent: Box<dyn Entity>) -> Self {
        let renderer = parent.get_scene().get_renderer();

        // Calls the renderers create_buffer_init, not wgpu's.
        let vertex_buffer = renderer.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Default mesh"),
            contents: &[0],
            usage: wgpu::BufferUsages::VERTEX,
        });

        Mesh {
            parent,
            vertex_buffer
        }
    }

    fn get_parent(&self) -> Box<&Box<dyn Entity>> {
        Box::from(&self.parent)
    }

    fn on_update(&mut self) {

    }

    fn on_start(&mut self) {

    }

    fn on_stop(&mut self) {

    }
}

