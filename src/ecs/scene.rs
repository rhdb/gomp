use super::component::Component;

/// A container that all the entities are stored in.
/// May be swapped in and out.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    /// A list of all the components we have.
    #[derivative(Debug="ignore")]
    pub components: Vec<Box<dyn Component>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            components: vec![],
        }
    }
}

