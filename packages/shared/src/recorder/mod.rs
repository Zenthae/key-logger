use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use rdev::Event;

pub struct Recorder {
    _handle: Option<JoinHandle<()>>,
    _alive: Arc<AtomicBool>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder {
            _handle: None,
            _alive: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn init(&mut self) {
        let alive = self._alive.clone();

        self._handle = Some(thread::spawn(move || {
            if let Err(_error) = rdev::listen(move |event: Event| {
                // Exit the function early when not logging.
                if !alive.load(Ordering::SeqCst) {
                    return;
                }

                println!("{:?}", event);
            }) {
                todo!("Log error")
            }
        }))
    }

    pub fn run(&mut self) {
        self._alive.store(true, Ordering::SeqCst);
    }

    pub fn stop(&mut self) {
        self._alive.store(false, Ordering::SeqCst);
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Self {
            _handle: None,
            _alive: Arc::new(AtomicBool::new(false)),
        }
    }
}
