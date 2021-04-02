use ctor::ctor;
use simplelog::SimpleLogger;
use log::LevelFilter;

#[ctor]
fn init_logger() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");
}
