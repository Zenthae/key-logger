use std::{
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use rdev::{listen, Event, EventType};

#[derive()]
enum State {
    Start,
    Stop,
    Pause,
    Resume,
}
pub struct KeyLogger {
    tx: Sender<State>,
    rx: Receiver<State>,
    pub handle: Option<JoinHandle<()>>,
    state: Arc<Mutex<State>>,
}

impl KeyLogger {
    pub fn new() -> KeyLogger {
        let (tx, rx) = channel::<State>();
        let state = Arc::new(Mutex::new(State::Stop));

        KeyLogger {
            tx,
            rx,
            handle: None,
            state,
        }
    }

    /// Do something when receive an event
    fn log(event: Event) {
        match event.event_type {
            _ => println!("{:?}", event),
            EventType::KeyPress(_) => todo!(),
            EventType::KeyRelease(_) => todo!(),
            EventType::ButtonPress(_) => todo!(),
            EventType::ButtonRelease(_) => todo!(),
            EventType::MouseMove { x, y } => todo!(),
            EventType::Wheel { delta_x, delta_y } => todo!(),
        };
    }

    /// Start a logger
    /// State::Start start logging keyboard events
    /// State::pause stop logging keyboard events
    /// State::resume resume logging events
    /// State::Stop stop logging, listening, and close the thread
    pub fn start(&mut self) {
        let data = Arc::clone(&self.state);

        let handle = thread::spawn(move || {
            if let Err(error) = listen(move |event: Event| {
                match &self.rx.try_recv() {
                    Ok(s) => {
                        let mut data = data.lock().unwrap();
                        *data = s;
                    }
                    Err(err) => {
                        if let TryRecvError::Disconnected = err {
                            panic!("Channel broke !")
                        }
                    }
                };

                match *data.lock().unwrap() {
                    State::Start => Self::log(event),
                    State::Stop => panic!(), // Ugly but only way of terminating the thread
                    _ => return,
                }
            }) {
                println!("Error: {:?}", error);
            }
        });

        self.handle = Some(handle);
    }

    /// Send a signal that stop and kill a logger
    pub fn stop(&mut self) {}

    /// Send a signal that pause the logger until a resume signal is send
    pub fn pause(&mut self) {}

    /// Send a signal that resume a paused logger
    pub fn resume(&mut self) {}
}
