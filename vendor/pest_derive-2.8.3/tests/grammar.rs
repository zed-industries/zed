// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use alloc::{format, vec::Vec};

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "tests/grammar.pest"]
struct GrammarParser;

#[test]
fn string() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::string,
        tokens: [
            string(0, 3)
        ]
    };
}

#[test]
fn insensitive() {
    parses_to! {
        parser: GrammarParser,
        input: "aBC",
        rule: Rule::insensitive,
        tokens: [
            insensitive(0, 3)
        ]
    };
}

#[test]
fn range() {
    parses_to! {
        parser: GrammarParser,
        input: "6",
        rule: Rule::range,
        tokens: [
            range(0, 1)
        ]
    };
}

#[test]
fn ident() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::ident,
        tokens: [
            ident(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn pos_pred() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::pos_pred,
        tokens: [
            pos_pred(0, 0)
        ]
    };
}

#[test]
fn neg_pred() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::neg_pred,
        tokens: [
            neg_pred(0, 0)
        ]
    };
}

#[test]
fn double_neg_pred() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::double_neg_pred,
        tokens: [
            double_neg_pred(0, 0)
        ]
    };
}

#[test]
fn sequence() {
    parses_to! {
        parser: GrammarParser,
        input: "abc   abc",
        rule: Rule::sequence,
        tokens: [
            sequence(0, 9, [
                string(0, 3),
                string(6, 9)
            ])
        ]
    };
}

#[test]
fn sequence_compound() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::sequence_compound,
        tokens: [
            sequence_compound(0, 6, [
                string(0, 3),
                string(3, 6)
            ])
        ]
    };
}

#[test]
fn sequence_atomic() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::sequence_atomic,
        tokens: [
            sequence_atomic(0, 6)
        ]
    };
}

#[test]
fn sequence_non_atomic() {
    parses_to! {
        parser: GrammarParser,
        input: "abc   abc",
        rule: Rule::sequence_non_atomic,
        tokens: [
            sequence_non_atomic(0, 9, [
                sequence(0, 9, [
                    string(0, 3),
                    string(6, 9)
                ])
            ])
        ]
    };
}

#[test]
#[should_panic]
fn sequence_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::sequence_atomic,
        tokens: []
    };
}

#[test]
fn sequence_atomic_compound() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::sequence_atomic_compound,
        tokens: [
            sequence_atomic_compound(0, 6, [
                sequence_compound(0, 6, [
                    string(0, 3),
                    string(3, 6)
                ])
            ])
        ]
    };
}

#[test]
fn sequence_compound_nested() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::sequence_compound_nested,
        tokens: [
            sequence_compound_nested(0, 6, [
                sequence_nested(0, 6, [
                    string(0, 3),
                    string(3, 6)
                ])
            ])
        ]
    };
}

#[test]
#[should_panic]
fn sequence_compound_nested_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::sequence_compound_nested,
        tokens: []
    };
}

#[test]
fn choice_string() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::choice,
        tokens: [
            choice(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn choice_range() {
    parses_to! {
        parser: GrammarParser,
        input: "0",
        rule: Rule::choice,
        tokens: [
            choice(0, 1, [
                range(0, 1)
            ])
        ]
    };
}

#[test]
fn choice_prefix() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::choice_prefix,
        tokens: [
            choice_prefix(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn node_tag() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::node_tag,
        tokens: [
            node_tag(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn optional_string() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::optional,
        tokens: [
            optional(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn optional_empty() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::optional,
        tokens: [
            optional(0, 0)
        ]
    };
}

#[test]
fn repeat_empty() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::repeat,
        tokens: [
            repeat(0, 0)
        ]
    };
}

#[test]
fn repeat_strings() {
    parses_to! {
        parser: GrammarParser,
        input: "abc   abc",
        rule: Rule::repeat,
        tokens: [
            repeat(0, 9, [
                string(0, 3),
                string(6, 9)
            ])
        ]
    };
}

#[test]
fn repeat_atomic_empty() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::repeat_atomic,
        tokens: [
            repeat_atomic(0, 0)
        ]
    };
}

#[test]
fn repeat_atomic_strings() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::repeat_atomic,
        tokens: [
            repeat_atomic(0, 6)
        ]
    };
}

#[test]
#[should_panic]
fn repeat_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_atomic,
        tokens: []
    };
}

#[test]
#[should_panic]
fn repeat_once_empty() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::repeat_once,
        tokens: []
    };
}

#[test]
fn repeat_once_strings() {
    parses_to! {
        parser: GrammarParser,
        input: "abc   abc",
        rule: Rule::repeat_once,
        tokens: [
            repeat_once(0, 9, [
                string(0, 3),
                string(6, 9)
            ])
        ]
    };
}

#[test]
#[should_panic]
fn repeat_once_atomic_empty() {
    parses_to! {
        parser: GrammarParser,
        input: "",
        rule: Rule::repeat_once_atomic,
        tokens: []
    };
}

#[test]
fn repeat_once_atomic_strings() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::repeat_once_atomic,
        tokens: [
            repeat_once_atomic(0, 6)
        ]
    };
}

#[test]
#[should_panic]
fn repeat_once_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_once_atomic,
        tokens: []
    };
}

#[test]
fn repeat_min_max_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_min_max,
        tokens: [
            repeat_min_max(0, 7, [
                string(0, 3),
                string(4, 7)
            ])
        ]
    };
}

#[test]
fn repeat_min_max_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc abc",
        rule: Rule::repeat_min_max,
        tokens: [
            repeat_min_max(0, 11, [
                string(0, 3),
                string(4, 7),
                string(8, 11)
            ])
        ]
    };
}

#[test]
fn repeat_min_max_atomic_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::repeat_min_max_atomic,
        tokens: [
            repeat_min_max_atomic(0, 6)
        ]
    };
}

#[test]
fn repeat_min_max_atomic_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabcabc",
        rule: Rule::repeat_min_max_atomic,
        tokens: [
            repeat_min_max_atomic(0, 9)
        ]
    };
}

#[test]
#[should_panic]
fn repeat_min_max_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_min_max_atomic,
        tokens: []
    };
}

#[test]
fn repeat_exact() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_exact,
        tokens: [
            repeat_exact(0, 7, [
                string(0, 3),
                string(4, 7)
            ])
        ]
    };
}

#[test]
#[should_panic]
fn repeat_min_once() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::repeat_min,
        tokens: []
    };
}

#[test]
fn repeat_min_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_min,
        tokens: [
            repeat_min(0, 7, [
                string(0, 3),
                string(4, 7)
            ])
        ]
    };
}

#[test]
fn repeat_min_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc  abc",
        rule: Rule::repeat_min,
        tokens: [
            repeat_min(0, 12, [
                string(0, 3),
                string(4, 7),
                string(9, 12)
            ])
        ]
    };
}

#[test]
#[should_panic]
fn repeat_min_atomic_once() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::repeat_min_atomic,
        tokens: []
    };
}

#[test]
fn repeat_min_atomic_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::repeat_min_atomic,
        tokens: [
            repeat_min_atomic(0, 6)
        ]
    };
}

#[test]
fn repeat_min_atomic_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabcabc",
        rule: Rule::repeat_min_atomic,
        tokens: [
            repeat_min_atomic(0, 9)
        ]
    };
}

#[test]
#[should_panic]
fn repeat_min_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_min_atomic,
        tokens: []
    };
}

#[test]
fn repeat_max_once() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::repeat_max,
        tokens: [
            repeat_max(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn repeat_max_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_max,
        tokens: [
            repeat_max(0, 7, [
                string(0, 3),
                string(4, 7)
            ])
        ]
    };
}

#[test]
#[should_panic]
fn repeat_max_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_max,
        tokens: []
    };
}

#[test]
fn repeat_max_atomic_once() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::repeat_max_atomic,
        tokens: [
            repeat_max_atomic(0, 3)
        ]
    };
}

#[test]
fn repeat_max_atomic_twice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabc",
        rule: Rule::repeat_max_atomic,
        tokens: [
            repeat_max_atomic(0, 6)
        ]
    };
}

#[test]
#[should_panic]
fn repeat_max_atomic_thrice() {
    parses_to! {
        parser: GrammarParser,
        input: "abcabcabc",
        rule: Rule::repeat_max_atomic,
        tokens: []
    };
}

#[test]
#[should_panic]
fn repeat_max_atomic_space() {
    parses_to! {
        parser: GrammarParser,
        input: "abc abc",
        rule: Rule::repeat_max_atomic,
        tokens: []
    };
}

#[test]
fn repeat_comment() {
    parses_to! {
        parser: GrammarParser,
        input: "abc$$$ $$$abc",
        rule: Rule::repeat_once,
        tokens: [
            repeat_once(0, 13, [
                string(0, 3),
                string(10, 13)
            ])
        ]
    };
}

#[test]
fn soi_at_start() {
    parses_to! {
        parser: GrammarParser,
        input: "abc",
        rule: Rule::soi_at_start,
        tokens: [
            soi_at_start(0, 3, [
                string(0, 3)
            ])
        ]
    };
}

#[test]
fn peek() {
    parses_to! {
        parser: GrammarParser,
        input: "0111",
        rule: Rule::peek_,
        tokens: [
            peek_(0, 4, [
                range(0, 1),
                range(1, 2)
            ])
        ]
    };
}

#[test]
fn peek_all() {
    parses_to! {
        parser: GrammarParser,
        input: "0110",
        rule: Rule::peek_all,
        tokens: [
            peek_all(0, 4, [
                range(0, 1),
                range(1, 2)
            ])
        ]
    };
}

#[test]
fn peek_slice_23() {
    parses_to! {
        parser: GrammarParser,
        input: "0123412",
        rule: Rule::peek_slice_23,
        tokens: [
            peek_slice_23(0, 7, [
                range(0, 1),
                range(1, 2),
                range(2, 3),
                range(3, 4),
                range(4, 5),
            ])
        ]
    };
}

#[test]
fn pop() {
    parses_to! {
        parser: GrammarParser,
        input: "0110",
        rule: Rule::pop_,
        tokens: [
            pop_(0, 4, [
                range(0, 1),
                range(1, 2)
            ])
        ]
    };
}

#[test]
fn pop_all() {
    parses_to! {
        parser: GrammarParser,
        input: "0110",
        rule: Rule::pop_all,
        tokens: [
            pop_all(0, 4, [
                range(0, 1),
                range(1, 2)
            ])
        ]
    };
}

#[test]
fn pop_fail() {
    parses_to! {
        parser: GrammarParser,
        input: "010",
        rule: Rule::pop_fail,
        tokens: [
            pop_fail(0, 3, [
                range(0, 1),
                range(1, 2)
            ])
        ]
    };
}

#[test]
fn repeat_mutate_stack() {
    parses_to! {
        parser: GrammarParser,
        input: "a,b,c,cba",
        rule: Rule::repeat_mutate_stack,
        tokens: [
            repeat_mutate_stack(0, 9)
        ]
    };
}

#[test]
fn stack_resume_after_fail() {
    parses_to! {
        parser: GrammarParser,
        input: "a,b,c,cba",
        rule: Rule::stack_resume_after_fail,
        tokens: [
            stack_resume_after_fail(0, 9, [
                repeat_mutate_stack_pop_all(0, 9)
            ])
        ]
    };
}

#[test]
fn checkpoint_restore() {
    parses_to! {
        parser: GrammarParser,
        input: "a",
        rule: Rule::checkpoint_restore,
        tokens: [
            checkpoint_restore(0, 1, [EOI(1, 1)])
        ]
    };
}

#[test]
fn ascii_digits() {
    parses_to! {
        parser: GrammarParser,
        input: "6",
        rule: Rule::ascii_digits,
        tokens: [
            ascii_digits(0, 1)
        ]
    };
}

#[test]
fn ascii_nonzero_digits() {
    parses_to! {
        parser: GrammarParser,
        input: "5",
        rule: Rule::ascii_nonzero_digits,
        tokens: [
            ascii_nonzero_digits(0, 1)
        ]
    };
}

#[test]
fn ascii_bin_digits() {
    parses_to! {
        parser: GrammarParser,
        input: "1",
        rule: Rule::ascii_bin_digits,
        tokens: [
            ascii_bin_digits(0, 1)
        ]
    };
}

#[test]
fn ascii_oct_digits() {
    parses_to! {
        parser: GrammarParser,
        input: "3",
        rule: Rule::ascii_oct_digits,
        tokens: [
            ascii_oct_digits(0, 1)
        ]
    };
}

#[test]
fn ascii_hex_digits() {
    parses_to! {
        parser: GrammarParser,
        input: "6bC",
        rule: Rule::ascii_hex_digits,
        tokens: [
            ascii_hex_digits(0, 3)
        ]
    };
}

#[test]
fn ascii_alpha_lowers() {
    parses_to! {
        parser: GrammarParser,
        input: "a",
        rule: Rule::ascii_alpha_lowers,
        tokens: [
            ascii_alpha_lowers(0, 1)
        ]
    };
}

#[test]
fn ascii_alpha_uppers() {
    parses_to! {
        parser: GrammarParser,
        input: "K",
        rule: Rule::ascii_alpha_uppers,
        tokens: [
            ascii_alpha_uppers(0, 1)
        ]
    };
}

#[test]
fn ascii_alphas() {
    parses_to! {
        parser: GrammarParser,
        input: "wF",
        rule: Rule::ascii_alphas,
        tokens: [
            ascii_alphas(0, 2)
        ]
    };
}

#[test]
fn ascii_alphanumerics() {
    parses_to! {
        parser: GrammarParser,
        input: "4jU",
        rule: Rule::ascii_alphanumerics,
        tokens: [
            ascii_alphanumerics(0, 3)
        ]
    };
}

#[test]
fn asciis() {
    parses_to! {
        parser: GrammarParser,
        input: "x02",
        rule: Rule::asciis,
        tokens: [
            asciis(0, 3)
        ]
    };
}

#[test]
fn newline() {
    parses_to! {
        parser: GrammarParser,
        input: "\n\r\n\r",
        rule: Rule::newline,
        tokens: [
            newline(0, 4)
        ]
    };
}

#[test]
fn unicode() {
    parses_to! {
        parser: GrammarParser,
        input: "نامهای",
        rule: Rule::unicode,
        tokens: [
            unicode(0, 12)
        ]
    }
}

#[test]
fn shadowing() {
    parses_to! {
        parser: GrammarParser,
        input: "shadows builtin",
        rule: Rule::SYMBOL,
        tokens: [
            SYMBOL(0, 15)
        ]
    }
}
