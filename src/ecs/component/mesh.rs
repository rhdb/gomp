use super::{Component, ComponentType};
use super::Entity;

use std::rc::{Rc, Weak};

pub struct Mesh {
    parent: Weak<Entity>,
}

impl Component for Mesh {
    /// Creates a new mesh.
    fn from(parent: Weak<Entity>) -> Self {
        Mesh {
            parent,
        }
    }

    fn type_of(&self) -> ComponentType {
        ComponentType::Mesh
    }

    fn get_weak_parent(&self) -> &Weak<Entity> {
        &self.parent
    }

    fn get_parent(&self) -> Rc<Entity> {
        self.parent.upgrade().expect("dangling weak pointer to parent")
    }

    fn on_update(&mut self) {

    }

    fn on_start(&mut self) {

    }

    fn on_stop(&mut self) {

    }
}

