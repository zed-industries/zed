use super::reject_attrs;
use crate::util::prelude::*;

/// At the time of this writing `Rust` doesn't support calling closures in `const`
/// contexts. This doesn't compile:
///
/// ```compile_fail
/// const { (|| {})() }
/// ```
///
/// However, we need this to work because we need to wrap user-supplied expressions
/// with closures. It gives us a guarantee that the user-supplied expression can't
/// break out of the surrounding scope via a `return`, `break`, or `continue` inside
/// of it.
///
/// Therefore, we use this function to allow very simple expressions to still be
/// embedded in the surrounding context while recommending to delegate to a function
/// call if more complex expression is required.
pub(crate) fn require_embeddable_const_expr(expr: &syn::Expr) -> Result {
    use require_embeddable_const_expr as recurse;

    fn recurse_block(block: &syn::Block) -> Result {
        let stmt = match block.stmts.as_slice() {
            [stmt] => stmt,
            [] => return Ok(()),
            _ => bail!(
                &block,
                "only a single expression in a block is allowed in this position"
            ),
        };

        match stmt {
            syn::Stmt::Expr(expr, None) => recurse(expr),
            _ => bail!(
                &stmt,
                "this kind of statement is not allowed in this position; \
                only a single expression is allowed in a block is allowed here"
            ),
        }
    }

    use syn::Expr::*;

    match expr {
        Array(arr) => {
            reject_attrs(&arr.attrs)?;
            arr.elems.iter().try_for_each(recurse)?;
        }
        Binary(binary) => {
            reject_attrs(&binary.attrs)?;
            recurse(&binary.left)?;
            recurse(&binary.right)?;
        }
        Block(block) => {
            reject_attrs(&block.attrs)?;
            recurse_block(&block.block)?;
        }
        Call(call) => {
            reject_attrs(&call.attrs)?;
            recurse(&call.func)?;
            call.args.iter().try_for_each(recurse)?;
        }
        Cast(cast) => {
            reject_attrs(&cast.attrs)?;
            recurse(&cast.expr)?;
        }
        Field(field) => {
            reject_attrs(&field.attrs)?;
            recurse(&field.base)?;
        }
        Group(group) => {
            reject_attrs(&group.attrs)?;
            recurse(&group.expr)?;
        }
        If(if_expr) => {
            reject_attrs(&if_expr.attrs)?;
            recurse(&if_expr.cond)?;
            recurse_block(&if_expr.then_branch)?;
            if let Some((_else, else_branch)) = &if_expr.else_branch {
                recurse(else_branch)?;
            }
        }
        Const(const_block) => {
            reject_attrs(&const_block.attrs)?;
            recurse_block(&const_block.block)?;
        }
        Index(index) => {
            reject_attrs(&index.attrs)?;
            recurse(&index.expr)?;
            recurse(&index.index)?;
        }
        Infer(infer) => reject_attrs(&infer.attrs)?,
        Lit(lit) => reject_attrs(&lit.attrs)?,
        Loop(loop_expr) => {
            reject_attrs(&loop_expr.attrs)?;
            recurse_block(&loop_expr.body)?;
        }
        Match(expr_match) => {
            reject_attrs(&expr_match.attrs)?;
            recurse(&expr_match.expr)?;
            expr_match.arms.iter().try_for_each(|arm| {
                reject_attrs(&arm.attrs)?;

                if let Some((_if, guard)) = &arm.guard {
                    recurse(guard)?;
                }

                recurse(&arm.body)
            })?;
        }
        MethodCall(method_call) => {
            reject_attrs(&method_call.attrs)?;
            recurse(&method_call.receiver)?;
            method_call.args.iter().try_for_each(recurse)?;
        }
        Paren(paren) => {
            reject_attrs(&paren.attrs)?;
            recurse(&paren.expr)?;
        }
        Path(path) => reject_attrs(&path.attrs)?,
        Range(range) => {
            reject_attrs(&range.attrs)?;
            if let Some(start) = &range.start {
                recurse(start)?;
            }
            if let Some(end) = &range.end {
                recurse(end)?;
            }
        }

        Reference(reference) => {
            reject_attrs(&reference.attrs)?;
            recurse(&reference.expr)?;
        }
        Repeat(repeat) => {
            reject_attrs(&repeat.attrs)?;
            recurse(&repeat.expr)?;
            recurse(&repeat.len)?;
        }
        Struct(struct_expr) => {
            reject_attrs(&struct_expr.attrs)?;
            struct_expr.fields.iter().try_for_each(|field| {
                reject_attrs(&field.attrs)?;
                recurse(&field.expr)
            })?;
            if let Some(rest) = &struct_expr.rest {
                recurse(rest)?;
            }
        }
        Tuple(tuple) => {
            reject_attrs(&tuple.attrs)?;
            tuple.elems.iter().try_for_each(recurse)?;
        }
        Unary(unary) => {
            reject_attrs(&unary.attrs)?;
            recurse(&unary.expr)?;
        }
        Unsafe(unsafe_expr) => {
            reject_attrs(&unsafe_expr.attrs)?;
            recurse_block(&unsafe_expr.block)?;
        }
        While(while_expr) => {
            reject_attrs(&while_expr.attrs)?;
            recurse(&while_expr.cond)?;
            recurse_block(&while_expr.body)?;
        }
        _ => {
            bail!(
                &expr,
                "this kind of expression is not allowed in this position; \
                if you need to use a complex expression such as this then \
                move it into a separate `const fn` and call that function \
                here instead"
            )
        }
    }

    Ok(())
}
