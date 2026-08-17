#[inline]
pub unsafe fn AcquireDeveloperLicense(hwndparent: Option<super::super::Foundation::HWND>) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_core::link!("wsclient.dll" "system" fn AcquireDeveloperLicense(hwndparent : super::super::Foundation:: HWND, pexpiration : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AcquireDeveloperLicense(hwndparent.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CheckDeveloperLicense() -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_core::link!("wsclient.dll" "system" fn CheckDeveloperLicense(pexpiration : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CheckDeveloperLicense(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RemoveDeveloperLicense(hwndparent: Option<super::super::Foundation::HWND>) -> windows_core::Result<()> {
    windows_core::link!("wsclient.dll" "system" fn RemoveDeveloperLicense(hwndparent : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    unsafe { RemoveDeveloperLicense(hwndparent.unwrap_or(core::mem::zeroed()) as _).ok() }
}
