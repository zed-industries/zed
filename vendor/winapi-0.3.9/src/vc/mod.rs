// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers that come with VC. Notably, these are not part of the Windows SDK.
#[cfg(feature = "excpt")] pub mod excpt;
#[cfg(feature = "limits")] pub mod limits;
#[cfg(feature = "vadefs")] pub mod vadefs;
#[cfg(feature = "vcruntime")] pub mod vcruntime;
