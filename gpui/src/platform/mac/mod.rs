mod app;
mod dispatcher;
mod event;
mod geometry;
mod runner;
mod window;

use crate::platform;
pub use app::App;
use cocoa::base::{BOOL, NO, YES};
pub use dispatcher::Dispatcher;
pub use runner::Runner;
use window::Window;

pub fn app() -> impl platform::App {
    App::new()
}

pub fn runner() -> impl platform::Runner {
    Runner::new()
}
trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self {
            YES
        } else {
            NO
        }
    }
}
