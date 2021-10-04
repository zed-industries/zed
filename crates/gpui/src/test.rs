use ctor::ctor;

#[ctor]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}
