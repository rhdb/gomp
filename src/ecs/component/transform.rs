use super::Component;
use super::Entity;

pub struct Transform {
    parent: Box<dyn Entity>,
}

impl Component for Transform {
    fn from(parent: Box<dyn Entity>) -> Self {
        Transform { parent }
    }

    fn get_parent(&self) -> Box<&Box<dyn Entity>> {
        Box::from(&self.parent)
    }

    fn on_update(&mut self) {

    }

    fn on_start(&mut self) {

    }

    fn on_stop(&mut self) {

    }
}

