pub mod friendly;

use std::rc::Weak;

use crate::ecs::{
    scene::Scene,
    component::Component,
};

/// A component that belongs to an entity.
/// The generic is the typre of entity that
/// this component (in paticular) belongs to.
pub struct Entity {
    scene: Weak<Scene>,

    /// The name of the entity.
    name: String,

    /// The IDs of the components in the scene that "belong" to the entity.
    components: Vec<Weak<Box<dyn Component>>>,
}

impl Entity {
    /// Create a new entity from a scene with a name.
    pub fn new(scene: Weak<Scene>, name: &str) -> Self {
        Self {
            scene,
            name: name.to_owned(),
            components: vec![],
        }
    }

    /// Gets the scene that the entity belongs to.
    pub fn get_scene(&self) -> &Weak<Scene> {
        &self.scene
    }

    /// Gets the name of the entity.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets weak references to every component that is "attached" to this entity.
    pub fn get_weak_components(&self) -> &Vec<Weak<Box<dyn Component>>> {
        &self.components
    }

    /// Pushes a component onto the entity. The weak reference should be taken from the reference
    /// counter that is contained within the parent scene.
    pub fn push_component(&mut self, component: Weak<Box<dyn Component>>) {
        self.components.push(component);
    }
}
