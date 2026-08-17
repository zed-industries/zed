/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::cow_rc_str::CowRcStr;
use crate::tokenizer::Token;

macro_rules! size_of_test {
    ($testname: ident, $t: ty, $expected_min_size: expr, $expected_max_size: expr) => {
        #[test]
        fn $testname() {
            let new = ::std::mem::size_of::<$t>();
            if new < $expected_min_size {
                panic!(
                    "Your changes have decreased the stack size of {} from {} to {}. \
                     Good work! Please update the expected size in {}.",
                    stringify!($t),
                    $expected_min_size,
                    new,
                    file!()
                )
            } else if new > $expected_max_size {
                panic!(
                    "Your changes have increased the stack size of {} from {} to {}. \
                     Please consider choosing a design which avoids this increase. \
                     If you feel that the increase is necessary, update the size in {}.",
                    stringify!($t),
                    $expected_max_size,
                    new,
                    file!()
                )
            }
        }
    };
    ($testname: ident, $t: ty, $expected_size: expr) => {
        size_of_test!($testname, $t, $expected_size, $expected_size);
    };
}

// Some of these assume 64-bit
size_of_test!(token, Token, 32);
size_of_test!(std_cow_str, std::borrow::Cow<'static, str>, 24, 32);
size_of_test!(cow_rc_str, CowRcStr, 16);

size_of_test!(tokenizer, crate::tokenizer::Tokenizer, 72);
size_of_test!(parser_input, crate::parser::ParserInput, 136);
size_of_test!(parser, crate::parser::Parser, 16);
size_of_test!(source_position, crate::SourcePosition, 8);
size_of_test!(parser_state, crate::ParserState, 24);

size_of_test!(basic_parse_error, crate::BasicParseError, 40, 48);
size_of_test!(parse_error_lower_bound, crate::ParseError<()>, 40, 48);
