use log::{Level, LevelFilter};
use std::io::Write;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        log::info!("[INFO] {}",format_args!($($arg)*));
    })
}

// Macro personalizada para MINSUR
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        log::info!("[ERROR] {}",format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => ({
        log::info!("[INPUT] {}",format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! answer {
    ($($arg:tt)*) => ({
        log::info!("[ANS] {}",format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! good {
    ($($arg:tt)*) => ({
        log::info!("[GOOD] {}",format_args!($($arg)*));
    })
}
pub fn init_logger() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Info)
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();

            match level {
                Level::Info => {
                    if record.args().to_string().contains("[INFO]") {
                        style
                            .set_color(env_logger::fmt::Color::Green)
                            .set_intense(true);
                    } else if record.args().to_string().contains("[ERROR]") {
                        style
                            .set_color(env_logger::fmt::Color::Red)
                            .set_intense(true);
                    } else if record.args().to_string().contains("[INPUT]") {
                        style
                            .set_color(env_logger::fmt::Color::Blue)
                            .set_intense(true);
                    } else if record.args().to_string().contains("[ANS]") {
                        style
                            .set_color(env_logger::fmt::Color::Magenta)
                            .set_intense(true);
                    } else if record.args().to_string().contains("[GOOD]") {
                        style
                            .set_color(env_logger::fmt::Color::Rgb(255, 230, 0))
                            .set_intense(true);
                    }
                }
                _ => {
                    style.set_bg(env_logger::fmt::Color::White);
                }
            }

            writeln!(buf, "{}", style.value(record.args()))
        })
        .init();
}
