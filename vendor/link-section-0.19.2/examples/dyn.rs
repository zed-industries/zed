//! Example usage of the `link-section` crate.
#![warn(missing_docs)]

use link_section::{in_section, section};

#[section(typed)]
static DATA_SECTION: link_section::TypedSection<&'static (dyn std::fmt::Debug + Sync)>;

/// A dynamic item in the `DATA_SECTION` section.
#[in_section(DATA_SECTION)]
pub static DATA_ITEM_VEC: &'static (dyn std::fmt::Debug + Sync) = &Vec::<u32>::new();

/// A string item in the `DATA_SECTION` section.
#[in_section(DATA_SECTION)]
pub static DATA_ITEM_STRING: &'static (dyn std::fmt::Debug + Sync) = &String::new();

fn main() {
    eprintln!("DATA_ITEM: {:?}", DATA_SECTION.as_slice());
}
