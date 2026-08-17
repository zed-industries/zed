use windows::Win32::Graphics::Direct3D12::ID3D12Object;

use crate::auxil::dxgi::result::HResult;

/// Helper trait for setting the name of a D3D12 object.
///
/// This is implemented on all types that can be converted to an [`ID3D12Object`].
pub trait ObjectExt {
    fn set_name(&self, name: &str) -> Result<(), crate::DeviceError>;
}

impl<T> ObjectExt for T
where
    // Windows impls `From` for all parent interfaces, so we can use that to convert to ID3D12Object.
    //
    // This includes implementations for references.
    for<'a> &'a ID3D12Object: From<&'a T>,
{
    fn set_name(&self, name: &str) -> Result<(), crate::DeviceError> {
        let name = windows::core::HSTRING::from(name);
        let object: &ID3D12Object = self.into();
        unsafe { object.SetName(&name).into_device_result("SetName") }
    }
}
