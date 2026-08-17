/*! Type rules.

An implementation of [`OverloadSet`] represents a set of type rules, each of
which has a list of types for its arguments, and a conclusion about the
type of the expression as a whole.

This module defines the [`Rule`] type, representing a type rule from an
[`OverloadSet`], and the [`Conclusion`] type, a specialized enum for
representing a type rule's conclusion.

[`OverloadSet`]: crate::proc::overloads::OverloadSet

*/

use crate::common::{DiagnosticDebug, ForDebugWithTypes};
use crate::ir;
use crate::proc::overloads::constructor_set::ConstructorSize;
use crate::proc::TypeResolution;
use crate::UniqueArena;

use alloc::vec::Vec;
use core::fmt;
use core::result::Result;

/// A single type rule.
#[derive(Clone)]
pub struct Rule {
    pub arguments: Vec<TypeResolution>,
    pub conclusion: Conclusion,
}

/// The result type of a [`Rule`].
///
/// A `Conclusion` value represents the return type of some operation
/// in the builtin function database.
///
/// This is very similar to [`TypeInner`], except that it represents
/// predeclared types using [`PredeclaredType`], so that overload
/// resolution can delegate registering predeclared types to its users.
///
/// [`TypeInner`]: ir::TypeInner
/// [`PredeclaredType`]: ir::PredeclaredType
#[derive(Clone, Debug)]
pub enum Conclusion {
    /// A type that can be entirely characterized by a [`TypeInner`] value.
    ///
    /// [`TypeInner`]: ir::TypeInner
    Value(ir::TypeInner),

    /// A type that should be registered in the module's
    /// [`SpecialTypes::predeclared_types`] table.
    ///
    /// This is used for operations like [`Frexp`] and [`Modf`].
    ///
    /// [`SpecialTypes::predeclared_types`]: ir::SpecialTypes::predeclared_types
    /// [`Frexp`]: crate::ir::MathFunction::Frexp
    /// [`Modf`]: crate::ir::MathFunction::Modf
    Predeclared(ir::PredeclaredType),
}

impl Conclusion {
    pub fn for_frexp_modf(
        function: ir::MathFunction,
        size: ConstructorSize,
        scalar: ir::Scalar,
    ) -> Self {
        use ir::MathFunction as Mf;
        use ir::PredeclaredType as Pt;

        let size = match size {
            ConstructorSize::Scalar => None,
            ConstructorSize::Vector(size) => Some(size),
            ConstructorSize::Matrix { .. } => {
                unreachable!("FrexpModf only supports scalars and vectors");
            }
        };

        let predeclared = match function {
            Mf::Frexp => Pt::FrexpResult { size, scalar },
            Mf::Modf => Pt::ModfResult { size, scalar },
            _ => {
                unreachable!("FrexpModf only supports Frexp and Modf");
            }
        };

        Conclusion::Predeclared(predeclared)
    }

    pub fn into_resolution(
        self,
        special_types: &ir::SpecialTypes,
    ) -> Result<TypeResolution, MissingSpecialType> {
        match self {
            Conclusion::Value(inner) => Ok(TypeResolution::Value(inner)),
            Conclusion::Predeclared(predeclared) => {
                let handle = *special_types
                    .predeclared_types
                    .get(&predeclared)
                    .ok_or(MissingSpecialType)?;
                Ok(TypeResolution::Handle(handle))
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Special type is not registered within the module")]
pub struct MissingSpecialType;

impl ForDebugWithTypes for &Rule {}

impl fmt::Debug for DiagnosticDebug<(&Rule, &UniqueArena<ir::Type>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rule, arena) = self.0;
        f.write_str("(")?;
        for (i, argument) in rule.arguments.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{:?}", argument.for_debug(arena))?;
        }
        write!(f, ") -> {:?}", rule.conclusion.for_debug(arena))
    }
}

impl ForDebugWithTypes for &Conclusion {}

impl fmt::Debug for DiagnosticDebug<(&Conclusion, &UniqueArena<ir::Type>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (conclusion, ctx) = self.0;

        #[cfg(any(feature = "wgsl-in", feature = "wgsl-out"))]
        {
            use crate::common::wgsl::TypeContext;
            ctx.write_type_conclusion(conclusion, f)?;
        }

        #[cfg(not(any(feature = "wgsl-in", feature = "wgsl-out")))]
        {
            let _ = ctx;
            write!(f, "{conclusion:?}")?;
        }

        Ok(())
    }
}
