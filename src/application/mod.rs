pub mod config;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    dpi::LogicalSize,
};

use super::renderer::Renderer;
use config::AppConfig;

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
    window: Window,

    renderer: Renderer,
}

impl Application {
    pub fn new(config: AppConfig) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = Renderer::new();

        window.set_resizable(config.resizable);
        window.set_title(&config.title);
        window.set_min_inner_size(Some(LogicalSize::new(config.width, config.height)));

        Self {
            event_loop,
            window,
            renderer,
        }
    }

    pub fn application_loop(self) {
        let event_loop = self.event_loop;
        let window = self.window;

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }
}

