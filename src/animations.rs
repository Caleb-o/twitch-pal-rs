use sfml::{SfBox, graphics::{Texture, Sprite}};
use std::collections::HashMap;
use crate::resources::Resources;

pub struct Animation {
	pub frames: Vec<SfBox<Texture>>,
	pub metadata: Vec<(u32, u32)>,
}

pub struct AnimController<'a> {
	pub current_frame: u32,
	pub frame_counter: u32,
	pub current_anim: String,
	pub animations: HashMap<String, &'a Animation>,
}

impl Animation {
	pub fn new<'a>(resources: &'a mut Resources, anim_path: String, frame_times: &[u32]) -> &'a Animation {
		// We create an animation to return
		let anim_name = anim_path.split('/').last().unwrap();
		
		// If animation already exists, then we can return it
		resources.animations.entry(anim_name.to_string()).or_insert_with(|| {
				let mut frames: Vec<SfBox<Texture>> = Vec::new();
				let mut metadata: Vec<(u32, u32)> = Vec::new();
				
				// FIXME: We assume that all frames exist in the directory, based on frame_times. We need to check whether the file even exists
				for idx in 0..frame_times.len() {
					let frame_id = format!("{anim_name}_{idx}");
					let frame_name = format!("{frame_id}.png");
					let frame_path = format!("{anim_path}/{frame_name}");

					// Load the animation
					let texture = Texture::from_file(&frame_path).unwrap();

					// Insert into frames and set frame time
					frames.push(texture);
					metadata.push((idx as u32, frame_times[idx]));
				}

				// Create the animation and return the reference
				Animation { frames, metadata }
			}
		)
	}
}

impl<'a> AnimController<'a> {
	pub fn new(resources: &'a Resources, default_state: String, anims_to_use: &[String]) -> AnimController<'a> {
		let animations: HashMap<String, &'a Animation> = anims_to_use.into_iter().map(|name| (name.clone(), resources.animations.get(name).unwrap())).collect();
		AnimController { current_frame: 0, frame_counter: 0, current_anim: default_state, animations }
	}

	pub fn set_action(&mut self, action_name: String) {
		if self.current_anim != action_name {
			self.current_anim = action_name;
			self.current_frame = 0;
			self.frame_counter = 0;
		}
	}

	pub fn update(&mut self) {
		self.frame_counter += 1;

		if self.frame_counter >= self.animations[&self.current_anim].metadata[self.current_frame as usize].1 {
			self.current_frame += 1;
			self.frame_counter = 0;
		}

		if self.current_frame as usize >= self.animations[&self.current_anim].metadata.len() {
			self.current_frame = 0;
		}
	}

	pub fn get_frame(&self) -> Sprite {
		Sprite::with_texture(&self.animations[&self.current_anim].frames[self.current_frame as usize])
	}
}