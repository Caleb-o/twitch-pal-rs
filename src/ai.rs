use crate::{animations::AnimController, resources::Resources};
use sfml::{
    graphics::{RenderTarget, RenderWindow, Transformable},
    system::Vector2f,
};

const MOVE_SPEED: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum UserState {
    Active,
    Leaving,
    Removable,
}

pub struct AI<'a> {
    pub name: String,
    pub role: String,
    pub state: UserState,
    pub position: Vector2f, // TODO: Remove this and just use sprite's position
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
        role: &str,
        position: Vector2f,
        destination: Vector2f,
    ) -> Self {
        Self {
            name: String::from(name),
            role: String::from(role),
            position,
            anim_controller: AnimController::new(
                resources,
                "run".to_string(),
                &["idle".to_string(), "run".to_string()],
            ),
            destination,
            state: UserState::Active,
            flipped: false, // true = Left | false = Right
        }
    }

    pub fn say(&mut self, message: String) {
        panic!("TODO: Unimplemented function");
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
                // TODO: Monitor this because it may become awkward and reset when we don't want to
                if self.state == UserState::Leaving {
                    self.state = UserState::Removable;
                }
            }
        }
    }

    pub fn render(&self, target: &mut RenderWindow) {
        // TODO: flip sprite based on flipped flag in AI
        let mut sprite = self.anim_controller.get_frame();
        sprite.set_position(self.position);
        sprite.scale(Vector2f::new(if self.flipped { -1. } else { 1. }, 1.));
        target.draw(&sprite);
    }
}
