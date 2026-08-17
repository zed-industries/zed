#[cfg(all(feature = "termcolor", not(feature = "paris")))]
use log::*;
#[cfg(feature = "termcolor")]
use simplelog::*;

#[cfg(feature = "termcolor")]
fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
    error!("Red error");
    warn!("Yellow warning");
    info!("Blue info");
    debug!("Cyan debug");
    trace!("White trace");
}

#[cfg(not(feature = "termcolor"))]
fn main() {
    println!("this example requires the termcolor feature.");
}
