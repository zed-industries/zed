/*!
Interpolation defaults.
*/

impl crate::Binding {
    /// Apply the usual default interpolation for `ty` to `binding`.
    ///
    /// This function is a utility front ends may use to satisfy the Naga IR's
    /// requirement, meant to ensure that input languages' policies have been
    /// applied appropriately, that all I/O `Binding`s from the vertex shader to the
    /// fragment shader must have non-`None` `interpolation` values.
    ///
    /// All the shader languages Naga supports have similar rules:
    /// perspective-correct, center-sampled interpolation is the default for any
    /// binding that can vary, and everything else either defaults to flat, or
    /// requires an explicit flat qualifier/attribute/what-have-you.
    ///
    /// If `binding` is not a [`Location`] binding, or if its [`interpolation`] is
    /// already set, then make no changes. Otherwise, set `binding`'s interpolation
    /// and sampling to reasonable defaults depending on `ty`, the type of the value
    /// being interpolated:
    ///
    /// - If `ty` is a floating-point scalar, vector, or matrix type, then
    ///   default to [`Perspective`] interpolation and [`Center`] sampling.
    ///
    /// - If `ty` is an integral scalar or vector, then default to [`Flat`]
    ///   interpolation, which has no associated sampling.
    ///
    /// - For any other types, make no change. Such types are not permitted as
    ///   user-defined IO values, and will probably be flagged by the verifier
    ///
    /// When structs appear in input or output types, each member ought to have its
    /// own [`Binding`], so structs are simply covered by the third case.
    ///
    /// [`Binding`]: crate::Binding
    /// [`Location`]: crate::Binding::Location
    /// [`interpolation`]: crate::Binding::Location::interpolation
    /// [`Perspective`]: crate::Interpolation::Perspective
    /// [`Flat`]: crate::Interpolation::Flat
    /// [`Center`]: crate::Sampling::Center
    pub fn apply_default_interpolation(&mut self, ty: &crate::TypeInner) {
        if let crate::Binding::Location {
            location: _,
            interpolation: ref mut interpolation @ None,
            ref mut sampling,
            blend_src: _,
            per_primitive: _,
        } = *self
        {
            match ty.scalar_kind() {
                Some(crate::ScalarKind::Float) => {
                    *interpolation = Some(crate::Interpolation::Perspective);
                    *sampling = Some(crate::Sampling::Center);
                }
                Some(crate::ScalarKind::Sint | crate::ScalarKind::Uint) => {
                    *interpolation = Some(crate::Interpolation::Flat);
                    *sampling = None;
                }
                Some(_) | None => {}
            }
        }
    }
}
