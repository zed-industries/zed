#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

struct Unit;

type Clone = Unit;
type Into = Unit;
type Option = Unit;
type Result = Unit;
type Some = Unit;
type String = Unit;

impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "()")
    }
}

impl core::fmt::Display for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "()")
    }
}

#[derive(Builder)]
struct IgnoreEmptyStruct {}

fn main() { }
