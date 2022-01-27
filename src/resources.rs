use crate::animations::Animation;
use sfml::{graphics::Texture, SfBox};
use std::collections::HashMap;

pub struct Resources {
    pub textures: HashMap<String, SfBox<Texture>>,
    pub animations: HashMap<String, Animation>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            textures: HashMap::new(),
            animations: HashMap::new(),
        }
    }
}
