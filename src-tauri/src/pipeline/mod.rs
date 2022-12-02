use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
    thread::{self, JoinHandle},
};

use rdev::Event;

pub struct Pipeline {
    _handle: Option<JoinHandle<()>>,
    _alive: Arc<AtomicBool>,
    // _rx: Receiver<Event>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            _handle: None,
            _alive: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline::new()
    }
}
