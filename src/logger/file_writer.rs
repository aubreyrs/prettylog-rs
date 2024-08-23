use crate::logger::settings::LoggerSettings;
use crate::types::LogType;
use chrono::Local;
use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

static LOGGER_FILE_WRITER: Lazy<Mutex<LoggerFileWriter>> =
    Lazy::new(|| Mutex::new(LoggerFileWriter::new()));

pub struct LoggerFileWriter {
    is_loaded: bool,
    file: Option<File>,
    unloaded_log_queue: Vec<(String, LogType)>,
}

impl LoggerFileWriter {
    fn new() -> Self {
        LoggerFileWriter {
            is_loaded: false,
            file: None,
            unloaded_log_queue: Vec::new(),
        }
    }

    pub fn load() {
        let mut writer = LOGGER_FILE_WRITER.lock().unwrap();

        if writer.is_loaded {
            return;
        }

        let settings = LoggerSettings::get();
        let mut save_directory_path = settings.save_directory_path.clone();
        if !save_directory_path.ends_with('/') {
            save_directory_path.push('/');
        }

        if !Path::new(&save_directory_path).exists() {
            std::fs::create_dir_all(&save_directory_path).unwrap();
        }

        let log_file_name = Local::now()
            .format(&settings.log_file_name_format)
            .to_string();
        let file_path = format!("{}{}.log", save_directory_path, log_file_name);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&file_path);

        match file {
            Ok(f) => {
                writer.file = Some(f);
                writer.is_loaded = true;

                for (message, log_type) in writer.unloaded_log_queue.drain(..) {
                    Self::write(&message, log_type);
                }
            }
            Err(e) => {
                eprintln!("Error opening log file: {:?}", e);
            }
        }
    }

    pub fn write(message: &str, log_type: LogType) {
        let mut writer = LOGGER_FILE_WRITER.lock().unwrap();
        if !writer.is_loaded {
            writer
                .unloaded_log_queue
                .push((message.to_string(), log_type));
            return;
        }

        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = format!("{} [{}] {}\n", date, log_type.name(), message);

        if let Some(file) = &mut writer.file {
            if let Err(e) = file.write_all(log_message.as_bytes()) {
                eprintln!("Error writing to log file: {:?}", e);
            }
            if let Err(e) = file.flush() {
                eprintln!("Error flushing log file: {:?}", e);
            }
        }
    }
}

pub fn directory_exists(directory_path: &str) -> bool {
    Path::new(directory_path).is_dir()
}
