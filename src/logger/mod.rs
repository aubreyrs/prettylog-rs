mod file_writer;
mod settings;
mod style;

use crate::types::LogType;
use file_writer::LoggerFileWriter;
use settings::LoggerSettings;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        LoggerFileWriter::load();
    });
}

pub fn log(message: &str, log_type: LogType) {
    let settings = LoggerSettings::get();
    let formatted_message = settings.logger_style.format(message, log_type);

    println!("{}", formatted_message);

    if settings.save_to_file {
        LoggerFileWriter::write(message, log_type);
    }
}

pub fn log_exception(exception: &dyn std::error::Error) {
    log(&format!("{}", exception), LogType::Exception);
    for (i, cause) in exception.source().into_iter().enumerate() {
        if i > 0 {
            log(&format!("   Caused by: {}", cause), LogType::Exception);
        }
    }
}
