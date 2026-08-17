//! Displaying Naga IR terms in diagnostic output.

use crate::proc::{GlobalCtx, Rule, TypeResolution};
use crate::{Handle, Scalar, Type};

#[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
use crate::common::wgsl::TypeContext;

use core::fmt;

/// A wrapper for displaying Naga IR terms in diagnostic output.
///
/// For some Naga IR type `T`, `DiagnosticDisplay<T>` implements
/// [`core::fmt::Display`] in a way that displays values of type `T`
/// appropriately for diagnostic messages presented to human readers.
///
/// For example, the implementation of [`Display`] for
/// `DiagnosticDisplay<Scalar>` formats the type represented by the
/// given [`Scalar`] appropriately for users.
///
/// Some types like `Handle<Type>` require contextual information like
/// a type arena to be displayed. In such cases, we implement [`Display`]
/// for a type like `DiagnosticDisplay<(Handle<Type>, GlobalCtx)>`, where
/// the [`GlobalCtx`] type provides the necessary context.
///
/// Do not implement this type for [`TypeInner`], as that does not
/// have enough information to display struct types correctly.
///
/// If you only need debugging output, [`DiagnosticDebug`] uses
/// easier-to-obtain context types but still does a good enough job
/// for logging or debugging.
///
/// [`Display`]: core::fmt::Display
/// [`GlobalCtx`]: crate::proc::GlobalCtx
/// [`TypeInner`]: crate::ir::TypeInner
/// [`DiagnosticDebug`]: super::DiagnosticDebug
///
/// ## Language-sensitive diagnostics
///
/// Diagnostic output ought to depend on the source language from
/// which the IR was produced: diagnostics resulting from processing
/// GLSL code should use GLSL type syntax, for example. That means
/// that `DiagnosticDisplay` ought to include some indication of which
/// notation to use.
///
/// For the moment, only WGSL output is implemented, so
/// `DiagnosticDisplay` lacks any support for this (#7268). However,
/// the plan is that all language-independent code in Naga should use
/// `DiagnosticDisplay` wherever appropriate, such that when its
/// definition is expanded to include some indication of the right
/// source language to use, any use site that does not supply this
/// indication will provoke a compile-time error.
pub struct DiagnosticDisplay<T>(pub T);

impl fmt::Display for DiagnosticDisplay<(&TypeResolution, GlobalCtx<'_>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (resolution, ctx) = self.0;

        #[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
        ctx.write_type_resolution(resolution, f)?;

        #[cfg(not(any(feature = "wgsl-in", feature = "wgsl-out")))]
        {
            let _ = ctx;
            write!(f, "{resolution:?}")?;
        }

        Ok(())
    }
}

impl fmt::Display for DiagnosticDisplay<(Handle<Type>, GlobalCtx<'_>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (handle, ref ctx) = self.0;

        #[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
        ctx.write_type(handle, f)?;

        #[cfg(not(any(feature = "wgsl-in", feature = "wgsl-out")))]
        {
            let _ = ctx;
            write!(f, "{handle:?}")?;
        }

        Ok(())
    }
}

impl fmt::Display for DiagnosticDisplay<(&str, &Rule, GlobalCtx<'_>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, rule, ref ctx) = self.0;

        #[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
        ctx.write_type_rule(name, rule, f)?;

        #[cfg(not(any(feature = "wgsl-in", feature = "wgsl-out")))]
        {
            let _ = ctx;
            write!(f, "{name}({:?}) -> {:?}", rule.arguments, rule.conclusion)?;
        }

        Ok(())
    }
}

impl fmt::Display for DiagnosticDisplay<Scalar> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scalar = self.0;

        #[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
        f.write_str(&crate::common::wgsl::TryToWgsl::to_wgsl_for_diagnostics(
            scalar,
        ))?;

        #[cfg(not(any(feature = "wgsl-in", feature = "wgsl-out")))]
        write!(f, "{scalar:?}")?;

        Ok(())
    }
}
