pub mod mesh;
pub mod transform;
pub mod script;

use super::entity::Entity;

/// A component that belongs to an entity.
/// The generic is the typre of entity that
/// this component (in paticular) belongs to.
pub trait Component {
    fn from(entity: Box<dyn Entity>) -> Self where Self: Sized;
    fn get_parent(&self) -> Box<&Box<dyn Entity>>;
    fn on_update(&mut self);
    fn on_start(&mut self);
    fn on_stop(&mut self);
}

