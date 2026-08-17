/*
This internal module contains the style and terminal writing implementation.

Its public API is available when the `termcolor` crate is available.
The terminal printing is shimmed when the `termcolor` crate is not available.
*/

#[cfg(feature = "color")]
mod termcolor;
#[cfg(feature = "color")]
pub(in crate::fmt) use self::termcolor::*;
#[cfg(not(feature = "color"))]
mod plain;
#[cfg(not(feature = "color"))]
pub(in crate::fmt) use plain::*;
