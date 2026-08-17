//! Test support for inspecting values.

use crate::{
    internal,
    std::{fmt, str, string::String},
    Error, ValueBag,
};

#[cfg(test)]
use crate::visit::Visit;

#[cfg(test)]
pub(crate) trait IntoValueBag<'v> {
    fn into_value_bag(self) -> ValueBag<'v>;
}

#[cfg(test)]
impl<'v, T> IntoValueBag<'v> for T
where
    T: Into<ValueBag<'v>>,
{
    fn into_value_bag(self) -> ValueBag<'v> {
        self.into()
    }
}

/**
A tokenized representation of the captured value for testing.
*/
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum TestToken {
    U64(u64),
    I64(i64),
    F64(f64),
    U128(u128),
    I128(i128),
    Char(char),
    Bool(bool),
    Str(String),
    None,

    #[cfg(feature = "error")]
    Error,

    #[cfg(feature = "sval2")]
    Sval {
        version: u32,
    },

    #[cfg(feature = "serde1")]
    Serde {
        version: u32,
    },

    #[cfg(feature = "seq")]
    Seq,

    Poisoned(String),
}

impl<'v> ValueBag<'v> {
    /**
    Convert the value bag into a token for testing.

    This _isn't_ a general-purpose API for working with values outside of testing.
    */
    pub fn to_test_token(&self) -> TestToken {
        struct TestVisitor(Option<TestToken>);

        impl<'v> internal::InternalVisitor<'v> for TestVisitor {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(crate::fill::Slot::new(self))
            }

            fn debug(&mut self, v: &dyn fmt::Debug) -> Result<(), Error> {
                self.0 = Some(TestToken::Str(format!("{:?}", v)));
                Ok(())
            }

            fn display(&mut self, v: &dyn fmt::Display) -> Result<(), Error> {
                self.0 = Some(TestToken::Str(format!("{}", v)));
                Ok(())
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                self.0 = Some(TestToken::U64(v));
                Ok(())
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                self.0 = Some(TestToken::I64(v));
                Ok(())
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                self.0 = Some(TestToken::U128(*v));
                Ok(())
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                self.0 = Some(TestToken::I128(*v));
                Ok(())
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                self.0 = Some(TestToken::F64(v));
                Ok(())
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                self.0 = Some(TestToken::Bool(v));
                Ok(())
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                self.0 = Some(TestToken::Char(v));
                Ok(())
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                self.0 = Some(TestToken::Str(v.into()));
                Ok(())
            }

            fn none(&mut self) -> Result<(), Error> {
                self.0 = Some(TestToken::None);
                Ok(())
            }

            #[cfg(feature = "error")]
            fn error(&mut self, _: &dyn internal::error::Error) -> Result<(), Error> {
                self.0 = Some(TestToken::Error);
                Ok(())
            }

            #[cfg(feature = "sval2")]
            fn sval2(&mut self, _: &dyn internal::sval::v2::Value) -> Result<(), Error> {
                self.0 = Some(TestToken::Sval { version: 2 });
                Ok(())
            }

            #[cfg(feature = "serde1")]
            fn serde1(&mut self, _: &dyn internal::serde::v1::Serialize) -> Result<(), Error> {
                self.0 = Some(TestToken::Serde { version: 1 });
                Ok(())
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, _: &dyn internal::seq::Seq) -> Result<(), Error> {
                self.0 = Some(TestToken::Seq);
                Ok(())
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                self.0 = Some(TestToken::Poisoned(msg.into()));
                Ok(())
            }
        }

        let mut visitor = TestVisitor(None);
        self.internal_visit(&mut visitor).unwrap();

        visitor.0.unwrap()
    }
}

#[cfg(test)]
pub(crate) struct TestVisit {
    pub i64: i64,
    pub u64: u64,
    pub i128: i128,
    pub u128: u128,
    pub f64: f64,
    pub bool: bool,
    pub str: &'static str,
    pub borrowed_str: &'static str,
    pub char: char,
}

#[cfg(test)]
impl Default for TestVisit {
    fn default() -> Self {
        TestVisit {
            i64: -42,
            u64: 42,
            i128: -42,
            u128: 42,
            f64: 11.0,
            bool: true,
            str: "some string",
            borrowed_str: "some borrowed string",
            char: 'n',
        }
    }
}

#[cfg(test)]
impl<'v> Visit<'v> for TestVisit {
    fn visit_any(&mut self, v: ValueBag) -> Result<(), Error> {
        panic!("unexpected value: {}", v)
    }

    fn visit_i64(&mut self, v: i64) -> Result<(), Error> {
        assert_eq!(self.i64, v);
        Ok(())
    }

    fn visit_u64(&mut self, v: u64) -> Result<(), Error> {
        assert_eq!(self.u64, v);
        Ok(())
    }

    fn visit_i128(&mut self, v: i128) -> Result<(), Error> {
        assert_eq!(self.i128, v);
        Ok(())
    }

    fn visit_u128(&mut self, v: u128) -> Result<(), Error> {
        assert_eq!(self.u128, v);
        Ok(())
    }

    fn visit_f64(&mut self, v: f64) -> Result<(), Error> {
        assert_eq!(self.f64, v);
        Ok(())
    }

    fn visit_bool(&mut self, v: bool) -> Result<(), Error> {
        assert_eq!(self.bool, v);
        Ok(())
    }

    fn visit_str(&mut self, v: &str) -> Result<(), Error> {
        assert_eq!(self.str, v);
        Ok(())
    }

    fn visit_borrowed_str(&mut self, v: &'v str) -> Result<(), Error> {
        assert_eq!(self.borrowed_str, v);
        Ok(())
    }

    fn visit_char(&mut self, v: char) -> Result<(), Error> {
        assert_eq!(self.char, v);
        Ok(())
    }

    #[cfg(feature = "error")]
    fn visit_error(&mut self, err: &(dyn crate::std::error::Error + 'static)) -> Result<(), Error> {
        assert!(err.downcast_ref::<crate::std::io::Error>().is_some());
        Ok(())
    }

    #[cfg(feature = "error")]
    fn visit_borrowed_error(
        &mut self,
        err: &'v (dyn crate::std::error::Error + 'static),
    ) -> Result<(), Error> {
        assert!(err.downcast_ref::<crate::std::io::Error>().is_some());
        Ok(())
    }
}
