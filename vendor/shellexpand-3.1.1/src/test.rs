
use super::*;

/// Convert a `str` to an `Xstr`, for use with expected results in test cases
fn s(s: &str) -> &Xstr { s.as_ref() }

/// Convert a `str` to a `OString`, for use with expected variable lookup errors
fn mk_var_name(vn: &str) -> OString { s(vn).into_winput().as_wstr().to_ostring() }

#[cfg(feature = "tilde")]
mod tilde_tests {
    use super::*;

    #[test]
    fn test_with_tilde_no_hd() {
        fn hd() -> Option<XString> {
            None
        }

        assert_eq!(tilde_with_context("whatever", hd), s("whatever"));
        assert_eq!(tilde_with_context("whatever/~", hd), s("whatever/~"));
        assert_eq!(tilde_with_context("~/whatever", hd), s("~/whatever"));
        assert_eq!(tilde_with_context("~", hd), s("~"));
        assert_eq!(tilde_with_context("~something", hd), s("~something"));
    }

    #[test]
    fn test_with_tilde() {
        fn hd() -> Option<XString> {
            Some("/home/dir".into())
        }

        assert_eq!(tilde_with_context("whatever/path", hd), s("whatever/path"));
        assert_eq!(tilde_with_context("whatever/~/path", hd), s("whatever/~/path"));
        assert_eq!(tilde_with_context("~", hd), s("/home/dir"));
        assert_eq!(tilde_with_context("~/path", hd), s("/home/dir/path"));
        assert_eq!(tilde_with_context("~whatever/path", hd), s("~whatever/path"));
    }

    #[test]
    fn test_global_tilde() {
        match dirs::home_dir() {
            Some(hd) => assert_eq!(tilde("~/something"), s(&format!("{}/something", hd.display()))),
            None => assert_eq!(tilde("~/something"), s("~/something")),
        }
    }
}

mod env_test {
    use super::*;

    macro_rules! table {
        ($env:expr, unwrap, $($source:expr => $target:expr),+) => {
            $(
                assert_eq!(env_with_context($source, $env).unwrap(), s($target));
            )+
        };
        ($env:expr, error, $($source:expr => $name:expr),+) => {
            $(
                assert_eq!(env_with_context($source, $env), Err(LookupError {
                    var_name: mk_var_name($name),
                    cause: ()
                }));
            )+
        }
    }

    #[test]
    fn test_empty_env() {
        fn e(_: &str) -> Result<Option<String>, ()> {
            Ok(None)
        }

        table! { e, unwrap,
            "whatever/path"        => "whatever/path",
            "$VAR/whatever/path"   => "$VAR/whatever/path",
            "whatever/$VAR/path"   => "whatever/$VAR/path",
            "whatever/path/$VAR"   => "whatever/path/$VAR",
            "${VAR}/whatever/path" => "${VAR}/whatever/path",
            "whatever/${VAR}path"  => "whatever/${VAR}path",
            "whatever/path/${VAR}" => "whatever/path/${VAR}",
            "${}/whatever/path"    => "${}/whatever/path",
            "whatever/${}path"     => "whatever/${}path",
            "whatever/path/${}"    => "whatever/path/${}",
            "$/whatever/path"      => "$/whatever/path",
            "whatever/$path"       => "whatever/$path",
            "whatever/path/$"      => "whatever/path/$",
            "$$/whatever/path"     => "$/whatever/path",
            "whatever/$$path"      => "whatever/$path",
            "whatever/path/$$"     => "whatever/path/$",
            "$A$B$C"               => "$A$B$C",
            "$A_B_C"               => "$A_B_C"
        };
    }

    #[test]
    fn test_error_env() {
        fn e(_: &str) -> Result<Option<String>, ()> {
            Err(())
        }

        table! { e, unwrap,
            "whatever/path" => "whatever/path",
            // check that escaped $ does nothing
            "whatever/$/path" => "whatever/$/path",
            "whatever/path$" => "whatever/path$",
            "whatever/$$path" => "whatever/$path"
        };

        table! { e, error,
            "$VAR/something" => "VAR",
            "${VAR}/something" => "VAR",
            "whatever/${VAR}/something" => "VAR",
            "whatever/${VAR}" => "VAR",
            "whatever/$VAR/something" => "VAR",
            "whatever/$VARsomething" => "VARsomething",
            "whatever/$VAR" => "VAR",
            "whatever/$VAR_VAR_VAR" => "VAR_VAR_VAR"
        };
    }

    #[test]
    fn test_regular_env() {
        fn e(s: &str) -> Result<Option<&'static str>, ()> {
            match s {
                "VAR" => Ok(Some("value")),
                "a_b" => Ok(Some("X_Y")),
                "EMPTY" => Ok(Some("")),
                "ERR" => Err(()),
                _ => Ok(None),
            }
        }

        table! { e, unwrap,
            // no variables
            "whatever/path" => "whatever/path",

            // empty string
            "" => "",

            // existing variable without braces in various positions
            "$VAR/whatever/path" => "value/whatever/path",
            "whatever/$VAR/path" => "whatever/value/path",
            "whatever/path/$VAR" => "whatever/path/value",
            "whatever/$VARpath" => "whatever/$VARpath",
            "$VAR$VAR/whatever" => "valuevalue/whatever",
            "/whatever$VAR$VAR" => "/whatevervaluevalue",
            "$VAR $VAR" => "value value",
            "$a_b" => "X_Y",
            "$a_b$VAR" => "X_Yvalue",

            // existing variable with braces in various positions
            "${VAR}/whatever/path" => "value/whatever/path",
            "whatever/${VAR}/path" => "whatever/value/path",
            "whatever/path/${VAR}" => "whatever/path/value",
            "whatever/${VAR}path" => "whatever/valuepath",
            "${VAR}${VAR}/whatever" => "valuevalue/whatever",
            "/whatever${VAR}${VAR}" => "/whatevervaluevalue",
            "${VAR} ${VAR}" => "value value",
            "${VAR}$VAR" => "valuevalue",

            // default values
            "/answer/${UNKNOWN:-42}" => "/answer/42",
            "/answer/${:-42}" => "/answer/${:-42}",
            "/whatever/${UNKNOWN:-other}$VAR" => "/whatever/othervalue",
            "/whatever/${UNKNOWN:-other}/$VAR" => "/whatever/other/value",
            ":-/whatever/${UNKNOWN:-other}/$VAR :-" => ":-/whatever/other/value :-",
            "/whatever/${VAR:-other}" => "/whatever/value",
            "/whatever/${VAR:-other}$VAR" => "/whatever/valuevalue",
            "/whatever/${VAR} :-" => "/whatever/value :-",
            "/whatever/${:-}" => "/whatever/${:-}",
            "/whatever/${UNKNOWN:-}" => "/whatever/",

            // empty variable in various positions
            "${EMPTY}/whatever/path" => "/whatever/path",
            "whatever/${EMPTY}/path" => "whatever//path",
            "whatever/path/${EMPTY}" => "whatever/path/"
        };

        table! { e, error,
            "$ERR" => "ERR",
            "${ERR}" => "ERR"
        };
    }

    #[test]
    fn test_unicode() {
        fn e(s: &str) -> Result<Option<&'static str>, ()> {
            match s {
                "élan" => Ok(Some("ἐκθυμία")),
                _ => Ok(None),
            }
        }

        table! { e, unwrap,
            "plain" => "plain",
            "with $élan lacking" => "with ἐκθυμία lacking",
            "with ${élan} enclosed" => "with ἐκθυμία enclosed"
        };
    }

    #[test]
    fn test_global_env() {
        match std::env::var("PATH") {
            Ok(value) => assert_eq!(env("x/$PATH/x").unwrap(), s(&format!("x/{}/x", value))),
            Err(e) => assert_eq!(
                env("x/$PATH/x"),
                Err(LookupError {
                    var_name: mk_var_name("PATH"),
                    cause: e
                })
            ),
        }
        match std::env::var("SOMETHING_DEFINITELY_NONEXISTING") {
            Ok(value) => assert_eq!(
                env("x/$SOMETHING_DEFINITELY_NONEXISTING/x").unwrap(),
                s(&format!("x/{}/x", value))
            ),
            Err(e) => assert_eq!(
                env("x/$SOMETHING_DEFINITELY_NONEXISTING/x"),
                Err(LookupError {
                    var_name: mk_var_name("SOMETHING_DEFINITELY_NONEXISTING"),
                    cause: e
                })
            ),
        }
    }
}

mod full_tests {
    use super::*;

    #[test]
    fn test_quirks() {
        fn hd() -> Option<XString> {
            Some("$VAR".into())
        }
        fn env(s: &str) -> Result<Option<&'static str>, ()> {
            match s {
                "VAR" => Ok(Some("value")),
                "SVAR" => Ok(Some("/value")),
                "TILDE" => Ok(Some("~")),
                _ => Ok(None),
            }
        }

        // any variable-like sequence in ~ expansion should not trigger variable expansion
        assert_eq!(
            full_with_context("~/something/$VAR", hd, env).unwrap(),
            s("$VAR/something/value")
        );

        // variable just after tilde should be substituted first and trigger regular tilde
        // expansion
        assert_eq!(full_with_context("~$VAR", hd, env).unwrap(), s("~value"));
        assert_eq!(full_with_context("~$SVAR", hd, env).unwrap(), s("$VAR/value"));

        // variable expanded into a tilde in the beginning should not trigger tilde expansion
        assert_eq!(
            full_with_context("$TILDE/whatever", hd, env).unwrap(),
            s("~/whatever")
        );
        assert_eq!(
            full_with_context("${TILDE}whatever", hd, env).unwrap(),
            s("~whatever")
        );
        assert_eq!(full_with_context("$TILDE", hd, env).unwrap(), s("~"));
    }

    #[test]
    fn test_tilde_expansion() {
        fn hd() -> Option<String> {
            Some("/home/user".into())
        }

        assert_eq!(
            tilde_with_context("~/some/dir", hd),
            s("/home/user/some/dir")
        );
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn test_tilde_expansion_windows() {
        fn home_dir() -> Option<PathBuf> {
            Some(Path::new("C:\\users\\public").into())
        }

        assert_eq!(
            tilde_with_context("~\\some\\dir", home_dir),
            "C:\\users\\public\\some\\dir"
        );
    }
}
