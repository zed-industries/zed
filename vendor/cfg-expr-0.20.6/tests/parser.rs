use cfg_expr::{
    Expression, ParseError,
    error::Reason,
    expr::{Predicate as P, TargetPredicate as TP},
    targets::*,
};

macro_rules! test_validate {
    (ok [$($text:expr => [$($expected:expr),*$(,)?]),+$(,)?]) => {
        $(
            let val_expr = Expression::parse($text).unwrap();
            let mut preds = val_expr.predicates().enumerate();

            $(
                let actual = preds.next().unwrap();

                similar_asserts::assert_eq!($expected, actual.1, "failed @ index {}", actual.0);
            )*

            if let Some((_, additional)) = preds.next() {
                assert!(false, "found additional requirement {:?}", additional);
            }
        )+
    };
}

macro_rules! err {
    ($text:expr => $reason:ident @ $range:expr) => {
        let act_err = Expression::parse($text).unwrap_err();

        let expected = ParseError {
            original: $text.to_owned(),
            span: $range,
            reason: Reason::$reason,
        };

        similar_asserts::assert_eq!(expected, act_err);
    };

    ($text:expr => $unexpected:expr; $range:expr) => {
        let act_err = Expression::parse($text).unwrap_err();

        let expected = ParseError {
            original: $text.to_owned(),
            span: $range,
            reason: Reason::Unexpected($unexpected),
        };

        similar_asserts::assert_eq!(expected, act_err);
    };
}

#[test]
fn fails_empty() {
    err!("" => Empty @ 0..0);
    err!(" " => Empty @ 0..1);
    err!("\n\t\n" => Empty @ 0..3);
}

#[test]
fn fails_malformed() {
    err!("," => &["<key>", "all", "any", "not"]; 0..1);
    // Keys can't begin with a number
    err!("8" => &["<key>", "all", "any", "not"]; 0..1);
    err!("=" => &["<key>", "all", "any", "not"]; 0..1);
    err!("(" => &["<key>", "all", "any", "not"]; 0..1);
    err!("key =" => &["\"<value>\""]; 5..5);
    err!("key1, key2" => MultipleRootPredicates @ 0..10);
    err!("key1, key2,     " => MultipleRootPredicates @ 0..16);
    err!("key1 = \"v\", key2" => MultipleRootPredicates @ 0..16);
}

#[test]
fn fails_unbalanced_parens() {
    err!("not(key" => UnclosedParens @ 3..7);
    err!("key)" => UnopenedParens @ 3..4);
    err!("foo (" => &["=", ",", ")"]; 4..5);
}

#[test]
fn fails_unbalanced_quotes() {
    err!("key = \"value" => UnclosedQuotes @ 6..12);
    err!("key = \"" => UnclosedQuotes @ 6..7);
    err!("key = \"value, key = \"value\"" => &[",", ")"]; 21..26);
    err!("all(key = \"value), key = \"value\"" => &[",", ")"]; 26..31);
    err!("not(key = \"value)" => UnclosedQuotes @ 10..17);
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn handles_single_predicate() {
    test_validate!(ok [
        "cfg(key)" => [P::Flag("key")],
        "unix"  => [P::Target(TP::Family(Family::unix))],
        "target_arch = \"mips\"" => [P::Target(TP::Arch(Arch::mips))],
        "feature = \"awesome\"" => [P::Feature("awesome")],
        "_key" => [P::Flag("_key")],
        " key" => [P::Flag("key")],
        " key  " => [P::Flag("key")],
        " key  = \"val\"" => [P::KeyValue{ key: "key", val: "val" }],
        "key=\"\"" => [P::KeyValue{ key: "key", val: "" }],
        " key=\"7\"       " => [P::KeyValue{ key: "key", val: "7" }],
        "key = \"7 q\" " => [P::KeyValue{ key: "key", val: "7 q" }],
        "target_has_atomic = \"ptr\"" => [P::Target(TP::HasAtomic(HasAtomic::Pointer))],
        "target_has_atomic = \"4\"" => [P::Target(TP::HasAtomic(HasAtomic::IntegerSize(4)))],
        "target_has_atomic = \"64\"" => [P::Target(TP::HasAtomic(HasAtomic::IntegerSize(64)))],
        "target_has_atomic = \"128\" " => [P::Target(TP::HasAtomic(HasAtomic::IntegerSize(128)))],
        "panic = \"unwind\"" => [P::Target(TP::Panic(Panic("unwind".into())))],
        "panic = \"abort\"" => [P::Target(TP::Panic(Panic("abort".into())))],
    ]);
}

#[test]
fn handles_simple_funcs() {
    test_validate!(ok [
        "any()" => [],
        "all()"  => [],
        "not(key = \"value\")" => [P::KeyValue { key: "key", val: "value" }],
    ]);
}

#[test]
fn fails_invalid_funcs() {
    err!("nope()" => &["=", ",", ")"]; 4..5);
    err!("all(nope())" => &["=", ",", ")"]; 8..9);
    err!("any(,)" => &["<key>", ")", "all", "any", "not"]; 4..5);
    err!("blah(key)" => &["=", ",", ")"]; 4..5);
}

#[test]
fn ensures_not_has_one_predicate() {
    assert_eq!(
        Expression::parse("not()").unwrap_err(),
        ParseError {
            original: "not()".to_owned(),
            span: 0..5,
            reason: Reason::InvalidNot(0),
        }
    );

    assert_eq!(
        Expression::parse("not(key_one, key_two)").unwrap_err(),
        ParseError {
            original: "not(key_one, key_two)".to_owned(),
            span: 0..21,
            reason: Reason::InvalidNot(2),
        }
    );

    assert_eq!(
        Expression::parse("any(not(not(key_one, key_two)))").unwrap_err(),
        ParseError {
            original: "any(not(not(key_one, key_two)))".to_owned(),
            span: 8..29,
            reason: Reason::InvalidNot(2),
        }
    );

    test_validate!(ok [
        "not(key)" => [P::Flag("key")],
        "not(key,)" => [P::Flag("key")],
        "not(key = \"value\")" => [P::KeyValue { key: "key", val: "value" }],
        "not(key = \"value\",)" => [P::KeyValue { key: "key", val: "value" }],
        "not(not(not(key = \"value\",)))" => [P::KeyValue { key: "key", val: "value" }],
    ]);
}
