use bon::Builder;

#[derive(Builder)]
struct InvalidWithExpr {
    #[builder(with = 42)]
    x: u32,
}

#[derive(Builder)]
struct ConflictingInto {
    #[builder(into, with = |x: u32| x + 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectForSyntax {
    #[builder(with = for<'a> |x: &'a u32| -> u32 { x + 1 })]
    value: u32,
}

#[derive(Builder)]
struct RejectConstSyntax {
    #[builder(with = const || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectStaticSyntax {
    #[builder(with = static || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectAsyncSyntax {
    #[builder(with = async || 1)]
    value: u32,
}

#[derive(Builder)]
struct RejectMoveSyntax {
    #[builder(with = move || 1)]
    value: u32,
}

#[derive(Builder)]
struct UnexpectedReturnTypeShape {
    #[builder(with = |x: u32| -> u32 { x + 1 })]
    value: u32,
}

#[derive(Builder)]
struct InvalidReturnTypeInResultClosure {
    #[builder(with = |value: &str| -> ::core::result::Result<_, core::num::ParseIntError> {
        Ok(value)
    })]
    value: u32,
}

#[derive(Builder)]
struct InvalidReturnTypeInImplTraitClosure {
    #[builder(with = |value: impl Into<::core::net::IpAddr>| value)]
    value: u32,
}

#[derive(Builder)]
struct NoGenericArgsResultReturnType1 {
    #[builder(with = |value: u32| -> Result {})]
    value: u32,
}

#[derive(Builder)]
struct NoGenericArgsResultReturnType2 {
    #[builder(with = |value: u32| -> Result<> {})]
    value: u32,
}

#[derive(Builder)]
struct ThreeGenericArgsResultReturnType {
    #[builder(with = |value: u32| -> ::core::result::Result<A, B, C> {})]
    value: u32,
}

#[derive(Builder)]
struct NonOptionWithSome1 {
    #[builder(with = Some)]
    value: u32,
}

#[derive(Builder)]
struct NonOptionWithSome2 {
    #[builder(with = Some)]
    value: Option<u32>,
}

#[derive(Builder)]
struct NonCollectionWithFromIter1 {
    #[builder(with = FromIterator::from_iter)]
    value: Option<u32>,
}

#[derive(Builder)]
struct NonCollectionWithFromIter2 {
    #[builder(with = <_>::from_iter)]
    value: Option<u32>,
}

#[derive(Builder)]
struct IncompatibleWithStartFn {
    #[builder(with = |x: u32| x + 1, start_fn)]
    value: u32,
}

#[derive(Builder)]
struct IncompatibleWithFinishFn {
    #[builder(with = |x: u32| x + 1, finish_fn)]
    value: u32,
}

#[derive(Builder)]
struct IncompatibleWithInto {
    #[builder(with = |x: u32| x + 1, into)]
    value: u32,
}

fn main() {}
