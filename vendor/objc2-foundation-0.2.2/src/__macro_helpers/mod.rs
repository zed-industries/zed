mod cached;
#[cfg(feature = "NSString")]
mod ns_string;

pub use self::cached::CachedId;
#[cfg(feature = "NSString")]
pub use self::ns_string::*;
