// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides error messages and some checkers.

use std::fmt::Display;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::attr::ParsedAttributes;
use syn;

//==============================================================================
// Item descriptions
//==============================================================================

/// Item name of structs.
pub const STRUCT: &str = "struct";

/// Item name of struct fields.
pub const STRUCT_FIELD: &str = "struct field";

/// Item name of enums.
pub const ENUM: &str = "enum";

/// Item name of enum variants.
pub const ENUM_VARIANT: &str = "enum variant";

/// Item name of enum variant fields.
pub const ENUM_VARIANT_FIELD: &str = "enum variant field";

/// Item name for a type variable.
pub const TY_VAR: &str = "a type variable";

//==============================================================================
// Checkers
//==============================================================================

/// Ensures that the type is not parametric over lifetimes.
pub fn if_has_lifetimes(ctx: Ctx, ast: &syn::DeriveInput) {
    if ast.generics.lifetimes().count() > 0 {
        has_lifetimes(ctx);
    }
}

/// Ensures that no attributes were specified on `item`.
pub fn if_anything_specified(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if_enum_attrs_present(ctx, attrs, item);
    if_strategy_present(ctx, attrs, item);
    if_specified_params(ctx, attrs, item);
    if_specified_filter(ctx, attrs, item);
}

/// Ensures that things only allowed on an enum variant is not present on
/// `item` which is not an enum variant.
pub fn if_enum_attrs_present(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if_skip_present(ctx, attrs, item);
    if_weight_present(ctx, attrs, item);
}

/// Ensures that parameters is not present on `item`.
pub fn if_specified_filter(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if !attrs.filter.is_empty() {
        meaningless_filter(ctx, item);
    }
}

/// Ensures that parameters is not present on `item`.
pub fn if_specified_params(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if attrs.params.is_set() {
        parent_has_param(ctx, item);
    }
}

/// Ensures that an explicit strategy or value is not present on `item`.
pub fn if_strategy_present(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    use crate::attr::StratMode::*;
    match attrs.strategy {
        Arbitrary => {}
        Strategy(_) => illegal_strategy(ctx, "strategy", item),
        Value(_) => illegal_strategy(ctx, "value", item),
        Regex(_) => illegal_regex(ctx, item),
    }
}

/// Ensures that a strategy, value, params, filter is not present on a unit variant.
pub fn if_present_on_unit_variant(ctx: Ctx, attrs: &ParsedAttributes) {
    /// Ensures that an explicit strategy or value is not present on a unit variant.
    use crate::attr::StratMode::*;
    match attrs.strategy {
        Arbitrary => {}
        Strategy(_) => strategy_on_unit_variant(ctx, "strategy"),
        Value(_) => strategy_on_unit_variant(ctx, "value"),
        Regex(_) => regex_on_unit_variant(ctx),
    }

    if attrs.params.is_set() {
        params_on_unit_variant(ctx)
    }

    if !attrs.filter.is_empty() {
        filter_on_unit_variant(ctx)
    }
}

/// Ensures that parameters or filter is not present on a unit struct.
pub fn if_present_on_unit_struct(ctx: Ctx, attrs: &ParsedAttributes) {
    if attrs.params.is_set() {
        params_on_unit_struct(ctx)
    }

    if !attrs.filter.is_empty() {
        filter_on_unit_struct(ctx)
    }
}

/// Ensures that skip is not present on `item`.
pub fn if_skip_present(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if attrs.skip {
        illegal_skip(ctx, item)
    }
}

/// Ensures that a weight is not present on `item`.
pub fn if_weight_present(ctx: Ctx, attrs: &ParsedAttributes, item: &str) {
    if attrs.weight.is_some() {
        illegal_weight(ctx, item)
    }
}

//==============================================================================
// Messages
//==============================================================================

/// Denotes that a fatal error happened in dealing somewhere in the
/// procedural macro pipeline. A fatal error is different from a
/// normal error in the sense that it halts progress in the macro
/// immediately instead of allowing other errors to be accumulated.
#[derive(Debug)]
pub struct Fatal;

/// The return type of a possibly fatal computation in the macro.
pub type DeriveResult<T> = Result<T, Fatal>;

/// A mutable view / shorthand for the context.
/// Prefer this type over `Context` in functions.
pub type Ctx<'ctx> = &'ctx mut Context;

/// The context / environment that the macro is operating in.
/// Right now, it simply tracks all the errors collected during
/// the running of the macro.
#[derive(Default)]
pub struct Context {
    errors: Vec<String>,
}

impl Context {
    /// Add a non-fatal error to the context.
    pub fn error<T: Display>(&mut self, msg: T) {
        self.errors.push(msg.to_string());
    }

    /// Add an error to the context and produce an erroring
    /// computation that will halt the macro.
    pub fn fatal<T: Display, A>(&mut self, msg: T) -> DeriveResult<A> {
        self.error(msg);
        Err(Fatal)
    }

    /// Consume the context and if there were any errors,
    /// emit `compile_error!(..)` such that the crate using
    /// `#[derive(Arbitrary)]` will fail to compile.
    pub fn check(mut self) -> Result<(), TokenStream> {
        fn compile_error(msg: &str) -> TokenStream {
            quote! {
                compile_error!(#msg);
            }
        }

        match self.errors.len() {
            0 => Ok(()),
            1 => Err(compile_error(&self.errors.pop().unwrap())),
            n => {
                let mut msg = format!("{} errors:", n);
                for err in self.errors {
                    msg.push_str("\n\t# ");
                    msg.push_str(&err);
                }
                Err(compile_error(&msg))
            }
        }
    }
}

//==============================================================================
// Messages
//==============================================================================

/// Produce an error string with the error `$code` which corresponds
/// to the given `$message`.
macro_rules! mk_err_msg {
    ($code: ident, $msg: expr) => {
        format!(
            "[proptest_derive, {}] during #[derive(Arbitrary)]:\n{} Please see: https://proptest-rs.github.io/proptest/proptest-derive/errors.html#{} for more information.",
            stringify!($code),
            $msg,
            (stringify!($code)).to_lowercase()
        )
    };
}

/// A macro constructing errors that do halt compilation immediately.
macro_rules! fatal {
    ($error: ident, $code: ident, $msg: expr) => {
        pub fn $error<T>(ctx: Ctx) -> DeriveResult<T> {
            ctx.fatal(mk_err_msg!($code, $msg))
        }
    };
    ($error: ident ($($arg: ident: $arg_ty: ty),*), $code: ident,
     $msg: expr, $($fmt: tt)+) => {
        pub fn $error<T>(ctx: Ctx, $($arg: $arg_ty),*) -> DeriveResult<T> {
            ctx.fatal(mk_err_msg!($code, format!($msg, $($fmt)+)))
        }
    };
}

/// A macro constructing fatal errors that do not halt compilation immediately.
macro_rules! error {
    ($error: ident, $code: ident, $msg: expr) => {
        pub fn $error(ctx: Ctx) {
            ctx.error(mk_err_msg!($code, $msg))
        }
    };
    ($error: ident ($($arg: ident: $arg_ty: ty),*), $code: ident,
     $msg: expr, $($fmt: tt)+) => {
        pub fn $error(ctx: Ctx, $($arg: $arg_ty),*) {
            ctx.error(mk_err_msg!($code, format!($msg, $($fmt)+)))
        }
    };
}

// Happens when we've been asked to derive `Arbitrary` for a type
// that is parametric over lifetimes. Since proptest does not support
// such types (yet), neither can we.
error!(
    has_lifetimes,
    E0001,
    "Cannot derive `Arbitrary` for types with generic lifetimes, such as: \
     `struct Foo<'a> { bar: &'a str }`. Currently, strategies for such types \
     are impossible to define."
);

// Happens when we've been asked to derive `Arbitrary` for something
// that is neither an enum nor a struct. Most likely, we've been given
// a union type. This might be supported in the future, but not yet.
fatal!(
    not_struct_or_enum,
    E0002,
    "Deriving is only possible for structs and enums. \
     It is currently not defined unions."
);

// Happens when a struct has at least one field that is uninhabited.
// There must at least exist one variant that we can construct.
error!(
    uninhabited_struct,
    E0003,
    "The struct you are deriving `Arbitrary` for is uninhabited since one of \
    its fields is uninhabited. An uninhabited type is by definition impossible \
    to generate."
);

// Happens when an enum has zero variants. Such an enum is obviously
// uninhabited and can not be constructed. There must at least exist
// one variant that we can construct.
fatal!(
    uninhabited_enum_with_no_variants,
    E0004,
    "The enum you are deriving `Arbitrary` for is uninhabited since it has no \
     variants. An example of such an `enum` is: `enum Void {}`. \
     An uninhabited type is by definition impossible to generate."
);

// Happens when an enum is uninhabited due all its variants being
// uninhabited (why has the user given us such a weird enum?..
// Nonetheless, we do our best to ensure soundness).
// There must at least exist one variant that we can construct.
fatal!(
    uninhabited_enum_variants_uninhabited,
    E0005,
    "The enum you are deriving `Arbitrary` for is uninhabited since all its \
     variants are uninhabited. \
     An uninhabited type is by definition impossible to generate."
);

// Happens when an enum becomes effectively uninhabited due
// to all inhabited variants having been skipped. There must
// at least exist one variant that we can construct.
error!(
    uninhabited_enum_because_of_skipped_variants,
    E0006,
    "The enum you are deriving `Arbitrary` for is uninhabited for all intents \
     and purposes since you have `#[proptest(skip)]`ed all inhabited variants. \
     An uninhabited type is by definition impossible to generate."
);

// Happens when `#[proptest(strategy = "<expr>")]` or
// `#[proptest(value = "<expr>")]` is specified on an `item`
// that does not support setting an explicit value or strategy.
// An enum or struct does not support that.
error!(illegal_strategy(attr: &str, item: &str), E0007,
    "`#[proptest({0} = \"<expr>\")]` is not allowed on {1}. Only struct fields, \
    enum variants and fields inside those can use an explicit {0}.",
    attr, item);

// Happens when `#[proptest(regex = "<string>")]` is specified on an `item`
// that does not support setting an explicit value or strategy.
// See `illegal_strategy` for more.
error!(
    illegal_regex(item: &str),
    E0007,
    "`#[proptest(regex = \"<string>\")]` is not allowed on {0}. Only struct \
     fields, enum variant fields can use an explicit regex.",
    item
);

// Happens when `#[proptest(skip)]` is specified on an `item` that does
// not support skipping. Only enum variants support skipping.
error!(
    illegal_skip(item: &str),
    E0008,
    "A {} can't be `#[proptest(skip)]`ed, only enum variants can be skipped.",
    item
);

// Happens when `#[proptest(weight = <integer>)]` is specified on an
// `item` that does not support weighting.
error!(
    illegal_weight(item: &str),
    E0009,
    "`#[proptest(weight = <integer>)]` is not allowed on {} as it is \
     meaningless. Only enum variants can be assigned weights.",
    item
);

// Happens when `#[proptest(params = <type>)]` is set on `item`
// but also on the parent of `item`. If the parent has set `params`
// then that applies, and the `params` on `item` would be meaningless
// wherefore it is forbidden.
error!(
    parent_has_param(item: &str),
    E0010,
    "Cannot set the associated type `Parameters` of `Arbitrary` with either \
     `#[proptest(no_params)]` or `#[proptest(params(<type>)]` on {} since it \
     was set on the parent.",
    item
);

// Happens when `#[proptest(params = <type>)]` is set on `item`
// but not `#[proptest(strategy = <expr>)]`.
// This does not apply to the top level type declaration.
fatal!(
    cant_set_param_but_not_strat(self_ty: &syn::Type, item: &str),
    E0011,
    "Cannot set `#[proptest(params = <type>)]` on {0} while not providing a \
     strategy for the {0} to use it since `<{1} as Arbitrary<'a>>::Strategy` \
     may require a different type than the one provided in `<type>`.",
    item,
    quote! { #self_ty }
);

// Happens when `#[proptest(filter = "<expr>")]` is set on `item`,
// but the parent of the `item` explicitly specifies a value or strategy,
// which would cause the value to be generated without consulting the
// `filter`.
error!(
    meaningless_filter(item: &str),
    E0012,
    "Cannot set `#[proptest(filter = <expr>)]` on {} since it is set on the \
     item which it is inside of that outer item specifies how to generate \
     itself.",
    item
);

// Happens when the form `#![proptest<..>]` is used. This will probably never
// happen - but just in case it does, we catch it and emit an error.
error!(
    inner_attr,
    E0013, "Inner attributes `#![proptest(..)]` are not currently supported."
);

// Happens when the form `#[proptest]` is used. The form contains no
// information for us to process, so we disallow it.
error!(
    bare_proptest_attr,
    E0014, "Bare `#[proptest]` attributes are not allowed."
);

// Happens when the form `#[proptest = <literal>)]` is used.
// Only the form `#[proptest(<contents>)]` is supported.
error!(
    literal_set_proptest,
    E0015, "The attribute form `#[proptest = <literal>]` is not allowed."
);

// Happens when `<modifier>` in `#[proptest(<modifier>)]` is a literal and
// not a real modifier.
error!(
    immediate_literals,
    E0016,
    "Literals immediately inside `#[proptest(..)]` as in \
     `#[proptest(<lit>, ..)]` are not allowed."
);

// Happens when `<modifier>` in `#[proptest(<modifier>)]` is set more than
// once.
error!(
    set_again(meta: &syn::Meta),
    E0017,
    "The attribute modifier `{}` inside `#[proptest(..)]` has already been \
     set. To fix the error, please remove at least one such modifier.",
    meta.path().into_token_stream()
);

// Happens when `<modifier>` in `#[proptest(<modifier>)]` is unknown to
// us but we can make an educated guess as to what the user meant.
error!(
    did_you_mean(found: &str, expected: &str),
    E0018,
    "Unknown attribute modifier `{}` inside #[proptest(..)] is not allowed. \
     Did you mean to use `{}` instead?",
    found,
    expected
);

// TODO: `unkown_modifier` is misspelled
// Happens when `<modifier>` in `#[proptest(<modifier>)]` is unknown to us.
error!(
    unkown_modifier(modifier: &str),
    E0018,
    "Unknown attribute modifier `{}` inside `#[proptest(..)]` is not allowed.",
    modifier
);

// Happens when `#[proptest(no_params)]` is malformed.
error!(
    no_params_malformed,
    E0019,
    "The attribute modifier `no_params` inside `#[proptest(..)]` does not \
     support any further configuration and must be a plain modifier as in \
     `#[proptest(no_params)]`."
);

// Happens when `#[proptest(skip)]` is malformed.
error!(
    skip_malformed,
    E0020,
    "The attribute modifier `skip` inside `#[proptest(..)]` does not support \
     any further configuration and must be a plain modifier as in \
     `#[proptest(skip)]`."
);

// Happens when `#[proptest(weight..)]` is malformed.
error!(
    weight_malformed(meta: &syn::Meta),
    E0021,
    "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
    format `#[proptest({0} = <integer>)]` where `<integer>` is an integer that \
    fits within a `u32`. An example: `#[proptest({0} = 2)]` to set a relative \
    weight of 2.",
    meta.path().into_token_stream()
);

// Happens when both `#[proptest(params = "<type>")]` and
// `#[proptest(no_params)]` were specified. They are mutually
// exclusive choices. The user can resolve this by picking one.
fatal!(
    overspecified_param,
    E0022,
    "Cannot set `#[proptest(no_params)]` as well as \
     `#[proptest(params(<type>))]` simultaneously. \
     Please pick one of these attributes."
);

// This happens when `#[proptest(params..)]` is malformed.
// For example, `#[proptest(params)]` is malformed. Another example is when
// `<type>` inside `#[proptest(params = "<type>")]` or
// `#[proptest(params("<type>"))]` is malformed. In other words, `<type>` is
// not a valid Rust type. Note that `syn` may not cover all valid Rust types.
error!(
    param_malformed,
    E0023,
    "The attribute modifier `params` inside #[proptest(..)] must have the \
     format `#[proptest(params = \"<type>\")]` where `<type>` is a valid type \
     in Rust. An example: `#[proptest(params = \"ComplexType<Foo>\")]`."
);

// Happens when more than one of `#[proptest(strategy..)]`,
// `#[proptest(value..)]`, or `#[proptest(regex..)]` were specified.
// They are mutually exclusive choices.
// The user can resolve this by picking one.
fatal!(
    overspecified_strat,
    E0025,
    "Cannot set more than one of `#[proptest(value = \"<expr>\")]`,
    `#[proptest(strategy = \"<expr>\")]`, `#[proptest(regex = \"<string>\")]` \
    simultaneously. Please pick one of these attributes."
);

// Happens when `#[proptest(strategy..)]` or `#[proptest(value..)]` is
// malformed. For example, `<expr>` inside `#[proptest(strategy = "<expr>")]`
// or `#[proptest(value = "<expr>")]` is malformed. In other words, `<expr>`
// is not a valid Rust expression.
error!(
    strategy_malformed(meta: &syn::Meta),
    E0026,
    "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
     format `#[proptest({0} = \"<expr>\")]` where `<expr>` is a valid Rust \
     expression.",
    meta.path().into_token_stream()
);

// Happens when `#[proptest(filter..)]` is malformed.
// For example, `<expr>` inside `#[proptest(filter = "<expr>")]` or
// is malformed. In other words, `<expr>` is not a valid Rust expression.
error!(
    filter_malformed(meta: &syn::Meta),
    E0027,
    "The attribute modifier `{0}` inside `#[proptest(..)]` must have the \
     format `#[proptest({0} = \"<expr>\")]` where `<expr>` is a valid Rust \
     expression.",
    meta.path().into_token_stream()
);

// Any attributes on a skipped variant has no effect - so we emit this error
// to the user so that they are aware.
error!(
    skipped_variant_has_weight(item: &str),
    E0028,
    "A variant has been skipped. Setting `#[proptest(weight = <value>)]` on \
     the {} is meaningless and is not allowed.",
    item
);

// Any attributes on a skipped variant has no effect - so we emit this error
// to the user so that they are aware.
error!(
    skipped_variant_has_param(item: &str),
    E0028,
    "A variant has been skipped. Setting `#[proptest(no_param)]` or \
    `#[proptest(params(<type>))]` on the {} is meaningless and is not allowed.",
    item
);

// Any attributes on a skipped variant has no effect - so we emit this error
// to the user so that they are aware.
error!(
    skipped_variant_has_strat(item: &str),
    E0028,
    "A variant has been skipped. Setting `#[proptest(value = \"<expr>\")]` or \
     `#[proptest(strategy = \"<expr>\")]` on the {} is meaningless and is not \
     allowed.",
    item
);

// Any attributes on a skipped variant has no effect - so we emit this error
// to the user so that they are aware. Unfortunately, there's no way to
// emit a warning to the user, so we emit an error instead.
error!(skipped_variant_has_filter(item: &str), E0028,
    "A variant has been skipped. Setting `#[proptest(filter = \"<expr>\")]` or \
    on the {} is meaningless and is not allowed.",
    item);

// There's only one way to produce a specific unit variant, so setting
// `#[proptest(strategy = "<expr>")]` or `#[proptest(value = "<expr>")]`
// would be pointless.
error!(
    strategy_on_unit_variant(what: &str),
    E0029,
    "Setting `#[proptest({0} = \"<expr>\")]` on a unit variant has no effect \
     and is redundant because there is nothing to configure.",
    what
);

// See `strategy_on_unit_variant`.
error!(regex_on_unit_variant, E0029,
    "Setting `#[proptest(regex = \"<string>\")]` on a unit variant has no effect \
    and is redundant because there is nothing to configure.");

// There's only one way to produce a specific unit variant, so setting
// `#[proptest(params = "<type>")]` would be pointless.
error!(
    params_on_unit_variant,
    E0029,
    "Setting `#[proptest(params = \"<type>\")]` on a unit variant has \
     no effect and is redundant because there is nothing to configure."
);

// There's only one way to produce a specific unit variant, so setting
// `#[proptest(filter = "<expr>")]` would be pointless.
error!(
    filter_on_unit_variant,
    E0029,
    "Setting `#[proptest(filter = \"<expr>\")]` on a unit variant has \
     no effect and is redundant because there is nothing to further filter."
);

// Occurs when `#[proptest(params = "<type>")]` is specified on a unit
// struct. There's only one way to produce a unit struct, so specifying
// `Parameters` would be pointless.
error!(params_on_unit_struct, E0030,
    "Setting `#[proptest(params = \"<type>\")]` on a unit struct has no effect \
    and is redundant because there is nothing to configure.");

// Occurs when `#[proptest(filter = "<expr>")]` is specified on a unit
// struct. There's only one way to produce a unit struct, so filtering
// would be pointless.
error!(filter_on_unit_struct, E0030,
    "Setting `#[proptest(filter = \"<expr>\")]` on a unit struct has no effect \
    and is redundant because there is nothing to filter.");

// Occurs when `#[proptest(no_bound)]` is specified
// on something that is not a type variable.
error!(
    no_bound_set_on_non_tyvar,
    E0031,
    "Setting `#[proptest(no_bound)]` on something that is not a type variable \
     has no effect and is redundant. Therefore it is not allowed."
);

// Happens when `#[proptest(no_bound)]` is malformed.
error!(
    no_bound_malformed,
    E0032,
    "The attribute modifier `no_bound` inside `#[proptest(..)]` does not \
     support any further configuration and must be a plain modifier as in \
     `#[proptest(no_bound)]`."
);

// Happens when the sum of weights on enum variants overflowing an u32.
error!(
    weight_overflowing,
    E0033,
    "The sum of the weights specified on variants of the enum you are \
     deriving `Arbitrary` for overflows an `u32` which it can't do."
);

// Happens when `#[proptest(regex..)]` is malformed.
// For example, `#[proptest(regex = 1)]` is not a valid form.
error!(
    regex_malformed,
    E0034,
    "The attribute modifier `regex` inside `#[proptest(..)]` must have the \
    format `#[proptest(regex = \"<string>\")]` where `<string>` is a valid
    regular expression embedded in a Rust string slice."
);

// Happens when `#[proptest(params = <type>)]` is set on `item` and then
// `#[proptest(regex = "<string>")]` is also set. We reject this because
// the params can't be used. TODO: reduce this to a warning once we can
// emit warnings.
error!(
    cant_set_param_and_regex(item: &str),
    E0035,
    "Cannot set #[proptest(regex = \"<string>\")] and \
     `#[proptest(params = <type>)]` on {0} because the latter is a logic bug \
     since `params` cannot be used in `<string>`.",
    item
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_mk_err_msg_format() {
        assert_eq!(
            mk_err_msg!(E0001, "This is a sample error message."),
            "[proptest_derive, E0001] during #[derive(Arbitrary)]:\nThis is a sample error message. Please see: https://proptest-rs.github.io/proptest/proptest-derive/errors.html#e0001 for more information."
        );
    }
}
