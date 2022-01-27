use sfml::{ SfBox, graphics::Texture};
use std::collections::HashMap;
use crate::animations::Animation;


pub struct Resources {
	pub textures: HashMap<String, SfBox<Texture>>,
	pub animations: HashMap<String, Animation>,
}

impl Resources {
	pub fn new() -> Resources {
		Resources { textures: HashMap::new(), animations: HashMap::new() }
	}
}