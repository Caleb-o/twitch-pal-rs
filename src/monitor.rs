use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::userhandling::UserHandler;

pub struct Monitor<'a> {
    handler: &'a UserHandler<'a>,
    tx: Sender<String>,
    rx: Receiver<String>,
    handle: Option<JoinHandle<()>>,
}

impl<'a> Monitor<'a> {
    pub fn new(handler: &'a UserHandler) -> Monitor<'a> {
        let (tx, rx) = mpsc::channel();
        Monitor {
            handler,
            tx,
            rx,
            handle: None,
        }
    }

    pub fn start(&mut self) {
        let thread_tx: Sender<String> = self.tx.clone();

        self.handle = Some(thread::spawn(move || {
            // TODO: Fill in
            panic!("TODO: Unreachable");
        }));
    }

    pub fn run(&mut self) {
        let resp = self.rx.recv_timeout(Duration::from_millis(500));

        // If ok, we should receive the new chatters list in string format
        if let Ok(new_chatters) = resp {
            // TODO: Deserialize the JSON and pass relevant data to user handler
        }
    }

    pub fn close(&mut self) {
        panic!("TODO: Unreachable");
    }
}
