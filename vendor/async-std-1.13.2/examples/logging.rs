//! Prints the runtime's execution log on the standard output.

use async_std::task;

fn main() {
    femme::with_level(log::LevelFilter::Trace);

    task::block_on(async {
        let handle = task::spawn(async {
            log::info!("Hello world!");
        });

        handle.await;
    })
}
