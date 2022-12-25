use chrono::DateTime;
use rdev::Event;
use sea_orm::{DatabaseConnection, Set};
use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
};
use tokio::task::{self, JoinHandle};

use crate::database::{self, query::event};

pub struct Pipeline {
    _handle: Option<JoinHandle<()>>,
    _alive: Arc<AtomicBool>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            _handle: None,
            _alive: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn init(&mut self, conn: DatabaseConnection) -> Sender<Event> {
        let alive = self._alive.clone();
        let (tx, rx) = mpsc::channel::<Event>();

        self._handle = Some(task::spawn(async move {
            if !alive.load(Ordering::SeqCst) {
                return;
            }

            for event in rx {
                database::query::event::insert_one(
                    &conn,
                    entity::event::ActiveModel {
                        key: Set(serde_json::to_string(&event.event_type)
                            .expect("Failed to serialize event type")),
                        time: Set(DateTime::from(event.time)),
                        ..Default::default()
                    },
                )
                .await;
            }
        }));

        tx
    }

    pub fn run(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(true, Ordering::SeqCst),
            None => panic!("Can't start an uninitialized pipeline."),
        };
    }

    pub fn stop(&mut self) {
        match self._handle {
            Some(_) => self._alive.store(false, Ordering::SeqCst),
            None => panic!("Can't stop an uninitialized recorder."),
        };
    }
}
