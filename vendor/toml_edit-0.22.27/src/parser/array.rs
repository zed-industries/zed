use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::peek;
use winnow::combinator::separated;
use winnow::combinator::trace;

use crate::parser::trivia::ws_comment_newline;
use crate::parser::value::value;
use crate::{Array, Item, RawString};

use crate::parser::prelude::*;

// ;; Array

// array = array-open array-values array-close
pub(crate) fn array<'i>(input: &mut Input<'i>) -> ModalResult<Array> {
    trace("array", move |input: &mut Input<'i>| {
        delimited(
            ARRAY_OPEN,
            cut_err(array_values),
            cut_err(ARRAY_CLOSE)
                .context(StrContext::Label("array"))
                .context(StrContext::Expected(StrContextValue::CharLiteral(']'))),
        )
        .parse_next(input)
    })
    .parse_next(input)
}

// note: we're omitting ws and newlines here, because
// they should be part of the formatted values
// array-open  = %x5B ws-newline  ; [
pub(crate) const ARRAY_OPEN: u8 = b'[';
// array-close = ws-newline %x5D  ; ]
const ARRAY_CLOSE: u8 = b']';
// array-sep = ws %x2C ws  ; , Comma
const ARRAY_SEP: u8 = b',';

// array-values =  ws-comment-newline val ws-comment-newline array-sep array-values
// array-values =/ ws-comment-newline val ws-comment-newline [ array-sep ]
fn array_values(input: &mut Input<'_>) -> ModalResult<Array> {
    if peek(opt(ARRAY_CLOSE)).parse_next(input)?.is_some() {
        // Optimize for empty arrays, avoiding `value` from being expected to fail
        return Ok(Array::new());
    }

    let array = separated(0.., array_value, ARRAY_SEP).parse_next(input)?;
    let mut array = Array::with_vec(array);
    if !array.is_empty() {
        let comma = opt(ARRAY_SEP).parse_next(input)?.is_some();
        array.set_trailing_comma(comma);
    }
    let trailing = ws_comment_newline.span().parse_next(input)?;
    array.set_trailing(RawString::with_span(trailing));

    Ok(array)
}

fn array_value(input: &mut Input<'_>) -> ModalResult<Item> {
    let prefix = ws_comment_newline.span().parse_next(input)?;
    let value = value.parse_next(input)?;
    let suffix = ws_comment_newline.span().parse_next(input)?;
    let value = value.decorated(RawString::with_span(prefix), RawString::with_span(suffix));
    let value = Item::Value(value);
    Ok(value)
}

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn arrays() {
        let inputs = [
            r#"[]"#,
            r#"[   ]"#,
            r#"[
  1, 2, 3
]"#,
            r#"[
  1,
  2, # this is ok
]"#,
            r#"[# comment
# comment2


   ]"#,
            r#"[# comment
# comment2
      1

#sd
,
# comment3

   ]"#,
            r#"[1]"#,
            r#"[1,]"#,
            r#"[ "all", 'strings', """are the same""", '''type''']"#,
            r#"[ 100, -2,]"#,
            r#"[1, 2, 3]"#,
            r#"[1.1, 2.1, 3.1]"#,
            r#"["a", "b", "c"]"#,
            r#"[ [ 1, 2 ], [3, 4, 5] ]"#,
            r#"[ [ 1, 2 ], ["a", "b", "c"] ]"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in inputs {
            dbg!(input);
            let mut parsed = array.parse(new_input(input));
            if let Ok(parsed) = &mut parsed {
                parsed.despan(input);
            }
            assert_eq!(parsed.map(|a| a.to_string()), Ok(input.to_owned()));
        }
    }

    #[test]
    fn invalid_arrays() {
        let invalid_inputs = [r#"["#, r#"[,]"#, r#"[,2]"#, r#"[1e165,,]"#];
        for input in invalid_inputs {
            dbg!(input);
            let mut parsed = array.parse(new_input(input));
            if let Ok(parsed) = &mut parsed {
                parsed.despan(input);
            }
            assert!(parsed.is_err());
        }
    }
}
