#![cfg(feature = "owned")]
#![feature(test)]

extern crate test;

use value_bag::ValueBag;

#[bench]
fn u8_to_owned(b: &mut test::Bencher) {
    let bag = ValueBag::from(1u8);

    b.iter(|| bag.to_owned());
}

#[bench]
fn str_to_owned(b: &mut test::Bencher) {
    let bag = ValueBag::from("a string");

    b.iter(|| bag.to_owned());
}

#[bench]
fn str_to_owned_clone(b: &mut test::Bencher) {
    let bag = ValueBag::from("a string").to_owned();

    b.iter(|| bag.clone());
}

#[bench]
fn str_to_shared(b: &mut test::Bencher) {
    let bag = ValueBag::from("a string");

    b.iter(|| bag.to_shared());
}

#[bench]
fn str_to_shared_clone(b: &mut test::Bencher) {
    let bag = ValueBag::from("a string").to_shared();

    b.iter(|| bag.clone());
}

#[bench]
fn display_to_owned(b: &mut test::Bencher) {
    let bag = ValueBag::from_display(&42);

    b.iter(|| bag.to_owned());
}

#[cfg(feature = "serde1")]
#[bench]
fn serde1_to_owned(b: &mut test::Bencher) {
    use value_bag_serde1::lib::ser::{Serialize, SerializeStruct, Serializer};

    pub struct MyData<'a> {
        a: i32,
        b: &'a str,
    }

    impl<'a> Serialize for MyData<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut serializer = serializer.serialize_struct("MyData", 2)?;

            serializer.serialize_field("a", &self.a)?;
            serializer.serialize_field("b", &self.b)?;

            serializer.end()
        }
    }

    let bag = ValueBag::from_serde1(&MyData {
        a: 42,
        b: "a string",
    });

    b.iter(|| bag.to_owned());
}

#[cfg(feature = "sval2")]
#[bench]
fn sval2_to_owned(b: &mut test::Bencher) {
    use value_bag_sval2::lib::{Label, Result, Stream, Value};

    pub struct MyData<'a> {
        a: i32,
        b: &'a str,
    }

    impl<'a> Value for MyData<'a> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.record_begin(None, Some(&Label::new("MyData")), None, Some(2))?;

            stream.record_value_begin(None, &Label::new("a"))?;
            stream.value(&self.a)?;
            stream.record_value_end(None, &Label::new("a"))?;

            stream.record_value_begin(None, &Label::new("b"))?;
            stream.value(&self.b)?;
            stream.record_value_end(None, &Label::new("b"))?;

            stream.record_end(None, Some(&Label::new("MyData")), None)
        }
    }

    let bag = ValueBag::from_sval2(&MyData {
        a: 42,
        b: "a string",
    });

    b.iter(|| bag.to_owned());
}
