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
        }
    }

    /// Sets the title.
    pub fn set_title(&mut self, title: String) -> Self { self.title = title; *self }

    /// Sets if resizable.
    pub fn set_resizable(&mut self, resizable: bool) -> Self { self.resizable = resizable; *self }

    /// Sets the width.
    pub fn set_width(&mut self, width: u32) -> Self { self.width = width; *self }

    /// Sets the height.
    pub fn set_height(&mut self, height: u32) -> Self { self.height = height; *self }
}
