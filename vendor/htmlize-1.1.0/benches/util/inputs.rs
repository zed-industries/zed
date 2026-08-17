// rustc doesn’t seem to realize these are used
#![allow(dead_code)]

pub const CLEAN_SMALL: &str = "Björk and Борис OBrien ❤️, “love beats hate”";
pub const CLEAN_MEDIUM: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.";
pub const CLEAN_BIG: &str = include_str!("../../tests/corpus/html-cleaned.txt");

pub const DIRTY_SMALL: &str = r#"Björk & Борис O'Brien <3, "love > hate""#;
pub const DIRTY_MEDIUM: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa&aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa<aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa>";
pub const DIRTY_BIG: &str = include_str!("../../tests/corpus/html-raw.txt");

pub fn make_sample(count: usize, entity: &str, padding: &str) -> String {
    let mut s = padding.repeat(count);
    s.push_str(entity);
    s.repeat(count)
}
