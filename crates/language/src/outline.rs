use std::ops::Range;

#[derive(Debug)]
pub struct Outline(pub Vec<OutlineItem>);

#[derive(Debug)]
pub struct OutlineItem {
    pub id: usize,
    pub depth: usize,
    pub range: Range<usize>,
    pub text: String,
    pub name_range_in_text: Range<usize>,
}
