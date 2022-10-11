use std::time::UNIX_EPOCH;

use diesel::SqliteConnection;
use rdev::{listen, Event, EventType};

use crate::db::{models::NewEvent, query::create_event};

struct Listener {
    // Thread handler
    // Channel to send start and stop message
}

impl Listener {
    fn start() {}

    fn stop() {}

    fn pause() {}

    pub fn new() -> Listener {
        Listener {}
    }
}

pub fn start_listening(conn: &mut SqliteConnection) {
    if let Err(error) = listen(cb) {
        panic!("Error: {:?}", error);
    }

    fn cb(event: Event) {
        match event.event_type {
            EventType::KeyRelease(key) => {
                let new_event = NewEvent {
                    event_time: &i32::try_from(
                        event
                            .time
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backward")
                            .as_secs(),
                    )
                    .unwrap(),
                    key_name: &format!("{:?}", key),
                };

                create_event(conn, event);
            }
            _ => return,
        }
    }
}
