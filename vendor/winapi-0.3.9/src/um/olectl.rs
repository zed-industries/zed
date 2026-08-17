// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! OLE Control interfaces
use shared::winerror::{FACILITY_ITF, SEVERITY_ERROR, SEVERITY_SUCCESS};
use um::winnt::HRESULT;
pub const SELFREG_E_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x0200);
pub const SELFREG_E_LAST: HRESULT = MAKE_SCODE!(SEVERITY_ERROR, FACILITY_ITF, 0x020F);
pub const SELFREG_S_FIRST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x0200);
pub const SELFREG_S_LAST: HRESULT = MAKE_SCODE!(SEVERITY_SUCCESS, FACILITY_ITF, 0x020F);
pub const SELFREG_E_TYPELIB: HRESULT = SELFREG_E_FIRST + 0;
pub const SELFREG_E_CLASS: HRESULT = SELFREG_E_FIRST + 1;
