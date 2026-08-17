//! Utilities for working with and combining the results of
//! [`Arbitrary::size_hint`][crate::Arbitrary::size_hint].

pub(crate) const MAX_DEPTH: usize = 20;

/// Protects against potential infinite recursion when calculating size hints
/// due to indirect type recursion.
///
/// When the depth is not too deep, calls `f` with `depth + 1` to calculate the
/// size hint.
///
/// Otherwise, returns the default size hint: `(0, None)`.
///
/// <div class="warning">This method is deprecated. Users should instead implement <a href="../trait.Arbitrary.html#method.try_size_hint"><code>try_size_hint</code></a> and use <a href="fn.try_recursion_guard.html"><code>try_recursion_guard</code></a></div>
#[inline]
#[deprecated(note = "use `try_recursion_guard` instead")]
pub fn recursion_guard(
    depth: usize,
    f: impl FnOnce(usize) -> (usize, Option<usize>),
) -> (usize, Option<usize>) {
    if depth > MAX_DEPTH {
        (0, None)
    } else {
        f(depth + 1)
    }
}

/// Protects against potential infinite recursion when calculating size hints
/// due to indirect type recursion.
///
/// When the depth is not too deep, calls `f` with `depth + 1` to calculate the
/// size hint.
///
/// Otherwise, returns an error.
///
/// This should be used when implementing [`try_size_hint`](crate::Arbitrary::try_size_hint)
#[inline]
pub fn try_recursion_guard(
    depth: usize,
    f: impl FnOnce(usize) -> Result<(usize, Option<usize>), crate::MaxRecursionReached>,
) -> Result<(usize, Option<usize>), crate::MaxRecursionReached> {
    if depth > MAX_DEPTH {
        Err(crate::MaxRecursionReached {})
    } else {
        f(depth + 1)
    }
}

/// Take the sum of the `lhs` and `rhs` size hints.
#[inline]
pub fn and(lhs: (usize, Option<usize>), rhs: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower = lhs.0 + rhs.0;
    let upper = lhs.1.and_then(|lhs| rhs.1.map(|rhs| lhs + rhs));
    (lower, upper)
}

/// Take the sum of all of the given size hints.
///
/// If `hints` is empty, returns `(0, Some(0))`, aka the size of consuming
/// nothing.
#[inline]
pub fn and_all(hints: &[(usize, Option<usize>)]) -> (usize, Option<usize>) {
    hints.iter().copied().fold((0, Some(0)), and)
}

/// Take the minimum of the lower bounds and maximum of the upper bounds in the
/// `lhs` and `rhs` size hints.
#[inline]
pub fn or(lhs: (usize, Option<usize>), rhs: (usize, Option<usize>)) -> (usize, Option<usize>) {
    let lower = std::cmp::min(lhs.0, rhs.0);
    let upper = lhs
        .1
        .and_then(|lhs| rhs.1.map(|rhs| std::cmp::max(lhs, rhs)));
    (lower, upper)
}

/// Take the maximum of the `lhs` and `rhs` size hints.
///
/// If `hints` is empty, returns `(0, Some(0))`, aka the size of consuming
/// nothing.
#[inline]
pub fn or_all(hints: &[(usize, Option<usize>)]) -> (usize, Option<usize>) {
    if let Some(head) = hints.first().copied() {
        hints[1..].iter().copied().fold(head, or)
    } else {
        (0, Some(0))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn and() {
        assert_eq!((5, Some(5)), super::and((2, Some(2)), (3, Some(3))));
        assert_eq!((5, None), super::and((2, Some(2)), (3, None)));
        assert_eq!((5, None), super::and((2, None), (3, Some(3))));
        assert_eq!((5, None), super::and((2, None), (3, None)));
    }

    #[test]
    fn or() {
        assert_eq!((2, Some(3)), super::or((2, Some(2)), (3, Some(3))));
        assert_eq!((2, None), super::or((2, Some(2)), (3, None)));
        assert_eq!((2, None), super::or((2, None), (3, Some(3))));
        assert_eq!((2, None), super::or((2, None), (3, None)));
    }

    #[test]
    fn and_all() {
        assert_eq!((0, Some(0)), super::and_all(&[]));
        assert_eq!(
            (7, Some(7)),
            super::and_all(&[(1, Some(1)), (2, Some(2)), (4, Some(4))])
        );
        assert_eq!(
            (7, None),
            super::and_all(&[(1, Some(1)), (2, Some(2)), (4, None)])
        );
        assert_eq!(
            (7, None),
            super::and_all(&[(1, Some(1)), (2, None), (4, Some(4))])
        );
        assert_eq!(
            (7, None),
            super::and_all(&[(1, None), (2, Some(2)), (4, Some(4))])
        );
    }

    #[test]
    fn or_all() {
        assert_eq!((0, Some(0)), super::or_all(&[]));
        assert_eq!(
            (1, Some(4)),
            super::or_all(&[(1, Some(1)), (2, Some(2)), (4, Some(4))])
        );
        assert_eq!(
            (1, None),
            super::or_all(&[(1, Some(1)), (2, Some(2)), (4, None)])
        );
        assert_eq!(
            (1, None),
            super::or_all(&[(1, Some(1)), (2, None), (4, Some(4))])
        );
        assert_eq!(
            (1, None),
            super::or_all(&[(1, None), (2, Some(2)), (4, Some(4))])
        );
    }
}
