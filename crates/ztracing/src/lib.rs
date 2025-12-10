#[cfg(ztracing)]
pub use tracing::instrument;
#[cfg(not(ztracing))]
pub use ztracing_macro::instrument;

#[cfg(ztracing)]
pub fn init() {
    use tracing_subscriber::prelude::*;
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default()),
    )
    .expect("setup tracy layer");
}

#[cfg(not(ztracing))]
pub fn init() {}
