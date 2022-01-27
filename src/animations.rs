use sfml::{SfBox, graphics::{Texture, Sprite}};
use std::collections::HashMap;
use crate::resources::Resources;

pub struct Animation {
	pub frames: HashMap<String, SfBox<Texture>>,
	pub metadata: Vec<(u32, u32)>,
}

impl Animation {
	pub fn new<'a>(resources: &'a mut Resources, anim_path: String, frame_times: &[u32]) -> &'a Animation {
		// We create an animation to return
		let anim_name = anim_path.split('/').last().unwrap();
		
		// If animation already exists, then we can return it
		resources.animations.entry(anim_name.to_string()).or_insert_with(|| {
				let mut frames: HashMap<String, SfBox<Texture>> = HashMap::new();
				let mut metadata: Vec<(u32, u32)> = Vec::new();
				
				// FIXME: We assume that all frames exist in the directory, based on frame_times. We need to check whether the file even exists
				for idx in 0..frame_times.len() {
					let frame_id = format!("{anim_name}_{idx}");
					let frame_name = format!("{frame_id}.png");
					let frame_path = format!("{anim_path}/{frame_name}");

					// Load the animation
					let texture = Texture::from_file(&frame_path).unwrap();

					// Insert into frames and set frame time
					frames.insert(frame_id, texture);
					metadata.push((idx as u32, frame_times[idx]));
				}

				// Create the animation and return the reference
				Animation { frames, metadata }
			}
		)
	}
}