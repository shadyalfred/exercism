// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut log_message = String::from("[");

    log_message.push_str(match level {
        LogLevel::Error => "ERROR",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
    });

    log_message.push_str("]: ");

    log_message.push_str(message);

    log_message
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
