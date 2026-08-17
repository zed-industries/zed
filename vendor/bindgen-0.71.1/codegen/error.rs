use std::error;
use std::fmt;

/// Errors that can occur during code generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Error {
    /// Tried to generate an opaque blob for a type that did not have a layout.
    NoLayoutForOpaqueBlob,

    /// Tried to instantiate an opaque template definition, or a template
    /// definition that is too difficult for us to understand (like a partial
    /// template specialization).
    InstantiationOfOpaqueType,

    /// Function ABI is not supported.
    UnsupportedAbi(&'static str),

    /// The pointer type size does not match the target's pointer size.
    InvalidPointerSize {
        ty_name: String,
        ty_size: usize,
        ptr_size: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoLayoutForOpaqueBlob => {
                "Tried to generate an opaque blob, but had no layout.".fmt(f)
            }
            Error::InstantiationOfOpaqueType => {
                "Instantiation of opaque template type or partial template specialization."
                    .fmt(f)
            }
            Error::UnsupportedAbi(abi) => {
                 write!(
                    f,
                    "{abi} ABI is not supported by the configured Rust target."
                )
            }
            Error::InvalidPointerSize { ty_name, ty_size, ptr_size } => {
                write!(f, "The {ty_name} pointer type has size {ty_size} but the current target's pointer size is {ptr_size}.")
            }
        }
    }
}

impl error::Error for Error {}

/// A `Result` of `T` or an error of `bindgen::codegen::error::Error`.
pub(crate) type Result<T> = ::std::result::Result<T, Error>;
