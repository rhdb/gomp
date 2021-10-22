pub mod body;
pub mod script;

/// A component that belongs to an entity.
/// The generic is the typre of entity that
/// this component (in paticular) belongs to.
pub trait Component<T> {
    fn new() -> T;
    fn get_parent(self) -> T;
    fn on_update(&mut self);
    fn on_start(&mut self);
    fn on_stop(&mut self);
}

