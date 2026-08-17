//! Utility functions for working with identifiers.

use heck::{CamelCase, SnakeCase};

/// Converts a `camelCase` or `SCREAMING_SNAKE_CASE` identifier to a `lower_snake` case Rust field
/// identifier.
pub fn to_snake(s: &str) -> String {
    let mut ident = s.to_snake_case();

    // Use a raw identifier if the identifier matches a Rust keyword:
    // https://doc.rust-lang.org/reference/keywords.html.
    match ident.as_str() {
        // 2015 strict keywords.
        | "as" | "break" | "const" | "continue" | "else" | "enum" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
        | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "true"
        | "type" | "unsafe" | "use" | "where" | "while"
        // 2018 strict keywords.
        | "dyn"
        // 2015 reserved keywords.
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield"
        // 2018 reserved keywords.
        | "async" | "await" | "try" => ident.insert_str(0, "r#"),
        // the following keywords are not supported as raw identifiers and are therefore suffixed with an underscore.
        "self" | "super" | "extern" | "crate" => ident += "_",
        _ => (),
    }
    ident
}

/// Converts a `snake_case` identifier to an `UpperCamel` case Rust type identifier.
pub fn to_upper_camel(s: &str) -> String {
    let mut ident = s.to_camel_case();

    // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
    if ident == "Self" {
        ident += "_";
    }
    ident
}

#[cfg(test)]
mod tests {

    #![allow(clippy::cognitive_complexity)]

    use super::*;

    #[test]
    fn test_to_snake() {
        assert_eq!("foo_bar", &to_snake("FooBar"));
        assert_eq!("foo_bar_baz", &to_snake("FooBarBAZ"));
        assert_eq!("foo_bar_baz", &to_snake("FooBarBAZ"));
        assert_eq!("xml_http_request", &to_snake("XMLHttpRequest"));
        assert_eq!("r#while", &to_snake("While"));
        assert_eq!("fuzz_buster", &to_snake("FUZZ_BUSTER"));
        assert_eq!("foo_bar_baz", &to_snake("foo_bar_baz"));
        assert_eq!("fuzz_buster", &to_snake("FUZZ_buster"));
        assert_eq!("fuzz", &to_snake("_FUZZ"));
        assert_eq!("fuzz", &to_snake("_fuzz"));
        assert_eq!("fuzz", &to_snake("_Fuzz"));
        assert_eq!("fuzz", &to_snake("FUZZ_"));
        assert_eq!("fuzz", &to_snake("fuzz_"));
        assert_eq!("fuzz", &to_snake("Fuzz_"));
        assert_eq!("fuz_z", &to_snake("FuzZ_"));

        // From test_messages_proto3.proto.
        assert_eq!("fieldname1", &to_snake("fieldname1"));
        assert_eq!("field_name2", &to_snake("field_name2"));
        assert_eq!("field_name3", &to_snake("_field_name3"));
        assert_eq!("field_name4", &to_snake("field__name4_"));
        assert_eq!("field0name5", &to_snake("field0name5"));
        assert_eq!("field_0_name6", &to_snake("field_0_name6"));
        assert_eq!("field_name7", &to_snake("fieldName7"));
        assert_eq!("field_name8", &to_snake("FieldName8"));
        assert_eq!("field_name9", &to_snake("field_Name9"));
        assert_eq!("field_name10", &to_snake("Field_Name10"));

        // TODO(withoutboats/heck#3)
        //assert_eq!("field_name11", &to_snake("FIELD_NAME11"));
        assert_eq!("field_name12", &to_snake("FIELD_name12"));
        assert_eq!("field_name13", &to_snake("__field_name13"));
        assert_eq!("field_name14", &to_snake("__Field_name14"));
        assert_eq!("field_name15", &to_snake("field__name15"));
        assert_eq!("field_name16", &to_snake("field__Name16"));
        assert_eq!("field_name17", &to_snake("field_name17__"));
        assert_eq!("field_name18", &to_snake("Field_name18__"));
    }

    #[test]
    fn test_to_snake_raw_keyword() {
        assert_eq!("r#as", &to_snake("as"));
        assert_eq!("r#break", &to_snake("break"));
        assert_eq!("r#const", &to_snake("const"));
        assert_eq!("r#continue", &to_snake("continue"));
        assert_eq!("r#else", &to_snake("else"));
        assert_eq!("r#enum", &to_snake("enum"));
        assert_eq!("r#false", &to_snake("false"));
        assert_eq!("r#fn", &to_snake("fn"));
        assert_eq!("r#for", &to_snake("for"));
        assert_eq!("r#if", &to_snake("if"));
        assert_eq!("r#impl", &to_snake("impl"));
        assert_eq!("r#in", &to_snake("in"));
        assert_eq!("r#let", &to_snake("let"));
        assert_eq!("r#loop", &to_snake("loop"));
        assert_eq!("r#match", &to_snake("match"));
        assert_eq!("r#mod", &to_snake("mod"));
        assert_eq!("r#move", &to_snake("move"));
        assert_eq!("r#mut", &to_snake("mut"));
        assert_eq!("r#pub", &to_snake("pub"));
        assert_eq!("r#ref", &to_snake("ref"));
        assert_eq!("r#return", &to_snake("return"));
        assert_eq!("r#static", &to_snake("static"));
        assert_eq!("r#struct", &to_snake("struct"));
        assert_eq!("r#trait", &to_snake("trait"));
        assert_eq!("r#true", &to_snake("true"));
        assert_eq!("r#type", &to_snake("type"));
        assert_eq!("r#unsafe", &to_snake("unsafe"));
        assert_eq!("r#use", &to_snake("use"));
        assert_eq!("r#where", &to_snake("where"));
        assert_eq!("r#while", &to_snake("while"));
        assert_eq!("r#dyn", &to_snake("dyn"));
        assert_eq!("r#abstract", &to_snake("abstract"));
        assert_eq!("r#become", &to_snake("become"));
        assert_eq!("r#box", &to_snake("box"));
        assert_eq!("r#do", &to_snake("do"));
        assert_eq!("r#final", &to_snake("final"));
        assert_eq!("r#macro", &to_snake("macro"));
        assert_eq!("r#override", &to_snake("override"));
        assert_eq!("r#priv", &to_snake("priv"));
        assert_eq!("r#typeof", &to_snake("typeof"));
        assert_eq!("r#unsized", &to_snake("unsized"));
        assert_eq!("r#virtual", &to_snake("virtual"));
        assert_eq!("r#yield", &to_snake("yield"));
        assert_eq!("r#async", &to_snake("async"));
        assert_eq!("r#await", &to_snake("await"));
        assert_eq!("r#try", &to_snake("try"));
    }

    #[test]
    fn test_to_snake_non_raw_keyword() {
        assert_eq!("self_", &to_snake("self"));
        assert_eq!("super_", &to_snake("super"));
        assert_eq!("extern_", &to_snake("extern"));
        assert_eq!("crate_", &to_snake("crate"));
    }

    #[test]
    fn test_to_upper_camel() {
        assert_eq!("", &to_upper_camel(""));
        assert_eq!("F", &to_upper_camel("F"));
        assert_eq!("Foo", &to_upper_camel("FOO"));
        assert_eq!("FooBar", &to_upper_camel("FOO_BAR"));
        assert_eq!("FooBar", &to_upper_camel("_FOO_BAR"));
        assert_eq!("FooBar", &to_upper_camel("FOO_BAR_"));
        assert_eq!("FooBar", &to_upper_camel("_FOO_BAR_"));
        assert_eq!("FuzzBuster", &to_upper_camel("fuzzBuster"));
        assert_eq!("FuzzBuster", &to_upper_camel("FuzzBuster"));
        assert_eq!("Self_", &to_upper_camel("self"));
    }
}
