#[inline]
pub unsafe fn RegisterLicenseKeyWithExpiration<P0>(licensekey: P0, validityindays: u32) -> windows_core::Result<LicenseProtectionStatus>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("licenseprotection.dll" "system" fn RegisterLicenseKeyWithExpiration(licensekey : windows_core::PCWSTR, validityindays : u32, status : *mut LicenseProtectionStatus) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RegisterLicenseKeyWithExpiration(licensekey.param().abi(), validityindays, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ValidateLicenseKeyProtection<P0>(licensekey: P0, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("licenseprotection.dll" "system" fn ValidateLicenseKeyProtection(licensekey : windows_core::PCWSTR, notvalidbefore : *mut super::super::Foundation:: FILETIME, notvalidafter : *mut super::super::Foundation:: FILETIME, status : *mut LicenseProtectionStatus) -> windows_core::HRESULT);
    ValidateLicenseKeyProtection(licensekey.param().abi(), notvalidbefore, notvalidafter, status).ok()
}
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = LicenseProtectionStatus(4i32);
pub const LicenseKeyCorrupted: LicenseProtectionStatus = LicenseProtectionStatus(3i32);
pub const LicenseKeyNotFound: LicenseProtectionStatus = LicenseProtectionStatus(1i32);
pub const LicenseKeyUnprotected: LicenseProtectionStatus = LicenseProtectionStatus(2i32);
pub const Success: LicenseProtectionStatus = LicenseProtectionStatus(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LicenseProtectionStatus(pub i32);
impl windows_core::TypeKind for LicenseProtectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LicenseProtectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LicenseProtectionStatus").field(&self.0).finish()
    }
}
