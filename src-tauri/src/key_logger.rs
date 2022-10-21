use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
};

use rdev::{Event, EventType};

/// Listen to every input event and send them to the processing pipeline
pub struct KeyLogger {
    handle: Option<JoinHandle<()>>,
    alive: Arc<AtomicBool>,
    /// processing pipeline access
    tx: Sender<Event>,
}

impl KeyLogger {
    pub fn new(tx: Sender<Event>) -> KeyLogger {
        KeyLogger {
            handle: None,
            alive: Arc::new(AtomicBool::new(false)),
            tx,
        }
    }

    /// Spawn a listener thread and start logging
    pub fn start(&mut self) {
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();
        let tx = self.tx.clone();

        self.handle = Some(thread::spawn(move || {
            if let Err(error) = rdev::listen(move |event: Event| {
                // Since rdev crate is bullshit
                // instead of exiting the thread we just return from the logging function
                if !alive.load(Ordering::SeqCst) {
                    return;
                }

                tx.send(event).expect("Failed to send the event.");
            }) {
                println!("{:?}", error);
            };
        }));
    }

    /// Set the listening mod to off
    pub fn stop(&mut self) {
        println!("Stopping the logger.");
        self.alive.store(false, Ordering::SeqCst);

        // No join because rdev never exit
        // self.handle
        //     .take()
        //     .expect("Called stop on non-running thread")
        //     .join()
        //     .expect("Could not join spawned thread");
        println!("Logger stopped.");
    }

    /// Resume logging (alive = true)
    pub fn resume(&mut self) {
        self.alive.store(true, Ordering::SeqCst);
    }
}
