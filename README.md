# prettylog-rs

[![Crates.io](https://img.shields.io/crates/v/prettylog-rs.svg)](https://crates.io/crates/prettylog-rs)

A Rust logging library focused on readability in console.
prettylog-rs takes advantage of ANSI color codes to make your logs look ✨ ***pretty*** ✨.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
prettylog-rs = "0.1.0"
```

## Logging

Logging is very easy, just call the `log(message, type)` function. The `type` parameter is optional and defaults to `LogType::Runtime`.

```rust
use prettylog_rs::{log, LogType};

fn main() {
    prettylog_rs::logger::init();

    log("Hello there!", LogType::Information);
    log("general kenobi", LogType::Network);
}
```

You can also log exceptions!

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    prettylog_rs::logger::init();

    // Some code that might throw an error
    let result = std::fs::read_to_string("non_existent_file.txt");

    if let Err(e) = result {
        log_exception(&e);
    }

    Ok(())
}
```

## Logger Settings

You can change settings by modifying the `LoggerSettings`:

```rust
use prettylog_rs::logger::{LoggerSettings, LoggerStyle};

fn main() {
    let mut settings = LoggerSettings::get();
    settings.save_to_file = true;
    settings.save_directory_path = String::from("./logs/");
    settings.log_file_name_format = String::from("%Y-%m-%d-%H%M%S");
    settings.logger_style = LoggerStyle::Prefix;
}
```

## Log Types

There are 16 default log types: **Debug**, **Information**, **Runtime**, **Network**, **Success**, **Warning**, **Error**, **Exception**, **Critical**, **Audit**, **Trace**, **Security**, **User Action**, **Performance**, **Config**, and **Fatal**.

### Custom log Types
Needs to be properly implemented 

You can create custom log types by implementing your own enum and associated functions:

```rust
use prettylog_rs::types::{AnsiColor, AnsiPair};

pub enum CustomLogType {
    Cute,
    Git,
    FireWarning,
}

impl CustomLogType {
    pub fn name(&self) -> &'static str {
        match self {
            CustomLogType::Cute => "≽^•⩊•^≼",
            CustomLogType::Git => "🤖 Git",
            CustomLogType::FireWarning => "🔥 Fire Warning",
        }
    }

    pub fn color_pair(&self) -> AnsiPair {
        match self {
            CustomLogType::Cute => AnsiPair::new(AnsiColor::CutePinkBackground, AnsiColor::CutePink),
            CustomLogType::Git => AnsiPair::new(AnsiColor::AquaBackground, AnsiColor::Aqua),
            CustomLogType::FireWarning => AnsiPair::new(AnsiColor::OrangeBackground, AnsiColor::Orange),
        }
    }
}

fn main() {
    prettylog_rs::logger::init();

    log("T-This is vewy cuwute message OwO", CustomLogType::Cute);
    log("refusing to merge unrelated histories", CustomLogType::Git);
    log("SERVER ROOM ON FIRE, DONT LET ASO RUN WHILE LOOPS EVER AGAIN", CustomLogType::FireWarning);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Todo

**Properly implement CustomLogType**

## Credits

https://github.com/LukynkaCZE
https://github.com/LukynkaCZE/PrettyLog
