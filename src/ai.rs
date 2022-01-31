use crate::{animations::AnimController, resources::Resources, role};
use sfml::{
    SfBox,
    graphics::{RenderTexture, Text, Color, Font, Sprite, RenderTarget, RenderWindow, Transformable},
    system::Vector2f,
};

const MOVE_SPEED: f32 = 2.0;
const FONT_SIZE: u32 = 8;

#[derive(PartialEq, Eq)]
pub enum UserState {
    Active,
    Leaving,
    Removable,
}

pub struct AI<'a> {
    pub name: String,
    pub role: role::RoleType,
    pub state: UserState,
    pub position: Vector2f, // TODO: Remove this and just use sprite's position (Must use a Sprite, return Texture from anim controller)
    nameplate: RenderTexture,
    anim_controller: AnimController<'a>,
    destination: Vector2f,
    flipped: bool,
}

// Get distance between two points
fn dist(pos_a: &Vector2f, pos_b: &Vector2f) -> f32 {
    ((pos_a.x - pos_b.x) + (pos_a.y - pos_b.y)).sqrt()
}

impl<'a> AI<'a> {
    pub fn new(
        resources: &'a Resources,
        name: &str,
        role: role::RoleType,
        font: &SfBox<Font>,
        position: Vector2f,
        destination: Vector2f,
    ) -> Self {
        // TODO: Find a better way to get text centered
        let mut nameplate = RenderTexture::new((name.len() * (FONT_SIZE as f32 * 0.75) as usize) as u32, (FONT_SIZE as f32 * 1.75) as u32, false).unwrap();
        nameplate.clear(Color::BLACK);
        let mut txt = Text::new(&name.to_string(), font, FONT_SIZE);
        txt.set_fill_color(role::get_colour(role));
        txt.set_position(Vector2f::new(FONT_SIZE as f32 / 2., FONT_SIZE as f32 * 0.25));
        nameplate.draw(&txt);


        Self {
            name: String::from(name),
            role,
            position,
            anim_controller: AnimController::new(
                resources,
                "run".to_string(),
                &["idle".to_string(), "run".to_string()],
            ),
            nameplate,
            destination,
            state: UserState::Active,
            flipped: false, // true = Left | false = Right
        }
    }

    pub fn say(&mut self, message: String) {
        println!("{}: {message}", self.name);
    }

    pub fn move_to(&mut self, destination: Vector2f) {
        self.destination = destination;
        self.anim_controller.set_action("run".to_string());
        self.flipped = if destination.x < self.position.x {
            true
        } else {
            false
        };
    }

    pub fn move_to_leave(&mut self, destination: Vector2f) {
        self.move_to(destination);
        self.state = UserState::Leaving;
    }

    pub fn update(&mut self) {
        self.anim_controller.update();

        // Check if we have a destination set
        if self.position != self.destination {
            self.position.x += if self.position.x < self.destination.x {
                MOVE_SPEED
            } else {
                -MOVE_SPEED
            };

            if dist(&self.position, &self.destination) <= MOVE_SPEED {
                self.anim_controller.set_action("idle".to_string());
                self.position.x = self.destination.x;

                // Set can_remove to leaving
                if self.state == UserState::Leaving {
                    self.state = UserState::Removable;
                }
            }
        }
    }

    pub fn render(&self, target: &mut RenderWindow) {
        // TODO: Make this less trash
        let mut sprite = self.anim_controller.get_frame();
        let size = sprite.texture().unwrap().size();
        sprite.set_origin(Vector2f::new((size.x / 2) as f32, 0.));
        sprite.scale(Vector2f::new(if self.flipped { -1. } else { 1. }, 1.));
        sprite.set_position(self.position);
        
        let mut nameplate = Sprite::with_texture(self.nameplate.texture());
        nameplate.scale(Vector2f::new(1., -1.)); // This inverts the Y because it renders unpside down

        let name_size = self.nameplate.texture().size();
        nameplate.set_origin(Vector2f::new((name_size.x / 2) as f32, 0.));
        nameplate.set_position(self.position);
        
        target.draw(&sprite);
        target.draw(&nameplate);
    }
}
