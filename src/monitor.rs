use std::{
    fs,
    io::Read,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use reqwest;
use serde_json::{self, Value};
use sfml::graphics::RenderWindow;

use crate::{config::Config, userhandling::UserHandler, role};


pub struct Monitor<'a> {
    cfg: &'a Config,
    handler: &'a mut UserHandler<'a>,
    tx: Sender<String>,
    rx: Receiver<String>,
    extx: Option<Sender<()>>,
    blacklist: Vec<String>,
}

impl<'a> Monitor<'a> {
    pub fn new(cfg: &'a Config, handler: &'a mut UserHandler<'a>) -> Monitor<'a> {
        let (tx, rx) = mpsc::channel();
        let blacklist: Vec<String> = fs::read_to_string("res/blacklist.txt").unwrap().lines().map(|s| s.to_string()).collect();

        Monitor {
            cfg,
            handler,
            tx,
            rx,
            extx: None,
            blacklist
        }
    }

    pub fn start(&mut self, channel: String) {
        let thread_tx: Sender<String> = self.tx.clone();
        
        let (extx, exrx) = mpsc::channel();
        self.extx = Some(extx);

        thread::spawn(move || {
            let channel = channel;

            loop {
                // Wait for a response from the web request
                match reqwest::blocking::get(format!("https://tmi.twitch.tv/group/user/{channel}/chatters")) {
                    Ok(res) => {
                        let mut res = res;
                        let mut body = String::new();
                        res.read_to_string(&mut body).unwrap();

                        thread_tx.send(body).unwrap();
                    },
                    _ => {}
                }

                // Wait for an exit response
                if let Ok(_) = exrx.try_recv() {
                    break;
                }

                // Sleep so we don't constantly get requests from the website
                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    pub fn run(&mut self) {
        self.handler.update();

        // If ok, we should receive the new chatters list in string format
        if let Ok(resp) = self.rx.try_recv() {
            // Setup for building the vec of users
            let json_resp: Value = serde_json::from_str(&resp).unwrap();
            let mut current_chatters: Vec<(String, role::RoleType)> = Vec::new();
            let capture_list = self.cfg.settings["CAPTURE"].as_array().unwrap();
            
            // Capture all users within the specified capture list, filtering out the blacklist also
            for role in capture_list {
                let mut other: Vec<(String, role::RoleType)> = json_resp["chatters"][&role.as_str().unwrap()]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|name| {
                        (name.as_str().unwrap().to_string(), role::get_roletype(role.as_str().unwrap().to_string()))
                    })
                    .filter(|(n, _)| !self.blacklist.contains(n))
                    .collect();

                current_chatters.append(&mut other);
            }

            self.handler.create_users(&current_chatters);
            self.handler.remove_departed(current_chatters.into_iter().map(|(n, _)| n).collect());
        }
    }

    pub fn render(&self, window: &mut RenderWindow) {
        self.handler.render(window);
    }

    pub fn close(&mut self) {
        // Tell thread to close
        self.extx.as_ref().unwrap().send(()).unwrap();
    }
}
