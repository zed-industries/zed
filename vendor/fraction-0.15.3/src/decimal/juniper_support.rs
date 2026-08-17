use super::GenericDecimal;
use generic::GenericInteger;
use juniper::{ParseScalarResult, ParseScalarValue, Value};
use std::fmt;
use std::str::FromStr;

impl<__S, T, P> ::juniper::GraphQLValueAsync<__S> for GenericDecimal<T, P>
where
    Self: Sync,
    Self::TypeInfo: Sync,
    Self::Context: Sync,
    T: Clone + GenericInteger + From<u8> + fmt::Display,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
    __S: ::juniper::ScalarValue + Send + Sync,
{
    fn resolve_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        selection_set: Option<&'a [::juniper::Selection<__S>]>,
        executor: &'a ::juniper::Executor<Self::Context, __S>,
    ) -> ::juniper::BoxFuture<'a, ::juniper::ExecutionResult<__S>> {
        use juniper::futures::future;
        let v = ::juniper::GraphQLValue::resolve(self, info, selection_set, executor);
        Box::pin(future::ready(v))
    }
}
impl<S, T, P> ::juniper::marker::IsInputType<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger + From<u8> + fmt::Display,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
}
impl<S, T, P> ::juniper::marker::IsOutputType<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
}
impl<S, T, P> ::juniper::GraphQLType<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger + From<u8> + fmt::Display,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    fn name(_: &Self::TypeInfo) -> Option<&'static str> {
        Some("Decimal")
    }
    fn meta<'r>(
        info: &Self::TypeInfo,
        registry: &mut ::juniper::Registry<'r, S>,
    ) -> ::juniper::meta::MetaType<'r, S>
    where
        S: 'r,
    {
        registry
            .build_scalar_type::<Self>(info)
            .description("Decimal")
            .into_meta()
    }
}
impl<S, T, P> ::juniper::GraphQLValue<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger + From<u8> + fmt::Display,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    type Context = ();
    type TypeInfo = ();
    fn type_name<'__i>(&self, info: &'__i Self::TypeInfo) -> Option<&'__i str> {
        <Self as ::juniper::GraphQLType<S>>::name(info)
    }
    fn resolve(
        &self,
        _info: &(),
        _selection: Option<&[::juniper::Selection<S>]>,
        _executor: &::juniper::Executor<Self::Context, S>,
    ) -> ::juniper::ExecutionResult<S> {
        Ok(Value::scalar(format!("{}", self)))
    }
}
impl<S, T, P> ::juniper::ToInputValue<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn to_input_value(&self) -> ::juniper::InputValue<S> {
        ::juniper::ToInputValue::to_input_value(&format!("{}", self))
    }
}
impl<S, T, P> ::juniper::FromInputValue<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize> + From<u8>,
{
    fn from_input_value(value: &::juniper::InputValue<S>) -> Option<Self> {
        {
            let val = match value.as_string_value() {
                None => return None,
                Some(string) => string,
            };
            Some(FromStr::from_str(val).ok())?
        }
    }
}
impl<S, T, P> ::juniper::ParseScalarValue<S> for GenericDecimal<T, P>
where
    S: ::juniper::ScalarValue,
    T: Clone + GenericInteger,
    P: Copy + GenericInteger + Into<usize>,
{
    fn from_str(value: ::juniper::parser::ScalarToken<'_>) -> ParseScalarResult<'_, S> {
        {
            <String as ParseScalarValue<S>>::from_str(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fraction::GenericFraction;
    use juniper::{FromInputValue, InputValue, ToInputValue};

    type D = GenericDecimal<u64, u8>;
    type F = GenericFraction<u64>;
    fn get_tests() -> Vec<(&'static str, D)> {
        vec![
            ("NaN", D::nan()),
            ("-inf", D::neg_infinity()),
            ("inf", D::infinity()),
            ("0", D::from_fraction(F::new(0u64, 1u64))),
            ("2", D::from_fraction(F::new(2u64, 1u64))),
            ("-2", D::from_fraction(F::new_neg(2u64, 1u64))),
            ("0.5", D::from_fraction(F::new(1u64, 2u64))),
            ("-0.5", D::from_fraction(F::new_neg(1u64, 2u64))),
            ("0.625", D::from_fraction(F::new(5u64, 8u64))),
            (
                "246.5856420264",
                D::from_fraction(F::new(308232052533u64, 1250000000u64)),
            ),
        ]
    }

    #[test]
    fn to_input_value() {
        for (s, v) in get_tests() {
            let value = <D as ToInputValue>::to_input_value(&v);
            let str_value = value.as_scalar_value::<String>();

            assert!(str_value.is_some());
            assert_eq!(s, str_value.unwrap());
        }
    }

    #[test]
    fn from_input_value() {
        for (s, v) in get_tests() {
            let value = <D as FromInputValue>::from_input_value(&InputValue::scalar(s.to_owned()));

            assert_eq!(value, Some(v));
            assert_eq!(value.unwrap().get_precision(), v.get_precision())
        }
    }
}
