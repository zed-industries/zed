use bon::builder;

#[builder(on(String, into))]
fn unnecessary_into(#[builder(into)] _x: String) {}

#[builder(on(&dyn std::fmt::Debug, into))]
fn invalid_type_pattern() {}

#[builder(on(fn(#[attr] a: u32), into))]
fn attrs_in_on_type_pattern() {}

#[builder(on)]
fn incomplete_on() {}

#[builder(on())]
fn incomplete_on2() {}

#[builder(on(_))]
fn incomplete_on3() {}

#[builder(on(_,))]
fn incomplete_on4() {}

#[builder(
    on(_, required),
    finish_fn = finish,
    on(String, into),
)]
fn non_consecutive_on1() {}

#[builder(
    start_fn = start,
    on(_, required),
    finish_fn = finish,
    on(String, into),
)]
fn non_consecutive_on2() {}

#[builder(
    start_fn = start,
    on(_, required),
    finish_fn = finish,
    on(String, into),
    builder_type = Builder,
)]
fn non_consecutive_on3() {}

#[builder(on(_, into), on(_, required))]
fn non_first_required() {}

#[builder(on(u8, required))]
fn non_wildcard_required() {}

fn main() {}
