use winit::event::WindowEvent;

#[derive(Debug)]
pub struct State {

}

impl State {
    /// Creates a new state
    pub fn new() -> Self {
        Self {
        
        }
    }

    /// Handle our inputs.
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    /// Run scripts, call the physics, etc...
    pub fn update(&mut self) {
    
    }
}

