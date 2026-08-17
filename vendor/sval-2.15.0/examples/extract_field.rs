#[macro_use]
extern crate sval_derive_macros;

pub fn get_i32<'sval>(field: &str, value: impl sval::Value) -> Option<i32> {
    struct Extract<'a> {
        depth: usize,
        field: &'a str,
        matched_field: bool,
        extracted: Option<i32>,
    }

    impl<'a, 'sval> sval::Stream<'sval> for Extract<'a> {
        fn record_value_begin(
            &mut self,
            _: Option<&sval::Tag>,
            label: &sval::Label,
        ) -> sval::Result {
            self.matched_field = label.as_str() == self.field;
            Ok(())
        }

        fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
            Ok(())
        }

        fn i64(&mut self, v: i64) -> sval::Result {
            if self.matched_field {
                self.extracted = v.try_into().ok();
            }

            Ok(())
        }

        fn null(&mut self) -> sval::Result {
            Ok(())
        }

        fn bool(&mut self, _: bool) -> sval::Result {
            Ok(())
        }

        fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
            Ok(())
        }

        fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
            Ok(())
        }

        fn text_end(&mut self) -> sval::Result {
            Ok(())
        }

        fn f64(&mut self, _: f64) -> sval::Result {
            Ok(())
        }

        fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
            self.depth += 1;

            Ok(())
        }

        fn seq_value_begin(&mut self) -> sval::Result {
            Ok(())
        }

        fn seq_value_end(&mut self) -> sval::Result {
            Ok(())
        }

        fn seq_end(&mut self) -> sval::Result {
            self.depth -= 1;

            Ok(())
        }
    }

    let mut stream = Extract {
        depth: 0,
        field,
        matched_field: false,
        extracted: None,
    };

    sval::stream(&mut stream, &value).ok()?;
    stream.extracted
}

#[derive(Value)]
struct MyData {
    field_0: bool,
    field_1: i32,
}

fn main() {
    let data = MyData {
        field_0: true,
        field_1: 42,
    };

    assert_eq!(Some(42), get_i32("field_1", data));
}
