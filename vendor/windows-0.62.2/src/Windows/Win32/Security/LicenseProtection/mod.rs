#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<P0>(licensekey: P0, validityindays: u32) -> windows_core::Result<LicenseProtectionStatus>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("licenseprotection.dll" "system" fn RegisterLicenseKeyWithExpiration(licensekey : windows_core::PCWSTR, validityindays : u32, status : *mut LicenseProtectionStatus) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RegisterLicenseKeyWithExpiration(licensekey.param().abi(), validityindays, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<P0>(licensekey: P0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("licenseprotection.dll" "system" fn ValidateLicenseKeyProtection(licensekey : windows_core::PCWSTR, notvalidbefore : *mut super::super::Foundation:: FILETIME, notvalidafter : *mut super::super::Foundation:: FILETIME, status : *mut LicenseProtectionStatus) -> windows_core::HRESULT);
    unsafe { ValidateLicenseKeyProtection(licensekey.param().abi(), notvalidbefore as _, notvalidafter as _, status as _).ok() }
}
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LicenseProtectionStatus(pub i32);
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
