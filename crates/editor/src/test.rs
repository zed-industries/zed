#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    // std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}
