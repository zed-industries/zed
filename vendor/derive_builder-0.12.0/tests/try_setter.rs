#[macro_use]
extern crate derive_builder;

use std::convert::TryFrom;
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub struct MyAddr(IpAddr);

impl From<IpAddr> for MyAddr {
    fn from(v: IpAddr) -> Self {
        MyAddr(v)
    }
}

impl<'a> TryFrom<&'a str> for MyAddr {
    type Error = AddrParseError;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        Ok(MyAddr(v.parse()?))
    }
}

#[derive(Debug, PartialEq, Builder)]
#[builder(try_setter, setter(into))]
struct Lorem {
    pub source: MyAddr,
    pub dest: MyAddr,
}

#[derive(Debug, PartialEq, Builder)]
#[builder(try_setter, setter(into, prefix = "set"))]
struct Ipsum {
    pub source: MyAddr,
}

fn exact_helper() -> Result<Lorem, LoremBuilderError> {
    LoremBuilder::default()
        .source(IpAddr::from_str("1.2.3.4").unwrap())
        .dest(IpAddr::from_str("0.0.0.0").unwrap())
        .build()
}

fn try_helper() -> Result<Lorem, LoremBuilderError> {
    LoremBuilder::default()
        .try_source("1.2.3.4")
        .map_err(|e| e.to_string())?
        .try_dest("0.0.0.0")
        .map_err(|e| e.to_string())?
        .build()
}

#[test]
fn infallible_set() {
    let _ = LoremBuilder::default()
        .source(IpAddr::from_str("1.2.3.4").unwrap())
        .dest(IpAddr::from_str("0.0.0.0").unwrap())
        .build();
}

#[test]
fn fallible_set() {
    let mut builder = LoremBuilder::default();
    let try_result = builder.try_source("1.2.3.4");
    let built = try_result
        .expect("Passed well-formed address")
        .dest(IpAddr::from_str("0.0.0.0").unwrap())
        .build()
        .unwrap();
    assert_eq!(built, exact_helper().unwrap());
}

#[test]
fn with_helper() {
    assert_eq!(exact_helper().unwrap(), try_helper().unwrap());
}

#[test]
fn renamed() {
    IpsumBuilder::default()
        .try_set_source("0.0.0.0")
        .unwrap()
        .build()
        .expect("All fields were provided");
}
