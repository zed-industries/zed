use ctor::ctor;

#[ctor]
fn init_logger() {
    env_logger::init();
}
