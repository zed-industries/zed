// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn main() {
    #[cfg(feature = "tracy")]
    {
        use tracing_subscriber::layer::SubscriberExt;

        tracy_client::register_demangler!();
        tracy_client::Client::start();
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
        )
        .expect("setup tracy layer");
    }

    // separated out so that the file containing the main function can be imported by other crates,
    // while having all gpui resources that are registered in main (primarily actions) initialized
    zed::main();
}
