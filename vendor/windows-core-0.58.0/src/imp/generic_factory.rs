use crate::Interface;
use core::ffi::c_void;
use core::mem::{transmute_copy, zeroed};

// A streamlined version of the IActivationFactory interface used by WinRT class factories used internally by the windows crate
// to simplify code generation. Components should implement the `IActivationFactory` interface published by the windows crate.
super::define_interface!(
    IGenericFactory,
    IGenericFactory_Vtbl,
    0x00000035_0000_0000_c000_000000000046
);
super::interface_hierarchy!(IGenericFactory, crate::IUnknown, crate::IInspectable);

impl IGenericFactory {
    pub fn ActivateInstance<I: Interface>(&self) -> crate::Result<I> {
        unsafe {
            let mut result__ = zeroed();
            (Interface::vtable(self).ActivateInstance)(
                transmute_copy(self),
                &mut result__ as *mut _ as *mut _,
            )
            .and_then(|| crate::Type::from_abi(result__))
            .and_then(|interface: crate::IInspectable| interface.cast())
        }
    }
}

#[repr(C)]
pub struct IGenericFactory_Vtbl {
    pub base__: crate::IInspectable_Vtbl,
    pub ActivateInstance:
        unsafe extern "system" fn(this: *mut c_void, instance: *mut *mut c_void) -> crate::HRESULT,
}
