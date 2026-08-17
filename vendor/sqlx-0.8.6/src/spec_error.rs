use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display};

// Autoderef specialization similar to `clap::value_parser!()`.
pub struct SpecErrorWrapper<E>(pub E);

pub trait SpecError<E>: Sized {
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static>;
}

impl<E> SpecError<E> for &&&&SpecErrorWrapper<E>
where
    E: Error + Send + Sync + 'static,
{
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static> {
        |e| Box::new(e.0)
    }
}

impl<E> SpecError<E> for &&&SpecErrorWrapper<E>
where
    E: Display,
{
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static> {
        |e| e.0.to_string().into()
    }
}

impl<E> SpecError<E> for &&SpecErrorWrapper<E>
where
    E: Debug,
{
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static> {
        |e| format!("{:?}", e.0).into()
    }
}

impl<E> SpecError<E> for &SpecErrorWrapper<E>
where
    E: Any,
{
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static> {
        |_e| format!("unprintable error: {}", std::any::type_name::<E>()).into()
    }
}

impl<E> SpecError<E> for SpecErrorWrapper<E> {
    fn __sqlx_spec_error(
        &self,
    ) -> fn(SpecErrorWrapper<E>) -> Box<dyn Error + Send + Sync + 'static> {
        |_e| "unprintable error: (unprintable type)".into()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __spec_error {
    ($e:expr) => {{
        use $crate::spec_error::{SpecError, SpecErrorWrapper};

        let wrapper = SpecErrorWrapper($e);
        let wrap_err = wrapper.__sqlx_spec_error();
        wrap_err(wrapper)
    }};
}

#[test]
fn test_spec_error() {
    #[derive(Debug)]
    struct DebugError;

    struct AnyError;

    let _e: Box<dyn Error + Send + Sync + 'static> =
        __spec_error!(std::io::Error::from(std::io::ErrorKind::Unsupported));

    let _e: Box<dyn Error + Send + Sync + 'static> = __spec_error!("displayable error");

    let _e: Box<dyn Error + Send + Sync + 'static> = __spec_error!(DebugError);

    let _e: Box<dyn Error + Send + Sync + 'static> = __spec_error!(AnyError);

    let _e: Box<dyn Error + Send + Sync + 'static> = __spec_error!(&1i32);
}
