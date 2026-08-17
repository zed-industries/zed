/*!
Values are Rust structures that represent a single instance of some datatype.

This example implements the `Value` trait automatically using Rust's `#[derive]` attribute.
*/

#[macro_use]
extern crate sval_derive_macros;

pub mod stream;

#[derive(Value)]
pub struct MyRecord<'a> {
    id: u64,
    title: &'a str,
}

#[derive(Value)]
pub struct MyTuple<'a>(u64, &'a str);

fn main() -> sval::Result {
    stream(MyRecord {
        id: 547,
        title: "Some data",
    })?;

    stream(MyTuple(547, "Some data"))?;

    Ok(())
}

fn stream(v: impl sval::Value) -> sval::Result {
    v.stream(&mut stream::simple::MyStream)?;
    println!();

    Ok(())
}
