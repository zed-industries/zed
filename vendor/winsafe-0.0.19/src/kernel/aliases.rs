use crate::co;

/// A [`Result` alias](crate#errors-and-result-aliases) which returns a `Box<dyn
/// Error + Send + Sync>` on failure.
///
/// This is the most generic [`Result`](std::result::Result) possible – any
/// other `Result` can be converted into it.
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// A [`Result` alias](crate#errors-and-result-aliases) for native system error
/// codes, which returns an [`ERROR`](crate::co::ERROR) on failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let sys_result: w::SysResult<()> = Err(co::ERROR::SUCCESS);
///
/// let err_result: w::AnyResult<()> = sys_result.map_err(|err| err.into());
/// ```
pub type SysResult<T> = Result<T, co::ERROR>;
