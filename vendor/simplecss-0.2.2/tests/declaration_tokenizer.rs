// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Declaration Tokenizer

use simplecss::*;

macro_rules! tokenize {
    ($name:ident, $text:expr, $( $token:expr ),*) => (
        #[test]
        fn $name() {
            let mut t = DeclarationTokenizer::from($text);
            $(
                assert_eq!(t.next().unwrap(), $token);
            )*

            assert!(t.next().is_none());
        }
    )
}

fn declare<'a>(name: &'a str, value: &'a str) -> Declaration<'a> {
    Declaration {
        name,
        value,
        important: false,
    }
}

fn declare_important<'a>(name: &'a str, value: &'a str) -> Declaration<'a> {
    Declaration {
        name,
        value,
        important: true,
    }
}

tokenize!(tokenize_01, "",);

tokenize!(tokenize_02, " ",);

tokenize!(tokenize_03, "/**/",);

tokenize!(tokenize_04, "color:red", declare("color", "red"));

tokenize!(tokenize_05, "color:red;", declare("color", "red"));

tokenize!(tokenize_06, "color:red ", declare("color", "red"));

tokenize!(tokenize_07, " color: red; ", declare("color", "red"));

tokenize!(tokenize_08, "  color  :  red  ; ", declare("color", "red"));

tokenize!(
    tokenize_09,
    "  color:red;;;;color:red; ",
    declare("color", "red"),
    declare("color", "red")
);

tokenize!(
    tokenize_10,
    "background: url(\"img.png\");",
    declare("background", "url(\"img.png\")")
);

tokenize!(
    tokenize_11,
    "background: url(\"{}\");",
    declare("background", "url(\"{}\")")
);

tokenize!(
    tokenize_12,
    "color: red ! important",
    declare_important("color", "red")
);

tokenize!(
    tokenize_13,
    "color: red !important",
    declare_important("color", "red")
);

tokenize!(
    tokenize_14,
    "color: red!important",
    declare_important("color", "red")
);

tokenize!(
    tokenize_15,
    "color: red !/**/important",
    declare_important("color", "red")
);

tokenize!(
    tokenize_16,
    "border: 1em solid blue",
    declare("border", "1em solid blue")
);

tokenize!(
    tokenize_17,
    "background: navy url(support/diamond.png) -2em -2em no-repeat",
    declare(
        "background",
        "navy url(support/diamond.png) -2em -2em no-repeat"
    )
);

tokenize!(tokenize_18, "/**/color:red", declare("color", "red"));

tokenize!(tokenize_19, "/* *\\/*/color: red;", declare("color", "red"));

tokenize!(
    tokenize_20,
    "/**/color/**/:/**/red/**/;/**/",
    declare("color", "red")
);

tokenize!(tokenize_21, "\ncolor\n:\nred\n;\n", declare("color", "red"));

tokenize!(tokenize_22, "{color:red}",);

tokenize!(tokenize_23, "(color:red)",);

tokenize!(tokenize_24, "[color:red]",);

tokenize!(tokenize_25, "color:",);

tokenize!(tokenize_26, "value:\"text\"", declare("value", "\"text\""));

tokenize!(tokenize_27, "value:'text'", declare("value", "'text'"));

tokenize!(tokenize_28, "color:#fff", declare("color", "#fff"));
tokenize!(tokenize_29, "color:0.5", declare("color", "0.5"));

tokenize!(tokenize_30, "color:.5", declare("color", ".5"));

tokenize!(tokenize_31, "color:#FFF", declare("color", "#FFF"));

tokenize!(
    tokenize_32,
    "content: counter(chapno, upper-roman) \". \"",
    declare("content", "counter(chapno, upper-roman) \". \"")
);

tokenize!(
    tokenize_33,
    "font-family:'Noto Serif','DejaVu Serif',serif",
    declare("font-family", "'Noto Serif','DejaVu Serif',serif")
);

tokenize!(tokenize_34, "*zoom:1;", declare("zoom", "1"));

//tokenize!(tokenize_, "@unsupported { splines: reticulating } color: green",
//    declare("color", "green")
//);

//tokenize!(tokenize_, "/*\\*/*/color: red;", declare("color", "red"));

//tokenize!(tokenize_, "\"this is a string]}\"\"[{\\\"'\";  /*should be parsed as a string but be ignored*/
//    {{}}[]'';                     /*should be parsed as nested blocks and a string but be ignored*/
//    color: red;", declare("color", "red"));
