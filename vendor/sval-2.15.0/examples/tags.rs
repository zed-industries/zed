/*!
Tags can be used to define new data types for streams to interpret differently.

Some tags are built-in to `sval`, but libraries can define their own.

This example defines a tag for Unix timestamps, where a 64bit number is interpreted
as seconds since the Unix Epoch.
*/

#[macro_use]
extern crate sval_derive_macros;

pub const UNIX_TIMESTAMP: sval::Tag = sval::Tag::new("unixts");

#[derive(Value)]
#[sval(tag = "UNIX_TIMESTAMP")]
pub struct Timestamp(u64);

fn main() -> sval::Result {
    stream(Timestamp(1675154443))?;

    Ok(())
}

// This is the same stream as the `stream_simple` case, except with an extra
// `u64` method that handles our timestamps
pub struct MyStream {
    is_unix_ts: bool,
}

impl<'sval> sval::Stream<'sval> for MyStream {
    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if let Some(&UNIX_TIMESTAMP) = tag {
            self.is_unix_ts = true;
        }

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if let Some(&UNIX_TIMESTAMP) = tag {
            self.is_unix_ts = false;
        }

        Ok(())
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        // If the value is tagged as a Unix timestamp then print it using a human-readable RFC3339 format
        if self.is_unix_ts {
            print!(
                "{}",
                humantime::format_rfc3339(
                    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(v)
                )
            );
        } else {
            print!("{}", v);
        }

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        print!("null");
        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        print!("{}", v);

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        print!("{}", fragment.escape_debug());

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("[ ");
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        print!(", ");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        print!("]");
        Ok(())
    }
}

fn stream(v: impl sval::Value) -> sval::Result {
    v.stream(&mut MyStream { is_unix_ts: false })?;
    println!();

    Ok(())
}
