use alloc::string::{String, ToString};
use core::fmt;
use core::num::TryFromIntError;

/// A WebAssembly translation error.
///
/// When a WebAssembly function can't be translated, one of these error codes will be returned
/// to describe the failure.
#[derive(Debug)]
pub enum WasmError {
    /// The input WebAssembly code is invalid.
    ///
    /// This error code is used by a WebAssembly translator when it encounters invalid WebAssembly
    /// code. This should never happen for validated WebAssembly code.
    InvalidWebAssembly {
        /// A string describing the validation error.
        message: String,
        /// The bytecode offset where the error occurred.
        offset: usize,
    },

    /// A feature used by the WebAssembly code is not supported by the embedding environment.
    ///
    /// Embedding environments may have their own limitations and feature restrictions.
    Unsupported(String),

    /// An implementation limit was exceeded.
    ///
    /// Cranelift can compile very large and complicated functions, but the [implementation has
    /// limits][limits] that cause compilation to fail when they are exceeded.
    ///
    /// [limits]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/docs/ir.md#implementation-limits
    ImplLimitExceeded,

    /// Any user-defined error.
    User(String),
}

/// Return an `Err(WasmError::Unsupported(msg))` where `msg` the string built by calling `format!`
/// on the arguments to this macro.
#[macro_export]
macro_rules! wasm_unsupported {
    ($($arg:tt)*) => { $crate::WasmError::Unsupported($crate::__format!($($arg)*)) }
}
#[doc(hidden)]
pub use alloc::format as __format;

impl From<wasmparser::BinaryReaderError> for WasmError {
    /// Convert from a `BinaryReaderError` to a `WasmError`.
    fn from(e: wasmparser::BinaryReaderError) -> Self {
        Self::InvalidWebAssembly {
            message: e.message().into(),
            offset: e.offset(),
        }
    }
}

impl From<TryFromIntError> for WasmError {
    /// Convert from a `TryFromIntError` to a `WasmError`.
    fn from(e: TryFromIntError) -> Self {
        Self::InvalidWebAssembly {
            message: e.to_string(),
            offset: 0,
        }
    }
}

/// A convenient alias for a `Result` that uses `WasmError` as the error type.
pub type WasmResult<T> = Result<T, WasmError>;

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmError::InvalidWebAssembly { message, offset } => {
                write!(
                    f,
                    "Invalid input WebAssembly code at offset {offset}: {message}"
                )
            }
            WasmError::Unsupported(s) => {
                write!(f, "Unsupported feature: {s}")
            }
            WasmError::ImplLimitExceeded => {
                write!(f, "Implementation limit exceeded")
            }
            WasmError::User(s) => {
                write!(f, "User error: {s}")
            }
        }
    }
}

impl core::error::Error for WasmError {}
