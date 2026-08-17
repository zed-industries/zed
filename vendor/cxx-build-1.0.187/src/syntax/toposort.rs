use crate::syntax::map::{Entry, UnorderedMap as Map};
use crate::syntax::report::Errors;
use crate::syntax::{Api, Struct, Type, Types};

enum Mark {
    Visiting,
    Visited,
}

pub(crate) fn sort<'a>(cx: &mut Errors, apis: &'a [Api], types: &Types<'a>) -> Vec<&'a Struct> {
    let mut sorted = Vec::new();
    let ref mut marks = Map::new();
    for api in apis {
        if let Api::Struct(strct) = api {
            let _ = visit(cx, strct, &mut sorted, marks, types);
        }
    }
    sorted
}

fn visit<'a>(
    cx: &mut Errors,
    strct: &'a Struct,
    sorted: &mut Vec<&'a Struct>,
    marks: &mut Map<*const Struct, Mark>,
    types: &Types<'a>,
) -> Result<(), ()> {
    match marks.entry(strct) {
        Entry::Occupied(entry) => match entry.get() {
            Mark::Visiting => return Err(()), // not a DAG
            Mark::Visited => return Ok(()),
        },
        Entry::Vacant(entry) => {
            entry.insert(Mark::Visiting);
        }
    }
    let mut result = Ok(());
    for field in &strct.fields {
        if let Type::Ident(ident) = &field.ty {
            if let Some(inner) = types.structs.get(&ident.rust) {
                if visit(cx, inner, sorted, marks, types).is_err() {
                    cx.error(field, "unsupported cyclic data structure");
                    result = Err(());
                }
            }
        }
    }
    marks.insert(strct, Mark::Visited);
    sorted.push(strct);
    result
}
