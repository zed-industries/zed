/// Subtraction for a type which saturates at numeric bounds.
pub trait SaturatingSub<Rhs = Self> {
    type Output;

    /// Computes `a - b`, saturating at numeric bounds.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn saturating_sub(self, other: Rhs) -> Self::Output;
}
