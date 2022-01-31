mod ai;
mod animations;
mod config;
mod monitor;
mod resources;
mod userhandling;

use crate::{
    animations::Animation, config::Config, resources::Resources, userhandling::UserHandler,
};

use monitor::Monitor;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    system::Vector2,
    window::{Event, Key, Style},
};

const WINDOW_SCALE: u32 = 2;

fn main() {
    let cfg = Config::new();

    // Create a new window
    let window_size = cfg.display_size;
    let display_size = (window_size.0 / WINDOW_SCALE, window_size.1 / WINDOW_SCALE);

    let view = View::new(
        Vector2::new(
            display_size.0 as f32 / WINDOW_SCALE as f32,
            display_size.1 as f32 / WINDOW_SCALE as f32,
        ),
        Vector2::new(display_size.0 as f32, display_size.1 as f32),
    );

    let mut window = RenderWindow::new(window_size, "Twitch Pals", Style::CLOSE, &Default::default());

    window.set_view(&view);

    // Limit the framerate to 30 frames per second (this step is optional)
    window.set_framerate_limit(30);

    let mut resources = Resources::new();

    let _ = Animation::new(
        &mut resources,
        "res/player_animations/idle".to_string(),
        &[7, 7, 40],
    );
    let _ = Animation::new(
        &mut resources,
        "res/player_animations/run".to_string(),
        &[7, 7],
    );

    let mut user_handler = UserHandler::new(&cfg, display_size, &resources);
    user_handler.create_users(vec![("modprog".to_string(), "vips".to_string())]);

    let mut monitor = Monitor::new(&mut user_handler);
    monitor.start(cfg.settings["CHANNEL"].as_str().unwrap().to_string());

    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            match event {
                // TODO: Add quit for viewer_monitor
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => {
                    window.close();
                }
                _ => {}
            }
        }

        // Activate the window for OpenGL rendering
        window.set_active(true);
        window.clear(Color::BLACK);

        // OpenGL drawing commands go here...
        monitor.run();
        monitor.render(&mut window);

        // End the current frame and display its contents on screen
        window.display();
    }

    // Handle monitor after we close the window
    monitor.close();
}
