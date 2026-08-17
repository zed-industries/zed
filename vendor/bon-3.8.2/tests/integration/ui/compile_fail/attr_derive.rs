use bon::{bon, builder, Builder};

struct NoTraitImpls;

#[derive(Builder)]
#[builder(derive(Clone, Debug))]
struct StructContainsNonTrait {
    #[builder(start_fn)]
    no_impl_start_fn: NoTraitImpls,

    no_impls_required: NoTraitImpls,

    no_impl_optional: Option<NoTraitImpls>,

    #[builder(default = NoTraitImpls)]
    no_impl_optional_2: NoTraitImpls,

    x: u32,
}

#[builder(derive(Clone, Debug))]
fn fn_contains_non_trait(
    #[builder(start_fn)] //
    _no_impl_start_fn: NoTraitImpls,

    _no_impls_required: NoTraitImpls,

    _no_impl_optional: Option<NoTraitImpls>,

    #[builder(default = NoTraitImpls)] //
    _no_impl_optional_2: NoTraitImpls,

    _x: u32,
) {
}

#[bon]
impl StructContainsNonTrait {
    #[builder(derive(Clone, Debug))]
    fn method_contains_non_trait(
        self,

        #[builder(start_fn)] _no_impl_start_fn: NoTraitImpls,

        _no_impls_required: NoTraitImpls,

        _no_impl_optional: Option<NoTraitImpls>,

        #[builder(default = NoTraitImpls)] //
        _no_impl_optional_2: NoTraitImpls,

        _x: u32,
    ) {
    }
}

#[derive(Builder)]
#[builder(derive())]
struct EmptyDerives {}

#[derive(Builder)]
#[builder(derive(Clone()))]
struct EmptyParamsForDerive {}

#[derive(Builder)]
#[builder(derive(Clone(bounds {})))]
struct WrongDelimInBounds {}

#[builder(derive(Into))]
fn derive_into_with_finish_fn_member(#[builder(finish_fn)] _finish_fn: fn() -> u32) -> u32 {
    99
}

#[builder(derive(Into))]
fn function_with_unit_return_type() {}

#[builder(derive(Into))]
async fn async_function() -> u32 {
    99
}

#[builder(derive(Into))]
unsafe fn unsafe_function() -> u32 {
    99
}

#[builder(derive(Into))]
async unsafe fn unsafe_async_function() -> u32 {
    99
}

struct MethodWithUnsupportedIntoDerive;

#[bon]
impl MethodWithUnsupportedIntoDerive {
    #[builder(derive(Into))]
    fn derive_into_with_finish_fn_member(#[builder(finish_fn)] _finish_fn: fn() -> u32) -> u32 {
        99
    }
}

#[bon]
impl MethodWithUnsupportedIntoDerive {
    #[builder(derive(Into))]
    fn function_with_unit_return_type() {}
}

#[bon]
impl MethodWithUnsupportedIntoDerive {
    #[builder(derive(Into))]
    async fn async_function() -> Self {
        Self
    }
}

#[bon]
impl MethodWithUnsupportedIntoDerive {
    #[builder(derive(Into))]
    unsafe fn unsafe_function() -> Self {
        Self
    }
}

#[bon]
impl MethodWithUnsupportedIntoDerive {
    #[builder(derive(Into))]
    async unsafe fn unsafe_async_function() -> Self {
        Self
    }
}

#[builder(derive(Into(bounds(u32: Copy))))]
fn unsupported_bounds_on_derive_into() -> u32 {
    99
}

fn main() {}
