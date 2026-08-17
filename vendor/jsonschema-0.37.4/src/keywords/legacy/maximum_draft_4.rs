use crate::{
    compiler,
    keywords::{minmax, CompilationResult},
};
use serde_json::{Map, Value};

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    parent: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    if let Some(Value::Bool(true)) = parent.get("exclusiveMaximum") {
        minmax::compile_exclusive_maximum(ctx, parent, schema)
    } else {
        minmax::compile_maximum(ctx, parent, schema)
    }
}
