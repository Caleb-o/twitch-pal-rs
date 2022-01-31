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

    // NOTE: We will need a helper function to create colours from json array, for the colour palette
    let raw_col: Vec<u8> = cfg.settings["BG_COL"].as_array().unwrap().iter().map(|s| s.as_f64().unwrap() as u8).collect();
    let background_col = Color::rgb(raw_col[0], raw_col[1], raw_col[2]);


    let mut user_handler = UserHandler::new(&cfg, display_size, &resources);
    let mut monitor = Monitor::new(&cfg, &mut user_handler);
    monitor.start(cfg.settings["CHANNEL"].as_str().unwrap().to_string());

    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            match event {
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
        window.clear(background_col);

        // OpenGL drawing commands go here...
        monitor.run();
        monitor.render(&mut window);

        // End the current frame and display its contents on screen
        window.display();
    }

    // Handle monitor after we close the window
    monitor.close();
}
