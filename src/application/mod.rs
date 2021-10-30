//! The overarching "game controller".
//!
//! # Finally!
//! 
//! Something the clients are actually encouraged to use. This module is the main driver for the
//! engine. It drives the renderer, handles physics, optimizes the ECS, etc... You can see more
//! complex exampes in the examples directory, but we have provided some here.
//!
//! # Examples!
//!
//! ```
//! // This is configuration struct that you pass into the application contructor. You can look in
//! // the modules documentation to get all the values that you can set, and what they do. If needed,
//! // you can also extract certain values from the application (in reference form), and apply
//! // mutations there.
//! // Note that this is not encouraged due to possible backend shift (we don't plan
//! // on this, but better safe than sorry).
//! let config = AppConfig::new()
//!     .with_title("Complex Demo")
//!     .with_height(300) // Physical size of 300px.
//!     .with_width(400); // Physical size of 400px.
//!
//! // Actually construct the application.
//! let app = match Application::new(config) {
//!     Ok(a) => a,
//!     Err(e) => {
//!     error!("Failed to create application: {}", e);
//!
//!     return;
//!     }
//! };
//!
//! // Start the application loop. This is not technically required, as you can write your own
//! // application loop, but in the future "loop hooks" may be added to extend the application loop
//! // without actually rewritting it yourself.
//! match app.application_loop() {
//!     Ok(()) => (),
//!     Err(e) => {
//!         error!("Something went mal in the application that cannot be recovered from.");
//!
//!         return;
//!     }
//! };
//! ```
//!
//! # Panics!
//!
//! Everything is designed to propagate errors back to the client, so nothing should panic. If
//! something panics, chances are it's a bug (e.g. a contributor/author forgetting to add a check,
//! and then unwraping a value).
//!
//! The only thing that could panic is shader compilation. You can see the shader module inside the
//! renderer module for more information on why this happens. Note that this panic *will not* cause
//! your application to crash.

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
use super::ecs::scene::Scene;
use config::AppConfig;

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,

    renderer: Renderer,
    state: State,
    active_scene_index: usize,
    // We will always have scenes in our application, so this is OK.
    scenes: Vec<Scene>,
}

impl Application {
    /// Creates a new application with a config.
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = executor::block_on(Renderer::new(&window))?;
        let scenes = vec![Scene::new("main")];

        window.set_resizable(config.resizable);
        window.set_title(&config.title);
        window.set_min_inner_size(Some(LogicalSize::new(config.width, config.height)));
        window.set_always_on_top(config.top);
        window.set_decorations(config.décor);

        Ok(Self {
            event_loop,
            window,
            renderer,
            state: State {},
            active_scene_index: 0,
            scenes,
        })
    }

    /// Returns the underlying window type, in case we don't expose
    /// something that you need.
    pub fn expose(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Handle our inputs.
    pub fn input(&mut self, _event: &WindowEvent) -> bool {
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
        let scenes = self.scenes;
        let active_scene_index = self.active_scene_index;

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
                match renderer.render(scenes.get(active_scene_index).expect("invalud active_scene_index during renderer invokation")) {
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

    /// Get a reference to the renderer.
    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }

    /// Gets a mutable reference to the active scene.
    pub fn get_mut_active_scene(&mut self) -> &mut Scene {
        self.scenes.get_mut(self.active_scene_index).expect("invalid active_scene_index during renderer invokation")
    }

    /// Get a reference to the active scene. Gomp does not provide a method to get all the scenes,
    /// as it is required Gomp manages them for runtime safety.
    pub fn get_active_scene(&self) -> &Scene {
        &self.scenes.get(self.active_scene_index).expect("invalid active_scene_index during renderer invokation")
    }
}

