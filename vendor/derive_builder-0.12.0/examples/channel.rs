#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use std::convert::From;

#[derive(PartialEq, Default, Debug, Clone)]
struct Uuid(i32);
#[derive(PartialEq, Default, Debug, Clone)]
struct Authentication(i32);

impl From<i32> for Uuid {
    fn from(x: i32) -> Uuid {
        Uuid(x)
    }
}

impl From<i32> for Authentication {
    fn from(x: i32) -> Authentication {
        Authentication(x)
    }
}

#[derive(Debug, Default, Builder)]
#[builder(setter(into))]
struct Channel {
    id: Uuid,
    token: Authentication,
    special_info: i32,
}

fn main() {
    let ch = ChannelBuilder::default()
        .special_info(42)
        .id(0)
        .token(5_494_192)
        .build()
        .unwrap();
    println!("{:?}", ch);
}
