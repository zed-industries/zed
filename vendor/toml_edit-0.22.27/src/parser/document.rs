use std::cell::RefCell;

use winnow::combinator::cut_err;
use winnow::combinator::eof;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::repeat;
use winnow::combinator::trace;
use winnow::token::any;
use winnow::token::one_of;

use crate::key::Key;
use crate::parser::inline_table::KEYVAL_SEP;
use crate::parser::key::key;
use crate::parser::prelude::*;
use crate::parser::state::ParseState;
use crate::parser::table::table;
use crate::parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use crate::parser::value::value;
use crate::Item;
use crate::RawString;

// ;; TOML

// toml = expression *( newline expression )

// expression = ( ( ws comment ) /
//                ( ws keyval ws [ comment ] ) /
//                ( ws table ws [ comment ] ) /
//                  ws )
pub(crate) fn document<'s, 'i>(
    state_ref: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        (
            // Remove BOM if present
            opt(b"\xEF\xBB\xBF"),
            parse_ws(state_ref),
            repeat(0.., (
                dispatch! {peek(any);
                    crate::parser::trivia::COMMENT_START_SYMBOL => cut_err(parse_comment(state_ref)),
                    crate::parser::table::STD_TABLE_OPEN => cut_err(table(state_ref)),
                    crate::parser::trivia::LF |
                    crate::parser::trivia::CR => parse_newline(state_ref),
                    _ => cut_err(keyval(state_ref)),
                },
                parse_ws(state_ref),
            ))
            .map(|()| ()),
            eof,
        ).void().parse_next(i)
    }
}

fn parse_comment<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        (comment, line_ending)
            .span()
            .map(|span| {
                state.borrow_mut().on_comment(span);
            })
            .parse_next(i)
    }
}

fn parse_ws<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        ws.span()
            .map(|span| state.borrow_mut().on_ws(span))
            .parse_next(i)
    }
}

fn parse_newline<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        newline
            .span()
            .map(|span| state.borrow_mut().on_ws(span))
            .parse_next(i)
    }
}

fn keyval<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        parse_keyval
            .try_map(|(p, kv)| state.borrow_mut().on_keyval(p, kv))
            .parse_next(i)
    }
}

// keyval = key keyval-sep val
fn parse_keyval(input: &mut Input<'_>) -> ModalResult<(Vec<Key>, (Key, Item))> {
    trace(
        "keyval",
        (
            key,
            cut_err((
                one_of(KEYVAL_SEP)
                    .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
                    .context(StrContext::Expected(StrContextValue::CharLiteral('='))),
                (
                    ws.span(),
                    value,
                    line_trailing
                        .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                        .context(StrContext::Expected(StrContextValue::CharLiteral('#'))),
                ),
            )),
        )
            .try_map::<_, _, std::str::Utf8Error>(|(key, (_, v))| {
                let mut path = key;
                let key = path.pop().expect("grammar ensures at least 1");

                let (pre, v, suf) = v;
                let pre = RawString::with_span(pre);
                let suf = RawString::with_span(suf);
                let v = v.decorated(pre, suf);
                Ok((path, (key, Item::Value(v))))
            }),
    )
    .parse_next(input)
}
