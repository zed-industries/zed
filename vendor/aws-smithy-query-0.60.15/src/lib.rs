/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    // missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

//! Abstractions for the Smithy AWS Query protocol

use aws_smithy_types::date_time::{DateTimeFormatError, Format};
use aws_smithy_types::primitive::Encoder;
use aws_smithy_types::{DateTime, Number};
use std::borrow::Cow;
use std::fmt::Write;
use urlencoding::encode;

pub struct QueryWriter<'a> {
    output: &'a mut String,
}

impl<'a> QueryWriter<'a> {
    pub fn new(output: &'a mut String, action: &str, version: &str) -> Self {
        output.push_str("Action=");
        output.push_str(&encode(action));
        output.push_str("&Version=");
        output.push_str(&encode(version));
        QueryWriter { output }
    }

    pub fn prefix(&mut self, prefix: &'a str) -> QueryValueWriter<'_> {
        QueryValueWriter::new(self.output, Cow::Borrowed(prefix))
    }

    pub fn finish(self) {
        // Calling this drops self
    }
}

#[must_use]
pub struct QueryMapWriter<'a> {
    output: &'a mut String,
    prefix: Cow<'a, str>,
    flatten: bool,
    key_name: &'static str,
    value_name: &'static str,
    next_index: usize,
}

impl<'a> QueryMapWriter<'a> {
    fn new(
        output: &'a mut String,
        prefix: Cow<'a, str>,
        flatten: bool,
        key_name: &'static str,
        value_name: &'static str,
    ) -> QueryMapWriter<'a> {
        QueryMapWriter {
            prefix,
            output,
            flatten,
            key_name,
            value_name,
            next_index: 1,
        }
    }

    pub fn entry(&mut self, key: &str) -> QueryValueWriter<'_> {
        let entry = if self.flatten { "" } else { ".entry" };
        write!(
            &mut self.output,
            "&{}{}.{}.{}={}",
            self.prefix,
            entry,
            self.next_index,
            self.key_name,
            encode(key)
        )
        // The `Write` implementation for `String` is infallible,
        // see https://doc.rust-lang.org/src/alloc/string.rs.html#2815
        .unwrap();
        let value_name = format!(
            "{}{}.{}.{}",
            self.prefix, entry, self.next_index, self.value_name
        );

        self.next_index += 1;
        QueryValueWriter::new(self.output, Cow::Owned(value_name))
    }

    pub fn finish(self) {
        // Calling this drops self
    }
}

#[must_use]
pub struct QueryListWriter<'a> {
    output: &'a mut String,
    prefix: Cow<'a, str>,
    flatten: bool,
    member_override: Option<&'a str>,
    next_index: usize,
}

impl<'a> QueryListWriter<'a> {
    fn new(
        output: &'a mut String,
        prefix: Cow<'a, str>,
        flatten: bool,
        member_override: Option<&'a str>,
    ) -> QueryListWriter<'a> {
        QueryListWriter {
            prefix,
            output,
            flatten,
            member_override,
            next_index: 1,
        }
    }

    pub fn entry(&mut self) -> QueryValueWriter<'_> {
        let value_name = if self.flatten {
            format!("{}.{}", self.prefix, self.next_index)
        } else if self.member_override.is_some() {
            format!(
                "{}.{}.{}",
                self.prefix,
                self.member_override.unwrap(),
                self.next_index
            )
        } else {
            format!("{}.member.{}", self.prefix, self.next_index)
        };

        self.next_index += 1;
        QueryValueWriter::new(self.output, Cow::Owned(value_name))
    }

    pub fn finish(self) {
        // https://github.com/awslabs/smithy/commit/715b1d94ab14764ad43496b016b0c2e85bcf1d1f
        // If the list was empty, just serialize the parameter name
        if self.next_index == 1 {
            QueryValueWriter::new(self.output, self.prefix).write_param_name();
        }
    }
}

#[must_use]
pub struct QueryValueWriter<'a> {
    output: &'a mut String,
    prefix: Cow<'a, str>,
}

impl<'a> QueryValueWriter<'a> {
    pub fn new(output: &'a mut String, prefix: Cow<'a, str>) -> QueryValueWriter<'a> {
        QueryValueWriter { output, prefix }
    }

    /// Starts a new prefix.
    pub fn prefix(&mut self, prefix: &'a str) -> QueryValueWriter<'_> {
        QueryValueWriter::new(
            self.output,
            Cow::Owned(format!("{}.{}", self.prefix, prefix)),
        )
    }

    /// Writes the boolean `value`.
    pub fn boolean(mut self, value: bool) {
        self.write_param_name();
        self.output.push_str(match value {
            true => "true",
            _ => "false",
        });
    }

    /// Writes a string `value`.
    pub fn string(mut self, value: &str) {
        self.write_param_name();
        self.output.push_str(&encode(value));
    }

    /// Writes a number `value`.
    pub fn number(self, value: Number) {
        match value {
            Number::PosInt(value) => {
                // itoa::Buffer is a fixed-size stack allocation, so this is cheap
                self.string(Encoder::from(value).encode());
            }
            Number::NegInt(value) => {
                self.string(Encoder::from(value).encode());
            }
            Number::Float(value) => self.string(Encoder::from(value).encode()),
        }
    }

    /// Writes a date-time `value` with the given `format`.
    pub fn date_time(
        self,
        date_time: &DateTime,
        format: Format,
    ) -> Result<(), DateTimeFormatError> {
        self.string(&date_time.fmt(format)?);
        Ok(())
    }

    /// Starts a map.
    pub fn start_map(
        self,
        flat: bool,
        key_name: &'static str,
        value_name: &'static str,
    ) -> QueryMapWriter<'a> {
        QueryMapWriter::new(self.output, self.prefix, flat, key_name, value_name)
    }

    /// Starts a list.
    pub fn start_list(self, flat: bool, member_override: Option<&'a str>) -> QueryListWriter<'a> {
        QueryListWriter::new(self.output, self.prefix, flat, member_override)
    }

    fn write_param_name(&mut self) {
        self.output.push('&');
        self.output.push_str(&self.prefix);
        self.output.push('=');
    }
}

#[cfg(test)]
mod tests {
    use crate::QueryWriter;
    use aws_smithy_types::date_time::Format;
    use aws_smithy_types::{DateTime, Number};

    #[test]
    fn no_params() {
        let mut out = String::new();
        let writer = QueryWriter::new(&mut out, "SomeAction", "1.0");
        writer.finish();
        assert_eq!("Action=SomeAction&Version=1.0", out);
    }

    #[test]
    fn query_list_writer_empty_list() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");
        writer.prefix("myList").start_list(false, None).finish();
        writer.finish();
        assert_eq!("Action=SomeAction&Version=1.0&myList=", out);
    }

    #[test]
    fn maps() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        let mut map = writer.prefix("MapArg").start_map(false, "key", "value");
        map.entry("bar").string("Bar");
        map.entry("foo").string("Foo");
        map.finish();

        let mut map = writer
            .prefix("Some.Flattened")
            .start_map(true, "key", "value");
        map.entry("bar").string("Bar");
        map.entry("foo").string("Foo");
        map.finish();

        let mut map = writer.prefix("RenamedKVs").start_map(false, "K", "V");
        map.entry("bar").string("Bar");
        map.finish();

        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &MapArg.entry.1.key=bar\
            &MapArg.entry.1.value=Bar\
            &MapArg.entry.2.key=foo\
            &MapArg.entry.2.value=Foo\
            &Some.Flattened.1.key=bar\
            &Some.Flattened.1.value=Bar\
            &Some.Flattened.2.key=foo\
            &Some.Flattened.2.value=Foo\
            &RenamedKVs.entry.1.K=bar\
            &RenamedKVs.entry.1.V=Bar\
            ",
            out
        );
    }

    #[test]
    fn lists() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        let mut list = writer.prefix("ListArg").start_list(false, None);
        list.entry().string("foo");
        list.entry().string("bar");
        list.entry().string("baz");
        list.finish();

        let mut list = writer.prefix("FlattenedListArg").start_list(true, None);
        list.entry().string("A");
        list.entry().string("B");
        list.finish();

        let mut list = writer.prefix("ItemList").start_list(false, Some("item"));
        list.entry().string("foo");
        list.entry().string("bar");
        list.finish();

        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &ListArg.member.1=foo\
            &ListArg.member.2=bar\
            &ListArg.member.3=baz\
            &FlattenedListArg.1=A\
            &FlattenedListArg.2=B\
            &ItemList.item.1=foo\
            &ItemList.item.2=bar\
            ",
            out
        );
    }

    #[test]
    fn prefixes() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        let mut first = writer.prefix("first");
        let second = first.prefix("second");
        second.string("second_val");
        first.string("first_val");

        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &first.second=second_val\
            &first=first_val\
            ",
            out
        );
    }

    #[test]
    fn timestamps() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        writer
            .prefix("epoch_seconds")
            .date_time(&DateTime::from_secs_f64(5.2), Format::EpochSeconds)
            .unwrap();
        writer
            .prefix("date_time")
            .date_time(
                &DateTime::from_str("2021-05-24T15:34:50.123Z", Format::DateTime).unwrap(),
                Format::DateTime,
            )
            .unwrap();
        writer
            .prefix("http_date")
            .date_time(
                &DateTime::from_str("Wed, 21 Oct 2015 07:28:00 GMT", Format::HttpDate).unwrap(),
                Format::HttpDate,
            )
            .unwrap();
        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &epoch_seconds=5.2\
            &date_time=2021-05-24T15%3A34%3A50.123Z\
            &http_date=Wed%2C%2021%20Oct%202015%2007%3A28%3A00%20GMT\
            ",
            out
        );
    }

    #[test]
    fn numbers() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        writer.prefix("PosInt").number(Number::PosInt(5));
        writer.prefix("NegInt").number(Number::NegInt(-5));
        writer
            .prefix("Infinity")
            .number(Number::Float(f64::INFINITY));
        writer
            .prefix("NegInfinity")
            .number(Number::Float(f64::NEG_INFINITY));
        writer.prefix("NaN").number(Number::Float(f64::NAN));
        writer.prefix("Floating").number(Number::Float(5.2));
        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &PosInt=5\
            &NegInt=-5\
            &Infinity=Infinity\
            &NegInfinity=-Infinity\
            &NaN=NaN\
            &Floating=5.2\
            ",
            out
        );
    }

    #[test]
    fn booleans() {
        let mut out = String::new();
        let mut writer = QueryWriter::new(&mut out, "SomeAction", "1.0");

        writer.prefix("IsTrue").boolean(true);
        writer.prefix("IsFalse").boolean(false);
        writer.finish();

        assert_eq!(
            "Action=SomeAction\
            &Version=1.0\
            &IsTrue=true\
            &IsFalse=false\
            ",
            out
        );
    }

    #[test]
    fn action_version_escaping() {
        let mut out = String::new();
        QueryWriter::new(&mut out, "Some Action", "1 2").finish();
        assert_eq!("Action=Some%20Action&Version=1%202", out);
    }
}
