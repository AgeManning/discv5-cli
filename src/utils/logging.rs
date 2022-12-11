use clap::ValueEnum;

/// The log level of the application.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level
    Error,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => log::LevelFilter::Trace,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Error => log::LevelFilter::Error,
        }
    }
}

/// Constructs a simple logger with the provided logging level.
pub fn construct_simple_logger(level: LogLevel) {
    // let filter = level.map(|f| f.into()).expect("Log level must be present");
    if simple_logger::SimpleLogger::new()
        .with_level(level.into())
        .with_utc_timestamps()
        .init()
        .is_err()
    {
        log::error!("Failed to initialize logger. Please try again.");
    }
}
