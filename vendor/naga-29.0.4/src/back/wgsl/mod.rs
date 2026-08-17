/*!
Backend for [WGSL][wgsl] (WebGPU Shading Language).

[wgsl]: https://gpuweb.github.io/gpuweb/wgsl.html
*/

mod polyfill;
mod writer;

use alloc::format;
use alloc::string::String;

use thiserror::Error;

pub use writer::{Writer, WriterFlags};

use crate::common::wgsl;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    Unimplemented(String), // TODO: Error used only during development
    #[error("Unsupported relational function: {0:?}")]
    UnsupportedRelationalFunction(crate::RelationalFunction),
    #[error("Unsupported {kind}: {value}")]
    Unsupported {
        /// What kind of unsupported thing this is: interpolation, builtin, etc.
        kind: &'static str,

        /// The debug form of the Naga IR value that this backend can't express.
        value: String,
    },
}

impl Error {
    /// Produce an [`Unsupported`] error for `value`.
    ///
    /// [`Unsupported`]: Error::Unsupported
    fn unsupported<T: core::fmt::Debug>(kind: &'static str, value: T) -> Error {
        Error::Unsupported {
            kind,
            value: format!("{value:?}"),
        }
    }
}

trait ToWgslIfImplemented {
    fn to_wgsl_if_implemented(self) -> Result<&'static str, Error>;
}

impl<T> ToWgslIfImplemented for T
where
    T: wgsl::TryToWgsl + core::fmt::Debug + Copy,
{
    fn to_wgsl_if_implemented(self) -> Result<&'static str, Error> {
        self.try_to_wgsl()
            .ok_or_else(|| Error::unsupported(T::DESCRIPTION, self))
    }
}

pub fn write_string(
    module: &crate::Module,
    info: &crate::valid::ModuleInfo,
    flags: WriterFlags,
) -> Result<String, Error> {
    let mut w = Writer::new(String::new(), flags);
    w.write(module, info)?;
    let output = w.finish();
    Ok(output)
}

impl crate::AtomicFunction {
    const fn to_wgsl(self) -> &'static str {
        match self {
            Self::Add => "Add",
            Self::Subtract => "Sub",
            Self::And => "And",
            Self::InclusiveOr => "Or",
            Self::ExclusiveOr => "Xor",
            Self::Min => "Min",
            Self::Max => "Max",
            Self::Exchange { compare: None } => "Exchange",
            Self::Exchange { .. } => "CompareExchangeWeak",
        }
    }
}

pub const fn supported_capabilities() -> crate::valid::Capabilities {
    // WGSL regurgitation supports almost everything, though browser webgpu can't parse most of these.
    use crate::valid::Capabilities as Caps;
    Caps::all()
}
