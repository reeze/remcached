use log;

use log::{LogLevel, SetLoggerError, LogMetadata, LogRecord};
use chrono::*;

pub struct RemcachedLogger {
    level: LogLevel
}

impl RemcachedLogger {
    pub fn new(level: LogLevel) -> Self {
        RemcachedLogger {level: level}
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        let result = log::set_logger(|max_logger_level| {
            max_logger_level.set(self.level.to_log_level_filter()); 
            Box::new(self)
        }); 

        info!("logging initialized");

        result
    }

    pub fn enable_logging(log_level: LogLevel) {
        Self::new(log_level).init().ok();
    }
}

impl log::Log for RemcachedLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level 
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let local: DateTime<UTC> = UTC::now();

            println!("{} {} {}:{} {}", local.to_rfc3339(), record.level(), record.location().module_path(), record.location().line(), record.args());
        }
    }
}