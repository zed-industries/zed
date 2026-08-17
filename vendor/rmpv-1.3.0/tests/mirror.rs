#[macro_use]
extern crate quickcheck;

use rmpv::decode::read_value;
use rmpv::encode::write_value;
use rmpv::Value;

fn mirror_test<T: Clone>(xs: T) -> bool
    where Value: From<T>
{
    let mut buf = Vec::new();
    write_value(&mut buf, &Value::from(xs.clone())).unwrap();

    Value::from(xs) == read_value(&mut &buf[..]).unwrap()
}

quickcheck! {
    fn mirror_uint(xs: u64) -> bool {
        mirror_test(xs)
    }

    fn mirror_sint(xs: i64) -> bool {
        mirror_test(xs)
    }

    fn mirror_f32_value(xs: f32) -> bool {
        let mut buf = Vec::new();
        write_value(&mut buf, &Value::from(xs)).unwrap();
        let eq = Value::from(xs) == read_value(&mut &buf[..]).unwrap();

        eq || (!eq && xs.is_nan())
    }

    fn mirror_f64_value(xs: f64) -> bool {
        let mut buf = Vec::new();
        write_value(&mut buf, &Value::from(xs)).unwrap();
        let eq = Value::from(xs) == read_value(&mut &buf[..]).unwrap();

        eq || (!eq && xs.is_nan())
    }

    fn mirror_str(xs: String) -> bool {
        mirror_test(xs)
    }
}
