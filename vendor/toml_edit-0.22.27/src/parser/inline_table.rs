use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::separated;
use winnow::combinator::trace;
use winnow::token::one_of;

use crate::key::Key;
use crate::parser::error::CustomError;
use crate::parser::key::key;
use crate::parser::prelude::*;
use crate::parser::trivia::ws;
use crate::parser::value::value;
use crate::{InlineTable, Item, RawString, Value};

use indexmap::map::Entry;

// ;; Inline Table

// inline-table = inline-table-open inline-table-keyvals inline-table-close
pub(crate) fn inline_table<'i>(input: &mut Input<'i>) -> ModalResult<InlineTable> {
    trace("inline-table", move |input: &mut Input<'i>| {
        delimited(
            INLINE_TABLE_OPEN,
            cut_err(inline_table_keyvals.try_map(|(kv, p)| table_from_pairs(kv, p))),
            cut_err(INLINE_TABLE_CLOSE)
                .context(StrContext::Label("inline table"))
                .context(StrContext::Expected(StrContextValue::CharLiteral('}'))),
        )
        .parse_next(input)
    })
    .parse_next(input)
}

fn table_from_pairs(
    v: Vec<(Vec<Key>, (Key, Item))>,
    preamble: RawString,
) -> Result<InlineTable, CustomError> {
    let mut root = InlineTable::new();
    root.set_preamble(preamble);
    // Assuming almost all pairs will be directly in `root`
    root.items.reserve(v.len());

    for (path, (key, value)) in v {
        let table = descend_path(&mut root, &path, true)?;

        // "Likewise, using dotted keys to redefine tables already defined in [table] form is not allowed"
        let mixed_table_types = table.is_dotted() == path.is_empty();
        if mixed_table_types {
            return Err(CustomError::DuplicateKey {
                key: key.get().into(),
                table: None,
            });
        }

        match table.items.entry(key) {
            Entry::Vacant(o) => {
                o.insert(value);
            }
            Entry::Occupied(o) => {
                return Err(CustomError::DuplicateKey {
                    key: o.key().get().into(),
                    table: None,
                });
            }
        }
    }
    Ok(root)
}

fn descend_path<'a>(
    mut table: &'a mut InlineTable,
    path: &'a [Key],
    dotted: bool,
) -> Result<&'a mut InlineTable, CustomError> {
    for (i, key) in path.iter().enumerate() {
        table = match table.entry_format(key) {
            crate::InlineEntry::Vacant(entry) => {
                let mut new_table = InlineTable::new();
                new_table.set_implicit(true);
                new_table.set_dotted(dotted);

                entry
                    .insert(Value::InlineTable(new_table))
                    .as_inline_table_mut()
                    .unwrap()
            }
            crate::InlineEntry::Occupied(entry) => {
                match entry.into_mut() {
                    Value::InlineTable(ref mut sweet_child_of_mine) => {
                        // Since tables cannot be defined more than once, redefining such tables using a
                        // [table] header is not allowed. Likewise, using dotted keys to redefine tables
                        // already defined in [table] form is not allowed.
                        if dotted && !sweet_child_of_mine.is_implicit() {
                            return Err(CustomError::DuplicateKey {
                                key: key.get().into(),
                                table: None,
                            });
                        }
                        sweet_child_of_mine
                    }
                    ref v => {
                        return Err(CustomError::extend_wrong_type(path, i, v.type_name()));
                    }
                }
            }
        };
    }
    Ok(table)
}

// inline-table-open  = %x7B ws     ; {
pub(crate) const INLINE_TABLE_OPEN: u8 = b'{';
// inline-table-close = ws %x7D     ; }
const INLINE_TABLE_CLOSE: u8 = b'}';
// inline-table-sep   = ws %x2C ws  ; , Comma
const INLINE_TABLE_SEP: u8 = b',';
// keyval-sep = ws %x3D ws ; =
pub(crate) const KEYVAL_SEP: u8 = b'=';

// inline-table-keyvals = [ inline-table-keyvals-non-empty ]
// inline-table-keyvals-non-empty =
// ( key keyval-sep val inline-table-sep inline-table-keyvals-non-empty ) /
// ( key keyval-sep val )

fn inline_table_keyvals(
    input: &mut Input<'_>,
) -> ModalResult<(Vec<(Vec<Key>, (Key, Item))>, RawString)> {
    (
        separated(0.., keyval, INLINE_TABLE_SEP),
        ws.span().map(RawString::with_span),
    )
        .parse_next(input)
}

fn keyval(input: &mut Input<'_>) -> ModalResult<(Vec<Key>, (Key, Item))> {
    (
        key,
        cut_err((
            one_of(KEYVAL_SEP)
                .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
                .context(StrContext::Expected(StrContextValue::CharLiteral('='))),
            (ws.span(), value, ws.span()),
        )),
    )
        .map(|(key, (_, v))| {
            let mut path = key;
            let key = path.pop().expect("grammar ensures at least 1");

            let (pre, v, suf) = v;
            let pre = RawString::with_span(pre);
            let suf = RawString::with_span(suf);
            let v = v.decorated(pre, suf);
            (path, (key, Item::Value(v)))
        })
        .parse_next(input)
}

#[cfg(test)]
#[cfg(feature = "parse")]
#[cfg(feature = "display")]
mod test {
    use super::*;

    #[test]
    fn inline_tables() {
        let inputs = [
            r#"{}"#,
            r#"{   }"#,
            r#"{a = 1e165}"#,
            r#"{ hello = "world", a = 1}"#,
            r#"{ hello.world = "a" }"#,
        ];
        for input in inputs {
            dbg!(input);
            let mut parsed = inline_table.parse(new_input(input));
            if let Ok(parsed) = &mut parsed {
                parsed.despan(input);
            }
            assert_eq!(parsed.map(|a| a.to_string()), Ok(input.to_owned()));
        }
    }

    #[test]
    fn invalid_inline_tables() {
        let invalid_inputs = [r#"{a = 1e165"#, r#"{ hello = "world", a = 2, hello = 1}"#];
        for input in invalid_inputs {
            dbg!(input);
            let mut parsed = inline_table.parse(new_input(input));
            if let Ok(parsed) = &mut parsed {
                parsed.despan(input);
            }
            assert!(parsed.is_err());
        }
    }
}
