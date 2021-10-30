use super::{Component, ComponentType};
use super::Entity;

use std::rc::{Rc, Weak};

pub struct Transform {
    parent: Weak<Entity>,
    x: f64,
    y: f64,
    z: f64,
}

impl Component for Transform {
    fn from(parent: Weak<Entity>) -> Self {
        Transform {
            parent,
            x: 420.0,
            y: 69.0,
            z: 516.0,
        }
    }

    fn type_of(&self) -> ComponentType {
        ComponentType::Transform
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

