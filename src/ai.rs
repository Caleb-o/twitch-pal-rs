use sfml::{
	system::Vector2f,
	graphics::{RenderWindow, RenderTarget, Texture, Sprite, Transformable},
};

const MOVE_SPEED: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum UserState {
	Active, Leaving, Removable
}


pub struct AI<'a> {
	pub name: String,
	pub role: String,
	pub state: UserState,
	pub position: Vector2f, // TODO: Remove this and just use sprite's position
	destination: Vector2f,
	sprite: Sprite<'a>,
}

// Get distance between two points
fn dist(pos_a: &Vector2f, pos_b: &Vector2f) -> f32 {
	((pos_a.x - pos_b.x) + (pos_a.y - pos_b.y)).sqrt()
}

impl<'a> AI<'a> {
	pub fn new(name: &str, role: &str, position: Vector2f, destination: Vector2f, texture: &'a Texture) -> Self {
		let mut sprite = Sprite::with_texture(texture);
		sprite.set_position(position);

		Self { name: String::from(name), role: String::from(role), position, destination, sprite, state: UserState::Active }
	}

	pub fn say(&mut self, message: String) {
		assert!(false, "TODO: Unimplemented function");
	}

	pub fn move_to(&mut self, destination: Vector2f) {
		 self.destination = destination;
		 // TODO: Call to animation handler to change action and set flipped flag (if required)
	}

	pub fn move_to_leave(&mut self, destination: Vector2f) {
		self.move_to(destination);
		self.state = UserState::Leaving;
   }

	pub fn update(&mut self) {
		// Check if we have a destination set
		if self.position != self.destination {
			self.position.x += if self.position.x < self.destination.x { MOVE_SPEED } else { -MOVE_SPEED };
			self.sprite.set_position(self.position);

			if dist(&self.position, &self.destination) <= MOVE_SPEED {
				// TODO: Set anim to idle
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
		target.draw(&self.sprite);
	}
}