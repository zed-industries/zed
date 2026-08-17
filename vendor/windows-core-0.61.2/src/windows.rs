mod agile_reference;
pub use agile_reference::*;

mod array;
pub use array::*;

#[cfg(feature = "std")]
mod event;
#[cfg(feature = "std")]
pub use event::*;

mod handles;
pub use handles::*;

pub use windows_strings::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::load_factory::<C, I>()
}

impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl Param<PCWSTR> for &HSTRING {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl Param<PCWSTR> for PWSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.0))
    }
}

impl Param<PCSTR> for PSTR {
    unsafe fn param(self) -> ParamValue<PCSTR> {
        ParamValue::Owned(PCSTR(self.0))
    }
}

impl RuntimeType for HSTRING {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"string");
}

impl TypeKind for PWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}

impl TypeKind for HSTRING {
    type TypeKind = CloneType;
}

impl TypeKind for BSTR {
    type TypeKind = CloneType;
}
