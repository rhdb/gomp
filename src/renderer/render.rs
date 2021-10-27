use super::Renderer;

impl Renderer {
    /// Actually render to a frame.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.draw(0..3, 0..1); // 3.
        }
    
        // Submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

