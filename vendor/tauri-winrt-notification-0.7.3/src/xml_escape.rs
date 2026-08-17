// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Copied from https://github.com/tafia/quick-xml/blob/master/src/escape.rs

use std::{borrow::Cow, fmt::Write};

pub fn escape<'a>(raw: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    let escape_chars = |ch| matches!(ch, b'<' | b'>' | b'&' | b'\'' | b'\"');
    let raw = raw.into();
    let bytes = raw.as_bytes();
    let mut escaped = None;
    let mut iter = bytes.iter();
    let mut pos = 0;
    while let Some(i) = iter.position(|&b| escape_chars(b)) {
        if escaped.is_none() {
            escaped = Some(String::with_capacity(raw.len()));
        }
        let escaped = escaped.as_mut().expect("initialized");
        let new_pos = pos + i;
        // SAFETY: It should fail only on OOM
        escape_char(escaped, &raw, pos, new_pos).unwrap();
        pos = new_pos + 1;
    }

    if let Some(mut escaped) = escaped {
        if let Some(raw) = raw.get(pos..) {
            // SAFETY: It should fail only on OOM
            escaped.write_str(raw).unwrap();
        }
        Cow::Owned(escaped)
    } else {
        raw
    }
}

fn escape_char<W>(writer: &mut W, value: &str, from: usize, to: usize) -> std::fmt::Result
where
    W: Write,
{
    writer.write_str(&value[from..to])?;
    match value.as_bytes()[to] {
        b'<' => writer.write_str("&lt;")?,
        b'>' => writer.write_str("&gt;")?,
        b'\'' => writer.write_str("&apos;")?,
        b'&' => writer.write_str("&amp;")?,
        b'"' => writer.write_str("&quot;")?,

        // This set of escapes handles characters that should be escaped
        // in elements of xs:lists, because those characters works as
        // delimiters of list elements
        b'\t' => writer.write_str("&#9;")?,
        b'\n' => writer.write_str("&#10;")?,
        b'\r' => writer.write_str("&#13;")?,
        b' ' => writer.write_str("&#32;")?,
        _ => unreachable!("Only '<', '>','\', '&', '\"', '\\t', '\\r', '\\n', and ' ' are escaped"),
    }
    Ok(())
}
