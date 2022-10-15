use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use rdev::{Event, EventType};

pub struct KeyLogger {
    handle: Option<JoinHandle<()>>,
    alive: Arc<AtomicBool>,
}

impl KeyLogger {
    pub fn new() -> KeyLogger {
        KeyLogger {
            handle: None,
            alive: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) {
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            rdev::listen(move |event: Event| {
                println!("{:?}", event.event_type);
                if !alive.load(Ordering::SeqCst) {
                    // process::exit(0);
                    panic!();
                }

                match event.event_type {
                    _ => return,
                    EventType::KeyPress(_) => todo!(),
                    EventType::KeyRelease(_) => todo!(),
                    EventType::ButtonPress(_) => todo!(),
                    EventType::ButtonRelease(_) => todo!(),
                    EventType::MouseMove { x, y } => todo!(),
                    EventType::Wheel { delta_x, delta_y } => todo!(),
                };
            });
        }));
    }

    pub fn stop(&mut self) {
        println!("Stopping the logger");
        self.alive.store(false, Ordering::SeqCst);

        rdev::simulate(&EventType::Wheel {
            delta_x: 0,
            delta_y: 1,
        });

        // self.handle
        //     .take()
        //     .expect("Called stop on non-running thread")
        //     .join()
        //     .expect("Could not join spawned thread");
        println!("Logger stopped");
    }

    /// deprecated
    fn key_logger(&self) {
        let alive = self.alive.clone();

        rdev::listen(move |event: Event| {
            if !alive.load(Ordering::SeqCst) {
                return;
            }

            match event.event_type {
                EventType::KeyPress(_) => todo!(),
                EventType::KeyRelease(_) => todo!(),
                EventType::ButtonPress(_) => todo!(),
                EventType::ButtonRelease(_) => todo!(),
                EventType::MouseMove { x, y } => todo!(),
                EventType::Wheel { delta_x, delta_y } => todo!(),
            };
        });
    }
}
