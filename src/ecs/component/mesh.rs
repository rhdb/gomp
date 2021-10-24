use super::Component;
use super::Entity;

pub struct Mesh {
    parent: Box<dyn Entity>,
}

impl Component for Mesh {
    fn from(parent: Box<dyn Entity>) -> Self {
        Mesh { parent }
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

