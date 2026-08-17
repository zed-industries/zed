use heck::ToUpperCamelCase;
use quote::format_ident;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Meta, MetaNameValue};

pub(crate) fn field_not_ignored(field: &Field) -> bool {
    for attr in field.attrs.iter() {
        if let Some(ident) = attr.path().get_ident() {
            if ident != "sea_orm" {
                continue;
            }
        } else {
            continue;
        }

        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
            for meta in list.iter() {
                if let Meta::Path(path) = meta {
                    if let Some(name) = path.get_ident() {
                        if name == "ignore" {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

pub(crate) fn format_field_ident(field: Field) -> Ident {
    format_ident!("{}", field.ident.unwrap().to_string())
}

pub(crate) fn trim_starting_raw_identifier<T>(string: T) -> String
where
    T: ToString,
{
    string
        .to_string()
        .trim_start_matches(RAW_IDENTIFIER)
        .to_string()
}

pub(crate) fn escape_rust_keyword<T>(string: T) -> String
where
    T: ToString,
{
    let string = string.to_string();
    if RUST_KEYWORDS.iter().any(|s| s.eq(&string)) {
        format!("r#{string}")
    } else if RUST_SPECIAL_KEYWORDS.iter().any(|s| s.eq(&string)) {
        format!("{string}_")
    } else {
        string
    }
}

/// Turn a string to PascalCase while escaping all special characters in ASCII words.
///
/// (camel_case is used here to match naming of heck.)
///
/// In ActiveEnum, string_value will be PascalCased and made
/// an identifier in {Enum}Variant.
///
/// However Rust only allows for XID_Start char followed by
/// XID_Continue characters as identifiers; this causes a few
/// problems:
///
/// - `string_value = ""` will cause a panic;
/// - `string_value` containing only non-alphanumerics will become `""`
///   and cause the above panic;
/// - `string_values`:
///      - `"A B"`
///      - `"A  B"`
///      - `"A_B"`
///      - `"A_ B"`
///
/// All shares the same identifier of `"AB"`;
///
/// This function does the PascelCase conversion with a few special escapes:
/// - Non-Unicode Standard Annex #31 compliant characters will converted to their hex notation;
/// - `"_"` into `"0x5F"`;
/// - `" "` into `"0x20"`;
/// - Empty strings will become special keyword of `"__Empty"`
///
/// Note that this does NOT address:
///
/// - case-sensitivity. String value "ABC" and "abc" remains
///   conflicted after .camel_case().
///
/// Example Conversions:
///
/// ```ignore
/// assert_eq!(camel_case_with_escaped_non_uax31(""), "__Empty");
/// assert_eq!(camel_case_with_escaped_non_uax31(" "), "_0x20");
/// assert_eq!(camel_case_with_escaped_non_uax31("  "), "_0x200x20");
/// assert_eq!(camel_case_with_escaped_non_uax31("_"), "_0x5F");
/// assert_eq!(camel_case_with_escaped_non_uax31("foobar"), "Foobar");
/// assert_eq!(camel_case_with_escaped_non_uax31("foo bar"), "Foo0x20bar");
/// ```
pub(crate) fn camel_case_with_escaped_non_uax31<T>(string: T) -> String
where
    T: ToString,
{
    let additional_chars_to_replace: [char; 2] = ['_', ' '];

    let mut rebuilt = string
        .to_string()
        .chars()
        .enumerate()
        .map(|(pos, char_)| {
            if !additional_chars_to_replace.contains(&char_)
                && match pos {
                    0 => unicode_ident::is_xid_start(char_),
                    _ => unicode_ident::is_xid_continue(char_),
                }
            {
                char_.to_string()
            } else {
                format!("{:#X}", char_ as u32)
            }
        })
        .reduce(
            // Join the "characters" (now strings)
            // back together
            |lhs, rhs| lhs + rhs.as_str(),
        )
        .map_or(
            // if string_value is ""
            // Make sure the default does NOT go through camel_case,
            // as the __ will be removed! The underscores are
            // what guarantees this being special case avoiding
            // all potential conflicts.
            String::from("__Empty"),
            |s| s.to_upper_camel_case(),
        );

    if rebuilt
        .chars()
        .next()
        .map(char::is_numeric)
        .unwrap_or(false)
    {
        rebuilt = String::from("_") + &rebuilt;
    }

    rebuilt
}

pub(crate) const RAW_IDENTIFIER: &str = "r#";

pub(crate) const RUST_KEYWORDS: [&str; 49] = [
    "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use",
    "where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

pub(crate) const RUST_SPECIAL_KEYWORDS: [&str; 3] = ["crate", "Self", "self"];

pub(crate) trait GetMeta {
    fn exists(&self, k: &str) -> bool;
    fn get_as_kv(&self, k: &str) -> Option<String>;
}

impl GetMeta for Meta {
    fn exists(&self, key: &str) -> bool {
        let Meta::Path(path) = self else {
            return false;
        };
        path.is_ident(key)
    }

    fn get_as_kv(&self, key: &str) -> Option<String> {
        let Meta::NameValue(MetaNameValue {
            path,
            value: syn::Expr::Lit(exprlit),
            ..
        }) = self
        else {
            return None;
        };

        let syn::Lit::Str(litstr) = &exprlit.lit else {
            return None;
        };

        if path.is_ident(key) {
            Some(litstr.value())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_uax31_escape() {
        // Test empty string
        assert_eq!(camel_case_with_escaped_non_uax31(""), "__Empty");

        // Test additional_chars_to_replace (to_camel_case related characters)
        assert_eq!(camel_case_with_escaped_non_uax31(" "), "_0x20");

        // Test additional_chars_to_replace (multiples. ensure distinct from single)
        assert_eq!(camel_case_with_escaped_non_uax31("  "), "_0x200x20");

        // Test additional_chars_to_replace (udnerscores)
        assert_eq!(camel_case_with_escaped_non_uax31("_"), "_0x5F");

        // Test typical use case
        assert_eq!(camel_case_with_escaped_non_uax31("foobar"), "Foobar");

        // Test spaced words distinct from non-spaced
        assert_eq!(camel_case_with_escaped_non_uax31("foo bar"), "Foo0x20bar");

        // Test underscored words distinct from non-spaced and spaced
        assert_eq!(camel_case_with_escaped_non_uax31("foo_bar"), "Foo0x5Fbar");

        // Test leading numeric characters
        assert_eq!(camel_case_with_escaped_non_uax31("1"), "_0x31");

        // Test escaping also works on full string following lead numeric character
        // This was previously a fail condition.
        assert_eq!(
            camel_case_with_escaped_non_uax31("1 2 3"),
            "_0x310x2020x203"
        );
    }
}
