use std::{
    io::Read,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};
use reqwest;
use sfml::graphics::RenderWindow;

use crate::userhandling::UserHandler;


pub struct Monitor<'a> {
    handler: &'a mut UserHandler<'a>,
    tx: Sender<String>,
    rx: Receiver<String>,
    extx: Option<Sender<()>>,
}

impl<'a> Monitor<'a> {
    pub fn new(handler: &'a mut UserHandler<'a>) -> Monitor<'a> {
        let (tx, rx) = mpsc::channel();

        Monitor {
            handler,
            tx,
            rx,
            extx: None,
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
            // TODO: Deserialize the JSON and pass relevant data to user handler
            // println!("Response :: '{resp}'");
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
