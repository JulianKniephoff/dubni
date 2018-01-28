use std::fmt::Write;
use log::{self, Record, Level, LevelFilter, Metadata, SetLoggerError};


/// A simple logger implementation using the `console` browser API to show the
/// log messages.
struct WebLogger;


/// Initializes the global logger. Call only once!
pub fn init() -> Result<(), SetLoggerError> {
    static LOGGER: WebLogger = WebLogger;

    log::set_max_level(LevelFilter::Trace);
    log::set_logger(&LOGGER)
}

impl log::Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // If we have module path and location information, we add it to the
        // log message.
        let location = match (record.module_path(), record.file(), record.line()) {
            (Some(mod_path), Some(file), Some(line)) => {
                format!(" ⟨[{}] {}:{}⟩", mod_path, file, line)
            }
            _ => "".into(),
        };

        // Prepare string to output
        let level = record.level();
        let mut message = String::new();
        write!(
            &mut message,
            "{}{}",
            record.args(),
            location,
        ).unwrap();

        match level {
            Level::Error => {
                js! { console.error(@{message}) }
            }
            Level::Warn => {
                js! { console.warn(@{message}) }
            }
            Level::Info => {
                js! { console.info(@{message}) }
            }
            Level::Debug => {
                js! { console.log("DEBUG: " + @{message}) }
            }
            Level::Trace => {
                js! { console.log("TRACE: " + @{message}) }
            }
        }
    }

    fn flush(&self) {}
}
