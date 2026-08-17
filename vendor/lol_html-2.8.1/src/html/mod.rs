use crate::base::Bytes;
use memchr::{memchr, memchr3};

#[macro_use]
mod tag;

mod local_name;
mod namespace;
mod text_type;

pub use self::local_name::{LocalName, LocalNameHash};
pub use self::namespace::Namespace;
pub use self::tag::Tag;
pub use self::text_type::TextType;

/// Convert text to HTML
#[inline]
pub(crate) fn escape_body_text(mut content: &str, output_handler: &mut impl FnMut(&str)) {
    loop {
        if let Some(pos) = memchr3(b'&', b'<', b'>', content.as_bytes()) {
            let Some((chunk_before, rest)) = content.split_at_checked(pos) else {
                return;
            };
            let Some((matched, rest)) = rest.split_at_checked(1) else {
                return;
            };

            content = rest;
            let matched = matched.as_bytes()[0];

            if !chunk_before.is_empty() {
                (output_handler)(chunk_before);
            }
            (output_handler)(match matched {
                b'<' => "&lt;",
                b'>' => "&gt;",
                _ => "&amp;",
            });
        } else {
            if !content.is_empty() {
                (output_handler)(content);
            }
            return;
        }
    }
}

/// Replace `"` with `&quot;` ONLY, leaving `&` unescaped
pub(crate) fn escape_double_quotes_only(content: Bytes<'_>, output_handler: &mut dyn FnMut(&[u8])) {
    let mut content = &*content;
    loop {
        if let Some(pos) = memchr(b'"', content) {
            let Some((chunk_before, rest)) = content
                .split_at_checked(pos)
                .and_then(|(before, rest)| Some((before, rest.get(1..)?)))
            else {
                return;
            };
            content = rest;

            if !chunk_before.is_empty() {
                (output_handler)(chunk_before);
            }
            (output_handler)(b"&quot;");
        } else {
            if !content.is_empty() {
                (output_handler)(content);
            }
            return;
        }
    }
}
