/*!
Using `env_logger::Logger` and the `log::Log` trait directly.

This example doesn't rely on environment variables, or having a static logger installed.
*/

use env_logger::{Builder, WriteStyle};

use log::{Level, LevelFilter, Log, MetadataBuilder, Record};

fn record() -> Record<'static> {
    let error_metadata = MetadataBuilder::new()
        .target("myApp")
        .level(Level::Error)
        .build();

    Record::builder()
        .metadata(error_metadata)
        .args(format_args!("Error!"))
        .line(Some(433))
        .file(Some("app.rs"))
        .module_path(Some("server"))
        .build()
}

fn main() {
    let stylish_logger = Builder::new()
        .filter(None, LevelFilter::Error)
        .write_style(WriteStyle::Always)
        .build();

    let unstylish_logger = Builder::new()
        .filter(None, LevelFilter::Error)
        .write_style(WriteStyle::Never)
        .build();

    stylish_logger.log(&record());
    unstylish_logger.log(&record());
}
