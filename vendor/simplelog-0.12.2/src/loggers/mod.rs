mod comblog;
pub mod logging;
mod simplelog;
#[cfg(feature = "termcolor")]
mod termlog;
#[cfg(feature = "test")]
mod testlog;
mod writelog;

pub use self::comblog::CombinedLogger;
pub use self::simplelog::SimpleLogger;
#[cfg(feature = "termcolor")]
pub use self::termlog::{TermLogger, TerminalMode};
#[cfg(feature = "test")]
pub use self::testlog::TestLogger;
pub use self::writelog::WriteLogger;
