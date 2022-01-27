use sfml::{SfBox, graphics::{Texture, Sprite}};
use std::collections::HashMap;
use crate::resources::Resources;

pub struct Animation {
	pub frames: Vec<SfBox<Texture>>,
	pub metadata: Vec<(u32, u32)>,
}

pub struct AnimController<'a> {
	pub currentFrame: u32,
	pub frameCounter: u32,
	pub currentAnim: String,
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
	pub fn new(resources: &'a Resources, anims_to_use: &[String]) -> AnimController<'a> {
		let animations: HashMap<String, &'a Animation> = anims_to_use.into_iter().map(|name| (name.clone(), resources.animations.get(name).unwrap())).collect();
		AnimController { currentFrame: 0, frameCounter: 0, currentAnim: "None".to_string(), animations }
	}

	pub fn set_action(&mut self, action_name: String) {
		if self.currentAnim != action_name {
			self.currentAnim = action_name;
			self.currentFrame = 0;
			self.frameCounter = 0;
		}
	}

	pub fn update(&mut self) {
		self.frameCounter += 1;

		if self.frameCounter >= self.animations[&self.currentAnim].metadata[self.currentFrame as usize].1 {
			self.currentFrame += 1;
			self.frameCounter = 0;
		}

		if self.currentFrame as usize >= self.animations[&self.currentAnim].metadata.len() {
			self.currentFrame = 0;
		}
	}

	pub fn get_frame(&self) -> Sprite {
		Sprite::with_texture(&self.animations[&self.currentAnim].frames[self.currentFrame as usize])
	}
}