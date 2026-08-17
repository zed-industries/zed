use alloc::vec;

use super::Error;
use crate::arena::{Arena, Handle};

/// Create a default value for an output built-in.
pub fn generate_default_built_in(
    built_in: Option<crate::BuiltIn>,
    ty: Handle<crate::Type>,
    global_expressions: &mut Arena<crate::Expression>,
    span: crate::Span,
) -> Result<Handle<crate::Expression>, Error> {
    let expr = match built_in {
        Some(crate::BuiltIn::Position { .. }) => {
            let zero = global_expressions
                .append(crate::Expression::Literal(crate::Literal::F32(0.0)), span);
            let one = global_expressions
                .append(crate::Expression::Literal(crate::Literal::F32(1.0)), span);
            crate::Expression::Compose {
                ty,
                components: vec![zero, zero, zero, one],
            }
        }
        Some(crate::BuiltIn::PointSize) => crate::Expression::Literal(crate::Literal::F32(1.0)),
        Some(crate::BuiltIn::FragDepth) => crate::Expression::Literal(crate::Literal::F32(0.0)),
        Some(crate::BuiltIn::SampleMask) => {
            crate::Expression::Literal(crate::Literal::U32(u32::MAX))
        }
        // Note: `crate::BuiltIn::ClipDistance` is intentionally left for the default path
        _ => crate::Expression::ZeroValue(ty),
    };
    Ok(global_expressions.append(expr, span))
}
