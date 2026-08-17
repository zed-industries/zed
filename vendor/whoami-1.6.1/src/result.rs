use std::io::Error;

/// This crate's convenience type alias for [`Result`](std::result::Result)s
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
