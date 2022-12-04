use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
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

    /// Create a thread that listen to input events end
    /// Return the receiving side of the communication channel.
    pub fn init(&mut self) -> mpsc::Receiver<Event> {
        let alive = self._alive.clone();
        let (tx, rx) = mpsc::channel::<Event>();

        self._handle = Some(thread::spawn(move || {
            if let Err(_error) = rdev::listen(move |event: Event| {
                // Exit the function early when not logging.
                if !alive.load(Ordering::SeqCst) {
                    return;
                }

                tx.send(event)
                    .expect("Failed to send an event in the channel.");
            }) {
                todo!("Log error")
            }
        }));

        rx
    }

    /// Start or resume an initialized recorder.
    pub fn run(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(true, Ordering::SeqCst),
            None => panic!("Can't start an uninitialized recorder."),
        };
    }

    /// Stop an initialized recorder.
    pub fn stop(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(false, Ordering::SeqCst),
            None => panic!("Can't stop an uninitialized recorder."),
        };
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder::new()
    }
}
