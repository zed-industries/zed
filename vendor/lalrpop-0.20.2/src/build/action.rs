//! Code for generating action code.
//!
//! From the outside, action fns have one of two forms. If they take
//! symbols as input, e.g. from a production like `X = Y Z => ...`
//! (which takes Y and Z as input), they have this form:
//!
//! ```
//! fn __action17<
//!     'input,                       // user-declared type parameters (*)
//! >(
//!     input: &'input str,           // user-declared parameters
//!     __0: (usize, usize, usize),   // symbols being reduced, if any
//!     ...
//!     __N: (usize, Foo, usize),     // each has a type (L, T, L)
//! ) -> Box<Expr<'input>>
//! ```
//!
//! Otherwise, they have this form:
//!
//! ```
//! fn __action17<
//!     'input,                       // user-declared type parameters (*)
//! >(
//!     input: &'input str,           // user-declared parameters
//!    __lookbehind: &usize,          // value for @R -- "end of previous token"
//!    __lookahead: &usize,           // value for @L -- "start of next token"
//! ) -> Box<Expr<'input>>
//! ```
//!
//! * -- in this case, those "user-declared" parameters are inserted by
//! the "internal tokenizer".

use crate::grammar::repr as r;
use crate::rust::RustWrite;
use std::io::{self, Write};

pub fn emit_action_code<W: Write>(grammar: &r::Grammar, rust: &mut RustWrite<W>) -> io::Result<()> {
    for (i, defn) in grammar.action_fn_defns.iter().enumerate() {
        rust!(rust, "");

        // we always thread the parameters through to the action code,
        // even if they are not used, and hence we need to disable the
        // unused variables lint, which otherwise gets very excited.
        if !grammar.parameters.is_empty() {
            rust!(rust, "#[allow(unused_variables)]");
        }

        match defn.kind {
            r::ActionFnDefnKind::User(ref data) => {
                emit_user_action_code(grammar, rust, i, defn, data)?
            }
            r::ActionFnDefnKind::Lookaround(ref variant) => {
                emit_lookaround_action_code(grammar, rust, i, defn, variant)?
            }
            r::ActionFnDefnKind::Inline(ref data) => {
                emit_inline_action_code(grammar, rust, i, defn, data)?
            }
        }
    }

    Ok(())
}

fn ret_type_string(grammar: &r::Grammar, defn: &r::ActionFnDefn) -> String {
    if defn.fallible {
        format!(
            "Result<{},{}lalrpop_util::ParseError<{},{},{}>>",
            defn.ret_type,
            grammar.prefix,
            grammar.types.terminal_loc_type(),
            grammar.types.terminal_token_type(),
            grammar.types.error_type()
        )
    } else {
        format!("{}", defn.ret_type)
    }
}

fn emit_user_action_code<W: Write>(
    grammar: &r::Grammar,
    rust: &mut RustWrite<W>,
    index: usize,
    defn: &r::ActionFnDefn,
    data: &r::UserActionFnDefn,
) -> io::Result<()> {
    let ret_type = ret_type_string(grammar, defn);

    // For each symbol to be reduced, we will receive
    // a (L, T, L) triple where the Ls are locations and
    // the T is the data. Ignore the locations and bind
    // the data to the name the user gave.
    let mut arguments: Vec<String> = data
        .arg_patterns
        .iter()
        .zip(
            data.arg_types
                .iter()
                .cloned()
                .map(|t| grammar.types.spanned_type(t)),
        )
        .map(|(name, ty)| format!("(_, {}, _): {}", name, ty))
        .collect();

    // If this is a reduce of an empty production, we will
    // automatically add position information in the form of
    // lookbehind/lookahead values. Otherwise, those values would be
    // determined from the arguments themselves.
    if data.arg_patterns.is_empty() {
        arguments.extend(vec![
            format!(
                "{}lookbehind: &{}",
                grammar.prefix,
                grammar.types.terminal_loc_type()
            ),
            format!(
                "{}lookahead: &{}",
                grammar.prefix,
                grammar.types.terminal_loc_type()
            ),
        ]);
    }

    rust!(rust, "#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]");
    rust.fn_header(
        &r::Visibility::Priv,
        format!("{}action{}", grammar.prefix, index),
    )
    .with_grammar(grammar)
    .with_parameters(arguments)
    .with_return_type(ret_type)
    .emit()?;

    rust!(rust, "{{");

    // The user did not provide any code
    if data.code != "()" {
        rust!(rust, "{}", data.code);
    }

    rust!(rust, "}}");
    Ok(())
}

fn emit_lookaround_action_code<W: Write>(
    grammar: &r::Grammar,
    rust: &mut RustWrite<W>,
    index: usize,
    _defn: &r::ActionFnDefn,
    data: &r::LookaroundActionFnDefn,
) -> io::Result<()> {
    rust.fn_header(
        &r::Visibility::Priv,
        format!("{}action{}", grammar.prefix, index),
    )
    .with_grammar(grammar)
    .with_parameters(vec![
        format!(
            "{}lookbehind: &{}",
            grammar.prefix,
            grammar.types.terminal_loc_type()
        ),
        format!(
            "{}lookahead: &{}",
            grammar.prefix,
            grammar.types.terminal_loc_type()
        ),
    ])
    .with_return_type(format!("{}", grammar.types.terminal_loc_type()))
    .emit()?;

    rust!(rust, "{{");
    match *data {
        r::LookaroundActionFnDefn::Lookahead => {
            // take the lookahead, if any; otherwise, we are
            // at EOF, so taker the lookbehind (end of last
            // pushed token); if that is missing too, then
            // supply default.
            rust!(rust, "*{}lookahead", grammar.prefix);
        }
        r::LookaroundActionFnDefn::Lookbehind => {
            // take lookbehind or supply default
            rust!(rust, "*{}lookbehind", grammar.prefix);
        }
    }
    rust!(rust, "}}");
    Ok(())
}

fn emit_inline_action_code<W: Write>(
    grammar: &r::Grammar,
    rust: &mut RustWrite<W>,
    index: usize,
    defn: &r::ActionFnDefn,
    data: &r::InlineActionFnDefn,
) -> io::Result<()> {
    let ret_type = ret_type_string(grammar, defn);

    let arg_types: Vec<_> = data
        .symbols
        .iter()
        .flat_map(|sym| match *sym {
            r::InlinedSymbol::Original(ref s) => vec![s.clone()],
            r::InlinedSymbol::Inlined(_, ref syms) => syms.clone(),
        })
        .map(|s| s.ty(&grammar.types))
        .collect();

    // this is the number of symbols we expect to be passed in; it is
    // distinct from data.symbols.len(), because sometimes we have
    // inlined actions with no input symbols
    let num_flat_args = arg_types.len();

    let mut arguments: Vec<_> = arg_types
        .iter()
        .map(|&t| grammar.types.spanned_type(t.clone()))
        .enumerate()
        .map(|(i, t)| format!("{}{}: {}", grammar.prefix, i, t))
        .collect();

    // If no symbols are being reduced, add in the
    // lookbehind/lookahead.
    if arguments.is_empty() {
        arguments.extend(vec![
            format!(
                "{}lookbehind: &{}",
                grammar.prefix,
                grammar.types.terminal_loc_type()
            ),
            format!(
                "{}lookahead: &{}",
                grammar.prefix,
                grammar.types.terminal_loc_type()
            ),
        ]);
    }

    rust!(
        rust,
        "#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]"
    );
    rust.fn_header(
        &r::Visibility::Priv,
        format!("{}action{}", grammar.prefix, index),
    )
    .with_grammar(grammar)
    .with_parameters(arguments)
    .with_return_type(ret_type)
    .emit()?;
    rust!(rust, "{{");

    // For each inlined thing, compute the start/end locations.
    // Do this first so that none of the arguments have been moved
    // yet and we can easily access their locations.
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(_, ref syms) => {
                if !syms.is_empty() {
                    // If we are reducing symbols, then start and end
                    // can be the start/end location of the first/last
                    // symbol respectively. Easy peezy.

                    rust!(
                        rust,
                        "let {}start{} = {}{}.0;",
                        grammar.prefix,
                        temp_counter,
                        grammar.prefix,
                        arg_counter
                    );

                    let last_arg_index = arg_counter + syms.len() - 1;
                    rust!(
                        rust,
                        "let {}end{} = {}{}.2;",
                        grammar.prefix,
                        temp_counter,
                        grammar.prefix,
                        last_arg_index
                    );
                } else {
                    // If we have no symbols, then `arg_counter`
                    // represents index of the first symbol after this
                    // inlined item (if any), and `arg_counter-1`
                    // represents index of the symbol before this
                    // item.

                    if arg_counter > 0 {
                        rust!(
                            rust,
                            "let {}start{} = {}{}.2;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix,
                            arg_counter - 1
                        );
                    } else if num_flat_args > 0 {
                        rust!(
                            rust,
                            "let {}start{} = {}{}.0;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix,
                            arg_counter
                        );
                    } else {
                        rust!(
                            rust,
                            "let {}start{} = *{}lookbehind;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix
                        );
                    }

                    if arg_counter < num_flat_args {
                        rust!(
                            rust,
                            "let {}end{} = {}{}.0;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix,
                            arg_counter
                        );
                    } else if num_flat_args > 0 {
                        rust!(
                            rust,
                            "let {}end{} = {}{}.2;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix,
                            num_flat_args - 1
                        );
                    } else {
                        rust!(
                            rust,
                            "let {}end{} = *{}lookahead;",
                            grammar.prefix,
                            temp_counter,
                            grammar.prefix
                        );
                    }
                }

                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }

    // Now create temporaries for the inlined things
    let mut arg_counter = 0;
    let mut temp_counter = 0;

    // if there are type parameters then type annotation is required
    let annotate = !grammar.non_lifetime_type_parameters().is_empty();
    let lparen = if annotate { "::<" } else { "(" };

    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(inlined_action, ref syms) => {
                // execute the inlined reduce action
                rust!(
                    rust,
                    "let {}temp{} = {}action{}{}",
                    grammar.prefix,
                    temp_counter,
                    grammar.prefix,
                    inlined_action.index(),
                    lparen
                );
                for t in grammar.non_lifetime_type_parameters() {
                    rust!(rust, "{},", t);
                }
                if annotate {
                    rust!(rust, ">(");
                }
                for parameter in &grammar.parameters {
                    rust!(rust, "{},", parameter.name);
                }
                for i in 0..syms.len() {
                    rust!(rust, "{}{},", grammar.prefix, arg_counter + i);
                }
                if syms.is_empty() {
                    rust!(rust, "&{}start{},", grammar.prefix, temp_counter);
                    rust!(rust, "&{}end{},", grammar.prefix, temp_counter);
                }

                if grammar.action_is_fallible(inlined_action) {
                    rust!(rust, ")?;");
                } else {
                    rust!(rust, ");");
                }

                // wrap up the inlined value along with its span
                rust!(
                    rust,
                    "let {}temp{} = ({}start{}, {}temp{}, {}end{});",
                    grammar.prefix,
                    temp_counter,
                    grammar.prefix,
                    temp_counter,
                    grammar.prefix,
                    temp_counter,
                    grammar.prefix,
                    temp_counter
                );

                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }

    let final_action_fallible = grammar.action_is_fallible(data.action);
    let (ok_begin, ok_end) = match (defn.fallible, final_action_fallible) {
        (true, true) | (false, false) => ("", ""),
        (true, false) => ("Ok(", ")"),
        (false, true) => unreachable!(),
    };

    rust!(
        rust,
        "{}{}action{}{}",
        ok_begin,
        grammar.prefix,
        data.action.index(),
        lparen
    );
    for t in grammar.non_lifetime_type_parameters() {
        rust!(rust, "{},", t);
    }
    if annotate {
        rust!(rust, ">(");
    }
    for parameter in &grammar.parameters {
        rust!(rust, "{},", parameter.name);
    }
    let mut arg_counter = 0;
    let mut temp_counter = 0;
    for symbol in &data.symbols {
        match *symbol {
            r::InlinedSymbol::Original(_) => {
                rust!(rust, "{}{},", grammar.prefix, arg_counter);
                arg_counter += 1;
            }
            r::InlinedSymbol::Inlined(_, ref syms) => {
                rust!(rust, "{}temp{},", grammar.prefix, temp_counter);
                temp_counter += 1;
                arg_counter += syms.len();
            }
        }
    }
    assert!(!data.symbols.is_empty());
    rust!(rust, "){}", ok_end);

    rust!(rust, "}}");
    Ok(())
}
