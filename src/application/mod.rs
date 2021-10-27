pub mod config;

use futures::executor;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    dpi::LogicalSize,
};

use log::{trace, debug, info, error};

use super::renderer::Renderer;
use super::state::State;
use config::AppConfig;

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,

    renderer: Renderer,
    state: State,
}

impl Application {
    /// Creates a new application with a config.
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = executor::block_on(Renderer::new(&window))?;

        window.set_resizable(config.resizable);
        window.set_title(&config.title);
        window.set_min_inner_size(Some(LogicalSize::new(config.width, config.height)));
        window.set_always_on_top(config.top);
        window.set_decorations(config.décor);

        Ok(Self {
            event_loop,
            window,
            renderer,
            state: State {}
        })
    }

    /// Returns the underlying window type, in case we don't expose
    /// something that you need.
    pub fn expose(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Handle our inputs.
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    /// Run scripts, call the physics, etc...
    pub fn update(&mut self) {
    
    }

    /// The application loops. Calls the renderer, runs the scripts,
    /// does the physics, all in one neat, nice, contained function.
    pub fn application_loop(self) {
        info!("Starting application loop");

        let event_loop = self.event_loop;
        let window = self.window;
        let mut renderer = self.renderer;
        let mut state = self.state;

        debug!("Moving into window event loop");
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == window.id() && !state.input(event) => match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    trace!("Window resized");

                    renderer.resize(*physical_size);
                },

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    trace!("Window scale factor changed");

                    renderer.resize(**new_inner_size);
                },

                _ => {},
            },
            
            Event::RedrawRequested(_) => {
                state.update();
                match renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => error!("Failed to render frame: {:?}", e),
                }
            },

            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            },

            _ => {},
        });
    }
}

