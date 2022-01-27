mod ai;
mod resources;
mod userhandling;
mod config;
mod animations;

use animations::Animation;
use config::Config;
use resources::Resources;
use userhandling::UserHandler;

use sfml::{
    system::Vector2,
    window::{Key, Event, Style},
    graphics::{View, Texture, RenderWindow, RenderTarget, Color, Sprite, Transformable},
};


const WINDOW_SCALE: u32 = 2;


fn main() {
    let cfg = Config::new();

    // Create a new window
    let window_size = cfg.display_size;
    let display_size = (window_size.0 / WINDOW_SCALE, window_size.1 / WINDOW_SCALE);

    let view = View::new(
        Vector2::new(display_size.0 as f32 / WINDOW_SCALE as f32, display_size.1 as f32 / WINDOW_SCALE as f32),
        Vector2::new(display_size.0 as f32, display_size.1 as f32)
    );

    let mut window = RenderWindow::new(window_size,
        "SFML Test",
        Style::CLOSE,
        &Default::default()
    );

    window.set_view(&view);

    // Limit the framerate to 30 frames per second (this step is optional)
    window.set_framerate_limit(30);

    let mut resources = Resources::new();
    resources.textures.insert("idle_0".to_string(), Texture::from_file("res/player_animations/idle/idle_0.png").unwrap());
    
    let test_anim: &Animation = Animation::new(&mut resources, "res/player_animations/idle".to_string(), &[7, 7, 40]);
    let texture = test_anim.frames[&"idle_0".to_string()].clone();

    let mut user_handler = UserHandler::new(&cfg, display_size, &resources);
    user_handler.create_users(vec![("modprog".to_string(), "vips".to_string())]);


    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            match event {
                // TODO: Add quit for viewer_monitor
                Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => window.close(),
                _ => {}
            }
        }

        // Activate the window for OpenGL rendering
        window.set_active(true);
        window.clear(Color::BLACK);

        // OpenGL drawing commands go here...
        user_handler.update();
        user_handler.render(&mut window);

        // End the current frame and display its contents on screen
        window.display();
    }
}