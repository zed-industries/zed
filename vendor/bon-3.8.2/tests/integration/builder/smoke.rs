use crate::prelude::*;

#[cfg(feature = "alloc")]
#[test]
fn smoke_fn() {
    /// Function-level docs
    /// multiline.
    #[builder]
    fn sut(
        /// ### Documentation
        /// **Docs** for arg1.
        ///
        /// Multiline with `code` *examples* __even__!
        ///
        /// ```
        /// let wow_such_code = true;
        /// println!("Code is so lovely! {wow_such_code}");
        /// ```
        ///
        /// - List item 1
        /// - List item 2
        arg1: bool,

        /// Docs for arg2
        arg2: &'_ str,
        arg3: String,
        arg4: u32,

        /// Docs on optional parameter
        arg5: Option<u32>,
        arg6: Option<&str>,
        arg7: Vec<String>,
        arg8: (u32, &[bool]),
    ) -> String {
        drop((arg1, arg2, arg4, arg5, arg6, arg7, arg8));
        arg3
    }

    let actual = sut()
        .arg1(true)
        .arg2("arg2")
        .arg3("arg3".to_owned())
        .arg4(1)
        .arg7(vec!["arg7".to_owned()])
        .arg8((1, &[true]))
        .call();

    assert_eq!(actual, "arg3");
}

#[test]
fn smoke_struct() {
    /// Docs on struct itself.
    /// Multiline.
    #[allow(dead_code)]
    #[derive(Debug, Builder)]
    pub(crate) struct Sut<'a> {
        /// Docs on bool field.
        /// Multiline.
        bool: bool,

        str_ref: &'a str,

        #[builder(default)]
        u32: u32,

        /// Docs on option field.
        /// Multiline.
        option_u32: Option<u32>,

        option_str_ref: Option<&'a str>,
        tuple: (u32, &'a [bool]),

        #[builder(skip)]
        skipped: u32,
    }

    let actual = Sut::builder()
        .bool(true)
        .str_ref("str_ref")
        .maybe_option_u32(Some(42))
        .option_str_ref("value")
        .tuple((42, &[true, false]))
        .build();

    let expected = expect![[r#"
        Sut {
            bool: true,
            str_ref: "str_ref",
            u32: 0,
            option_u32: Some(
                42,
            ),
            option_str_ref: Some(
                "value",
            ),
            tuple: (
                42,
                [
                    true,
                    false,
                ],
            ),
            skipped: 0,
        }
    "#]];

    expected.assert_debug_eq(&actual);
}
