use crate::types::{AnsiColor, LogType};

pub enum LoggerStyle {
    Full,
    Prefix,
    Suffix,
    TextOnly,
    PrefixWhiteText,
    BracketPrefix,
    BracketPrefixWhiteText,
}

impl LoggerStyle {
    pub fn format(&self, message: &str, log_type: LogType) -> String {
        let pattern = self.pattern();
        let formatted = pattern
            .replace("<background>", log_type.color_pair().background.code())
            .replace("<foreground>", log_type.color_pair().foreground.code())
            .replace("<black>", AnsiColor::Black.code())
            .replace("<prefix>", log_type.name())
            .replace("<message>", message)
            .replace("<reset>", AnsiColor::Reset.code())
            .replace("<bold>", AnsiColor::Bold.code());

        format!("{}{}", formatted, AnsiColor::Reset.code())
    }

    fn pattern(&self) -> &str {
        match self {
            LoggerStyle::Full => "<background><black><prefix>: <message>",
            LoggerStyle::Prefix => "<background><black><prefix>:<reset> <foreground><message>",
            LoggerStyle::Suffix => "<foreground><prefix>: <background><black><message>",
            LoggerStyle::TextOnly => "<foreground><prefix>: <message>",
            LoggerStyle::PrefixWhiteText => "<background><black><prefix>:<reset> <message>",
            LoggerStyle::BracketPrefix => {
                "<foreground><bold>[<prefix>]<reset><foreground> <message>"
            }
            LoggerStyle::BracketPrefixWhiteText => "<foreground><bold>[<prefix>] <reset><message>",
        }
    }
}
