use super::component::Component;
use super::super::renderer::Renderer;

/// A container that all the entities are stored in.
/// May be swapped in and out.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene<'a> {
    /// A list of all the components we have.
    #[derivative(Debug="ignore")]
    components: Vec<Box<dyn Component>>,

    /// The renderer that renders the scene.
    renderer: &'a Renderer,
}

impl<'a> Scene<'a> {
    pub fn new(renderer: &'a Renderer) -> Self {
        Self {
            components: vec![],
            renderer,
        }
    }

    /// Gets all the components in the scene. Wondering why we don't just get all the entities?
    /// Check out a general overview on effient ECSs. In short, it's much more effect to be able to
    /// iterate over all our components, pick out the ones we need, and perform an action on them.
    /// Plus, all the memory is in one spot, so it's much faster.
    pub fn get_components(&self) -> &Vec<Box<dyn Component>> {
        &self.components
    }

    /// Gets the renderer that renders the scene.
    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }
}

