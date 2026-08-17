/*!
Low-level value formatting.
*/

use core::fmt::{self, Write};

use crate::tags;

/**
A token-aware [`fmt::Write`].

This trait can be used to customize the way various tokens are written, such
as colorizing numbers and booleans differently.
 */
pub trait TokenWrite: Write {
    /**
    Write a token fragment.
    */
    fn write_token<T: fmt::Display>(&mut self, tag: &sval::Tag, token: T) -> fmt::Result {
        let _ = tag;
        self.write_fmt(format_args!("{}", token))
    }

    /**
    Write a number.
    */
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u16(&mut self, value: u16) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u32(&mut self, value: u32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u64(&mut self, value: u64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u128(&mut self, value: u128) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i8(&mut self, value: i8) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i16(&mut self, value: i16) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i32(&mut self, value: i32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i64(&mut self, value: i64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i128(&mut self, value: i128) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_f32(&mut self, value: f32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_f64(&mut self, value: f64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        self.write_token(&tags::NUMBER, num)
    }

    /**
    Write null or unit.
    */
    fn write_null(&mut self) -> fmt::Result {
        self.write_atom("()")
    }

    /**
    Write a boolean.
    */
    fn write_bool(&mut self, value: bool) -> fmt::Result {
        self.write_atom(value)
    }

    /**
    Write an atom, like `true` or `()`.
    */
    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        self.write_token(&tags::ATOM, atom)
    }

    /**
    Write a type name.
    */
    fn write_type(&mut self, ty: &str) -> fmt::Result {
        self.write_ident(ty)
    }

    /**
    Write a field name.
    */
    fn write_field(&mut self, field: &str) -> fmt::Result {
        self.write_ident(field)
    }

    /**
    Write an identifier.
    */
    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        self.write_token(&tags::IDENT, ident)
    }

    /**
    Write a fragment of punctuation, like `:` or `,`.
    */
    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        self.write_token(&tags::PUNCT, punct)
    }

    /**
    Write whitespace.
    */
    fn write_ws(&mut self, ws: &str) -> fmt::Result {
        self.write_token(&tags::WS, ws)
    }

    /**
    Write an opening or closing quote.

    By default, a double quote (`"`) is used.
    */
    fn write_text_quote(&mut self) -> fmt::Result {
        self.write_token(&tags::TEXT, "\"")
    }

    /**
    Write a fragment of text.
    */
    fn write_text(&mut self, text: &str) -> fmt::Result {
        write_escape_debug(text, |text| self.write_token(&tags::TEXT, text))
    }

    /**
    Write the start of a map.
    */
    fn write_map_begin(&mut self) -> fmt::Result {
        self.write_punct("{")
    }

    /**
    Write a separator between a map value and the next key.
    */
    fn write_map_key_begin(&mut self, is_first: bool) -> fmt::Result {
        if !is_first {
            self.write_punct(",")?;
        }

        self.write_ws(" ")
    }

    /**
    Write a separator between a map key and its value.
    */
    fn write_map_value_begin(&mut self, is_first: bool) -> fmt::Result {
        let _ = is_first;

        self.write_punct(":")?;
        self.write_ws(" ")
    }

    /**
    Write the end of a map.
    */
    fn write_map_end(&mut self, is_empty: bool) -> fmt::Result {
        if !is_empty {
            self.write_ws(" ")?;
        }

        self.write_punct("}")
    }

    /**
    Write the type of a record.
    */
    fn write_record_type(&mut self, ty: &str) -> fmt::Result {
        self.write_type(ty)?;
        self.write_ws(" ")
    }

    /**
    Write the start of a record.
    */
    fn write_record_begin(&mut self) -> fmt::Result {
        self.write_punct("{")
    }

    /**
    Write a record field.
    */
    fn write_record_value_begin(&mut self, field: &str, is_first: bool) -> fmt::Result {
        if !is_first {
            self.write_punct(",")?;
        }

        self.write_ws(" ")?;

        self.write_field(field)?;

        self.write_punct(":")?;
        self.write_ws(" ")
    }

    /**
    Write the end of a record.
    */
    fn write_record_end(&mut self, is_empty: bool) -> fmt::Result {
        if !is_empty {
            self.write_ws(" ")?;
        }

        self.write_punct("}")
    }

    /**
    Write the start of a sequence.
    */
    fn write_seq_begin(&mut self) -> fmt::Result {
        self.write_punct("[")
    }

    /**
    Write a separator between sequence elements.
    */
    fn write_seq_value_begin(&mut self, is_first: bool) -> fmt::Result {
        if !is_first {
            self.write_punct(",")?;
            self.write_ws(" ")?;
        }

        Ok(())
    }

    /**
    Write the end of a sequence.
    */
    fn write_seq_end(&mut self, is_empty: bool) -> fmt::Result {
        let _ = is_empty;

        self.write_punct("]")
    }

    /**
    Write the type of a tuple.
    */
    fn write_tuple_type(&mut self, ty: &str) -> fmt::Result {
        self.write_type(ty)
    }

    /**
    Write the start of a tuple.
    */
    fn write_tuple_begin(&mut self) -> fmt::Result {
        self.write_punct("(")
    }

    /**
    Write a separator between tuple values.
    */
    fn write_tuple_value_begin(&mut self, is_first: bool) -> fmt::Result {
        if !is_first {
            self.write_punct(",")?;
            self.write_ws(" ")?;
        }

        Ok(())
    }

    /**
    Write the end of a tuple.
    */
    fn write_tuple_end(&mut self, is_empty: bool) -> fmt::Result {
        let _ = is_empty;

        self.write_punct(")")
    }
}

fn write_escape_debug(
    input: impl fmt::Display,
    output: impl FnMut(&str) -> fmt::Result,
) -> fmt::Result {
    struct Writer<F>(F);

    impl<F: FnMut(&str) -> fmt::Result> fmt::Write for Writer<F> {
        fn write_str(&mut self, input: &str) -> fmt::Result {
            let mut from = 0;

            // Iterate over each character, escaping it if necessary
            for (i, c) in input.char_indices() {
                let esc = c.escape_debug();

                // A character is escaped if it produces more than an
                // escape sequence with more than a single character in it
                if esc.len() > 1 {
                    let flush = &input[from..i];
                    if flush.len() > 0 {
                        (self.0)(flush)?;
                    }

                    let mut buf = [0; 4];
                    for c in esc {
                        (self.0)(c.encode_utf8(&mut buf))?;
                    }

                    // Skip over the original character without writing it
                    from = i + c.len_utf8();
                }
            }

            let flush = &input[from..];
            if flush.len() > 0 {
                (self.0)(flush)
            } else {
                Ok(())
            }
        }
    }

    write!(Writer(output), "{}", input)
}

impl<'a, W: TokenWrite + ?Sized> TokenWrite for &'a mut W {
    fn write_token<T: fmt::Display>(&mut self, tag: &sval::Tag, token: T) -> fmt::Result {
        (**self).write_token(tag, token)
    }

    fn write_u8(&mut self, value: u8) -> fmt::Result {
        (**self).write_u8(value)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        (**self).write_u16(value)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        (**self).write_u32(value)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        (**self).write_u64(value)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        (**self).write_u128(value)
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        (**self).write_i8(value)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        (**self).write_i16(value)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        (**self).write_i32(value)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        (**self).write_i64(value)
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        (**self).write_i128(value)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        (**self).write_f32(value)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        (**self).write_f64(value)
    }

    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        (**self).write_number(num)
    }

    fn write_null(&mut self) -> fmt::Result {
        (**self).write_null()
    }

    fn write_bool(&mut self, value: bool) -> fmt::Result {
        (**self).write_bool(value)
    }

    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        (**self).write_atom(atom)
    }

    fn write_type(&mut self, ty: &str) -> fmt::Result {
        (**self).write_type(ty)
    }

    fn write_field(&mut self, field: &str) -> fmt::Result {
        (**self).write_field(field)
    }

    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        (**self).write_ident(ident)
    }

    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        (**self).write_punct(punct)
    }

    fn write_ws(&mut self, ws: &str) -> fmt::Result {
        (**self).write_ws(ws)
    }

    fn write_text_quote(&mut self) -> fmt::Result {
        (**self).write_text_quote()
    }

    fn write_text(&mut self, text: &str) -> fmt::Result {
        (**self).write_text(text)
    }

    fn write_map_begin(&mut self) -> fmt::Result {
        (**self).write_map_begin()
    }

    fn write_map_key_begin(&mut self, is_first: bool) -> fmt::Result {
        (**self).write_map_key_begin(is_first)
    }

    fn write_map_value_begin(&mut self, is_first: bool) -> fmt::Result {
        (**self).write_map_value_begin(is_first)
    }

    fn write_map_end(&mut self, is_empty: bool) -> fmt::Result {
        (**self).write_map_end(is_empty)
    }

    fn write_record_type(&mut self, ty: &str) -> fmt::Result {
        (**self).write_record_type(ty)
    }

    fn write_record_begin(&mut self) -> fmt::Result {
        (**self).write_record_begin()
    }

    fn write_record_value_begin(&mut self, field: &str, is_first: bool) -> fmt::Result {
        (**self).write_record_value_begin(field, is_first)
    }

    fn write_record_end(&mut self, is_empty: bool) -> fmt::Result {
        (**self).write_record_end(is_empty)
    }

    fn write_seq_begin(&mut self) -> fmt::Result {
        (**self).write_seq_begin()
    }

    fn write_seq_value_begin(&mut self, is_first: bool) -> fmt::Result {
        (**self).write_seq_value_begin(is_first)
    }

    fn write_seq_end(&mut self, is_empty: bool) -> fmt::Result {
        (**self).write_seq_end(is_empty)
    }

    fn write_tuple_type(&mut self, ty: &str) -> fmt::Result {
        (**self).write_tuple_type(ty)
    }

    fn write_tuple_begin(&mut self) -> fmt::Result {
        (**self).write_tuple_begin()
    }

    fn write_tuple_value_begin(&mut self, is_first: bool) -> fmt::Result {
        (**self).write_tuple_value_begin(is_first)
    }

    fn write_tuple_end(&mut self, is_empty: bool) -> fmt::Result {
        (**self).write_tuple_end(is_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::string::String;

    #[test]
    fn escape_debug() {
        let cases = [
            ("hello", r#"hello"#),
            ("\\", r#"\\"#),
            ("\r", r#"\r"#),
            ("\n", r#"\n"#),
            ("\t", r#"\t"#),
            ("\"", r#"\""#),
            ("'", r#"\'"#),
            ("⛰️", r#"⛰\u{fe0f}"#),
        ];

        for (ai, ae) in cases {
            for (bi, be) in cases {
                let mut expected = String::new();
                expected.push_str(ae);
                expected.push_str(be);

                let mut actual = String::new();
                write_escape_debug(ai, |i| Ok(actual.push_str(i))).unwrap();
                write_escape_debug(bi, |i| Ok(actual.push_str(i))).unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}
