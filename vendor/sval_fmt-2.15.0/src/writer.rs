use crate::{tags, TokenWrite};
use core::fmt::{self, Display, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_number: bool,
    out: W,
}

impl<'a> TokenWrite for fmt::Formatter<'a> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        value.fmt(self)
    }
}

pub(crate) struct GenericWriter<W>(pub W);

impl<W: Write> Write for GenericWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args)
    }
}

impl<W: Write> TokenWrite for GenericWriter<W> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
    }
}

impl<W> Writer<W> {
    pub fn new(out: W) -> Self {
        Writer {
            is_current_depth_empty: true,
            is_number: false,
            out,
        }
    }
}

impl<'sval, W: TokenWrite> sval::Stream<'sval> for Writer<W> {
    fn null(&mut self) -> sval::Result {
        self.out.write_null().map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.out.write_bool(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_number {
            Ok(())
        } else {
            self.out.write_text_quote().map_err(|_| sval::Error::new())
        }
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        if self.is_number {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())
        } else {
            self.out
                .write_text(fragment)
                .map_err(|_| sval::Error::new())
        }
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_number {
            Ok(())
        } else {
            self.out.write_text_quote().map_err(|_| sval::Error::new())
        }
    }

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.seq_begin(num_bytes)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        for b in fragment {
            self.seq_value_begin()?;
            self.u8(*b)?;
            self.seq_value_end()?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.out.write_u8(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.out.write_u16(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.out.write_u32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.out.write_u64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.out.write_u128(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.out.write_i8(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.out.write_i16(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.out.write_i32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.out.write_i64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.out.write_i128(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.out.write_f32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.out.write_f64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        self.out.write_map_begin().map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.out
            .write_map_key_begin(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.out
            .write_map_value_begin(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.out
            .write_map_end(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        self.out.write_seq_begin().map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.out
            .write_seq_value_begin(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.out
            .write_seq_end(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn enum_begin(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_current_depth_empty = true;

        Ok(())
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        if self.is_current_depth_empty {
            self.tag(tag, label, index)?;
        }

        Ok(())
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_current_depth_empty = false;

        if tag == Some(&tags::NUMBER) {
            self.is_number = true;
        }

        if let Some(label) = label {
            self.out
                .write_tuple_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
            self.out
                .write_tuple_begin()
                .map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if tag == Some(&tags::NUMBER) {
            self.is_number = false;
        }

        if label.is_some() {
            self.out
                .write_tuple_end(false)
                .map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn tag(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_current_depth_empty = false;

        if let Some(label) = label {
            self.out
                .write_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
        } else {
            self.null()?;
        }

        Ok(())
    }

    fn record_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: Option<usize>,
    ) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        if let Some(label) = label {
            self.out
                .write_record_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
        }

        self.out
            .write_record_begin()
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.out
            .write_record_value_begin(label.as_str(), self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn record_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.out
            .write_record_end(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn tuple_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: Option<usize>,
    ) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        if let Some(label) = label {
            self.out
                .write_tuple_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
        }

        self.out
            .write_tuple_begin()
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn tuple_value_begin(&mut self, _: Option<&sval::Tag>, _: &sval::Index) -> sval::Result {
        self.out
            .write_tuple_value_begin(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn tuple_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Index) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn tuple_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.out
            .write_tuple_end(self.is_current_depth_empty)
            .map_err(|_| sval::Error::new())?;

        Ok(())
    }
}
