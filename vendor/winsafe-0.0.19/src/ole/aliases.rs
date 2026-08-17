use crate::co;

/// A [`Result` alias](crate#errors-and-result-aliases) for COM error codes,
/// which returns an [`HRESULT`](crate::co::HRESULT) on failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let hr_result: w::HrResult<()> = Err(co::HRESULT::E_INVALIDARG);
///
/// let err_result: w::AnyResult<()> = hr_result.map_err(|err| err.into());
/// ```
///
/// Converting from an [`SysResult`](crate::SysResult):
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let win_result: w::SysResult<()> = Err(co::ERROR::FILE_NOT_FOUND);
///
/// let hr_result: w::HrResult<()> = win_result.map_err(|err| err.to_hresult());
/// ```
pub type HrResult<T> = Result<T, co::HRESULT>;
