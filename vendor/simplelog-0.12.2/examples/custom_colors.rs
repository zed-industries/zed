#[cfg(all(feature = "termcolor", not(feature = "paris")))]
use log::*;
#[cfg(feature = "termcolor")]
use simplelog::*;

#[cfg(feature = "termcolor")]
fn main() {
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Trace, Some(Color::Green))
        .build();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    error!("Magenta error");
    warn!("Yellow warning");
    info!("Blue info");
    debug!("Cyan debug");
    trace!("Green trace");
}

#[cfg(not(feature = "termcolor"))]
fn main() {
    println!("this example requires the termcolor feature.");
}
