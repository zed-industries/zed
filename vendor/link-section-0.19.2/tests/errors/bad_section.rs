use link_section::section;

#[section(untyped)]
static BAD_SECTION: link_section::TypedSection<u32>;

fn main() {
}
