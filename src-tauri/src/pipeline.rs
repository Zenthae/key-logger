use chrono::prelude::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::db::{models::NewEvent, PooledConn};
use diesel::prelude::*;
use rdev::{Event, EventType};

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
    pub fn open(&mut self, connection: PooledConn) -> Sender<Event> {
        self.alive.store(true, Ordering::SeqCst);

        // let alive = self.alive.clone();
        let (tx, rx) = mpsc::channel::<Event>();
        let mut connection = connection;
        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            for event in rx {
                if !alive.load(Ordering::SeqCst) {
                    return;
                }

                match event.event_type {
                    EventType::KeyRelease(key) => {
                        use crate::db::schema::key_event::dsl::*;

                        let key = serde_json::to_string(&key).unwrap();
                        let time: DateTime<Utc> = DateTime::from(event.time);
                        // Deref PooledConnection
                        // https://github.com/sfackler/r2d2/issues/37
                        let conn = &mut *connection;

                        diesel::insert_into(key_event)
                            .values(&NewEvent {
                                key_name: &key,
                                event_time: &time,
                            })
                            .execute(conn)
                            .unwrap();
                    }
                    _ => continue,
                }
            }
        }));

        tx
    }

    /// Close the thread and wait for it
    pub fn close(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
