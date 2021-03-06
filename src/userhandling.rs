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

pub struct UserHandler {
    cfg: Config,
    display: (u32, u32),
    font: SfBox<Font>,
    users: HashMap<String, AI>,
    resources: Resources,
    rng: ThreadRng,

    user_limit: usize,
    colours: Vec<serde_json::Value>,
}

impl UserHandler {
    pub fn new(cfg: Config, display: (u32, u32), resources: Resources) -> Self {
        let user_limit = serde_json::from_slice::<usize>(cfg.settings["USER_LIMIT"].to_string().as_bytes()).unwrap();
        let colours = cfg.settings["COLOUR_PALETTE"].as_array().unwrap().to_vec();

        Self {
            cfg,
            display,
            font: Font::from_file("res/fonts/lucon.ttf").unwrap(),
            users: HashMap::new(),
            resources,
            rng: rand::thread_rng(),
            user_limit,
            colours,
        }
    }

    pub fn handle_user_change(&mut self, new_chatters: &Vec<(String, RoleType)>) {
        for (new_user, role) in new_chatters {
            if self.user_limit > 0 && self.users.len() >= self.user_limit {
                return;
            }

            // Add user, they're not in the current list or in the removal requests
            if !self.users.contains_key(new_user) && !self.cfg.requests.contains(new_user) {
                let start_x: i32 = if self.rng.gen_range(0_u32..2_u32) == 0 {
                    -(USER_START_POS as i32)
                } else {
                    self.display.0 as i32 + USER_START_POS as i32
                };
                let goto_x: u32 = self
                    .rng
                    .gen_range(USER_BOUNDS..self.display.0 - USER_BOUNDS);

                let colour = utils::colour_from_json(&self.colours[self.rng.gen_range(0_usize..self.colours.len())]);

                self.users.insert(
                    new_user.clone(),
                    AI::new(
                        &self.resources,
                        &new_user,
                        *role,
                        &self.font,
                        colour,
                        Vector2f::new(start_x as f32, self.display.1 as f32 - 13.0),
                        Vector2f::new(goto_x as f32, self.display.1 as f32 - 13.0),
                    ),
                );
            }
        }

        self.remove_departed(new_chatters);
    }

    pub fn remove_departed(&mut self, new_chatters: &Vec<(String, RoleType)>) {
        let mut users_departed: Vec<String> = Vec::new();

        self.users.iter().for_each(|(name, ai)| {
            if !new_chatters.contains(&(name.to_string(), ai.role)) || self.cfg.requests.contains(name) {
                users_departed.push(name.clone());
            }
        });

        users_departed.into_iter().for_each(|name| {
            match self.users[&name].state {
                UserState::Removable => {
                    let _ = self.users.remove(&name);
                }
                UserState::Active => {
                    self.users
                        .get_mut(&name)
                        .unwrap()
                        .say("Goodbye".to_string());
                    self.trigger_leave_on(&name);
                }
                _ => {}
            }
        });
    }

    fn trigger_leave_on(&mut self, name: &String) {
        let user = self.users.get_mut(name).unwrap();
        user.move_to_leave(&self.resources, Vector2f::new(-30.0, user.position.y));

        println!("{name} is leaving...");
    }

    pub fn _say(&mut self, user_name: String, message: String) {
        self.users.get_mut(&user_name).unwrap().say(message);
    }

    pub fn update(&mut self) {
        self.users.values_mut().for_each(|u| {
            u.update(&self.resources);

            // Move the user randomly, between the window bounds with padding
            if u.state != UserState::Leaving && self.rng.gen_range(0_u32..50_000) < 200 {
                let pos = Vector2f::new(
                    self.rng
                        .gen_range(USER_BOUNDS..self.display.0 - USER_BOUNDS)
                        as f32,
                    u.position.y,
                );
                u.move_to(&self.resources, pos);
            }
        });
    }

    pub fn render(&self, window: &mut RenderWindow) {
        self.users.values().for_each(|u| u.render(window));
    }
}
