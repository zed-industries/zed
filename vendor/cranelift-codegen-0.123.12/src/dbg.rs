//! Debug tracing helpers.
use core::fmt;

/// Prefix added to the log file names, just before the thread name or id.
pub static LOG_FILENAME_PREFIX: &str = "cranelift.dbg.";

/// Helper for printing lists.
pub struct DisplayList<'a, T>(pub &'a [T])
where
    T: 'a + fmt::Display;

impl<'a, T> fmt::Display for DisplayList<'a, T>
where
    T: 'a + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.split_first() {
            None => write!(f, "[]"),
            Some((first, rest)) => {
                write!(f, "[{first}")?;
                for x in rest {
                    write!(f, ", {x}")?;
                }
                write!(f, "]")
            }
        }
    }
}
