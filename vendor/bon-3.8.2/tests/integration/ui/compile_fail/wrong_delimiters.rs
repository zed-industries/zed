#[derive(bon::Builder)]
#[builder(
    builder_type{},
    state_mod{},
    start_fn{},
    finish_fn{},
)]
struct CurlyBraces {}

#[derive(bon::Builder)]
struct CurlyBracesInField {
    #[builder(setters{})]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type[doc[]],
    state_mod[doc[]],
    start_fn[doc[]],
    finish_fn[doc[]],
)]
struct SquareBrackets {
    #[builder(setters[])]
    x: u32,
}

#[derive(bon::Builder)]
struct SquareBracketsInFieldSetters {
    #[builder(setters[])]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type(doc[]),
    state_mod(doc[]),
    start_fn(doc[]),
    finish_fn(doc[]),
)]
struct SquareBracketsInFieldDoc {
    #[builder(setters(doc[]))]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type(doc()),
    state_mod(doc()),
    start_fn(doc()),
    finish_fn(doc())
)]
struct Parentheses {
    #[builder(setters(doc()))]
    x: u32,
}

fn main() {}
