#[cfg(all(
    not(target_family = "windows"),
    feature = "termcolor",
    not(feature = "paris")
))]
use log::*;
#[cfg(all(not(target_family = "windows"), feature = "termcolor"))]
use simplelog::*;

#[cfg(all(not(target_family = "windows"), feature = "termcolor"))]
fn main() {
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Rgb(191, 0, 0)))
        .set_level_color(Level::Warn, Some(Color::Rgb(255, 127, 0)))
        .set_level_color(Level::Info, Some(Color::Rgb(192, 192, 0)))
        .set_level_color(Level::Debug, Some(Color::Rgb(63, 127, 0)))
        .set_level_color(Level::Trace, Some(Color::Rgb(127, 127, 255)))
        .build();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    error!("Red error");
    warn!("Orange warning");
    info!("Yellow info");
    debug!("Dark green debug");
    trace!("Light blue trace");
}

#[cfg(any(target_family = "windows", not(feature = "termcolor")))]
fn main() {
    println!("this example requires the termcolor feature and a non-Windows OS.");
}
