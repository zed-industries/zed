use crate::{FromQueryResult, SelectColumns};

/// A trait for a part of [Model](super::model::ModelTrait)
pub trait PartialModelTrait: FromQueryResult {
    /// Select specific columns this [PartialModel] needs
    ///
    /// If you are implementing this by hand, please make sure to read the hints in the
    /// documentation for `select_cols_nested` and ensure to implement both methods.
    fn select_cols<S: SelectColumns>(select: S) -> S;

    /// Used when nesting these structs into each other.
    ///
    /// This will stop being a provided method in a future major release.
    /// Please implement this method manually when implementing this trait by hand,
    /// and ensure that your `select_cols` implementation is calling it with `_prefix` as `None`.
    fn select_cols_nested<S: SelectColumns>(select: S, _prefix: Option<&str>) -> S {
        Self::select_cols(select)
    }
}

impl<T: PartialModelTrait> PartialModelTrait for Option<T> {
    fn select_cols<S: SelectColumns>(select: S) -> S {
        Self::select_cols_nested(select, None)
    }

    fn select_cols_nested<S: SelectColumns>(select: S, prefix: Option<&str>) -> S {
        T::select_cols_nested(select, prefix)
    }
}
