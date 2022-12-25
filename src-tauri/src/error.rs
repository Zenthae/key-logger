use std::{io, sync::mpsc};

pub type Result<T> = std::result::Result<T, Error<T>>;

#[derive(Debug)]
pub enum Error<T> {
    Io(io::Error),

    Channel(mpsc::SendError<T>),

    Listen(rdev::ListenError),
}

impl<T> From<io::Error> for Error<T> {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl<T> From<mpsc::SendError<T>> for Error<T> {
    fn from(error: mpsc::SendError<T>) -> Self {
        Self::Channel(error)
    }
}
