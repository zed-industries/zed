use serde_derive::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod common;

use crate::common::{deserializes_to, serializes_to, Val};

#[test]
fn serializes_example_infinite() {
    let mut map = HashMap::new();
    map.insert("inf".to_string(), Val::Number(f64::INFINITY));
    serializes_to(Val::Object(map), "{\"inf\":Infinity}");

    serializes_to(json!({ "inf": f64::INFINITY }), "{\"inf\":null}");

    let mut map2 = HashMap::new();
    map2.insert("neg_inf".to_string(), Val::Number(f64::NEG_INFINITY));
    serializes_to(Val::Object(map2), "{\"neg_inf\":-Infinity}");

    serializes_to(
        json!({ "neg_inf": f64::NEG_INFINITY }),
        "{\"neg_inf\":null}",
    );
}

#[test]
fn deserializes_example_infinite() {
    let mut contents = String::new();
    File::open("tests/assets/infinite.json5")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    #[derive(Deserialize, PartialEq, Debug)]
    struct InfiniteF64 {
        inf: f64,
        neg_inf: f64,
    }

    let expected_f64 = InfiniteF64 {
        inf: f64::INFINITY,
        neg_inf: f64::NEG_INFINITY,
    };

    deserializes_to(&contents, expected_f64);

    #[derive(Deserialize, PartialEq, Debug)]
    struct InfiniteF32 {
        inf: f32,
        neg_inf: f32,
    }
    let expected_f32 = InfiniteF32 {
        inf: f32::INFINITY,
        neg_inf: f32::NEG_INFINITY,
    };

    deserializes_to(&contents, expected_f32);

    let mut map = HashMap::new();
    map.insert("inf".to_owned(), Val::Number(f64::INFINITY));
    map.insert("neg_inf".to_owned(), Val::Number(f64::NEG_INFINITY));
    deserializes_to::<Val>(&contents, Val::Object(map));

    deserializes_to::<Value>(
        &contents,
        json!({
            "inf": null,
            "neg_inf": null
        }),
    )
}

#[test]
fn serializes_example_nan() {
    let mut map = HashMap::new();
    map.insert("nan".to_string(), Val::Number(f64::NAN));
    serializes_to(Val::Object(map), "{\"nan\":NaN}");

    serializes_to(json!({ "nan": f64::NAN }), "{\"nan\":null}");

    let mut map2 = HashMap::new();
    map2.insert("neg_nan".to_string(), Val::Number(f64::NAN));
    serializes_to(Val::Object(map2), "{\"neg_nan\":NaN}");

    serializes_to(json!({ "neg_nan": f64::NAN }), "{\"neg_nan\":null}");
}

#[test]
fn deserializes_example_nan() {
    let mut contents = String::new();
    File::open("tests/assets/nan.json5")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    #[derive(Deserialize, PartialEq, Debug)]
    struct NanF64 {
        nan: f64,
        neg_nan: f64,
    }

    match json5::from_str::<NanF64>(&contents) {
        Ok(value) => assert!(value.nan.is_nan() && value.neg_nan.is_nan()),
        Err(err) => panic!(format!("{}", err)),
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct NanF32 {
        nan: f32,
        neg_nan: f32,
    }

    match json5::from_str::<NanF32>(&contents) {
        Ok(value) => assert!(value.nan.is_nan() && value.neg_nan.is_nan()),
        Err(err) => panic!(format!("{}", err)),
    }

    let mut map = HashMap::new();
    map.insert("nan".to_string(), Val::Number(f64::NAN));
    map.insert("neg_nan".to_string(), Val::Number(f64::NAN));
    match json5::from_str::<Val>(&contents) {
        Ok(value) => match value {
            Val::Object(v) => {
                let nan = match v.get(&"nan".to_string()).unwrap() {
                    Val::Number(n) => n,
                    _ => panic!("not NaN"),
                };
                let neg_nan = match v.get(&"neg_nan".to_string()).unwrap() {
                    Val::Number(n) => n,
                    _ => panic!("not NaN"),
                };
                assert!(nan.is_nan() && neg_nan.is_nan())
            }
            _ => panic!("not NaN"),
        },
        Err(err) => panic!(format!("{}", err)),
    }

    deserializes_to::<Value>(
        &contents,
        json!({
            "nan": null,
            "neg_nan": null
        }),
    )
}
