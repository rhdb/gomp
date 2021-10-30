//! Main rendering functionality.
//!
//! Actually does the rendering work. These functions are well documented, so you should be able to
//! build an interface to this module with little effort. Plus, you can see the way the Application
//! calls it for in-source examples.

use super::Renderer;
use crate::ecs::{
    scene::Scene,
    component::{
        Component,
        ComponentType
    },
};

use wgpu::RenderPass;
use log::debug;

impl Renderer {
    /// Actually render to a frame.
    pub fn render(&mut self, scene: &Scene) -> Result<(), wgpu::SurfaceError> {
        // Get somewhere to render to
        let output = self.surface.get_current_texture()?;

        // Control how the renderer interacts with the texture
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // How we actually send commands to the GPU
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Default render encoder"),
        });

        {
            // Create a render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Default render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            /* let verticies: &[Vertex] = &[
                Vertex::new([0.0, 0.5, 0.0], [1.0, 0.0, 0.0]),
                Vertex::new([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0]),
                Vertex::new([0.5, -0.5, 0.0], [0.0, 0.0, 1.0]),
            ];

            let vertex_buffer = Box::leak(Box::new(self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(verticies),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            )));

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..(verticies.len() as u32), 0..1); */
        
            self.render_scene_meshes(&mut render_pass, scene);
        }

        // Submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn render_scene_meshes(&mut self, pass: &mut RenderPass, scene: &Scene) {
        // Get all components and find the meshes.
        for component in scene.get_components() {
            if let ComponentType::Mesh = component.type_of() {
                self.render_mesh(pass, component);
            }
        }
    }

    fn render_mesh(&mut self, pass: &mut RenderPass, mesh: &Box<dyn Component>) {
        // We know for sure that mesh is a Mesh here, so no need to worry.
        debug!("Rendering {}", mesh.get_parent().get_name());
    }
}

