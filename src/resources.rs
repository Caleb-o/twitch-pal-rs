use crate::animations::{Animation, AnimationName};
use sfml::{graphics::Texture, SfBox};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Resources {
    pub textures: HashMap<String, SfBox<Texture>>,
    pub animations: HashMap<AnimationName, Rc<Animation>>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    pub fn load_animation(
        &mut self,
        anim_path: &str,
        anim_name: AnimationName,
        frame_times: &[u32]
    ) 
    {
        let _ = self.animations.insert(anim_name, Animation::new(anim_path, frame_times));
    }

    pub fn get_anim(&self, id: AnimationName) -> Option<Rc<Animation>> {
        if let Some(anim) = self.animations.get(&id) {
            return Some(Rc::clone(anim));
        }

        None
    }
}
