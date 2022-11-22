/// Log manager
pub struct Logger {
    _level: LogLevel,
}

impl Logger {}

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
