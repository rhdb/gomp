use super::component::Component;
use super::entity::Entity;

/// A container that all the entities are stored in.
/// May be swapped in and out.
pub struct Scene {
    /// A list of all the components we have.
    pub components: Vec<Box<dyn Component<Box<dyn Entity>>>>,
}

