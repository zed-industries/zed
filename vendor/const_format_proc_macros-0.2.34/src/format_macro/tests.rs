use crate::{parse_utils::MyParse, test_utils::StrExt};

fn process_str(s: &str) -> Result<String, String> {
    MyParse::parse_token_stream_2(s.parse().unwrap())
        .and_then(crate::format_macro::formatcp_impl)
        .map(|x| x.to_string())
        .map_err(|e| e.to_compile_error().to_string())
}

macro_rules! assert_ret {
    ($call:expr, |$res:ident| $predicate:expr $(,)*) => {
        let $res = $call;
        let $res = $res.as_ref();
        assert!($predicate, "\nreturned:\n{:?}\n\n", $res);
    };
}

#[test]
fn named_argument_followed_by_positional() {
    assert_ret!(process_str(r#"(""), (a=()), () "#), |s| {
        s.unwrap_err()
            .consecutive_in_self(&["named arguments", "cannot", "positional arguments"])
    });
}

#[test]
fn access_formatter_error() {
    let cases = [
        r#"("{}"), (|f| 100u8) "#,
        r#"("{}"), (|_| 100u8) "#,
        r#"("{foo}"), (foo = |f| 100u8) "#,
        r#"("{foo}"), (foo = |_| 100u8) "#,
        r#"("{foo}"), (foo = (|f| 100u8)) "#,
        r#"("{foo}"), (foo = (|_| 100u8)) "#,
    ];

    for case in cases.iter().copied() {
        assert_ret!(process_str(case), |s| {
            s.unwrap_err().consecutive_in_self(&["custom formatting"])
        });
    }
}

#[test]
fn nonexistent_argument() {
    assert_ret!(process_str(r#"("{1}"), () "#), |s| {
        s.unwrap_err()
            .consecutive_in_self(&["positional argument", "1"])
    });

    assert_ret!(process_str(r#"("{}{}"), () "#), |s| {
        s.unwrap_err()
            .consecutive_in_self(&["positional argument", "1"])
    });
    process_str(r#"("{}"), () "#).unwrap();
}

#[test]
fn unused_arguments() {
    assert_ret!(process_str(r#"("{}"), (), () "#), |s| {
        let e = s.unwrap_err();
        e.consecutive_in_self(&["argument", "1", "unused"])
    });
    assert_ret!(process_str(r#"("{}"), (), () , () "#), |s| {
        let e = s.unwrap_err();
        e.consecutive_in_self(&["argument", "1", "unused"])
            && e.consecutive_in_self(&["argument", "2", "unused"])
    });

    assert_ret!(process_str(r#"("{}"), (), (foooo = "") "#), |s| {
        s.unwrap_err()
            .consecutive_in_self(&["foooo", "argument", "unused"])
    });

    assert_ret!(
        process_str(r#"("{}"), (), (), (foooo = ""), (bar = "") "#),
        |s| {
            let e = s.unwrap_err();
            e.consecutive_in_self(&["argument", "1", "unused"])
                && e.consecutive_in_self(&["foooo", "argument", "unused"])
                && e.consecutive_in_self(&["bar", "argument", "unused"])
        }
    );
}
