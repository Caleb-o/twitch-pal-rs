use sfml::{ SfBox, graphics::Texture};
use std::collections::HashMap;

pub struct Resources {
	pub textures: HashMap<String, SfBox<Texture>>,
}

impl Resources {
	pub fn new() -> Self {
		Self { textures: HashMap::new() }
	}
}