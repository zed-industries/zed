// Mangled symbol arrangements:
//
//   (a) One-off internal symbol.
//          pattern:  {CXXBRIDGE} $ {NAME}
//          examples:
//             - cxxbridge1$exception
//          defining characteristics:
//             - 2 segments
//             - starts with cxxbridge
//
//   (b) Behavior on a builtin binding without generic parameter.
//          pattern:  {CXXBRIDGE} $ {TYPE} $ {NAME}
//          examples:
//             - cxxbridge1$string$len
//          defining characteristics:
//             - 3 segments
//             - starts with cxxbridge
//
//   (c) Behavior on a builtin binding with generic parameter.
//          pattern:  {CXXBRIDGE} $ {TYPE} $ {PARAM...} $ {NAME}
//          examples:
//             - cxxbridge1$box$org$rust$Struct$alloc
//             - cxxbridge1$unique_ptr$std$vector$u8$drop
//          defining characteristics:
//             - 4+ segments
//             - starts with cxxbridge
//
//   (d) User-defined extern function.
//          pattern:  {NAMESPACE...} $ {CXXBRIDGE} $ {NAME}
//          examples:
//             - cxxbridge1$new_client
//             - org$rust$cxxbridge1$new_client
//          defining characteristics:
//             - cxxbridge is second from end
//          FIXME: conflict with (a) if they collide with one of our one-off symbol names in the global namespace
//
//   (e) User-defined extern member function.
//          pattern:  {NAMESPACE...} $ {CXXBRIDGE} $ {TYPE} $ {NAME}
//          examples:
//             - org$cxxbridge1$Struct$get
//          defining characteristics:
//             - cxxbridge is third from end
//          FIXME: conflict with (b) if e.g. user binds a type in global namespace that collides with our builtin type names
//
//   (f) Operator overload.
//          pattern:  {NAMESPACE...} $ {CXXBRIDGE} $ {TYPE} $ operator $ {NAME}
//          examples:
//             - org$rust$cxxbridge1$Struct$operator$eq
//          defining characteristics:
//             - second segment from end is `operator` (not possible in type or namespace names)
//
//   (g) Closure trampoline.
//          pattern:  {NAMESPACE...} $ {CXXBRIDGE} $ {TYPE?} $ {NAME} $ {ARGUMENT} $ {DIRECTION}
//          examples:
//             - org$rust$cxxbridge1$Struct$invoke$f$0
//          defining characteristics:
//             - last symbol is `0` (C half) or `1` (Rust half) which are not legal identifiers on their own
//
//
// Mangled preprocessor variable arrangements:
//
//   (A) One-off internal variable.
//          pattern:  {CXXBRIDGE} _ {NAME}
//          examples:
//             - CXXBRIDGE1_PANIC
//             - CXXBRIDGE1_RUST_STRING
//          defining characteristics:
//             - NAME does not begin with STRUCT or ENUM
//
//   (B) Guard around user-defined type.
//          pattern:  {CXXBRIDGE} _ {STRUCT or ENUM} _ {NAMESPACE...} $ {TYPE}
//          examples:
//             - CXXBRIDGE1_STRUCT_org$rust$Struct
//             - CXXBRIDGE1_ENUM_Enabled

use crate::syntax::symbol::{self, Symbol};
use crate::syntax::{ExternFn, Pair, Types};

const CXXBRIDGE: &str = "cxxbridge1";

macro_rules! join {
    ($($segment:expr),+ $(,)?) => {
        symbol::join(&[$(&$segment),+])
    };
}

pub(crate) fn extern_fn(efn: &ExternFn, types: &Types) -> Symbol {
    match efn.self_type() {
        Some(self_type) => {
            let self_type_ident = types.resolve(self_type);
            join!(
                efn.name.namespace,
                CXXBRIDGE,
                self_type_ident.name.cxx,
                efn.name.rust,
            )
        }
        None => join!(efn.name.namespace, CXXBRIDGE, efn.name.rust),
    }
}

pub(crate) fn operator(receiver: &Pair, operator: &'static str) -> Symbol {
    join!(
        receiver.namespace,
        CXXBRIDGE,
        receiver.cxx,
        "operator",
        operator,
    )
}

// The C half of a function pointer trampoline.
pub(crate) fn c_trampoline(efn: &ExternFn, var: &Pair, types: &Types) -> Symbol {
    join!(extern_fn(efn, types), var.rust, 0)
}

// The Rust half of a function pointer trampoline.
pub(crate) fn r_trampoline(efn: &ExternFn, var: &Pair, types: &Types) -> Symbol {
    join!(extern_fn(efn, types), var.rust, 1)
}
