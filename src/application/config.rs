/// Configuration struct that is used to build
/// the application and renderer.
#[derive(Debug)]
pub struct AppConfig {
    /// The title of the window.
    pub title: String,

    /// Is the window resizable.
    pub resizable: bool,

    /// The width of the window.
    pub width: u32,

    /// The height of the window.
    pub height: u32,

    /// Should the window always be on top
    pub top: bool,

    /// Should the window have décor. I'm using
    /// é (e-acute) here to make the API really
    /// hard to use.
    pub décor: bool,

    /// Should the window be visibile
    pub visible: bool,
}

impl AppConfig {
    /// Create a new AppConfig, which can then be built
    /// opon.
    pub fn new() -> Self {
        Self {
            title: "Gomp Application".to_owned(),
            resizable: true,
            width: 400,
            height: 300,
            top: false,
            décor: true,
            visible: true,
        }
    }

    /// Sets the title.
    pub fn with_title(self, title: &str) -> Self { Self { title: title.to_owned(), .. self } }

    /// Sets if resizable.
    pub fn with_resizable(self, resizable: bool) -> Self { Self { resizable, .. self } }

    /// Sets the width.
    pub fn with_width(self, width: u32) -> Self { Self { width, .. self } }

    /// Sets the height.
    pub fn with_height(self, height: u32) -> Self { Self { height, .. self } }

    /// Sets if the window should always be on top
    pub fn with_on_top(self, top: bool) -> Self { Self { top, .. self } }

    /// Sets if the window should have décor
    pub fn with_decor(self, décor: bool) -> Self { Self { décor, .. self } }

    /// Sets if the window should be visible
    pub fn with_visibiliy(self, visible: bool) -> Self { Self { visible, .. self } }
}
