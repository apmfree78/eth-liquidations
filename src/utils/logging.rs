use colored::*;
use std::fs::File;

pub fn setup_logger() -> Result<(), fern::InitError> {
    // File::create("eth_liquidation.log").expect("Failed to create/log file");

    fern::Dispatch::new()
        .format(|out, message, record| {
            let color = match record.level() {
                log::Level::Info => "green",
                log::Level::Warn => "yellow",
                log::Level::Error => "red",
                log::Level::Debug => "magenta",
                log::Level::Trace => "bright black",
            };
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level().to_string().color(color),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        // .chain(fern::log_file("eth_liquidation.log")?)
        .apply()?;
    Ok(())
}
