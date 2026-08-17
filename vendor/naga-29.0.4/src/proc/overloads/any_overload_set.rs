//! Dynamically dispatched [`OverloadSet`]s.

use crate::common::DiagnosticDebug;
use crate::ir;
use crate::proc::overloads::{list, regular, OverloadSet, Rule};
use crate::proc::{GlobalCtx, TypeResolution};

use alloc::vec::Vec;
use core::fmt;

macro_rules! define_any_overload_set {
    { $( $module:ident :: $name:ident, )* } => {
        /// An [`OverloadSet`] that dynamically dispatches to concrete implementations.
        #[derive(Clone)]
        pub(in crate::proc::overloads) enum AnyOverloadSet {
            $(
                $name ( $module :: $name ),
            )*
        }

        $(
            impl From<$module::$name> for AnyOverloadSet {
                fn from(concrete: $module::$name) -> Self {
                    AnyOverloadSet::$name(concrete)
                }
            }
        )*

        impl OverloadSet for AnyOverloadSet {
            fn is_empty(&self) -> bool {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.is_empty(),
                    )*
                }
            }

            fn min_arguments(&self) -> usize {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.min_arguments(),
                    )*
                }
            }

            fn max_arguments(&self) -> usize {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.max_arguments(),
                    )*
                }
            }

            fn arg(
                &self,
                i: usize,
                ty: &ir::TypeInner,
                types: &crate::UniqueArena<ir::Type>,
            ) -> Self {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => AnyOverloadSet::$name(x.arg(i, ty, types)),
                    )*
                }
            }

            fn concrete_only(self, types: &crate::UniqueArena<ir::Type>) -> Self {
                match self {
                    $(
                        AnyOverloadSet::$name(x) => AnyOverloadSet::$name(x.concrete_only(types)),
                    )*
                }
            }

            fn most_preferred(&self) -> Rule {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.most_preferred(),
                    )*
                }
            }

            fn overload_list(&self, gctx: &GlobalCtx<'_>) -> Vec<Rule> {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.overload_list(gctx),
                    )*
                }
            }

            fn allowed_args(&self, i: usize, gctx: &GlobalCtx<'_>) -> Vec<TypeResolution> {
                match *self {
                    $(
                        AnyOverloadSet::$name(ref x) => x.allowed_args(i, gctx),
                    )*
                }
            }

            fn for_debug(&self, types: &crate::UniqueArena<ir::Type>) -> impl fmt::Debug {
                DiagnosticDebug((self, types))
            }
        }

        impl fmt::Debug for DiagnosticDebug<(&AnyOverloadSet, &crate::UniqueArena<ir::Type>)> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let (set, types) = self.0;
                match *set {
                    $(
                        AnyOverloadSet::$name(ref x) => DiagnosticDebug((x, types)).fmt(f),
                    )*
                }
            }
        }
    }
}

define_any_overload_set! {
    list::List,
    regular::Regular,
}
