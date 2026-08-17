#[inline]
pub unsafe fn AcquireDeveloperLicense<P0>(hwndparent: P0) -> windows_core::Result<super::super::Foundation::FILETIME>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wsclient.dll" "system" fn AcquireDeveloperLicense(hwndparent : super::super::Foundation:: HWND, pexpiration : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    AcquireDeveloperLicense(hwndparent.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CheckDeveloperLicense() -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("wsclient.dll" "system" fn CheckDeveloperLicense(pexpiration : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CheckDeveloperLicense(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RemoveDeveloperLicense<P0>(hwndparent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wsclient.dll" "system" fn RemoveDeveloperLicense(hwndparent : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    RemoveDeveloperLicense(hwndparent.param().abi()).ok()
}
