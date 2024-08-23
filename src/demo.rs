use crate::types::{AnsiColor, AnsiPair, CustomLogType}; // Import necessary traits and types
use crate::{log, log_exception, LogType};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

#[derive(Debug)]
struct CuteLog;

impl CustomLogType for CuteLog {
    fn name(&self) -> &'static str {
        "≽^•⩊•^≼"
    }

    fn color_pair(&self) -> AnsiPair {
        AnsiPair::new(AnsiColor::CutePinkBackground, AnsiColor::CutePink)
    }
}

#[derive(Debug)]
struct GitLog;

impl CustomLogType for GitLog {
    fn name(&self) -> &'static str {
        "🤖 Git"
    }

    fn color_pair(&self) -> AnsiPair {
        AnsiPair::new(AnsiColor::AquaBackground, AnsiColor::Aqua)
    }
}

pub fn run() {
    crate::logger::init();
    log("Running main() in Demo.rs..", LogType::Debug);
    log("Very informative information", LogType::Information);
    log("I am running on time!", LogType::Runtime);
    log("Downloading maxwell.mp3", LogType::Network);
    log("maxwell.mp3 has been downloaded!", LogType::Success);
    log(
        "Warning.. file maxwell.mp3 may be corrupted!",
        LogType::Warning,
    );
    log(
        "maxwell.mp3 cannot be played using IDrawableTrack",
        LogType::Error,
    );
    log(
        "Critical issue detected in the payment system!",
        LogType::Critical,
    );
    log(
        "User SkibidyToilet727 accessed the admin panel",
        LogType::Audit,
    );
    log("Entering detailed trace mode for debugging", LogType::Trace);
    log("Security breach attempt detected!", LogType::Security);
    log(
        "NeuroSama updated her profile picture to bread.png",
        LogType::UserAction,
    );
    log("Response time is 250ms", LogType::Performance);
    log("MaxConnections set to 1000", LogType::Config);
    log("Your life will be terminated", LogType::Fatal);

    // custom log types
    log("This is a very cute message OwO", LogType::Custom(&CuteLog));
    log(
        "Refusing to merge unrelated histories",
        LogType::Custom(&GitLog),
    );

    let error = CustomError {
        message: "maxwell.mp3 is not a valid music file in the format of MP3!".to_string(),
    };
    log_exception(&error);
}
