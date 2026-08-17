// Inspired by https://github.com/vorner/signal-hook/blob/3473f4520a710f05d352275731100807196de519/examples/print.rs

use async_signal::{Signal, Signals};
use futures_lite::prelude::*;
use signal_hook::low_level;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    async_io::block_on(async {
        // Register the signals we want to receive.
        #[cfg(unix)]
        let signals = Signals::new([
            Signal::Term,
            Signal::Quit,
            Signal::Int,
            Signal::Tstp,
            Signal::Winch,
            Signal::Hup,
            Signal::Child,
            Signal::Cont,
        ])?;

        #[cfg(windows)]
        let signals = Signals::new([Signal::Int])?;

        // Loop over the signals.
        signals
            .for_each(|signal| {
                // Print the signal.
                eprintln!("Received signal {signal:?}");

                // After printing it, do whatever the signal was supposed to do in the first place.
                low_level::emulate_default_handler(signal.unwrap() as i32).unwrap();
            })
            .await;

        Ok(())
    })
}
