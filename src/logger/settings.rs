use crate::logger::style::LoggerStyle;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct LoggerSettings {
    pub save_to_file: bool,
    pub save_directory_path: String,
    pub logger_style: LoggerStyle,
    pub log_file_name_format: String,
}

impl LoggerSettings {
    pub fn get() -> impl std::ops::Deref<Target = LoggerSettings> {
        static SETTINGS: Lazy<Mutex<LoggerSettings>> = Lazy::new(|| {
            Mutex::new(LoggerSettings {
                save_to_file: true,
                save_directory_path: String::from("./logs/"),
                logger_style: LoggerStyle::Prefix,
                log_file_name_format: String::from("%Y-%m-%d-%H%M%S"),
            })
        });
        SETTINGS.lock().unwrap()
    }
}
