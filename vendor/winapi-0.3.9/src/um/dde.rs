// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{PUINT_PTR, UINT_PTR};
use shared::minwindef::{BOOL, LPARAM, UINT};
extern "system" {
    pub fn PackDDElParam(
        msg: UINT,
        uiLo: UINT_PTR,
        uiHi: UINT_PTR,
    ) -> LPARAM;
    pub fn UnpackDDElParam(
        msg: UINT,
        lParam: LPARAM,
        puiLo: PUINT_PTR,
        puiHi: PUINT_PTR,
    ) -> BOOL;
}
