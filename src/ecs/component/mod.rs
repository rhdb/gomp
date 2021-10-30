pub mod mesh;
pub mod transform;
pub mod script;

use super::entity::Entity;

use std::rc::{Rc, Weak};

/// All the different types of components
pub enum ComponentType {
    /// A mesh component.
    Mesh,

    /// A transform component.
    Transform,

    /// A script component.
    Script,
}

/// A component that belongs to an entity.
/// The generic is the typre of entity that
/// this component (in paticular) belongs to.
pub trait Component {
    /// Creates a component from a entity.
    fn from(entity: Weak<Entity>) -> Self where Self: Sized;

    /// Helps enforce a OOP-like visitor pattern so we can dissern between different types of
    /// components.
    fn type_of(&self) -> ComponentType;
    
    /// Get the parent entity as a weak reference. Will never panic, completely safe.
    fn get_weak_parent(&self) -> &Weak<Entity>; 

    /// Get the parent entity as a reference counter reference. Has a possibility to panic, but if it does,
    /// this is considered a bug.
    fn get_parent(&self) -> Rc<Entity>;

    /// Called once per frame.
    fn on_update(&mut self);

    /// Called on initialisation.
    fn on_start(&mut self);

    /// Called on destruction.
    fn on_stop(&mut self);
}

/// A "friendly" (more human interactable/readable/writtable) component trait.
pub trait FriendlyComponent {

}
