use rdev::Event;
use sea_orm::DatabaseConnection;
use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
        Arc,
    },
    // thread::{self, JoinHandle},
};
use tokio::task::{self, JoinHandle};

use crate::database::{self, query::event};

pub struct Pipeline {
    _handle: Option<JoinHandle<()>>,
    _alive: Arc<AtomicBool>,
    _rx: Receiver<Event>,
}

impl Pipeline {
    pub fn new(receiver: Receiver<Event>) -> Self {
        Pipeline {
            _handle: None,
            _alive: Arc::new(AtomicBool::new(false)),
            _rx: receiver,
        }
    }

    pub fn init(&mut self, conn: DatabaseConnection) {
        let alive = self._alive.clone();

        self._handle = Some(task::spawn(async move {
            database::query::event::get_by_id(&conn, 1).await;
        }));
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
