//! Functions that perform path lookup manually, one component
//! at a time, with manual symlink resolution.

mod canonical_path;
mod canonicalize;
mod cow_component;
mod open;
#[cfg(not(any(windows, target_os = "freebsd")))]
mod open_entry;
mod read_link_one;

use canonical_path::CanonicalPath;
use cow_component::CowComponent;
use open::internal_open;
use read_link_one::read_link_one;

#[cfg(racy_asserts)]
pub(super) use canonicalize::canonicalize_with;

pub(crate) use canonicalize::canonicalize;
pub(crate) use open::{open, stat};
#[cfg(not(any(windows, target_os = "freebsd")))]
pub(crate) use open_entry::open_entry;
