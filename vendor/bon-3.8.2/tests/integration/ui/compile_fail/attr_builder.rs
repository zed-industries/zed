use bon::{bon, builder, Builder};

#[derive(Builder)]
#[builder]
struct EmptyTopLevelBuilderAttr {}

#[derive(Builder)]
#[builder()]
struct EmptyTopLevelBuilderAttrWithParens {}

#[derive(Builder)]
struct EmptyMemberLevelBuilderAttr {
    #[builder]
    x: u32,
}

#[derive(Builder)]
struct EmptyMemberLevelBuilderAttrWithParens {
    #[builder()]
    x: u32,
}

#[builder]
fn fn_empty_member_level_builder_attr(#[builder] _x: u32) {}

#[builder]
fn fn_empty_member_level_builder_attr_with_parens(#[builder()] _x: u32) {}

struct EmptyBuilderAttr;

#[bon]
impl EmptyBuilderAttr {
    #[builder]
    fn empty_member_level_builder_attr(#[builder] _x: u32) {}
}

#[bon]
impl EmptyBuilderAttr {
    #[builder]
    fn empty_member_level_builder_attr_with_parens(#[builder()] _x: u32) {}
}

#[bon]
impl EmptyBuilderAttr {
    #[builder()]
    fn empty_top_level_builder_attr_with_parens() {}
}

#[builder]
struct LegacyBuilderProcMacroAttrOnStruct {}

#[builder]
enum EnumsAreUnsupported {}

fn main() {}

#[builder]
#[must_use]
#[must_use]
fn double_must_use() {}

#[builder]
fn destructuring1((x, y): (u32, u32)) {
    let _ = x;
    let _ = y;
}

#[builder]
fn destructuring2((_, _): (u32, u32)) {}

#[builder]
#[track_caller]
#[track_caller]
fn double_track_caller() {}
#[builder]
#[target_feature(enable = "")]
#[target_feature(enable = "")]
fn double_invalid_target_feature() {}
