#![allow(clippy::enum_glob_use)]

use yaml_rust2::{scanner::TokenType::*, scanner::*};

macro_rules! next {
    ($p:ident, $tk:pat) => {{
        let tok = $p.next().unwrap();
        match tok.1 {
            $tk => {}
            _ => panic!("unexpected token: {:?}", tok),
        }
    }};
}

macro_rules! next_scalar {
    ($p:ident, $tk:expr, $v:expr) => {{
        let tok = $p.next().unwrap();
        match tok.1 {
            Scalar(style, ref v) => {
                assert_eq!(style, $tk);
                assert_eq!(v, $v);
            }
            _ => panic!("unexpected token: {:?}", tok),
        }
    }};
}

macro_rules! end {
    ($p:ident) => {{
        assert_eq!($p.next(), None);
    }};
}
/// test cases in libyaml scanner.c
#[test]
fn test_empty() {
    let s = "";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_scalar() {
    let s = "a scalar";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_explicit_scalar() {
    let s = "---
'a scalar'
...
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, DocumentStart);
    next!(p, Scalar(TScalarStyle::SingleQuoted, _));
    next!(p, DocumentEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_multiple_documents() {
    let s = "
'a scalar'
---
'a scalar'
---
'a scalar'
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, Scalar(TScalarStyle::SingleQuoted, _));
    next!(p, DocumentStart);
    next!(p, Scalar(TScalarStyle::SingleQuoted, _));
    next!(p, DocumentStart);
    next!(p, Scalar(TScalarStyle::SingleQuoted, _));
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_a_flow_sequence() {
    let s = "[item 1, item 2, item 3]";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, FlowSequenceStart);
    next_scalar!(p, TScalarStyle::Plain, "item 1");
    next!(p, FlowEntry);
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, FlowEntry);
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, FlowSequenceEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_a_flow_mapping() {
    let s = "
{
    a simple key: a value, # Note that the KEY token is produced.
    ? a complex key: another value,
}
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, FlowMappingStart);
    next!(p, Key);
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, Value);
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, FlowEntry);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "a complex key");
    next!(p, Value);
    next!(p, Scalar(TScalarStyle::Plain, _));
    next!(p, FlowEntry);
    next!(p, FlowMappingEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_block_sequences() {
    let s = "
- item 1
- item 2
-
  - item 3.1
  - item 3.2
-
  key 1: value 1
  key 2: value 2
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 2");
    next!(p, BlockEntry);
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 3.1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 3.2");
    next!(p, BlockEnd);
    next!(p, BlockEntry);
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 1");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 1");
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 2");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 2");
    next!(p, BlockEnd);
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_block_mappings() {
    let s = "
a simple key: a value   # The KEY token is produced here.
? a complex key
: another value
a mapping:
  key 1: value 1
  key 2: value 2
a sequence:
  - item 1
  - item 2
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, BlockMappingStart);
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value);
    next!(p, Scalar(_, _));
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value);
    next!(p, Scalar(_, _));
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value); // libyaml comment seems to be wrong
    next!(p, BlockMappingStart);
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value);
    next!(p, Scalar(_, _));
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value);
    next!(p, Scalar(_, _));
    next!(p, BlockEnd);
    next!(p, Key);
    next!(p, Scalar(_, _));
    next!(p, Value);
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next!(p, Scalar(_, _));
    next!(p, BlockEntry);
    next!(p, Scalar(_, _));
    next!(p, BlockEnd);
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_no_block_sequence_start() {
    let s = "
key:
- item 1
- item 2
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key");
    next!(p, Value);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 2");
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_collections_in_sequence() {
    let s = "
- - item 1
  - item 2
- key 1: value 1
  key 2: value 2
- ? complex key
  : complex value
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 2");
    next!(p, BlockEnd);
    next!(p, BlockEntry);
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 1");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 1");
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 2");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 2");
    next!(p, BlockEnd);
    next!(p, BlockEntry);
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "complex key");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "complex value");
    next!(p, BlockEnd);
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_collections_in_mapping() {
    let s = "
? a sequence
: - item 1
  - item 2
? a mapping
: key 1: value 1
  key 2: value 2
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "a sequence");
    next!(p, Value);
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "item 2");
    next!(p, BlockEnd);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "a mapping");
    next!(p, Value);
    next!(p, BlockMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 1");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 1");
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "key 2");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "value 2");
    next!(p, BlockEnd);
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_spec_ex7_3() {
    let s = "
{
    ? foo :,
    : bar,
}
";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, FlowMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "foo");
    next!(p, Value);
    next!(p, FlowEntry);
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "bar");
    next!(p, FlowEntry);
    next!(p, FlowMappingEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_plain_scalar_starting_with_indicators_in_flow() {
    // "Plain scalars must not begin with most indicators, as this would cause ambiguity with
    // other YAML constructs. However, the “:”, “?” and “-” indicators may be used as the first
    // character if followed by a non-space “safe” character, as this causes no ambiguity."

    let s = "{a: :b}";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, FlowMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "a");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, ":b");
    next!(p, FlowMappingEnd);
    next!(p, StreamEnd);
    end!(p);

    let s = "{a: ?b}";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, FlowMappingStart);
    next!(p, Key);
    next_scalar!(p, TScalarStyle::Plain, "a");
    next!(p, Value);
    next_scalar!(p, TScalarStyle::Plain, "?b");
    next!(p, FlowMappingEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_plain_scalar_starting_with_indicators_in_block() {
    let s = ":a";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next_scalar!(p, TScalarStyle::Plain, ":a");
    next!(p, StreamEnd);
    end!(p);

    let s = "?a";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next_scalar!(p, TScalarStyle::Plain, "?a");
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_plain_scalar_containing_indicators_in_block() {
    let s = "a:,b";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next_scalar!(p, TScalarStyle::Plain, "a:,b");
    next!(p, StreamEnd);
    end!(p);

    let s = ":,b";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next_scalar!(p, TScalarStyle::Plain, ":,b");
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_scanner_cr() {
    let s = "---\r\n- tok1\r\n- tok2";
    let mut p = Scanner::new(s.chars());
    next!(p, StreamStart(..));
    next!(p, DocumentStart);
    next!(p, BlockSequenceStart);
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "tok1");
    next!(p, BlockEntry);
    next_scalar!(p, TScalarStyle::Plain, "tok2");
    next!(p, BlockEnd);
    next!(p, StreamEnd);
    end!(p);
}

#[test]
fn test_uri() {
    // TODO
}

#[test]
fn test_uri_escapes() {
    // TODO
}
