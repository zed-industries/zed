use link_section::{section, in_section};

#[section(typed)]
static SECTION: link_section::TypedSection<u32>;

// Currently returns different errors between nightly and stable
// #[in_section(SECTION)]
// fn bad_item() {
//     println!("bad_item");
// }

#[in_section(SECTION)]
static BAD_ITEM: u64 = 1;

fn main() {
}
