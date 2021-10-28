use gomp::application::Application;
use gomp::application::config::AppConfig;

use log::error;
use fern::colors::{Color, ColoredLevelConfig};

fn main() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {} {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("wgpu_core", log::LevelFilter::Info)
        .level_for("wgpu_core::device", log::LevelFilter::Warn)
        .level_for("wgpu_hal", log::LevelFilter::Info)
        .level_for("naga", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
    
    let config = AppConfig::new()
        .with_title("Complex Demo")
        .with_height(300)
        .with_width(400);

    let app = match Application::new(config) {
        Ok(a) => a,
        Err(e) => {
            error!("Failed to create application: {}", e);

            return;
        }
    };

    app.application_loop();
}
