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

    /// Start or resume an initialized recorder.
    pub fn run(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(true, Ordering::SeqCst),
            None => panic!("Can't start an uninitialized recorder"),
        };
    }

    /// Stop an initialized recorder.
    pub fn stop(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(false, Ordering::SeqCst),
            None => panic!("Can't stop an uninitialized recorder"),
        };
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder::new()
    }
}
