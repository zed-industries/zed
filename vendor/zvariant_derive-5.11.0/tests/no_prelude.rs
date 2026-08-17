#![no_implicit_prelude]
#![allow(dead_code)]

use ::serde::{Deserialize, Serialize};
use ::zvariant_derive::Type;

use ::zvariant::as_value::{self, optional};

#[derive(Type)]
struct FooF(f64);

#[derive(Type)]
struct TestStruct {
    name: ::std::string::String,
    age: u8,
    blob: ::std::vec::Vec<u8>,
}

#[repr(u32)]
#[derive(Type)]
enum RequestNameFlags {
    AllowReplacement = 0x01,
    ReplaceExisting = 0x02,
    DoNotQueue = 0x04,
}

#[derive(Serialize, Deserialize, Type)]
#[zvariant(signature = "a{sv}")]
#[serde(deny_unknown_fields)]
struct Test {
    #[serde(
        with = "optional",
        skip_serializing_if = "::std::option::Option::is_none",
        default
    )]
    field_a: ::std::option::Option<u32>,
    #[serde(rename = "field-b")]
    #[serde(with = "as_value")]
    field_b: ::std::string::String,
    #[serde(with = "as_value")]
    field_c: ::std::vec::Vec<u8>,
}
