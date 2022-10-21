use chrono::prelude::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::UNIX_EPOCH,
};

use rdev::Event;

/// Process event send by key_logger and save them into the database.
pub struct Pipeline {
    handle: Option<JoinHandle<()>>,
    alive: Arc<AtomicBool>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            handle: None,
            alive: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create a new thread that filter and send data to the database
    /// Create a channel, pass the receiver to the thread and return the sender
    pub fn open(&mut self) -> Sender<Event> {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();
        let (tx, rx) = mpsc::channel::<Event>();

        self.handle = Some(thread::spawn(move || {
            for event in rx {
                // Use chrono for time
                let key = event.event_type;
                let time: DateTime<Utc> = DateTime::from(event.time);

                println!("{:?}", time);
            }
        }));

        tx
    }

    /// Close the thread and wait for it
    pub fn close(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
