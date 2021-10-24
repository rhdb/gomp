/// A component that belongs to an entity.
/// The generic is the typre of entity that
/// this component (in paticular) belongs to.
pub trait Entity {
    fn new() -> Self where Self: Sized;
    fn on_update(&mut self);
    fn on_start(&mut self);
    fn on_stop(&mut self);
}
