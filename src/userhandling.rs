use crate::{
    utils,
    role::RoleType,
    ai::{UserState, AI},
    config::Config,
    resources::Resources,
};
use rand::{rngs::ThreadRng, Rng};
use sfml::{
    graphics::{Font, RenderWindow},
    system::Vector2f,
    SfBox,
};
use std::collections::HashMap;

const USER_BOUNDS: u32 = 20;
const USER_START_POS: u32 = 50;

pub struct UserHandler<'a> {
    cfg: &'a Config,
    display: (u32, u32),
    font: SfBox<Font>,
    users: HashMap<String, AI<'a>>,
    resources: &'a Resources,
    rng: ThreadRng,
}

impl<'a> UserHandler<'a> {
    pub fn new(cfg: &'a Config, display: (u32, u32), resources: &'a Resources) -> Self {
        Self {
            cfg,
            display,
            font: Font::from_file("res/fonts/lucon.ttf").unwrap(),
            users: HashMap::new(),
            resources,
            rng: rand::thread_rng(),
        }
    }

    pub fn create_users(&mut self, new_chatters: &Vec<(String, RoleType)>) {
        let colours = self.cfg.settings["COLOUR_PALETTE"].as_array().unwrap();

        for (user, role) in new_chatters {
            if !self.users.contains_key(user) {
                let start_x: i32 = if self.rng.gen_range(0_u32..2_u32) == 0 {
                    -(USER_START_POS as i32)
                } else {
                    self.display.0 as i32 + USER_START_POS as i32
                };
                let goto_x: u32 = self
                    .rng
                    .gen_range(USER_BOUNDS..self.display.0 - USER_BOUNDS);

                let colour = utils::colour_from_json(&colours[self.rng.gen_range(0_usize..colours.len())]);

                self.users.insert(
                    user.clone(),
                    AI::new(
                        self.resources,
                        &user,
                        *role,
                        &self.font,
                        colour,
                        Vector2f::new(start_x as f32, self.display.1 as f32 - 13.0),
                        Vector2f::new(goto_x as f32, self.display.1 as f32 - 13.0),
                    ),
                );
            }
        }
    }

    fn trigger_leave_on(&mut self, name: &String) {
        let user = self.users.get_mut(name).unwrap();
        user.move_to_leave(Vector2f::new(-30.0, user.position.y));
    }

    pub fn _say(&mut self, user_name: String, message: String) {
        self.users.get_mut(&user_name).unwrap().say(message);
    }

    pub fn remove_departed(&mut self, user_list: Vec<String>) {
        // Bruh
        let keys: Vec<String> = self.users.keys().map(|s| s.clone()).collect();

        // Iterate over keys, removing and triggering a leave event
        for user_name in keys {
            if !user_list.contains(&user_name) || self.cfg.requests.contains(&user_name) {
                match self.users[&user_name].state {
                    UserState::Removable => {
                        let _ = self.users.remove(&user_name);
                    }
                    UserState::Active => {
                        self.users
                            .get_mut(&user_name)
                            .unwrap()
                            .say("Goodbye".to_string());
                        self.trigger_leave_on(&user_name);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.users.values_mut().for_each(|u| {
            u.update();

            // Move the user randomly, between the window bounds with padding
            if u.state != UserState::Leaving && self.rng.gen_range(0_u32..10_000) < 100 {
                let pos = Vector2f::new(
                    self.rng
                        .gen_range(USER_BOUNDS..self.display.0 - USER_BOUNDS)
                        as f32,
                    u.position.y,
                );
                u.move_to(pos);
            }
        });
    }

    pub fn render(&self, window: &mut RenderWindow) {
        self.users.values().for_each(|u| u.render(window));
    }
}
