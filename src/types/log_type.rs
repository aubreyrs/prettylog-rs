use crate::types::ansi::{AnsiColor, AnsiPair};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum LogType {
    Information,
    Runtime,
    Debug,
    Network,
    Success,
    Warning,
    Error,
    Exception,
    Critical,
    Audit,
    Trace,
    Security,
    UserAction,
    Performance,
    Config,
    Fatal,
    Custom(&'static dyn CustomLogType),
}

pub trait CustomLogType: Sync + Debug {
    fn name(&self) -> &'static str;
    fn color_pair(&self) -> AnsiPair;
}

impl LogType {
    pub fn name(&self) -> &'static str {
        match self {
            LogType::Information => "ℹ️ Information",
            LogType::Runtime => "✨ Runtime",
            LogType::Debug => "🔧 Debug",
            LogType::Network => "🔌 Network",
            LogType::Success => "✔️ Success",
            LogType::Warning => "⚠️ Warning",
            LogType::Error => "⛔ Error",
            LogType::Exception => "💣 Exception",
            LogType::Critical => "🚨 Critical",
            LogType::Audit => "📋 Audit",
            LogType::Trace => "🔍 Trace",
            LogType::Security => "🔒 Security",
            LogType::UserAction => "🧍 User Action",
            LogType::Performance => "⏱️ Performance",
            LogType::Config => "⚙️ Config",
            LogType::Fatal => "☠️ Fatal",
            LogType::Custom(custom) => custom.name(), // Handle CustomLogType
        }
    }

    pub fn color_pair(&self) -> AnsiPair {
        match self {
            LogType::Information => AnsiPair::new(AnsiColor::CyanBackground, AnsiColor::Cyan),
            LogType::Runtime => AnsiPair::new(AnsiColor::MagentaBackground, AnsiColor::Magenta),
            LogType::Debug => AnsiPair::new(AnsiColor::GrayBackground, AnsiColor::Gray),
            LogType::Network => AnsiPair::new(AnsiColor::BlueBackground, AnsiColor::Blue),
            LogType::Success => {
                AnsiPair::new(AnsiColor::BrightGreenBackground, AnsiColor::BrightGreen)
            }
            LogType::Warning => {
                AnsiPair::new(AnsiColor::BrightYellowBackground, AnsiColor::BrightYellow)
            }
            LogType::Error => AnsiPair::new(AnsiColor::RedBackground, AnsiColor::Red),
            LogType::Exception => AnsiPair::new(AnsiColor::RedBackground, AnsiColor::Red),
            LogType::Critical => {
                AnsiPair::new(AnsiColor::BrightRedBackground, AnsiColor::BrightRed)
            }
            LogType::Audit => AnsiPair::new(AnsiColor::YellowBackground, AnsiColor::Yellow),
            LogType::Trace => AnsiPair::new(AnsiColor::LightBlueBackground, AnsiColor::LightBlue),
            LogType::Security => AnsiPair::new(AnsiColor::PurpleBackground, AnsiColor::Purple),
            LogType::UserAction => {
                AnsiPair::new(AnsiColor::CutePinkBackground, AnsiColor::CutePink)
            }
            LogType::Performance => AnsiPair::new(AnsiColor::PinkBackground, AnsiColor::Pink),
            LogType::Config => AnsiPair::new(AnsiColor::LightGrayBackground, AnsiColor::LightGray),
            LogType::Fatal => AnsiPair::new(AnsiColor::DarkRedBackground, AnsiColor::DarkRed),
            LogType::Custom(custom) => custom.color_pair(), // Handle CustomLogType
        }
    }
}
