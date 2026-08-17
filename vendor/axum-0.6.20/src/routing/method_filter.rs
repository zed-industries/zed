use bitflags::bitflags;
use http::Method;
use std::{
    fmt,
    fmt::{Debug, Formatter},
};

bitflags! {
    /// A filter that matches one or more HTTP methods.
    pub struct MethodFilter: u16 {
        /// Match `DELETE` requests.
        const DELETE =  0b000000010;
        /// Match `GET` requests.
        const GET =     0b000000100;
        /// Match `HEAD` requests.
        const HEAD =    0b000001000;
        /// Match `OPTIONS` requests.
        const OPTIONS = 0b000010000;
        /// Match `PATCH` requests.
        const PATCH =   0b000100000;
        /// Match `POST` requests.
        const POST =    0b001000000;
        /// Match `PUT` requests.
        const PUT =     0b010000000;
        /// Match `TRACE` requests.
        const TRACE =   0b100000000;
    }
}

/// Error type used when converting a [`Method`] to a [`MethodFilter`] fails.
#[derive(Debug)]
pub struct NoMatchingMethodFilter {
    method: Method,
}

impl NoMatchingMethodFilter {
    /// Get the [`Method`] that couldn't be converted to a [`MethodFilter`].
    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl fmt::Display for NoMatchingMethodFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "no `MethodFilter` for `{}`", self.method.as_str())
    }
}

impl std::error::Error for NoMatchingMethodFilter {}

impl TryFrom<Method> for MethodFilter {
    type Error = NoMatchingMethodFilter;

    fn try_from(m: Method) -> Result<Self, NoMatchingMethodFilter> {
        match m {
            Method::DELETE => Ok(MethodFilter::DELETE),
            Method::GET => Ok(MethodFilter::GET),
            Method::HEAD => Ok(MethodFilter::HEAD),
            Method::OPTIONS => Ok(MethodFilter::OPTIONS),
            Method::PATCH => Ok(MethodFilter::PATCH),
            Method::POST => Ok(MethodFilter::POST),
            Method::PUT => Ok(MethodFilter::PUT),
            Method::TRACE => Ok(MethodFilter::TRACE),
            other => Err(NoMatchingMethodFilter { method: other }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_http_method() {
        assert_eq!(
            MethodFilter::try_from(Method::DELETE).unwrap(),
            MethodFilter::DELETE
        );

        assert_eq!(
            MethodFilter::try_from(Method::GET).unwrap(),
            MethodFilter::GET
        );

        assert_eq!(
            MethodFilter::try_from(Method::HEAD).unwrap(),
            MethodFilter::HEAD
        );

        assert_eq!(
            MethodFilter::try_from(Method::OPTIONS).unwrap(),
            MethodFilter::OPTIONS
        );

        assert_eq!(
            MethodFilter::try_from(Method::PATCH).unwrap(),
            MethodFilter::PATCH
        );

        assert_eq!(
            MethodFilter::try_from(Method::POST).unwrap(),
            MethodFilter::POST
        );

        assert_eq!(
            MethodFilter::try_from(Method::PUT).unwrap(),
            MethodFilter::PUT
        );

        assert_eq!(
            MethodFilter::try_from(Method::TRACE).unwrap(),
            MethodFilter::TRACE
        );

        assert!(MethodFilter::try_from(http::Method::CONNECT)
            .unwrap_err()
            .to_string()
            .contains("CONNECT"));
    }
}
