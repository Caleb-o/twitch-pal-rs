use crate::resources::{Resources, AnimationName};
use sfml::{
    graphics::{Sprite, Texture},
    SfBox,
};
use std::rc::Rc;

pub struct Animation {
    pub frames: Vec<SfBox<Texture>>,
    pub metadata: Vec<(u32, u32)>,
}

pub struct AnimController {
    pub current_frame: u32,
    pub frame_counter: u32,
    pub current_anim_name: AnimationName,
    pub current_anim: Rc<Animation>,
    // pub animations: 
}

impl Animation {
    pub fn new(
        anim_path: &str,
        frame_times: &[u32],
    ) -> Rc<Animation> {
        // We create an animation to return
        let anim_file_name = anim_path.split('/').last().unwrap();

        let mut frames: Vec<SfBox<Texture>> = Vec::new();
        let mut metadata: Vec<(u32, u32)> = Vec::new();

        for idx in 0..frame_times.len() {
            let frame_path = format!("{anim_path}/{anim_file_name}_{idx}.png");

            // Load the animation
            let texture = Texture::from_file(&frame_path).unwrap();

            // Insert into frames and set frame time
            frames.push(texture);
            metadata.push((idx as u32, frame_times[idx]));
        }

        // Create the animation and return the reference
        Rc::new(Animation { frames, metadata })
    }
}

impl AnimController {
    pub fn new(
        resources: &Resources,
        default_state: AnimationName,
    ) -> AnimController {
        AnimController {
            current_frame: 0,
            frame_counter: 0,
            // Probs shouldn't unwrap
            current_anim_name: default_state,
            current_anim: resources.get_anim(default_state).expect("I'm dumb here and probably forgot to load an animation. Soz."),
        }
    }

    pub fn set_action(&mut self, resources: &Resources, action_name: AnimationName) {
        if self.current_anim_name != action_name {
            self.current_anim = resources.get_anim(action_name).expect("I'm dumb here and probably forgot to load an animation. Soz.");
            self.current_anim_name = action_name;
            self.current_frame = 0;
            self.frame_counter = 0;
        }
    }

    pub fn update(&mut self) {
        self.frame_counter += 1;

        if self.frame_counter
            >= self.current_anim.metadata[self.current_frame as usize].1
        {
            self.current_frame += 1;
            self.frame_counter = 0;
        }

        if self.current_frame as usize >= self.current_anim.metadata.len() {
            self.current_frame = 0;
        }
    }

    pub fn get_frame(&self) -> Sprite {
        Sprite::with_texture(
            &self.current_anim.frames[self.current_frame as usize],
        )
    }
}
