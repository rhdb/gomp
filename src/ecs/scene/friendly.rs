use crate::ecs::{
    scene::Scene,
    entity::friendly::FriendlyEntity,
    component::Component,
};

use std::rc::Rc;

/// Never once is this used for actual rendering and updating inside the inner workings of the game
/// engine. The only purpose that this struct has is to act as a transport medium between the
/// optimised Scene, and the file/builder pattern that specifies (in more human readable terms) the
/// layout of a scene.
pub struct FriendlyScene {
    name: String,
    entities: Vec<FriendlyEntity>,
}

impl FriendlyScene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            entities: vec![],
        }
    }
}

impl From<FriendlyScene> for Scene {
    /// Morphs a FriendlyScene into a Scene.
    fn from(w: FriendlyScene) -> Scene {
        let mut scene = Scene::new(&w.name);
        let mut components: Vec<Rc<Box<dyn Component>>> = vec![];

        // Slow, but what can you do?
        for entity in w.entities {
            for friendly in entity.get_components() {
                components.push(Rc::new(**friendly));
            }
        }

        scene.set_components(components);

        todo!()
    }
}

