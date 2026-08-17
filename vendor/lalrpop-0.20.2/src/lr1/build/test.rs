use crate::generate;
use crate::grammar::repr::*;
use crate::lr1::core::*;
use crate::lr1::interpret::interpret;
use crate::lr1::lookahead::Token;
use crate::lr1::lookahead::TokenSet;
use crate::lr1::tls::Lr1Tls;
use crate::test_util::{compare, expect_debug, normalized_grammar};
use crate::tls::Tls;
use string_cache::DefaultAtom as Atom;

use super::{build_lr0_states, build_lr1_states, use_lane_table, Lr};

fn nt(t: &str) -> NonterminalString {
    NonterminalString(Atom::from(t))
}

const ITERATIONS: usize = 22;

fn random_test<'g>(grammar: &Grammar, states: &'g [Lr1State<'g>], start_symbol: NonterminalString) {
    for i in 0..ITERATIONS {
        let input_tree = generate::random_parse_tree(grammar, start_symbol.clone());
        let output_tree = interpret(states, input_tree.terminals()).unwrap();

        println!("test {}", i);
        println!("input_tree = {}", input_tree);
        println!("output_tree = {}", output_tree);

        compare(output_tree, input_tree);
    }
}

macro_rules! tokens {
    ($($x:expr),*) => {
        vec![$(TerminalString::quoted(Atom::from($x))),*]
    }
}

fn items<'g>(grammar: &'g Grammar, nonterminal: &str, index: usize, la: Token) -> Lr1Items<'g> {
    let set = TokenSet::from(la);
    let lr1: Lr<TokenSet> = Lr::new(grammar, nt(nonterminal), set.clone());
    let items = lr1.transitive_closure(lr1.items(&nt(nonterminal), index, &set));
    items
}

#[test]
fn start_state() {
    let grammar = normalized_grammar(
        r#"
grammar;
    extern { enum Tok { "C" => .., "D" => .. } }
    A = B "C";
    B: Option<u32> = {
        "D" => Some(1),
        () => None
    };
"#,
    );
    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    let items = items(&grammar, "A", 0, Token::Eof);
    expect_debug(
        items.vec,
        r#"[
    A = (*) B "C" [Eof],
    B = (*) ["C"],
    B = (*) "D" ["C"]
]"#,
    );
}

#[test]
fn start_state_1() {
    let grammar = normalized_grammar(
        r#"
grammar;
extern { enum Tok { "B1" => .., "C1" => .. } }
A = B C;
B: Option<u32> = {
    "B1" => Some(1),
    () => None
};
C: Option<u32> = {
    "C1" => Some(1),
    () => None
};
"#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    expect_debug(
        items(&grammar, "A", 0, Token::Eof).vec,
        r#"[
    A = (*) B C [Eof],
    B = (*) ["C1", Eof],
    B = (*) "B1" ["C1", Eof]
]"#,
    );

    expect_debug(
        items(&grammar, "A", 1, Token::Eof).vec,
        r#"[
    A = B (*) C [Eof],
    C = (*) [Eof],
    C = (*) "C1" [Eof]
]"#,
    );
}

#[test]
fn expr_grammar1() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(
        r#"
grammar;
    extern { enum Tok { "-" => .., "N" => .., "(" => .., ")" => .. } }

    S: () =
        E => ();

    E: () = {
        E "-" T => (),
        T => ()
    };

    T: () = {
        "N" => (),
        "(" E ")" => ()
    };
"#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    // for now, just test that process does not result in an error
    // and yields expected number of states.
    let states = build_lr1_states(&grammar, nt("S")).unwrap();
    println!("{:#?}", states);
    assert_eq!(states.len(), if use_lane_table() { 9 } else { 16 });

    // execute it on some sample inputs.
    let tree = interpret(&states, tokens!["N", "-", "(", "N", "-", "N", ")"]).unwrap();
    assert_eq!(
        &format!("{}", tree)[..],
        r#"[S: [E: [E: [T: "N"]], "-", [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]]]"#
    );

    // incomplete:
    assert!(interpret(&states, tokens!["N", "-", "(", "N", "-", "N"]).is_err());

    // incomplete:
    assert!(interpret(&states, tokens!["N", "-"]).is_err());

    // unexpected character:
    assert!(interpret(&states, tokens!["N", "-", ")", "N", "-", "N", "("]).is_err());

    // parens first:
    let tree = interpret(&states, tokens!["(", "N", "-", "N", ")", "-", "N"]).unwrap();
    println!("{}", tree);
    assert_eq!(
        &format!("{}", tree)[..],
        r#"[S: [E: [E: [T: "(", [E: [E: [T: "N"]], "-", [T: "N"]], ")"]], "-", [T: "N"]]]"#
    );

    // run some random tests
    random_test(&grammar, &states, nt("S"));
}

#[test]
fn shift_reduce_conflict1() {
    let _tls = Tls::test();

    // This grammar gets a shift-reduce conflict because if the input
    // is "&" (*) "L", then we see two possibilities, and we must decide
    // between them:
    //
    // "&" (*) "L" E
    //  |       |  |
    //  +-------+--|
    //          |
    //          E
    //
    // or
    //
    // "&"      (*) "L"
    //  |            |
    //  |  OPT_L     E
    //  |   |        |
    //  +---+---+----+
    //          |
    //          E
    //
    // to some extent this may be a false conflict, in that inlined
    // rules would address it, but it's an interesting one for
    // producing a useful error message.

    let grammar = normalized_grammar(
        r#"
        grammar;
        extern { enum Tok { "L" => .., "&" => .., } }
        E: () = {
            "L",
            "&" OPT_L E
        };
        OPT_L: () = {
            (),
            "L"
        };
    "#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    assert!(build_lr1_states(&grammar, nt("E")).is_err());
}

/// One of the few grammars that IS LR(0).
#[test]
fn lr0_expr_grammar_with_explicit_eof() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(
        r#"
grammar;

S: () = E "$";

E: () = {
    E "-" T,
    T,
};

T: () = {
    "N",
    "(" E ")",
};
"#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    // for now, just test that process does not result in an error
    // and yields expected number of states.
    let states = build_lr0_states(&grammar, nt("S")).unwrap();
    assert_eq!(states.len(), 10);
}

/// Without the artificial '$', grammar is not LR(0).
#[test]
fn lr0_expr_grammar_with_implicit_eof() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(
        r#"
grammar;

S: () = E;

E: () = {
    E "-" T,
    T,
};

T: () = {
    "N",
    "(" E ")",
};
"#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    build_lr0_states(&grammar, nt("S")).unwrap_err();
}

/// When we moved to storing items as (lr0 -> TokenSet) pairs, a bug
/// in the transitive closure routine could cause us to have `(Foo,
/// S0)` and `(Foo, S1)` as distinct items instead of `(Foo, S0|S1)`.
#[test]
fn issue_144() {
    let _tls = Tls::test();

    #[allow(clippy::needless_raw_string_hashes)] // Fix merged for next clippy release, after 1.72
    let grammar = normalized_grammar(
        r##"
grammar;

pub ForeignItem: () = {
  AttrsAndVis "item_foreign_fn",
  AttrsAndVis "unsafe" "item_foreign_fn",
};

AttrsAndVis: () = {
    MaybeOuterAttrs visibility,
};

MaybeOuterAttrs: () = {
    OuterAttrs,
    (),
};

visibility: () = {
  "pub",
  (),
};

OuterAttrs: () = {
    OuterAttr,
    OuterAttrs OuterAttr,
};

OuterAttr: () = {
    "#" "[" "]",
};

Ident: () = {
    "IDENT",
};

ty: () = {
    "ty"
};
"##,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());
    build_lr1_states(&grammar, nt("ForeignItem")).unwrap();
}

// Not sure if this is the right spot
#[test]
// This test requires regex's unicode case support
#[cfg_attr(not(feature = "unicode"), ignore)]
fn match_grammar() {
    let _tls = Tls::test();

    let grammar = normalized_grammar(
        r#"
grammar;

match {
    r"(?i)select" => SELECT
} else {
    _
}

pub Query = SELECT r"[a-z]+";
"#,
    );

    let _lr1_tls = Lr1Tls::install(grammar.terminals.clone());

    let states = build_lr0_states(&grammar, nt("Query")).expect("build states");
    println!("states: {:?}", states);
}
