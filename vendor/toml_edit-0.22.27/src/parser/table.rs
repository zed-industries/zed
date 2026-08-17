use std::cell::RefCell;
#[allow(unused_imports)]
use std::ops::DerefMut;

use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::peek;
use winnow::token::take;

// https://github.com/rust-lang/rust/issues/41358
use crate::parser::key::key;
use crate::parser::prelude::*;
use crate::parser::state::ParseState;
use crate::parser::trivia::line_trailing;

// std-table-open  = %x5B ws     ; [ Left square bracket
pub(crate) const STD_TABLE_OPEN: u8 = b'[';
// std-table-close = ws %x5D     ; ] Right square bracket
const STD_TABLE_CLOSE: u8 = b']';
// array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
const ARRAY_TABLE_OPEN: &[u8] = b"[[";
// array-table-close = ws %x5D.5D  ; ]] Double right square bracket
const ARRAY_TABLE_CLOSE: &[u8] = b"]]";

// ;; Standard Table

// std-table = std-table-open key *( table-key-sep key) std-table-close
fn std_table<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        (
            delimited(
                STD_TABLE_OPEN,
                cut_err(key),
                cut_err(STD_TABLE_CLOSE)
                    .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
                    .context(StrContext::Expected(StrContextValue::StringLiteral("]"))),
            )
            .with_span(),
            cut_err(line_trailing)
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('#'))),
        )
            .try_map(|((h, span), t)| state.borrow_mut().deref_mut().on_std_header(h, t, span))
            .parse_next(i)
    }
}

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
fn array_table<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        (
            delimited(
                ARRAY_TABLE_OPEN,
                cut_err(key),
                cut_err(ARRAY_TABLE_CLOSE)
                    .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
                    .context(StrContext::Expected(StrContextValue::StringLiteral("]]"))),
            )
            .with_span(),
            cut_err(line_trailing)
                .context(StrContext::Expected(StrContextValue::CharLiteral('\n')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('#'))),
        )
            .try_map(|((h, span), t)| state.borrow_mut().deref_mut().on_array_header(h, t, span))
            .parse_next(i)
    }
}

// ;; Table

// table = std-table / array-table
pub(crate) fn table<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl ModalParser<Input<'i>, (), ContextError> + 's {
    move |i: &mut Input<'i>| {
        dispatch!(peek::<_, &[u8],_,_>(take(2usize));
            b"[[" => array_table(state),
            _ => std_table(state),
        )
        .context(StrContext::Label("table header"))
        .parse_next(i)
    }
}
