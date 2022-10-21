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

use crate::db::{models::NewEvent, query::insert_event};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    SqliteConnection,
};
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
    pub fn open(
        &mut self,
        connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    ) -> Sender<Event> {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();
        let (tx, rx) = mpsc::channel::<Event>();

        self.handle = Some(thread::spawn(move || {
            for event in rx {
                match event.event_type {
                    EventType::KeyRelease(key) => {
                        let key = serde_json::to_string(&key).unwrap();
                        let time: DateTime<Utc> = DateTime::from(event.time);

                        insert_event(
                            connection,
                            NewEvent {
                                key_name: &key,
                                event_time: &time,
                            },
                        );
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
