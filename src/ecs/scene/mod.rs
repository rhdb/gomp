pub mod friendly;

use crate::ecs::{
    entity::Entity,
    component::Component,
};

use std::rc::Rc;

/// A container that all the entities are stored in.
/// May be swapped in and out.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    name: String,

    /// A list of all the components we have.
    #[derivative(Debug="ignore")]
    components: Vec<Rc<Box<dyn Component>>>,

    /// All the entities we have. Should *never* be used to search though, use components for that
    /// instead.
    #[derivative(Debug="ignore")]
    entities: Vec<Rc<Entity>>,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            components: vec![],
            entities: vec![],
        }
    }

    /// Gets all the components in the scene. Wondering why we don't just get all the entities?
    /// Check out a general overview on effient ECSs. In short, it's much more effect to be able to
    /// iterate over all our components, pick out the ones we need, and perform an action on them.
    /// Plus, all the memory is in one spot, so it's much faster.
    pub fn get_components(&self) -> &Vec<Rc<Box<dyn Component>>> {
        &self.components
    }

    /// Sets the components of a scene. Doesn't append or anything, just overwrites.
    pub fn set_components(&mut self, components: Vec<Rc<Box<dyn Component>>>) {
        self.components = components;
    }

    /// Get all the entities that belong to the scene. *Never* use for engine internal searching or
    /// computational work.
    pub fn get_entities(&self) -> &Vec<Rc<Entity>> {
        &self.entities
    }

    /// Sets the entities that belong to the scene. Overwrites, doesn't append. Do that on your own
    /// time ;)
    pub fn set_entities(&mut self, entities: Vec<Rc<Entity>>) {
        self.entities = entities;
    }

    /// Gets the name of the scene.
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

