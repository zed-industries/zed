/// Chains calls to `LazySelect::lazy_select` to mimic an if-else chain.
///
/// ```ignore
/// let result = lazy_select! {
///     if predicate1 => result1,
///     if predicate2 => result2,
///     else => result3,
/// };
/// ```
macro_rules! lazy_select {
    ( if $if_pred:expr => $if_body:expr, $(if $else_if_pred:expr => $else_if_body:expr,)* else => $else_body:expr $(,)?) => {
        crate::bool_mask::LazySelect::lazy_select(
            $if_pred,
            || $if_body,
            || lazy_select!($(if $else_if_pred => $else_if_body,)* else => $else_body)
        )
    };
    (else => $else_body:expr) => {
        $else_body
    }
}
