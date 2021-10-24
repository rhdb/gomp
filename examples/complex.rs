use gomp::application::Application;
use gomp::application::config::AppConfig;

fn main() {
    let config = AppConfig::new();
    let app = Application::new();

    app.application_loop();
}
