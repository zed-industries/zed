#[cfg(feature = "parse")]
pub(crate) mod de {
    pub(crate) use toml_edit::de::Error;
}

#[cfg(not(feature = "parse"))]
pub(crate) mod de {
    /// Errors that can occur when deserializing a type.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Error {
        inner: String,
    }

    impl Error {
        /// Add key while unwinding
        pub(crate) fn add_key(&mut self, _key: String) {}

        /// What went wrong
        pub(crate) fn message(&self) -> &str {
            self.inner.as_str()
        }
    }

    impl serde::de::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Error {
                inner: msg.to_string(),
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl std::error::Error for Error {}
}

#[cfg(feature = "display")]
pub(crate) mod ser {
    pub(crate) use toml_edit::ser::Error;
}

#[cfg(not(feature = "display"))]
pub(crate) mod ser {
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[non_exhaustive]
    pub(crate) enum Error {
        UnsupportedType(Option<&'static str>),
        UnsupportedNone,
        KeyNotString,
        Custom(String),
    }

    impl Error {
        pub(crate) fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Error::Custom(msg.to_string())
        }
    }

    impl serde::ser::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Self::custom(msg)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::UnsupportedType(Some(t)) => write!(formatter, "unsupported {t} type"),
                Self::UnsupportedType(None) => write!(formatter, "unsupported rust type"),
                Self::UnsupportedNone => "unsupported None value".fmt(formatter),
                Self::KeyNotString => "map key was not a string".fmt(formatter),
                Self::Custom(s) => s.fmt(formatter),
            }
        }
    }

    impl std::error::Error for Error {}
}
