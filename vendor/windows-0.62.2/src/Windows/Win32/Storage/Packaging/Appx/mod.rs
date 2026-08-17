#[inline]
pub unsafe fn ActivatePackageVirtualizationContext(context: PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::Result<usize> {
    windows_core::link!("kernel32.dll" "system" fn ActivatePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, cookie : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ActivatePackageVirtualizationContext(context, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn AddPackageDependency<P0>(packagedependencyid: P0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernelbase.dll" "system" fn AddPackageDependency(packagedependencyid : windows_core::PCWSTR, rank : i32, options : AddPackageDependencyOptions, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { AddPackageDependency(packagedependencyid.param().abi(), rank, options, packagedependencycontext as _, packagefullname.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetClrCompat(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyClrCompat) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetClrCompat(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetCreateFileAccess(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetCreateFileAccess(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetLifecycleManagement(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetLifecycleManagement(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetMediaFoundationCodecLoading(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetMediaFoundationCodecLoading(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetProcessTerminationMethod(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetProcessTerminationMethod(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetShowDeveloperDiagnostic(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetShowDeveloperDiagnostic(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetThreadInitializationType(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetThreadInitializationType(processtoken, policy as _) }
}
#[inline]
pub unsafe fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetWindowingModel(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyWindowingModel) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { AppPolicyGetWindowingModel(processtoken, policy as _) }
}
#[inline]
pub unsafe fn CheckIsMSIXPackage<P0>(packagefullname: P0) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CheckIsMSIXPackage(packagefullname : windows_core::PCWSTR, ismsixpackage : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CheckIsMSIXPackage(packagefullname.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn ClosePackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { ClosePackageInfo(packageinforeference) }
}
#[inline]
pub unsafe fn CreatePackageVirtualizationContext<P0>(packagefamilyname: P0) -> windows_core::Result<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreatePackageVirtualizationContext(packagefamilyname : windows_core::PCWSTR, context : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePackageVirtualizationContext(packagefamilyname.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DeactivatePackageVirtualizationContext(cookie: usize) {
    windows_core::link!("kernel32.dll" "system" fn DeactivatePackageVirtualizationContext(cookie : usize));
    unsafe { DeactivatePackageVirtualizationContext(cookie) }
}
#[inline]
pub unsafe fn DeletePackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernelbase.dll" "system" fn DeletePackageDependency(packagedependencyid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeletePackageDependency(packagedependencyid.param().abi()).ok() }
}
#[inline]
pub unsafe fn DuplicatePackageVirtualizationContext(sourcecontext: PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::Result<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn DuplicatePackageVirtualizationContext(sourcecontext : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, destcontext : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DuplicatePackageVirtualizationContext(sourcecontext, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn FindPackagesByPackageFamily<P0>(packagefamilyname: P0, packagefilters: u32, count: *mut u32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut u32, buffer: Option<windows_core::PWSTR>, packageproperties: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, packagefilters : u32, count : *mut u32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut u32, buffer : windows_core::PWSTR, packageproperties : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { FindPackagesByPackageFamily(packagefamilyname.param().abi(), packagefilters, count as _, packagefullnames.unwrap_or(core::mem::zeroed()) as _, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, packageproperties.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FormatApplicationUserModelId<P0, P1>(packagefamilyname: P0, packagerelativeapplicationid: P1, applicationusermodelidlength: *mut u32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FormatApplicationUserModelId(packagefamilyname : windows_core::PCWSTR, packagerelativeapplicationid : windows_core::PCWSTR, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { FormatApplicationUserModelId(packagefamilyname.param().abi(), packagerelativeapplicationid.param().abi(), applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetApplicationUserModelId(hprocess : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetApplicationUserModelId(hprocess, applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetApplicationUserModelIdFromToken(token : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetApplicationUserModelIdFromToken(token, applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentApplicationUserModelId(applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageFamilyName(packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackageFamilyName(packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageFullName(packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackageFullName(packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageId(bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackageId(bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackageInfo(flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackageInfo2(flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackageInfo2(flags, packagepathtype, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageInfo3(flags: u32, packageinfotype: PackageInfo3Type, bufferlength: *mut u32, buffer: Option<*mut core::ffi::c_void>, count: Option<*mut u32>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageInfo3(flags : u32, packageinfotype : PackageInfo3Type, bufferlength : *mut u32, buffer : *mut core::ffi::c_void, count : *mut u32) -> windows_core::HRESULT);
    unsafe { GetCurrentPackageInfo3(flags, packageinfotype, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackagePath(pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackagePath(pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackagePath2(packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetCurrentPackagePath2(packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageVirtualizationContext() -> PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageVirtualizationContext() -> PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE);
    unsafe { GetCurrentPackageVirtualizationContext() }
}
#[inline]
pub unsafe fn GetIdForPackageDependencyContext(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("kernelbase.dll" "system" fn GetIdForPackageDependencyContext(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetIdForPackageDependencyContext(packagedependencycontext, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackageApplicationIds(packageinforeference : *const _PACKAGE_INFO_REFERENCE, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageApplicationIds(packageinforeference, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackageFamilyName(hprocess : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageFamilyName(hprocess, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFamilyNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageFamilyNameFromToken(token, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackageFullName(hprocess : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageFullName(hprocess, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFullNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageFullNameFromToken(token, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageGraphRevisionId() -> u32 {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-6.dll" "system" fn GetPackageGraphRevisionId() -> u32);
    unsafe { GetPackageGraphRevisionId() }
}
#[inline]
pub unsafe fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackageId(hprocess : super::super::super::Foundation:: HANDLE, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageId(hprocess, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageInfo(packageinforeference, flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackageInfo2(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackageInfo2(packageinforeference, flags, packagepathtype, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: Option<u32>, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetPackagePath(packageid : *const PACKAGE_ID, reserved : u32, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackagePath(packageid, reserved.unwrap_or(core::mem::zeroed()) as _, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackagePathByFullName(packagefullname.param().abi(), pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetPackagesByPackageFamily<P0>(packagefamilyname: P0, count: *mut u32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut u32, buffer: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, count : *mut u32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut u32, buffer : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetPackagesByPackageFamily(packagefamilyname.param().abi(), count as _, packagefullnames.unwrap_or(core::mem::zeroed()) as _, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetProcessesInVirtualizationContext<P0>(packagefamilyname: P0, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcessesInVirtualizationContext(packagefamilyname : windows_core::PCWSTR, count : *mut u32, processes : *mut *mut super::super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { GetProcessesInVirtualizationContext(packagefamilyname.param().abi(), count as _, processes as _).ok() }
}
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernelbase.dll" "system" fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid : windows_core::PCWSTR, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetResolvedPackageFullNameForPackageDependency(packagedependencyid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetStagedPackageOrigin<P0>(packagefullname: P0, origin: *mut PackageOrigin) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetStagedPackageOrigin(packagefullname : windows_core::PCWSTR, origin : *mut PackageOrigin) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetStagedPackageOrigin(packagefullname.param().abi(), origin as _) }
}
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetStagedPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetStagedPackagePathByFullName(packagefullname.param().abi(), pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetStagedPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { GetStagedPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenPackageInfoByFullName<P0>(packagefullname: P0, reserved: Option<u32>, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenPackageInfoByFullName(packagefullname : windows_core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { OpenPackageInfoByFullName(packagefullname.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, packageinforeference as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<P1>(usersid: Option<super::super::super::Security::PSID>, packagefullname: P1, reserved: Option<u32>, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn OpenPackageInfoByFullNameForUser(usersid : super::super::super::Security:: PSID, packagefullname : windows_core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { OpenPackageInfoByFullNameForUser(usersid.unwrap_or(core::mem::zeroed()) as _, packagefullname.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, packageinforeference as _) }
}
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<P0>(packagefullname: P0, packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageFamilyNameFromFullName(packagefullname : windows_core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { PackageFamilyNameFromFullName(packagefullname.param().abi(), packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn PackageFamilyNameFromId(packageid : *const PACKAGE_ID, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { PackageFamilyNameFromId(packageid, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn PackageFullNameFromId(packageid : *const PACKAGE_ID, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { PackageFullNameFromId(packageid, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PackageIdFromFullName<P0>(packagefullname: P0, flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageIdFromFullName(packagefullname : windows_core::PCWSTR, flags : u32, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { PackageIdFromFullName(packagefullname.param().abi(), flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<P0>(packagefamilyname: P0, packagenamelength: *mut u32, packagename: Option<windows_core::PWSTR>, packagepublisheridlength: *mut u32, packagepublisherid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname : windows_core::PCWSTR, packagenamelength : *mut u32, packagename : windows_core::PWSTR, packagepublisheridlength : *mut u32, packagepublisherid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { PackageNameAndPublisherIdFromFamilyName(packagefamilyname.param().abi(), packagenamelength as _, packagename.unwrap_or(core::mem::zeroed()) as _, packagepublisheridlength as _, packagepublisherid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ParseApplicationUserModelId<P0>(applicationusermodelid: P0, packagefamilynamelength: *mut u32, packagefamilyname: Option<windows_core::PWSTR>, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ParseApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR, packagerelativeapplicationidlength : *mut u32, packagerelativeapplicationid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { ParseApplicationUserModelId(applicationusermodelid.param().abi(), packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _, packagerelativeapplicationidlength as _, packagerelativeapplicationid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ReleasePackageVirtualizationContext(context: PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn ReleasePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE));
    unsafe { ReleasePackageVirtualizationContext(context) }
}
#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> windows_core::Result<()> {
    windows_core::link!("kernelbase.dll" "system" fn RemovePackageDependency(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT) -> windows_core::HRESULT);
    unsafe { RemovePackageDependency(packagedependencycontext).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn TryCreatePackageDependency<P1, P5>(user: super::super::super::Security::PSID, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P5, options: CreatePackageDependencyOptions) -> windows_core::Result<windows_core::PWSTR>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernelbase.dll" "system" fn TryCreatePackageDependency(user : super::super::super::Security:: PSID, packagefamilyname : windows_core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_core::PCWSTR, options : CreatePackageDependencyOptions, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TryCreatePackageDependency(user, packagefamilyname.param().abi(), core::mem::transmute(minversion), packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VerifyApplicationUserModelId<P0>(applicationusermodelid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { VerifyApplicationUserModelId(applicationusermodelid.param().abi()) }
}
#[inline]
pub unsafe fn VerifyPackageFamilyName<P0>(packagefamilyname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFamilyName(packagefamilyname : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { VerifyPackageFamilyName(packagefamilyname.param().abi()) }
}
#[inline]
pub unsafe fn VerifyPackageFullName<P0>(packagefullname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFullName(packagefullname : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { VerifyPackageFullName(packagefullname.param().abi()) }
}
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageId(packageid : *const PACKAGE_ID) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { VerifyPackageId(packageid) }
}
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<P0>(packagerelativeapplicationid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    unsafe { VerifyPackageRelativeApplicationId(packagerelativeapplicationid.param().abi()) }
}
pub const APPLICATION_USER_MODEL_ID_MAX_LENGTH: u32 = 130u32;
pub const APPLICATION_USER_MODEL_ID_MIN_LENGTH: u32 = 20u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_CAPABILITIES(pub i32);
impl APPX_CAPABILITIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for APPX_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for APPX_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for APPX_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for APPX_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for APPX_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024i32);
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(4i32);
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = APPX_CAPABILITIES(2048i32);
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(8i32);
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = APPX_CAPABILITIES(128i32);
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = APPX_CAPABILITIES(1i32);
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(2i32);
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(64i32);
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(16i32);
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(4i32);
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = APPX_CAPABILITIES(512i32);
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = APPX_CAPABILITIES(256i32);
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *const windows_core::PCWSTR,
}
impl Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub i32);
impl APPX_ENCRYPTED_PACKAGE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1i32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0i32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_core::PCWSTR,
    pub useDiffusion: windows_core::BOOL,
    pub blockMapHashAlgorithm: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_core::PCWSTR,
    pub blockMapHashAlgorithm: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
    pub options: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(5i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(12i32);
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(11i32);
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(65535i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(9i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(0i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(14i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(5i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(12i32);
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(11i32);
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(9i32);
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub i32);
impl APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: windows_core::BOOL,
    pub hashMethod: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IStream>>,
    pub fileName: windows_core::PCWSTR,
    pub contentType: windows_core::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AddPackageDependencyOptions(pub i32);
impl AddPackageDependencyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AddPackageDependencyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AddPackageDependencyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AddPackageDependencyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AddPackageDependencyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AddPackageDependencyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyClrCompat(pub i32);
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyCreateFileAccess(pub i32);
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyLifecycleManagement(pub i32);
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyThreadInitializationType(pub i32);
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppPolicyWindowingModel(pub i32);
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
pub const AppxBundleFactory: windows_core::GUID = windows_core::GUID::from_u128(0x378e0446_5384_43b7_8877_e7dbdd883446);
pub const AppxEncryptionFactory: windows_core::GUID = windows_core::GUID::from_u128(0xdc664fdd_d868_46ee_8780_8d196cb739f7);
pub const AppxFactory: windows_core::GUID = windows_core::GUID::from_u128(0x5842a140_ff9f_4166_8f5c_62f5b7b0c781);
pub const AppxPackageEditor: windows_core::GUID = windows_core::GUID::from_u128(0xf004f2ca_aebc_4b0d_bf58_e516d5bcc0ab);
pub const AppxPackagingDiagnosticEventSinkManager: windows_core::GUID = windows_core::GUID::from_u128(0x50ca0a46_1588_4161_8ed2_ef9e469ced5d);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CreatePackageDependencyOptions(pub i32);
impl CreatePackageDependencyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CreatePackageDependencyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CreatePackageDependencyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CreatePackageDependencyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CreatePackageDependencyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CreatePackageDependencyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DX_FEATURE_LEVEL(pub i32);
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
windows_core::imp::define_interface!(IAppxAppInstallerReader, IAppxAppInstallerReader_Vtbl, 0xf35bc38c_1d2f_43db_a1f4_586430d1fed2);
windows_core::imp::interface_hierarchy!(IAppxAppInstallerReader, windows_core::IUnknown);
impl IAppxAppInstallerReader {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetXmlDom(&self) -> windows_core::Result<super::super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXmlDom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxAppInstallerReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetXmlDom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetXmlDom: usize,
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IAppxAppInstallerReader_Impl: windows_core::IUnknownImpl {
    fn GetXmlDom(&self) -> windows_core::Result<super::super::super::Data::Xml::MsXml::IXMLDOMDocument>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IAppxAppInstallerReader_Vtbl {
    pub const fn new<Identity: IAppxAppInstallerReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetXmlDom<Identity: IAppxAppInstallerReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxAppInstallerReader_Impl::GetXmlDom(this) {
                    Ok(ok__) => {
                        dom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetXmlDom: GetXmlDom::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxAppInstallerReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IAppxAppInstallerReader {}
windows_core::imp::define_interface!(IAppxBlockMapBlock, IAppxBlockMapBlock_Vtbl, 0x75cf3930_3244_4fe0_a8c8_e0bcb270b889);
windows_core::imp::interface_hierarchy!(IAppxBlockMapBlock, windows_core::IUnknown);
impl IAppxBlockMapBlock {
    pub unsafe fn GetHash(&self, buffersize: *mut u32) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHash)(windows_core::Interface::as_raw(self), buffersize as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCompressedSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompressedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetCompressedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAppxBlockMapBlock_Impl: windows_core::IUnknownImpl {
    fn GetHash(&self, buffersize: *mut u32) -> windows_core::Result<*mut u8>;
    fn GetCompressedSize(&self) -> windows_core::Result<u32>;
}
impl IAppxBlockMapBlock_Vtbl {
    pub const fn new<Identity: IAppxBlockMapBlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHash<Identity: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapBlock_Impl::GetHash(this, core::mem::transmute_copy(&buffersize)) {
                    Ok(ok__) => {
                        buffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCompressedSize<Identity: IAppxBlockMapBlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapBlock_Impl::GetCompressedSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHash: GetHash::<Identity, OFFSET>,
            GetCompressedSize: GetCompressedSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapBlock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBlockMapBlock {}
windows_core::imp::define_interface!(IAppxBlockMapBlocksEnumerator, IAppxBlockMapBlocksEnumerator_Vtbl, 0x6b429b5b_36ef_479e_b9eb_0c1482b49e16);
windows_core::imp::interface_hierarchy!(IAppxBlockMapBlocksEnumerator, windows_core::IUnknown);
impl IAppxBlockMapBlocksEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapBlock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlocksEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBlockMapBlocksEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapBlock>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBlockMapBlocksEnumerator_Vtbl {
    pub const fn new<Identity: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapBlocksEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        block.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapBlocksEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapBlocksEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapBlocksEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBlockMapBlocksEnumerator {}
windows_core::imp::define_interface!(IAppxBlockMapFile, IAppxBlockMapFile_Vtbl, 0x277672ac_4f63_42c1_8abc_beae3600eb59);
windows_core::imp::interface_hierarchy!(IAppxBlockMapFile, windows_core::IUnknown);
impl IAppxBlockMapFile {
    pub unsafe fn GetBlocks(&self) -> windows_core::Result<IAppxBlockMapBlocksEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLocalFileHeaderSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalFileHeaderSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUncompressedSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUncompressedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValidateFileHash<P0>(&self, filestream: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateFileHash)(windows_core::Interface::as_raw(self), filestream.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalFileHeaderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetUncompressedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValidateFileHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValidateFileHash: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapFile_Impl: windows_core::IUnknownImpl {
    fn GetBlocks(&self) -> windows_core::Result<IAppxBlockMapBlocksEnumerator>;
    fn GetLocalFileHeaderSize(&self) -> windows_core::Result<u32>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetUncompressedSize(&self) -> windows_core::Result<u64>;
    fn ValidateFileHash(&self, filestream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBlockMapFile_Vtbl {
    pub const fn new<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBlocks<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFile_Impl::GetBlocks(this) {
                    Ok(ok__) => {
                        blocks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lfhsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFile_Impl::GetLocalFileHeaderSize(this) {
                    Ok(ok__) => {
                        lfhsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFile_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUncompressedSize<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFile_Impl::GetUncompressedSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateFileHash<Identity: IAppxBlockMapFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filestream: *mut core::ffi::c_void, isvalid: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFile_Impl::ValidateFileHash(this, core::mem::transmute_copy(&filestream)) {
                    Ok(ok__) => {
                        isvalid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlocks: GetBlocks::<Identity, OFFSET>,
            GetLocalFileHeaderSize: GetLocalFileHeaderSize::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetUncompressedSize: GetUncompressedSize::<Identity, OFFSET>,
            ValidateFileHash: ValidateFileHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBlockMapFile {}
windows_core::imp::define_interface!(IAppxBlockMapFilesEnumerator, IAppxBlockMapFilesEnumerator_Vtbl, 0x02b856a2_4262_4070_bacb_1a8cbbc42305);
windows_core::imp::interface_hierarchy!(IAppxBlockMapFilesEnumerator, windows_core::IUnknown);
impl IAppxBlockMapFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBlockMapFilesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapFile>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBlockMapFilesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFilesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFilesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapFilesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapFilesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBlockMapFilesEnumerator {}
windows_core::imp::define_interface!(IAppxBlockMapReader, IAppxBlockMapReader_Vtbl, 0x5efec991_bca3_42d1_9ec2_e92d609ec22a);
windows_core::imp::interface_hierarchy!(IAppxBlockMapReader, windows_core::IUnknown);
impl IAppxBlockMapReader {
    pub unsafe fn GetFile<P0>(&self, filename: P0) -> windows_core::Result<IAppxBlockMapFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFiles(&self) -> windows_core::Result<IAppxBlockMapFilesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHashMethod(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHashMethod)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHashMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHashMethod: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapReader_Impl: windows_core::IUnknownImpl {
    fn GetFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxBlockMapFile>;
    fn GetFiles(&self) -> windows_core::Result<IAppxBlockMapFilesEnumerator>;
    fn GetHashMethod(&self) -> windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBlockMapReader_Vtbl {
    pub const fn new<Identity: IAppxBlockMapReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFile<Identity: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapReader_Impl::GetFile(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFiles<Identity: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapReader_Impl::GetFiles(this) {
                    Ok(ok__) => {
                        enumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHashMethod<Identity: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapReader_Impl::GetHashMethod(this) {
                    Ok(ok__) => {
                        hashmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IAppxBlockMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBlockMapReader_Impl::GetStream(this) {
                    Ok(ok__) => {
                        blockmapstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFile: GetFile::<Identity, OFFSET>,
            GetFiles: GetFiles::<Identity, OFFSET>,
            GetHashMethod: GetHashMethod::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBlockMapReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBlockMapReader {}
windows_core::imp::define_interface!(IAppxBundleFactory, IAppxBundleFactory_Vtbl, 0xbba65864_965f_4a5f_855f_f074bdbf3a7b);
windows_core::imp::interface_hierarchy!(IAppxBundleFactory, windows_core::IUnknown);
impl IAppxBundleFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64) -> windows_core::Result<IAppxBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBundleReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleManifestReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBundleManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBundleManifestReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleManifestReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleManifestReader: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory_Impl: windows_core::IUnknownImpl {
    fn CreateBundleWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, bundleversion: u64) -> windows_core::Result<IAppxBundleWriter>;
    fn CreateBundleReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBundleReader>;
    fn CreateBundleManifestReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBundleManifestReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory_Vtbl {
    pub const fn new<Identity: IAppxBundleFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBundleWriter<Identity: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleFactory_Impl::CreateBundleWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&bundleversion)) {
                    Ok(ok__) => {
                        bundlewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBundleReader<Identity: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleFactory_Impl::CreateBundleReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        bundlereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBundleManifestReader<Identity: IAppxBundleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleFactory_Impl::CreateBundleManifestReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        manifestreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBundleWriter: CreateBundleWriter::<Identity, OFFSET>,
            CreateBundleReader: CreateBundleReader::<Identity, OFFSET>,
            CreateBundleManifestReader: CreateBundleManifestReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleFactory {}
windows_core::imp::define_interface!(IAppxBundleFactory2, IAppxBundleFactory2_Vtbl, 0x7325b83d_0185_42c4_82ac_be34ab1a2a8a);
windows_core::imp::interface_hierarchy!(IAppxBundleFactory2, windows_core::IUnknown);
impl IAppxBundleFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBundleReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleReader2: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory2_Impl: windows_core::IUnknownImpl {
    fn CreateBundleReader2(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleFactory2_Vtbl {
    pub const fn new<Identity: IAppxBundleFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBundleReader2<Identity: IAppxBundleFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleFactory2_Impl::CreateBundleReader2(this, core::mem::transmute_copy(&inputstream), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        bundlereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateBundleReader2: CreateBundleReader2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleFactory2 {}
windows_core::imp::define_interface!(IAppxBundleManifestOptionalBundleInfo, IAppxBundleManifestOptionalBundleInfo_Vtbl, 0x515bf2e8_bcb0_4d69_8c48_e383147b6e12);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfo, windows_core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfo {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageInfoItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestOptionalBundleInfo_Impl: windows_core::IUnknownImpl {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
}
impl IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageId<Identity: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfo_Impl::GetPackageId(this) {
                    Ok(ok__) => {
                        packageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfo_Impl::GetFileName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfoitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfo_Impl::GetPackageInfoItems(this) {
                    Ok(ok__) => {
                        packageinfoitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestOptionalBundleInfo {}
windows_core::imp::define_interface!(IAppxBundleManifestOptionalBundleInfoEnumerator, IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl, 0x9a178793_f97e_46ac_aaca_dd5ba4c177c8);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfoEnumerator, windows_core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestOptionalBundleInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfo>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalbundle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        optionalbundle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestOptionalBundleInfoEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestOptionalBundleInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestOptionalBundleInfoEnumerator {}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo, IAppxBundleManifestPackageInfo_Vtbl, 0x54cd06c1_268f_40bb_8ed2_757a9ebaec8d);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo {
    pub unsafe fn GetPackageType(&self) -> windows_core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOffset(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> windows_core::HRESULT,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestPackageInfo_Impl: windows_core::IUnknownImpl {
    fn GetPackageType(&self) -> windows_core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>;
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetOffset(&self) -> windows_core::Result<u64>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
impl IAppxBundleManifestPackageInfo_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageType<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetPackageType(this) {
                    Ok(ok__) => {
                        packagetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageId<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetPackageId(this) {
                    Ok(ok__) => {
                        packageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetFileName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOffset<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetOffset(this) {
                    Ok(ok__) => {
                        offset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResources<Identity: IAppxBundleManifestPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo_Impl::GetResources(this) {
                    Ok(ok__) => {
                        resources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageType: GetPackageType::<Identity, OFFSET>,
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetOffset: GetOffset::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetResources: GetResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo {}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo2, IAppxBundleManifestPackageInfo2_Vtbl, 0x44c2acbc_b2cf_4ccb_bbdb_9c6da8c3bc9e);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo2, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo2 {
    pub unsafe fn GetIsPackageReference(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsPackageReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsNonQualifiedResourcePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIsDefaultApplicablePackage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsDefaultApplicablePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetIsDefaultApplicablePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestPackageInfo2_Impl: windows_core::IUnknownImpl {
    fn GetIsPackageReference(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetIsDefaultApplicablePackage(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBundleManifestPackageInfo2_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsPackageReference<Identity: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispackagereference: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo2_Impl::GetIsPackageReference(this) {
                    Ok(ok__) => {
                        ispackagereference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnonqualifiedresourcepackage: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo2_Impl::GetIsNonQualifiedResourcePackage(this) {
                    Ok(ok__) => {
                        isnonqualifiedresourcepackage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Identity: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isdefaultapplicablepackage: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo2_Impl::GetIsDefaultApplicablePackage(this) {
                    Ok(ok__) => {
                        isdefaultapplicablepackage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsPackageReference: GetIsPackageReference::<Identity, OFFSET>,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, OFFSET>,
            GetIsDefaultApplicablePackage: GetIsDefaultApplicablePackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo2 {}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo3, IAppxBundleManifestPackageInfo3_Vtbl, 0x6ba74b98_bb74_4296_80d0_5f4256a99675);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo3, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo3 {
    pub unsafe fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetDeviceFamilies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestPackageInfo3_Impl: windows_core::IUnknownImpl {
    fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
impl IAppxBundleManifestPackageInfo3_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamilies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo3_Impl::GetTargetDeviceFamilies(this) {
                    Ok(ok__) => {
                        targetdevicefamilies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo3 {}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo4, IAppxBundleManifestPackageInfo4_Vtbl, 0x5da6f13d_a8a7_4532_857c_1393d659371d);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo4, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo4 {
    pub unsafe fn GetIsStub(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsStub)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestPackageInfo4_Impl: windows_core::IUnknownImpl {
    fn GetIsStub(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBundleManifestPackageInfo4_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsStub<Identity: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isstub: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfo4_Impl::GetIsStub(this) {
                    Ok(ok__) => {
                        isstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsStub: GetIsStub::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfo4 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfo4 {}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfoEnumerator, IAppxBundleManifestPackageInfoEnumerator_Vtbl, 0xf9b856ee_49a6_4e19_b2b0_6a2406d63a32);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfoEnumerator, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestPackageInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestPackageInfoEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestPackageInfo>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfoEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        packageinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfoEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestPackageInfoEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestPackageInfoEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestPackageInfoEnumerator {}
windows_core::imp::define_interface!(IAppxBundleManifestReader, IAppxBundleManifestReader_Vtbl, 0xcf0ebbc1_cc99_4106_91eb_e67462e04fb0);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestReader, windows_core::IUnknown);
impl IAppxBundleManifestReader {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageInfoItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleManifestReader_Impl: windows_core::IUnknownImpl {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleManifestReader_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageId<Identity: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestReader_Impl::GetPackageId(this) {
                    Ok(ok__) => {
                        packageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageinfoitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestReader_Impl::GetPackageInfoItems(this) {
                    Ok(ok__) => {
                        packageinfoitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IAppxBundleManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifeststream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestReader_Impl::GetStream(this) {
                    Ok(ok__) => {
                        manifeststream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleManifestReader {}
windows_core::imp::define_interface!(IAppxBundleManifestReader2, IAppxBundleManifestReader2_Vtbl, 0x5517df70_033f_4af2_8213_87d766805c02);
windows_core::imp::interface_hierarchy!(IAppxBundleManifestReader2, windows_core::IUnknown);
impl IAppxBundleManifestReader2 {
    pub unsafe fn GetOptionalBundles(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionalBundles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionalBundles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxBundleManifestReader2_Impl: windows_core::IUnknownImpl {
    fn GetOptionalBundles(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator>;
}
impl IAppxBundleManifestReader2_Vtbl {
    pub const fn new<Identity: IAppxBundleManifestReader2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionalBundles<Identity: IAppxBundleManifestReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalbundles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleManifestReader2_Impl::GetOptionalBundles(this) {
                    Ok(ok__) => {
                        optionalbundles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOptionalBundles: GetOptionalBundles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleManifestReader2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleManifestReader2 {}
windows_core::imp::define_interface!(IAppxBundleReader, IAppxBundleReader_Vtbl, 0xdd75b8c0_ba76_43b0_ae0f_68656a1dc5c8);
windows_core::imp::interface_hierarchy!(IAppxBundleReader, windows_core::IUnknown);
impl IAppxBundleReader {
    pub unsafe fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFootprintFile)(windows_core::Interface::as_raw(self), filetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlockMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetManifest(&self) -> windows_core::Result<IAppxBundleManifestReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManifest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPayloadPackages(&self) -> windows_core::Result<IAppxFilesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPayloadPackages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPayloadPackage<P0>(&self, filename: P0) -> windows_core::Result<IAppxFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFootprintFile: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_BUNDLE_FOOTPRINT_FILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlockMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxBundleReader_Impl: windows_core::IUnknownImpl {
    fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile>;
    fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader>;
    fn GetManifest(&self) -> windows_core::Result<IAppxBundleManifestReader>;
    fn GetPayloadPackages(&self) -> windows_core::Result<IAppxFilesEnumerator>;
    fn GetPayloadPackage(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxFile>;
}
impl IAppxBundleReader_Vtbl {
    pub const fn new<Identity: IAppxBundleReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFootprintFile<Identity: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleReader_Impl::GetFootprintFile(this, core::mem::transmute_copy(&filetype)) {
                    Ok(ok__) => {
                        footprintfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlockMap<Identity: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleReader_Impl::GetBlockMap(this) {
                    Ok(ok__) => {
                        blockmapreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetManifest<Identity: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleReader_Impl::GetManifest(this) {
                    Ok(ok__) => {
                        manifestreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPayloadPackages<Identity: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, payloadpackages: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleReader_Impl::GetPayloadPackages(this) {
                    Ok(ok__) => {
                        payloadpackages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPayloadPackage<Identity: IAppxBundleReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, payloadpackage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxBundleReader_Impl::GetPayloadPackage(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        payloadpackage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFootprintFile: GetFootprintFile::<Identity, OFFSET>,
            GetBlockMap: GetBlockMap::<Identity, OFFSET>,
            GetManifest: GetManifest::<Identity, OFFSET>,
            GetPayloadPackages: GetPayloadPackages::<Identity, OFFSET>,
            GetPayloadPackage: GetPayloadPackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxBundleReader {}
windows_core::imp::define_interface!(IAppxBundleWriter, IAppxBundleWriter_Vtbl, 0xec446fe8_bfec_4c64_ab4f_49f038f0c6d2);
windows_core::imp::interface_hierarchy!(IAppxBundleWriter, windows_core::IUnknown);
impl IAppxBundleWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackage<P0, P1>(&self, filename: P0, packagestream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter_Impl: windows_core::IUnknownImpl {
    fn AddPayloadPackage(&self, filename: &windows_core::PCWSTR, packagestream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter_Vtbl {
    pub const fn new<Identity: IAppxBundleWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadPackage<Identity: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter_Impl::AddPayloadPackage(this, core::mem::transmute(&filename), core::mem::transmute_copy(&packagestream)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxBundleWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter {}
windows_core::imp::define_interface!(IAppxBundleWriter2, IAppxBundleWriter2_Vtbl, 0x6d8fe971_01cc_49a0_b685_233851279962);
windows_core::imp::interface_hierarchy!(IAppxBundleWriter2, windows_core::IUnknown);
impl IAppxBundleWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter2_Impl: windows_core::IUnknownImpl {
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter2_Vtbl {
    pub const fn new<Identity: IAppxBundleWriter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddExternalPackageReference<Identity: IAppxBundleWriter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter2_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter2 {}
windows_core::imp::define_interface!(IAppxBundleWriter3, IAppxBundleWriter3_Vtbl, 0xad711152_f969_4193_82d5_9ddf2786d21a);
windows_core::imp::interface_hierarchy!(IAppxBundleWriter3, windows_core::IUnknown);
impl IAppxBundleWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok() }
    }
    pub unsafe fn Close<P0>(&self, hashmethodstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), hashmethodstring.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter3_Impl: windows_core::IUnknownImpl {
    fn AddPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self, hashmethodstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter3_Vtbl {
    pub const fn new<Identity: IAppxBundleWriter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPackageReference<Identity: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter3_Impl::AddPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxBundleWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashmethodstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter3_Impl::Close(this, core::mem::transmute(&hashmethodstring)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPackageReference: AddPackageReference::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter3 {}
windows_core::imp::define_interface!(IAppxBundleWriter4, IAppxBundleWriter4_Vtbl, 0x9cd9d523_5009_4c01_9882_dc029fbd47a3);
windows_core::imp::interface_hierarchy!(IAppxBundleWriter4, windows_core::IUnknown);
impl IAppxBundleWriter4 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackage<P0, P1>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi(), isdefaultapplicablepackage.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPackageReference<P0, P1>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter4_Impl: windows_core::IUnknownImpl {
    fn AddPayloadPackage(&self, filename: &windows_core::PCWSTR, packagestream: windows_core::Ref<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxBundleWriter4_Vtbl {
    pub const fn new<Identity: IAppxBundleWriter4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadPackage<Identity: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter4_Impl::AddPayloadPackage(this, core::mem::transmute(&filename), core::mem::transmute_copy(&packagestream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
            }
        }
        unsafe extern "system" fn AddPackageReference<Identity: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter4_Impl::AddPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
            }
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: IAppxBundleWriter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxBundleWriter4_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackage: AddPayloadPackage::<Identity, OFFSET>,
            AddPackageReference: AddPackageReference::<Identity, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxBundleWriter4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxBundleWriter4 {}
windows_core::imp::define_interface!(IAppxContentGroup, IAppxContentGroup_Vtbl, 0x328f6468_c04f_4e3c_b6fa_6b8d27f3003a);
windows_core::imp::interface_hierarchy!(IAppxContentGroup, windows_core::IUnknown);
impl IAppxContentGroup {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFiles(&self) -> windows_core::Result<IAppxContentGroupFilesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxContentGroup_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFiles(&self) -> windows_core::Result<IAppxContentGroupFilesEnumerator>;
}
impl IAppxContentGroup_Vtbl {
    pub const fn new<Identity: IAppxContentGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroup_Impl::GetName(this) {
                    Ok(ok__) => {
                        groupname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFiles<Identity: IAppxContentGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroup_Impl::GetFiles(this) {
                    Ok(ok__) => {
                        enumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetFiles: GetFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxContentGroup {}
windows_core::imp::define_interface!(IAppxContentGroupFilesEnumerator, IAppxContentGroupFilesEnumerator_Vtbl, 0x1a09a2fd_7440_44eb_8c84_848205a6a1cc);
windows_core::imp::interface_hierarchy!(IAppxContentGroupFilesEnumerator, windows_core::IUnknown);
impl IAppxContentGroupFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxContentGroupFilesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxContentGroupFilesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupFilesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupFilesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupFilesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupFilesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxContentGroupFilesEnumerator {}
windows_core::imp::define_interface!(IAppxContentGroupMapReader, IAppxContentGroupMapReader_Vtbl, 0x418726d8_dd99_4f5d_9886_157add20de01);
windows_core::imp::interface_hierarchy!(IAppxContentGroupMapReader, windows_core::IUnknown);
impl IAppxContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequiredGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutomaticGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxContentGroupMapReader_Impl: windows_core::IUnknownImpl {
    fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl IAppxContentGroupMapReader_Vtbl {
    pub const fn new<Identity: IAppxContentGroupMapReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRequiredGroup<Identity: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupMapReader_Impl::GetRequiredGroup(this) {
                    Ok(ok__) => {
                        requiredgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: IAppxContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticgroupsenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupMapReader_Impl::GetAutomaticGroups(this) {
                    Ok(ok__) => {
                        automaticgroupsenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupMapReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxContentGroupMapReader {}
windows_core::imp::define_interface!(IAppxContentGroupMapWriter, IAppxContentGroupMapWriter_Vtbl, 0xd07ab776_a9de_4798_8c14_3db31e687c78);
windows_core::imp::interface_hierarchy!(IAppxContentGroupMapWriter, windows_core::IUnknown);
impl IAppxContentGroupMapWriter {
    pub unsafe fn AddAutomaticGroup<P0>(&self, groupname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomaticGroup)(windows_core::Interface::as_raw(self), groupname.param().abi()).ok() }
    }
    pub unsafe fn AddAutomaticFile<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomaticFile)(windows_core::Interface::as_raw(self), filename.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAutomaticGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddAutomaticFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxContentGroupMapWriter_Impl: windows_core::IUnknownImpl {
    fn AddAutomaticGroup(&self, groupname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddAutomaticFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IAppxContentGroupMapWriter_Vtbl {
    pub const fn new<Identity: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAutomaticGroup<Identity: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxContentGroupMapWriter_Impl::AddAutomaticGroup(this, core::mem::transmute(&groupname)).into()
            }
        }
        unsafe extern "system" fn AddAutomaticFile<Identity: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxContentGroupMapWriter_Impl::AddAutomaticFile(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxContentGroupMapWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxContentGroupMapWriter_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAutomaticGroup: AddAutomaticGroup::<Identity, OFFSET>,
            AddAutomaticFile: AddAutomaticFile::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupMapWriter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxContentGroupMapWriter {}
windows_core::imp::define_interface!(IAppxContentGroupsEnumerator, IAppxContentGroupsEnumerator_Vtbl, 0x3264e477_16d1_4d63_823e_7d2984696634);
windows_core::imp::interface_hierarchy!(IAppxContentGroupsEnumerator, windows_core::IUnknown);
impl IAppxContentGroupsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxContentGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxContentGroupsEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxContentGroupsEnumerator_Vtbl {
    pub const fn new<Identity: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupsEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupsEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxContentGroupsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxContentGroupsEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxContentGroupsEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxContentGroupsEnumerator {}
windows_core::imp::define_interface!(IAppxDigestProvider, IAppxDigestProvider_Vtbl, 0x9fe2702b_7640_4659_8e6c_349e43c4cdbd);
windows_core::imp::interface_hierarchy!(IAppxDigestProvider, windows_core::IUnknown);
impl IAppxDigestProvider {
    pub unsafe fn GetDigest(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxDigestProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxDigestProvider_Impl: windows_core::IUnknownImpl {
    fn GetDigest(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxDigestProvider_Vtbl {
    pub const fn new<Identity: IAppxDigestProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDigest<Identity: IAppxDigestProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, digest: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxDigestProvider_Impl::GetDigest(this) {
                    Ok(ok__) => {
                        digest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDigest: GetDigest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxDigestProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxDigestProvider {}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter, IAppxEncryptedBundleWriter_Vtbl, 0x80b0902f_7bf0_4117_b8c6_4279ef81ee77);
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1>(&self, filename: P0, packagestream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadPackageEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter_Impl: windows_core::IUnknownImpl {
    fn AddPayloadPackageEncrypted(&self, filename: &windows_core::PCWSTR, packagestream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter_Vtbl {
    pub const fn new<Identity: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedBundleWriter_Impl::AddPayloadPackageEncrypted(this, core::mem::transmute(&filename), core::mem::transmute_copy(&packagestream)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxEncryptedBundleWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedBundleWriter_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter {}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter2, IAppxEncryptedBundleWriter2_Vtbl, 0xe644be82_f0fa_42b8_a956_8d1cb48ee379);
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter2, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter2_Impl: windows_core::IUnknownImpl {
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter2_Vtbl {
    pub const fn new<Identity: IAppxEncryptedBundleWriter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddExternalPackageReference<Identity: IAppxEncryptedBundleWriter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedBundleWriter2_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter2 {}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter3, IAppxEncryptedBundleWriter3_Vtbl, 0x0d34deb3_5cae_4dd3_977c_504932a51d31);
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter3, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadPackageEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi(), isdefaultapplicablepackage.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter3_Impl: windows_core::IUnknownImpl {
    fn AddPayloadPackageEncrypted(&self, filename: &windows_core::PCWSTR, packagestream: windows_core::Ref<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddExternalPackageReference(&self, filename: &windows_core::PCWSTR, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedBundleWriter3_Vtbl {
    pub const fn new<Identity: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, packagestream: *mut core::ffi::c_void, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedBundleWriter3_Impl::AddPayloadPackageEncrypted(this, core::mem::transmute(&filename), core::mem::transmute_copy(&packagestream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
            }
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: IAppxEncryptedBundleWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, inputstream: *mut core::ffi::c_void, isdefaultapplicablepackage: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedBundleWriter3_Impl::AddExternalPackageReference(this, core::mem::transmute(&filename), core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&isdefaultapplicablepackage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedBundleWriter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedBundleWriter3 {}
windows_core::imp::define_interface!(IAppxEncryptedPackageWriter, IAppxEncryptedPackageWriter_Vtbl, 0xf43d0b0b_1379_40e2_9b29_682ea2bf42af);
windows_core::imp::interface_hierarchy!(IAppxEncryptedPackageWriter, windows_core::IUnknown);
impl IAppxEncryptedPackageWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFileEncrypted<P0, P2>(&self, filename: P0, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadFileEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), compressionoption, inputstream.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFileEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, APPX_COMPRESSION_OPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFileEncrypted: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter_Impl: windows_core::IUnknownImpl {
    fn AddPayloadFileEncrypted(&self, filename: &windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter_Vtbl {
    pub const fn new<Identity: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadFileEncrypted<Identity: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedPackageWriter_Impl::AddPayloadFileEncrypted(this, core::mem::transmute(&filename), core::mem::transmute_copy(&compressionoption), core::mem::transmute_copy(&inputstream)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxEncryptedPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedPackageWriter_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFileEncrypted: AddPayloadFileEncrypted::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedPackageWriter {}
windows_core::imp::define_interface!(IAppxEncryptedPackageWriter2, IAppxEncryptedPackageWriter2_Vtbl, 0x3e475447_3a25_40b5_8ad2_f953ae50c92d);
windows_core::imp::interface_hierarchy!(IAppxEncryptedPackageWriter2, windows_core::IUnknown);
impl IAppxEncryptedPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFilesEncrypted(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadFilesEncrypted)(windows_core::Interface::as_raw(self), payloadfiles.len().try_into().unwrap(), core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFilesEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFilesEncrypted: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter2_Impl: windows_core::IUnknownImpl {
    fn AddPayloadFilesEncrypted(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptedPackageWriter2_Vtbl {
    pub const fn new<Identity: IAppxEncryptedPackageWriter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Identity: IAppxEncryptedPackageWriter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptedPackageWriter2_Impl::AddPayloadFilesEncrypted(this, core::mem::transmute_copy(&filecount), core::mem::transmute_copy(&payloadfiles), core::mem::transmute_copy(&memorylimit)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptedPackageWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptedPackageWriter2 {}
windows_core::imp::define_interface!(IAppxEncryptionFactory, IAppxEncryptionFactory_Vtbl, 0x80e8e04d_8c88_44ae_a011_7cadf6fb2e72);
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory, windows_core::IUnknown);
impl IAppxEncryptionFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), keyinfo).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1>(&self, outputstream: P0, manifeststream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedPackageReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), keyinfo).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, core::mem::transmute(settings), keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedBundleReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptBundle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptBundle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleReader: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory_Impl: windows_core::IUnknownImpl {
    fn EncryptPackage(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn DecryptPackage(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, manifeststream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
    fn CreateEncryptedPackageReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxPackageReader>;
    fn EncryptBundle(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn DecryptBundle(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>;
    fn CreateEncryptedBundleReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory_Vtbl {
    pub const fn new<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncryptPackage<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory_Impl::EncryptPackage(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
            }
        }
        unsafe extern "system" fn DecryptPackage<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory_Impl::DecryptPackage(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&keyinfo)).into()
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory_Impl::CreateEncryptedPackageWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&manifeststream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory_Impl::CreateEncryptedPackageReader(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&keyinfo)) {
                    Ok(ok__) => {
                        packagereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory_Impl::EncryptBundle(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
            }
        }
        unsafe extern "system" fn DecryptBundle<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory_Impl::DecryptBundle(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&keyinfo)).into()
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory_Impl::CreateEncryptedBundleWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&bundleversion), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                    Ok(ok__) => {
                        bundlewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Identity: IAppxEncryptionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory_Impl::CreateEncryptedBundleReader(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&keyinfo)) {
                    Ok(ok__) => {
                        bundlereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, OFFSET>,
            DecryptPackage: DecryptPackage::<Identity, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET>,
            CreateEncryptedPackageReader: CreateEncryptedPackageReader::<Identity, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, OFFSET>,
            DecryptBundle: DecryptBundle::<Identity, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, OFFSET>,
            CreateEncryptedBundleReader: CreateEncryptedBundleReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory {}
windows_core::imp::define_interface!(IAppxEncryptionFactory2, IAppxEncryptionFactory2_Vtbl, 0xc1b11eee_c4ba_4ab2_a55d_d015fe8ff64f);
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory2, windows_core::IUnknown);
impl IAppxEncryptionFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), contentgroupmapstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory2_Impl: windows_core::IUnknownImpl {
    fn CreateEncryptedPackageWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, manifeststream: windows_core::Ref<super::super::super::System::Com::IStream>, contentgroupmapstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory2_Vtbl {
    pub const fn new<Identity: IAppxEncryptionFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: IAppxEncryptionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, contentgroupmapstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory2_Impl::CreateEncryptedPackageWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&manifeststream), core::mem::transmute_copy(&contentgroupmapstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory2 {}
windows_core::imp::define_interface!(IAppxEncryptionFactory3, IAppxEncryptionFactory3_Vtbl, 0x09edca37_cd64_47d6_b7e8_1cb11d4f7e05);
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory3, windows_core::IUnknown);
impl IAppxEncryptionFactory3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), contentgroupmapstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, core::mem::transmute(settings), keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptBundle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleWriter: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory3_Impl: windows_core::IUnknownImpl {
    fn EncryptPackage(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, manifeststream: windows_core::Ref<super::super::super::System::Com::IStream>, contentgroupmapstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>;
    fn EncryptBundle(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory3_Vtbl {
    pub const fn new<Identity: IAppxEncryptionFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncryptPackage<Identity: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory3_Impl::EncryptPackage(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
            }
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, manifeststream: *mut core::ffi::c_void, contentgroupmapstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory3_Impl::CreateEncryptedPackageWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&manifeststream), core::mem::transmute_copy(&contentgroupmapstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EncryptBundle<Identity: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory3_Impl::EncryptBundle(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)).into()
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: IAppxEncryptionFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory3_Impl::CreateEncryptedBundleWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&bundleversion), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles)) {
                    Ok(ok__) => {
                        bundlewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncryptPackage: EncryptPackage::<Identity, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory3 {}
windows_core::imp::define_interface!(IAppxEncryptionFactory4, IAppxEncryptionFactory4_Vtbl, 0xa879611f_12fd_41fe_85d5_06ae779bbaf5);
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory4, windows_core::IUnknown);
impl IAppxEncryptionFactory4 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), core::mem::transmute(settings), keyinfo, exemptedfiles, memorylimit).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory4_Impl: windows_core::IUnknownImpl {
    fn EncryptPackage(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory4_Vtbl {
    pub const fn new<Identity: IAppxEncryptionFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncryptPackage<Identity: IAppxEncryptionFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxEncryptionFactory4_Impl::EncryptPackage(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo), core::mem::transmute_copy(&exemptedfiles), core::mem::transmute_copy(&memorylimit)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EncryptPackage: EncryptPackage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory4 {}
windows_core::imp::define_interface!(IAppxEncryptionFactory5, IAppxEncryptionFactory5_Vtbl, 0x68d6e77a_f446_480f_b0f0_d91a24c60746);
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory5, windows_core::IUnknown);
impl IAppxEncryptionFactory5 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader2<P0, P2>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO, expecteddigest: P2) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedPackageReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader2<P0, P2>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO, expecteddigest: P2) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEncryptedBundleReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory5_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageReader2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_KEY_INFO, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleReader2: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory5_Impl: windows_core::IUnknownImpl {
    fn CreateEncryptedPackageReader2(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxPackageReader>;
    fn CreateEncryptedBundleReader2(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxEncryptionFactory5_Vtbl {
    pub const fn new<Identity: IAppxEncryptionFactory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEncryptedPackageReader2<Identity: IAppxEncryptionFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: windows_core::PCWSTR, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory5_Impl::CreateEncryptedPackageReader2(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&keyinfo), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        packagereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEncryptedBundleReader2<Identity: IAppxEncryptionFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: windows_core::PCWSTR, bundlereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxEncryptionFactory5_Impl::CreateEncryptedBundleReader2(this, core::mem::transmute_copy(&inputstream), core::mem::transmute_copy(&keyinfo), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        bundlereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEncryptedPackageReader2: CreateEncryptedPackageReader2::<Identity, OFFSET>,
            CreateEncryptedBundleReader2: CreateEncryptedBundleReader2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxEncryptionFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxEncryptionFactory5 {}
windows_core::imp::define_interface!(IAppxFactory, IAppxFactory_Vtbl, 0xbeb94909_e451_438b_b5a7_d79e767b75d8);
windows_core::imp::interface_hierarchy!(IAppxFactory, windows_core::IUnknown);
impl IAppxFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageWriter<P0>(&self, outputstream: P0, settings: *const APPX_PACKAGE_SETTINGS) -> windows_core::Result<IAppxPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), core::mem::transmute(settings), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateManifestReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlockMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBlockMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBlockMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateValidatedBlockMapReader<P0, P1>(&self, blockmapstream: P0, signaturefilename: P1) -> windows_core::Result<IAppxBlockMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateValidatedBlockMapReader)(windows_core::Interface::as_raw(self), blockmapstream.param().abi(), signaturefilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_PACKAGE_SETTINGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateManifestReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateManifestReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlockMapReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlockMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValidatedBlockMapReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValidatedBlockMapReader: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory_Impl: windows_core::IUnknownImpl {
    fn CreatePackageWriter(&self, outputstream: windows_core::Ref<super::super::super::System::Com::IStream>, settings: *const APPX_PACKAGE_SETTINGS) -> windows_core::Result<IAppxPackageWriter>;
    fn CreatePackageReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxManifestReader>;
    fn CreateBlockMapReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxBlockMapReader>;
    fn CreateValidatedBlockMapReader(&self, blockmapstream: windows_core::Ref<super::super::super::System::Com::IStream>, signaturefilename: &windows_core::PCWSTR) -> windows_core::Result<IAppxBlockMapReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory_Vtbl {
    pub const fn new<Identity: IAppxFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePackageWriter<Identity: IAppxFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory_Impl::CreatePackageWriter(this, core::mem::transmute_copy(&outputstream), core::mem::transmute_copy(&settings)) {
                    Ok(ok__) => {
                        packagewriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePackageReader<Identity: IAppxFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory_Impl::CreatePackageReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        packagereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateManifestReader<Identity: IAppxFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory_Impl::CreateManifestReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        manifestreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlockMapReader<Identity: IAppxFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory_Impl::CreateBlockMapReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        blockmapreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Identity: IAppxFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapstream: *mut core::ffi::c_void, signaturefilename: windows_core::PCWSTR, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory_Impl::CreateValidatedBlockMapReader(this, core::mem::transmute_copy(&blockmapstream), core::mem::transmute(&signaturefilename)) {
                    Ok(ok__) => {
                        blockmapreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageWriter: CreatePackageWriter::<Identity, OFFSET>,
            CreatePackageReader: CreatePackageReader::<Identity, OFFSET>,
            CreateManifestReader: CreateManifestReader::<Identity, OFFSET>,
            CreateBlockMapReader: CreateBlockMapReader::<Identity, OFFSET>,
            CreateValidatedBlockMapReader: CreateValidatedBlockMapReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory {}
windows_core::imp::define_interface!(IAppxFactory2, IAppxFactory2_Vtbl, 0xf1346df2_c282_4e22_b918_743a929a8d55);
windows_core::imp::interface_hierarchy!(IAppxFactory2, windows_core::IUnknown);
impl IAppxFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxContentGroupMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContentGroupMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSourceContentGroupMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxSourceContentGroupMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSourceContentGroupMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapWriter<P0>(&self, stream: P0) -> windows_core::Result<IAppxContentGroupMapWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContentGroupMapWriter)(windows_core::Interface::as_raw(self), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSourceContentGroupMapReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSourceContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapWriter: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory2_Impl: windows_core::IUnknownImpl {
    fn CreateContentGroupMapReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxContentGroupMapReader>;
    fn CreateSourceContentGroupMapReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxSourceContentGroupMapReader>;
    fn CreateContentGroupMapWriter(&self, stream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<IAppxContentGroupMapWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory2_Vtbl {
    pub const fn new<Identity: IAppxFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateContentGroupMapReader<Identity: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, contentgroupmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory2_Impl::CreateContentGroupMapReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        contentgroupmapreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Identity: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, reader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory2_Impl::CreateSourceContentGroupMapReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        reader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Identity: IAppxFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, contentgroupmapwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory2_Impl::CreateContentGroupMapWriter(this, core::mem::transmute_copy(&stream)) {
                    Ok(ok__) => {
                        contentgroupmapwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContentGroupMapReader: CreateContentGroupMapReader::<Identity, OFFSET>,
            CreateSourceContentGroupMapReader: CreateSourceContentGroupMapReader::<Identity, OFFSET>,
            CreateContentGroupMapWriter: CreateContentGroupMapWriter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory2 {}
windows_core::imp::define_interface!(IAppxFactory3, IAppxFactory3_Vtbl, 0x776b2c05_e21d_4e24_ba1a_cd529a8bfdbb);
windows_core::imp::interface_hierarchy!(IAppxFactory3, windows_core::IUnknown);
impl IAppxFactory3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePackageReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateManifestReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAppInstallerReader<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxAppInstallerReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAppInstallerReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageReader2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateManifestReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateManifestReader2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAppInstallerReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAppInstallerReader: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory3_Impl: windows_core::IUnknownImpl {
    fn CreatePackageReader2(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader2(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxManifestReader>;
    fn CreateAppInstallerReader(&self, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>, expecteddigest: &windows_core::PCWSTR) -> windows_core::Result<IAppxAppInstallerReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFactory3_Vtbl {
    pub const fn new<Identity: IAppxFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePackageReader2<Identity: IAppxFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, packagereader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory3_Impl::CreatePackageReader2(this, core::mem::transmute_copy(&inputstream), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        packagereader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateManifestReader2<Identity: IAppxFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory3_Impl::CreateManifestReader2(this, core::mem::transmute_copy(&inputstream), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        manifestreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAppInstallerReader<Identity: IAppxFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, expecteddigest: windows_core::PCWSTR, appinstallerreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFactory3_Impl::CreateAppInstallerReader(this, core::mem::transmute_copy(&inputstream), core::mem::transmute(&expecteddigest)) {
                    Ok(ok__) => {
                        appinstallerreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePackageReader2: CreatePackageReader2::<Identity, OFFSET>,
            CreateManifestReader2: CreateManifestReader2::<Identity, OFFSET>,
            CreateAppInstallerReader: CreateAppInstallerReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFactory3 {}
windows_core::imp::define_interface!(IAppxFile, IAppxFile_Vtbl, 0x91df827b_94fd_468f_827b_57f41b2f6f2e);
windows_core::imp::interface_hierarchy!(IAppxFile, windows_core::IUnknown);
impl IAppxFile {
    pub unsafe fn GetCompressionOption(&self) -> windows_core::Result<APPX_COMPRESSION_OPTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCompressionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCompressionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_COMPRESSION_OPTION) -> windows_core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFile_Impl: windows_core::IUnknownImpl {
    fn GetCompressionOption(&self) -> windows_core::Result<APPX_COMPRESSION_OPTION>;
    fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxFile_Vtbl {
    pub const fn new<Identity: IAppxFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCompressionOption<Identity: IAppxFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFile_Impl::GetCompressionOption(this) {
                    Ok(ok__) => {
                        compressionoption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContentType<Identity: IAppxFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFile_Impl::GetContentType(this) {
                    Ok(ok__) => {
                        contenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IAppxFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFile_Impl::GetName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IAppxFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFile_Impl::GetSize(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IAppxFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFile_Impl::GetStream(this) {
                    Ok(ok__) => {
                        stream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCompressionOption: GetCompressionOption::<Identity, OFFSET>,
            GetContentType: GetContentType::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxFile {}
windows_core::imp::define_interface!(IAppxFilesEnumerator, IAppxFilesEnumerator_Vtbl, 0xf007eeaf_9831_411c_9847_917cdc62d1fe);
windows_core::imp::interface_hierarchy!(IAppxFilesEnumerator, windows_core::IUnknown);
impl IAppxFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxFilesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxFile>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxFilesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxFilesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFilesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFilesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxFilesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxFilesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxFilesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxFilesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestApplication, IAppxManifestApplication_Vtbl, 0x5da89bf4_3773_46be_b650_7e744863b7e8);
windows_core::imp::interface_hierarchy!(IAppxManifestApplication, windows_core::IUnknown);
impl IAppxManifestApplication {
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAppUserModelId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAppUserModelId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetAppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestApplication_Impl: windows_core::IUnknownImpl {
    fn GetStringValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAppUserModelId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestApplication_Vtbl {
    pub const fn new<Identity: IAppxManifestApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringValue<Identity: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestApplication_Impl::GetStringValue(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAppUserModelId<Identity: IAppxManifestApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestApplication_Impl::GetAppUserModelId(this) {
                    Ok(ok__) => {
                        appusermodelid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetAppUserModelId: GetAppUserModelId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestApplication as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestApplication {}
windows_core::imp::define_interface!(IAppxManifestApplicationsEnumerator, IAppxManifestApplicationsEnumerator_Vtbl, 0x9eb8a55a_f04b_4d0d_808d_686185d4847a);
windows_core::imp::interface_hierarchy!(IAppxManifestApplicationsEnumerator, windows_core::IUnknown);
impl IAppxManifestApplicationsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplicationsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestApplicationsEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestApplication>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestApplicationsEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestApplicationsEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        application.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestApplicationsEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestApplicationsEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestApplicationsEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestApplicationsEnumerator {}
windows_core::imp::define_interface!(IAppxManifestCapabilitiesEnumerator, IAppxManifestCapabilitiesEnumerator_Vtbl, 0x11d22258_f470_42c1_b291_8361c5437e41);
windows_core::imp::interface_hierarchy!(IAppxManifestCapabilitiesEnumerator, windows_core::IUnknown);
impl IAppxManifestCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestCapabilitiesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capability: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestCapabilitiesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        capability.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestCapabilitiesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestCapabilitiesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestCapabilitiesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestCapabilitiesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestDeviceCapabilitiesEnumerator, IAppxManifestDeviceCapabilitiesEnumerator_Vtbl, 0x30204541_427b_4a1c_bacf_655bf463a540);
windows_core::imp::interface_hierarchy!(IAppxManifestDeviceCapabilitiesEnumerator, windows_core::IUnknown);
impl IAppxManifestDeviceCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestDeviceCapabilitiesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicecapability: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDeviceCapabilitiesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        devicecapability.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDeviceCapabilitiesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDeviceCapabilitiesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDeviceCapabilitiesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestDeviceCapabilitiesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestDriverConstraint, IAppxManifestDriverConstraint_Vtbl, 0xc031bee4_bbcc_48ea_a237_c34045c80a07);
windows_core::imp::interface_hierarchy!(IAppxManifestDriverConstraint, windows_core::IUnknown);
impl IAppxManifestDriverConstraint {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinDate(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetMinDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestDriverConstraint_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
    fn GetMinDate(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestDriverConstraint_Vtbl {
    pub const fn new<Identity: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraint_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraint_Impl::GetMinVersion(this) {
                    Ok(ok__) => {
                        minversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinDate<Identity: IAppxManifestDriverConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mindate: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraint_Impl::GetMinDate(this) {
                    Ok(ok__) => {
                        mindate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
            GetMinDate: GetMinDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestDriverConstraint {}
windows_core::imp::define_interface!(IAppxManifestDriverConstraintsEnumerator, IAppxManifestDriverConstraintsEnumerator_Vtbl, 0xd402b2d1_f600_49e0_95e6_975d8da13d89);
windows_core::imp::interface_hierarchy!(IAppxManifestDriverConstraintsEnumerator, windows_core::IUnknown);
impl IAppxManifestDriverConstraintsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverConstraint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestDriverConstraintsEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverConstraint>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverconstraint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraintsEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        driverconstraint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraintsEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverConstraintsEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverConstraintsEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestDriverConstraintsEnumerator {}
windows_core::imp::define_interface!(IAppxManifestDriverDependenciesEnumerator, IAppxManifestDriverDependenciesEnumerator_Vtbl, 0xfe039db2_467f_4755_8404_8f5eb6865b33);
windows_core::imp::interface_hierarchy!(IAppxManifestDriverDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestDriverDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestDriverDependenciesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverdependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverDependenciesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        driverdependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverDependenciesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverDependenciesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependenciesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestDriverDependenciesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestDriverDependency, IAppxManifestDriverDependency_Vtbl, 0x1210cb94_5a92_4602_be24_79f318af4af9);
windows_core::imp::interface_hierarchy!(IAppxManifestDriverDependency, windows_core::IUnknown);
impl IAppxManifestDriverDependency {
    pub unsafe fn GetDriverConstraints(&self) -> windows_core::Result<IAppxManifestDriverConstraintsEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDriverConstraints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDriverConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxManifestDriverDependency_Impl: windows_core::IUnknownImpl {
    fn GetDriverConstraints(&self) -> windows_core::Result<IAppxManifestDriverConstraintsEnumerator>;
}
impl IAppxManifestDriverDependency_Vtbl {
    pub const fn new<Identity: IAppxManifestDriverDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDriverConstraints<Identity: IAppxManifestDriverDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverconstraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestDriverDependency_Impl::GetDriverConstraints(this) {
                    Ok(ok__) => {
                        driverconstraints.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDriverConstraints: GetDriverConstraints::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestDriverDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestDriverDependency {}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependenciesEnumerator, IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl, 0x6427a646_7f49_433e_b1a6_0da309f6885a);
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestHostRuntimeDependenciesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostruntimedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        hostruntimedependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependenciesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependenciesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependenciesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependency, IAppxManifestHostRuntimeDependency_Vtbl, 0x3455d234_8414_410d_95c7_7b35255b8391);
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependency, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IAppxManifestHostRuntimeDependency_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
}
impl IAppxManifestHostRuntimeDependency_Vtbl {
    pub const fn new<Identity: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependency_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependency_Impl::GetPublisher(this) {
                    Ok(ok__) => {
                        publisher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependency_Impl::GetMinVersion(this) {
                    Ok(ok__) => {
                        minversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependency {}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependency2, IAppxManifestHostRuntimeDependency2_Vtbl, 0xc26f23a8_ee10_4ad6_b898_2b4d7aebfe6a);
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependency2, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependency2 {
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestHostRuntimeDependency2_Impl: windows_core::IUnknownImpl {
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestHostRuntimeDependency2_Vtbl {
    pub const fn new<Identity: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageFamilyName<Identity: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestHostRuntimeDependency2_Impl::GetPackageFamilyName(this) {
                    Ok(ok__) => {
                        packagefamilyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestHostRuntimeDependency2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestHostRuntimeDependency2 {}
windows_core::imp::define_interface!(IAppxManifestMainPackageDependenciesEnumerator, IAppxManifestMainPackageDependenciesEnumerator_Vtbl, 0xa99c4f00_51d2_4f0f_ba46_7ed5255ebdff);
windows_core::imp::interface_hierarchy!(IAppxManifestMainPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestMainPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestMainPackageDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestMainPackageDependenciesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestMainPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        mainpackagedependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependenciesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestMainPackageDependenciesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestMainPackageDependency, IAppxManifestMainPackageDependency_Vtbl, 0x05d0611c_bc29_46d5_97e2_84b9c79bd8ae);
windows_core::imp::interface_hierarchy!(IAppxManifestMainPackageDependency, windows_core::IUnknown);
impl IAppxManifestMainPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestMainPackageDependency_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestMainPackageDependency_Vtbl {
    pub const fn new<Identity: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependency_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependency_Impl::GetPublisher(this) {
                    Ok(ok__) => {
                        publisher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: IAppxManifestMainPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestMainPackageDependency_Impl::GetPackageFamilyName(this) {
                    Ok(ok__) => {
                        packagefamilyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestMainPackageDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestMainPackageDependency {}
windows_core::imp::define_interface!(IAppxManifestOSPackageDependenciesEnumerator, IAppxManifestOSPackageDependenciesEnumerator_Vtbl, 0xb84e2fc3_f8ec_4bc1_8ae2_156346f5ffea);
windows_core::imp::interface_hierarchy!(IAppxManifestOSPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestOSPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestOSPackageDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestOSPackageDependenciesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestOSPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ospackagedependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOSPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        ospackagedependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOSPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOSPackageDependenciesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestOSPackageDependenciesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestOSPackageDependency, IAppxManifestOSPackageDependency_Vtbl, 0x154995ee_54a6_4f14_ac97_d8cf0519644b);
windows_core::imp::interface_hierarchy!(IAppxManifestOSPackageDependency, windows_core::IUnknown);
impl IAppxManifestOSPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IAppxManifestOSPackageDependency_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetVersion(&self) -> windows_core::Result<u64>;
}
impl IAppxManifestOSPackageDependency_Vtbl {
    pub const fn new<Identity: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOSPackageDependency_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IAppxManifestOSPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOSPackageDependency_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetVersion: GetVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOSPackageDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestOSPackageDependency {}
windows_core::imp::define_interface!(IAppxManifestOptionalPackageInfo, IAppxManifestOptionalPackageInfo_Vtbl, 0x2634847d_5b5d_4fe5_a243_002ff95edc7e);
windows_core::imp::interface_hierarchy!(IAppxManifestOptionalPackageInfo, windows_core::IUnknown);
impl IAppxManifestOptionalPackageInfo {
    pub unsafe fn GetIsOptionalPackage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsOptionalPackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMainPackageName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainPackageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOptionalPackageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsOptionalPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetMainPackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestOptionalPackageInfo_Impl: windows_core::IUnknownImpl {
    fn GetIsOptionalPackage(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetMainPackageName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestOptionalPackageInfo_Vtbl {
    pub const fn new<Identity: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsOptionalPackage<Identity: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isoptionalpackage: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOptionalPackageInfo_Impl::GetIsOptionalPackage(this) {
                    Ok(ok__) => {
                        isoptionalpackage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMainPackageName<Identity: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestOptionalPackageInfo_Impl::GetMainPackageName(this) {
                    Ok(ok__) => {
                        mainpackagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsOptionalPackage: GetIsOptionalPackage::<Identity, OFFSET>,
            GetMainPackageName: GetMainPackageName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestOptionalPackageInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestOptionalPackageInfo {}
windows_core::imp::define_interface!(IAppxManifestPackageDependenciesEnumerator, IAppxManifestPackageDependenciesEnumerator_Vtbl, 0xb43bbcf9_65a6_42dd_bac0_8c6741e7f5a4);
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestPackageDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageDependenciesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestPackageDependency>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependenciesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        dependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependenciesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependenciesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependenciesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageDependenciesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestPackageDependency, IAppxManifestPackageDependency_Vtbl, 0xe4946b59_733e_43f0_a724_3bde4c1285a0);
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependency, windows_core::IUnknown);
impl IAppxManifestPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageDependency_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
}
impl IAppxManifestPackageDependency_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependency_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependency_Impl::GetPublisher(this) {
                    Ok(ok__) => {
                        publisher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: IAppxManifestPackageDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependency_Impl::GetMinVersion(this) {
                    Ok(ok__) => {
                        minversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency {}
windows_core::imp::define_interface!(IAppxManifestPackageDependency2, IAppxManifestPackageDependency2_Vtbl, 0xdda0b713_f3ff_49d3_898a_2786780c5d98);
impl core::ops::Deref for IAppxManifestPackageDependency2 {
    type Target = IAppxManifestPackageDependency;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependency2, windows_core::IUnknown, IAppxManifestPackageDependency);
impl IAppxManifestPackageDependency2 {
    pub unsafe fn GetMaxMajorVersionTested(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxMajorVersionTested)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency2_Vtbl {
    pub base__: IAppxManifestPackageDependency_Vtbl,
    pub GetMaxMajorVersionTested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageDependency2_Impl: IAppxManifestPackageDependency_Impl {
    fn GetMaxMajorVersionTested(&self) -> windows_core::Result<u16>;
}
impl IAppxManifestPackageDependency2_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageDependency2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxMajorVersionTested<Identity: IAppxManifestPackageDependency2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxmajorversiontested: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependency2_Impl::GetMaxMajorVersionTested(this) {
                    Ok(ok__) => {
                        maxmajorversiontested.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAppxManifestPackageDependency_Vtbl::new::<Identity, OFFSET>(), GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency2 as windows_core::Interface>::IID || iid == &<IAppxManifestPackageDependency as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency2 {}
windows_core::imp::define_interface!(IAppxManifestPackageDependency3, IAppxManifestPackageDependency3_Vtbl, 0x1ac56374_6198_4d6b_92e4_749d5ab8a895);
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependency3, windows_core::IUnknown);
impl IAppxManifestPackageDependency3 {
    pub unsafe fn GetIsOptional(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsOptional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsOptional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageDependency3_Impl: windows_core::IUnknownImpl {
    fn GetIsOptional(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestPackageDependency3_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageDependency3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsOptional<Identity: IAppxManifestPackageDependency3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isoptional: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageDependency3_Impl::GetIsOptional(this) {
                    Ok(ok__) => {
                        isoptional.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIsOptional: GetIsOptional::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageDependency3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageDependency3 {}
windows_core::imp::define_interface!(IAppxManifestPackageId, IAppxManifestPackageId_Vtbl, 0x283ce2d7_7153_4a91_9649_7a0f7240945f);
windows_core::imp::interface_hierarchy!(IAppxManifestPackageId, windows_core::IUnknown);
impl IAppxManifestPackageId {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetArchitecture(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArchitecture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResourceId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ComparePublisher<P0>(&self, other: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComparePublisher)(windows_core::Interface::as_raw(self), other.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageFullName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageFullName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_PACKAGE_ARCHITECTURE) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ComparePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageId_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetArchitecture(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE>;
    fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetVersion(&self) -> windows_core::Result<u64>;
    fn GetResourceId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn ComparePublisher(&self, other: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GetPackageFullName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestPackageId_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArchitecture<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetArchitecture(this) {
                    Ok(ok__) => {
                        architecture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPublisher<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publisher: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetPublisher(this) {
                    Ok(ok__) => {
                        publisher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersion<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetVersion(this) {
                    Ok(ok__) => {
                        packageversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourceId<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetResourceId(this) {
                    Ok(ok__) => {
                        resourceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComparePublisher<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, other: windows_core::PCWSTR, issame: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::ComparePublisher(this, core::mem::transmute(&other)) {
                    Ok(ok__) => {
                        issame.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageFullName<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefullname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetPackageFullName(this) {
                    Ok(ok__) => {
                        packagefullname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: IAppxManifestPackageId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefamilyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId_Impl::GetPackageFamilyName(this) {
                    Ok(ok__) => {
                        packagefamilyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetArchitecture: GetArchitecture::<Identity, OFFSET>,
            GetPublisher: GetPublisher::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetResourceId: GetResourceId::<Identity, OFFSET>,
            ComparePublisher: ComparePublisher::<Identity, OFFSET>,
            GetPackageFullName: GetPackageFullName::<Identity, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageId {}
windows_core::imp::define_interface!(IAppxManifestPackageId2, IAppxManifestPackageId2_Vtbl, 0x2256999d_d617_42f1_880e_0ba4542319d5);
impl core::ops::Deref for IAppxManifestPackageId2 {
    type Target = IAppxManifestPackageId;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageId2, windows_core::IUnknown, IAppxManifestPackageId);
impl IAppxManifestPackageId2 {
    pub unsafe fn GetArchitecture2(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetArchitecture2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId2_Vtbl {
    pub base__: IAppxManifestPackageId_Vtbl,
    pub GetArchitecture2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_PACKAGE_ARCHITECTURE2) -> windows_core::HRESULT,
}
pub trait IAppxManifestPackageId2_Impl: IAppxManifestPackageId_Impl {
    fn GetArchitecture2(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE2>;
}
impl IAppxManifestPackageId2_Vtbl {
    pub const fn new<Identity: IAppxManifestPackageId2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetArchitecture2<Identity: IAppxManifestPackageId2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestPackageId2_Impl::GetArchitecture2(this) {
                    Ok(ok__) => {
                        architecture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAppxManifestPackageId_Vtbl::new::<Identity, OFFSET>(), GetArchitecture2: GetArchitecture2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestPackageId2 as windows_core::Interface>::IID || iid == &<IAppxManifestPackageId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestPackageId2 {}
windows_core::imp::define_interface!(IAppxManifestProperties, IAppxManifestProperties_Vtbl, 0x03faf64d_f26f_4b2c_aaf7_8fe7789b8bca);
windows_core::imp::interface_hierarchy!(IAppxManifestProperties, windows_core::IUnknown);
impl IAppxManifestProperties {
    pub unsafe fn GetBoolValue<P0>(&self, name: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoolValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IAppxManifestProperties_Impl: windows_core::IUnknownImpl {
    fn GetBoolValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GetStringValue(&self, name: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl IAppxManifestProperties_Vtbl {
    pub const fn new<Identity: IAppxManifestProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBoolValue<Identity: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestProperties_Impl::GetBoolValue(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: IAppxManifestProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestProperties_Impl::GetStringValue(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoolValue: GetBoolValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestProperties {}
windows_core::imp::define_interface!(IAppxManifestQualifiedResource, IAppxManifestQualifiedResource_Vtbl, 0x3b53a497_3c5c_48d1_9ea3_bb7eac8cd7d4);
windows_core::imp::interface_hierarchy!(IAppxManifestQualifiedResource, windows_core::IUnknown);
impl IAppxManifestQualifiedResource {
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScale(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDXFeatureLevel(&self) -> windows_core::Result<DX_FEATURE_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDXFeatureLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDXFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DX_FEATURE_LEVEL) -> windows_core::HRESULT,
}
pub trait IAppxManifestQualifiedResource_Impl: windows_core::IUnknownImpl {
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetScale(&self) -> windows_core::Result<u32>;
    fn GetDXFeatureLevel(&self) -> windows_core::Result<DX_FEATURE_LEVEL>;
}
impl IAppxManifestQualifiedResource_Vtbl {
    pub const fn new<Identity: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguage<Identity: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResource_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        language.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScale<Identity: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scale: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResource_Impl::GetScale(this) {
                    Ok(ok__) => {
                        scale.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDXFeatureLevel<Identity: IAppxManifestQualifiedResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResource_Impl::GetDXFeatureLevel(this) {
                    Ok(ok__) => {
                        dxfeaturelevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetScale: GetScale::<Identity, OFFSET>,
            GetDXFeatureLevel: GetDXFeatureLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestQualifiedResource {}
windows_core::imp::define_interface!(IAppxManifestQualifiedResourcesEnumerator, IAppxManifestQualifiedResourcesEnumerator_Vtbl, 0x8ef6adfe_3762_4a8f_9373_2fc5d444c8d2);
windows_core::imp::interface_hierarchy!(IAppxManifestQualifiedResourcesEnumerator, windows_core::IUnknown);
impl IAppxManifestQualifiedResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestQualifiedResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestQualifiedResourcesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestQualifiedResource>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResourcesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResourcesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestQualifiedResourcesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestQualifiedResourcesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestQualifiedResourcesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestReader, IAppxManifestReader_Vtbl, 0x4e1bd148_55a0_4480_a3d1_15544710637c);
windows_core::imp::interface_hierarchy!(IAppxManifestReader, windows_core::IUnknown);
impl IAppxManifestReader {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<IAppxManifestProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPackageDependencies(&self) -> windows_core::Result<IAppxManifestPackageDependenciesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<APPX_CAPABILITIES> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResources(&self) -> windows_core::Result<IAppxManifestResourcesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> windows_core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrerequisite)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetApplications(&self) -> windows_core::Result<IAppxManifestApplicationsEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPackageDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_CAPABILITIES) -> windows_core::HRESULT,
    pub GetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrerequisite: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u64) -> windows_core::HRESULT,
    pub GetApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader_Impl: windows_core::IUnknownImpl {
    fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId>;
    fn GetProperties(&self) -> windows_core::Result<IAppxManifestProperties>;
    fn GetPackageDependencies(&self) -> windows_core::Result<IAppxManifestPackageDependenciesEnumerator>;
    fn GetCapabilities(&self) -> windows_core::Result<APPX_CAPABILITIES>;
    fn GetResources(&self) -> windows_core::Result<IAppxManifestResourcesEnumerator>;
    fn GetDeviceCapabilities(&self) -> windows_core::Result<IAppxManifestDeviceCapabilitiesEnumerator>;
    fn GetPrerequisite(&self, name: &windows_core::PCWSTR) -> windows_core::Result<u64>;
    fn GetApplications(&self) -> windows_core::Result<IAppxManifestApplicationsEnumerator>;
    fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader_Vtbl {
    pub const fn new<Identity: IAppxManifestReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPackageId<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetPackageId(this) {
                    Ok(ok__) => {
                        packageid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packageproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetProperties(this) {
                    Ok(ok__) => {
                        packageproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPackageDependencies<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetPackageDependencies(this) {
                    Ok(ok__) => {
                        dependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCapabilities<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetCapabilities(this) {
                    Ok(ok__) => {
                        capabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResources<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetResources(this) {
                    Ok(ok__) => {
                        resources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicecapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetDeviceCapabilities(this) {
                    Ok(ok__) => {
                        devicecapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPrerequisite<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetPrerequisite(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplications<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applications: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetApplications(this) {
                    Ok(ok__) => {
                        applications.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStream<Identity: IAppxManifestReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifeststream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader_Impl::GetStream(this) {
                    Ok(ok__) => {
                        manifeststream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPackageId: GetPackageId::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPackageDependencies: GetPackageDependencies::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetResources: GetResources::<Identity, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, OFFSET>,
            GetPrerequisite: GetPrerequisite::<Identity, OFFSET>,
            GetApplications: GetApplications::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader {}
windows_core::imp::define_interface!(IAppxManifestReader2, IAppxManifestReader2_Vtbl, 0xd06f67bc_b31d_4eba_a8af_638e73e77b4d);
impl core::ops::Deref for IAppxManifestReader2 {
    type Target = IAppxManifestReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader2, windows_core::IUnknown, IAppxManifestReader);
impl IAppxManifestReader2 {
    pub unsafe fn GetQualifiedResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQualifiedResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader2_Vtbl {
    pub base__: IAppxManifestReader_Vtbl,
    pub GetQualifiedResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader2_Impl: IAppxManifestReader_Impl {
    fn GetQualifiedResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader2_Vtbl {
    pub const fn new<Identity: IAppxManifestReader2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetQualifiedResources<Identity: IAppxManifestReader2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader2_Impl::GetQualifiedResources(this) {
                    Ok(ok__) => {
                        resources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAppxManifestReader_Vtbl::new::<Identity, OFFSET>(), GetQualifiedResources: GetQualifiedResources::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader2 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader2 {}
windows_core::imp::define_interface!(IAppxManifestReader3, IAppxManifestReader3_Vtbl, 0xc43825ab_69b7_400a_9709_cc37f5a72d24);
impl core::ops::Deref for IAppxManifestReader3 {
    type Target = IAppxManifestReader2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader3, windows_core::IUnknown, IAppxManifestReader, IAppxManifestReader2);
impl IAppxManifestReader3 {
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> windows_core::Result<IAppxManifestCapabilitiesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilitiesByCapabilityClass)(windows_core::Interface::as_raw(self), capabilityclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetDeviceFamilies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader3_Vtbl {
    pub base__: IAppxManifestReader2_Vtbl,
    pub GetCapabilitiesByCapabilityClass: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_CAPABILITY_CLASS_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader3_Impl: IAppxManifestReader2_Impl {
    fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> windows_core::Result<IAppxManifestCapabilitiesEnumerator>;
    fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader3_Vtbl {
    pub const fn new<Identity: IAppxManifestReader3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Identity: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader3_Impl::GetCapabilitiesByCapabilityClass(this, core::mem::transmute_copy(&capabilityclass)) {
                    Ok(ok__) => {
                        capabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: IAppxManifestReader3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamilies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader3_Impl::GetTargetDeviceFamilies(this) {
                    Ok(ok__) => {
                        targetdevicefamilies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IAppxManifestReader2_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Identity, OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader3 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID || iid == &<IAppxManifestReader2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader3 {}
windows_core::imp::define_interface!(IAppxManifestReader4, IAppxManifestReader4_Vtbl, 0x4579bb7c_741d_4161_b5a1_47bd3b78ad9b);
impl core::ops::Deref for IAppxManifestReader4 {
    type Target = IAppxManifestReader3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader4, windows_core::IUnknown, IAppxManifestReader, IAppxManifestReader2, IAppxManifestReader3);
impl IAppxManifestReader4 {
    pub unsafe fn GetOptionalPackageInfo(&self) -> windows_core::Result<IAppxManifestOptionalPackageInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionalPackageInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader4_Vtbl {
    pub base__: IAppxManifestReader3_Vtbl,
    pub GetOptionalPackageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader4_Impl: IAppxManifestReader3_Impl {
    fn GetOptionalPackageInfo(&self) -> windows_core::Result<IAppxManifestOptionalPackageInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxManifestReader4_Vtbl {
    pub const fn new<Identity: IAppxManifestReader4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionalPackageInfo<Identity: IAppxManifestReader4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionalpackageinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader4_Impl::GetOptionalPackageInfo(this) {
                    Ok(ok__) => {
                        optionalpackageinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAppxManifestReader3_Vtbl::new::<Identity, OFFSET>(), GetOptionalPackageInfo: GetOptionalPackageInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader4 as windows_core::Interface>::IID || iid == &<IAppxManifestReader as windows_core::Interface>::IID || iid == &<IAppxManifestReader2 as windows_core::Interface>::IID || iid == &<IAppxManifestReader3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxManifestReader4 {}
windows_core::imp::define_interface!(IAppxManifestReader5, IAppxManifestReader5_Vtbl, 0x8d7ae132_a690_4c00_b75a_6aae1feaac80);
windows_core::imp::interface_hierarchy!(IAppxManifestReader5, windows_core::IUnknown);
impl IAppxManifestReader5 {
    pub unsafe fn GetMainPackageDependencies(&self) -> windows_core::Result<IAppxManifestMainPackageDependenciesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader5_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMainPackageDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxManifestReader5_Impl: windows_core::IUnknownImpl {
    fn GetMainPackageDependencies(&self) -> windows_core::Result<IAppxManifestMainPackageDependenciesEnumerator>;
}
impl IAppxManifestReader5_Vtbl {
    pub const fn new<Identity: IAppxManifestReader5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMainPackageDependencies<Identity: IAppxManifestReader5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainpackagedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader5_Impl::GetMainPackageDependencies(this) {
                    Ok(ok__) => {
                        mainpackagedependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMainPackageDependencies: GetMainPackageDependencies::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader5 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestReader5 {}
windows_core::imp::define_interface!(IAppxManifestReader6, IAppxManifestReader6_Vtbl, 0x34deaca4_d3c0_4e3e_b312_e42625e3807e);
windows_core::imp::interface_hierarchy!(IAppxManifestReader6, windows_core::IUnknown);
impl IAppxManifestReader6 {
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIsNonQualifiedResourcePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader6_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestReader6_Impl: windows_core::IUnknownImpl {
    fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestReader6_Vtbl {
    pub const fn new<Identity: IAppxManifestReader6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: IAppxManifestReader6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isnonqualifiedresourcepackage: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader6_Impl::GetIsNonQualifiedResourcePackage(this) {
                    Ok(ok__) => {
                        isnonqualifiedresourcepackage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader6 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestReader6 {}
windows_core::imp::define_interface!(IAppxManifestReader7, IAppxManifestReader7_Vtbl, 0x8efe6f27_0ce0_4988_b32d_738eb63db3b7);
windows_core::imp::interface_hierarchy!(IAppxManifestReader7, windows_core::IUnknown);
impl IAppxManifestReader7 {
    pub unsafe fn GetDriverDependencies(&self) -> windows_core::Result<IAppxManifestDriverDependenciesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDriverDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOSPackageDependencies(&self) -> windows_core::Result<IAppxManifestOSPackageDependenciesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOSPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHostRuntimeDependencies(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependenciesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostRuntimeDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader7_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDriverDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOSPackageDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHostRuntimeDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxManifestReader7_Impl: windows_core::IUnknownImpl {
    fn GetDriverDependencies(&self) -> windows_core::Result<IAppxManifestDriverDependenciesEnumerator>;
    fn GetOSPackageDependencies(&self) -> windows_core::Result<IAppxManifestOSPackageDependenciesEnumerator>;
    fn GetHostRuntimeDependencies(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependenciesEnumerator>;
}
impl IAppxManifestReader7_Vtbl {
    pub const fn new<Identity: IAppxManifestReader7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDriverDependencies<Identity: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, driverdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader7_Impl::GetDriverDependencies(this) {
                    Ok(ok__) => {
                        driverdependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOSPackageDependencies<Identity: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ospackagedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader7_Impl::GetOSPackageDependencies(this) {
                    Ok(ok__) => {
                        ospackagedependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Identity: IAppxManifestReader7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hostruntimedependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestReader7_Impl::GetHostRuntimeDependencies(this) {
                    Ok(ok__) => {
                        hostruntimedependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDriverDependencies: GetDriverDependencies::<Identity, OFFSET>,
            GetOSPackageDependencies: GetOSPackageDependencies::<Identity, OFFSET>,
            GetHostRuntimeDependencies: GetHostRuntimeDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestReader7 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestReader7 {}
windows_core::imp::define_interface!(IAppxManifestResourcesEnumerator, IAppxManifestResourcesEnumerator_Vtbl, 0xde4dfbbd_881a_48bb_858c_d6f2baeae6ed);
windows_core::imp::interface_hierarchy!(IAppxManifestResourcesEnumerator, windows_core::IUnknown);
impl IAppxManifestResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestResourcesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestResourcesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestResourcesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestResourcesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        resource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestResourcesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestResourcesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestResourcesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestResourcesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestResourcesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestTargetDeviceFamiliesEnumerator, IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl, 0x36537f36_27a4_4788_88c0_733819575017);
windows_core::imp::interface_hierarchy!(IAppxManifestTargetDeviceFamiliesEnumerator, windows_core::IUnknown);
impl IAppxManifestTargetDeviceFamiliesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamily> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAppxManifestTargetDeviceFamiliesEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCurrent(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamily>;
    fn GetHasCurrent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn MoveNext(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub const fn new<Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrent<Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetdevicefamily: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        targetdevicefamily.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHasCurrent<Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hascurrent: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::GetHasCurrent(this) {
                    Ok(ok__) => {
                        hascurrent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<Identity: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hasnext: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamiliesEnumerator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        hasnext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrent: GetCurrent::<Identity, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, OFFSET>,
            MoveNext: MoveNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamiliesEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestTargetDeviceFamiliesEnumerator {}
windows_core::imp::define_interface!(IAppxManifestTargetDeviceFamily, IAppxManifestTargetDeviceFamily_Vtbl, 0x9091b09b_c8d5_4f31_8687_a338259faefb);
windows_core::imp::interface_hierarchy!(IAppxManifestTargetDeviceFamily, windows_core::IUnknown);
impl IAppxManifestTargetDeviceFamily {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxVersionTested(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxVersionTested)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamily_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetMaxVersionTested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IAppxManifestTargetDeviceFamily_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetMinVersion(&self) -> windows_core::Result<u64>;
    fn GetMaxVersionTested(&self) -> windows_core::Result<u64>;
}
impl IAppxManifestTargetDeviceFamily_Vtbl {
    pub const fn new<Identity: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamily_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinVersion<Identity: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamily_Impl::GetMinVersion(this) {
                    Ok(ok__) => {
                        minversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxVersionTested<Identity: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxversiontested: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxManifestTargetDeviceFamily_Impl::GetMaxVersionTested(this) {
                    Ok(ok__) => {
                        maxversiontested.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, OFFSET>,
            GetMaxVersionTested: GetMaxVersionTested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxManifestTargetDeviceFamily as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxManifestTargetDeviceFamily {}
windows_core::imp::define_interface!(IAppxPackageEditor, IAppxPackageEditor_Vtbl, 0xe2adb6dc_5e71_4416_86b6_86e5f5291a6b);
windows_core::imp::interface_hierarchy!(IAppxPackageEditor, windows_core::IUnknown);
impl IAppxPackageEditor {
    pub unsafe fn SetWorkingDirectory<P0>(&self, workingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), workingdirectory.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackage<P0, P1, P2>(&self, updatedpackagestream: P0, baselinepackagestream: P1, deltapackagestream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDeltaPackage)(windows_core::Interface::as_raw(self), updatedpackagestream.param().abi(), baselinepackagestream.param().abi(), deltapackagestream.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackageUsingBaselineBlockMap<P0, P1, P2, P3>(&self, updatedpackagestream: P0, baselineblockmapstream: P1, baselinepackagefullname: P2, deltapackagestream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDeltaPackageUsingBaselineBlockMap)(windows_core::Interface::as_raw(self), updatedpackagestream.param().abi(), baselineblockmapstream.param().abi(), baselinepackagefullname.param().abi(), deltapackagestream.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackage<P0, P1>(&self, baselinepackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdatePackage)(windows_core::Interface::as_raw(self), baselinepackagestream.param().abi(), deltapackagestream.param().abi(), updateoption).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateEncryptedPackage<P0, P1>(&self, baselineencryptedpackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateEncryptedPackage)(windows_core::Interface::as_raw(self), baselineencryptedpackagestream.param().abi(), deltapackagestream.param().abi(), updateoption, core::mem::transmute(settings), keyinfo).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackageManifest<P0, P1>(&self, packagestream: P0, updatedmanifeststream: P1, ispackageencrypted: bool, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdatePackageManifest)(windows_core::Interface::as_raw(self), packagestream.param().abi(), updatedmanifeststream.param().abi(), ispackageencrypted.into(), options).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackageUsingBaselineBlockMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackageUsingBaselineBlockMap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdatePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdatePackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateEncryptedPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateEncryptedPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdatePackageManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdatePackageManifest: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageEditor_Impl: windows_core::IUnknownImpl {
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateDeltaPackage(&self, updatedpackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, baselinepackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, deltapackagestream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CreateDeltaPackageUsingBaselineBlockMap(&self, updatedpackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, baselineblockmapstream: windows_core::Ref<super::super::super::System::Com::IStream>, baselinepackagefullname: &windows_core::PCWSTR, deltapackagestream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn UpdatePackage(&self, baselinepackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, deltapackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::Result<()>;
    fn UpdateEncryptedPackage(&self, baselineencryptedpackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, deltapackagestream: windows_core::Ref<super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>;
    fn UpdatePackageManifest(&self, packagestream: windows_core::Ref<super::super::super::System::Com::IStream>, updatedmanifeststream: windows_core::Ref<super::super::super::System::Com::IStream>, ispackageencrypted: windows_core::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageEditor_Vtbl {
    pub const fn new<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWorkingDirectory<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        unsafe extern "system" fn CreateDeltaPackage<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatedpackagestream: *mut core::ffi::c_void, baselinepackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::CreateDeltaPackage(this, core::mem::transmute_copy(&updatedpackagestream), core::mem::transmute_copy(&baselinepackagestream), core::mem::transmute_copy(&deltapackagestream)).into()
            }
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatedpackagestream: *mut core::ffi::c_void, baselineblockmapstream: *mut core::ffi::c_void, baselinepackagefullname: windows_core::PCWSTR, deltapackagestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::CreateDeltaPackageUsingBaselineBlockMap(this, core::mem::transmute_copy(&updatedpackagestream), core::mem::transmute_copy(&baselineblockmapstream), core::mem::transmute(&baselinepackagefullname), core::mem::transmute_copy(&deltapackagestream)).into()
            }
        }
        unsafe extern "system" fn UpdatePackage<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselinepackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::UpdatePackage(this, core::mem::transmute_copy(&baselinepackagestream), core::mem::transmute_copy(&deltapackagestream), core::mem::transmute_copy(&updateoption)).into()
            }
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineencryptedpackagestream: *mut core::ffi::c_void, deltapackagestream: *mut core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::UpdateEncryptedPackage(this, core::mem::transmute_copy(&baselineencryptedpackagestream), core::mem::transmute_copy(&deltapackagestream), core::mem::transmute_copy(&updateoption), core::mem::transmute_copy(&settings), core::mem::transmute_copy(&keyinfo)).into()
            }
        }
        unsafe extern "system" fn UpdatePackageManifest<Identity: IAppxPackageEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagestream: *mut core::ffi::c_void, updatedmanifeststream: *mut core::ffi::c_void, ispackageencrypted: windows_core::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageEditor_Impl::UpdatePackageManifest(this, core::mem::transmute_copy(&packagestream), core::mem::transmute_copy(&updatedmanifeststream), core::mem::transmute_copy(&ispackageencrypted), core::mem::transmute_copy(&options)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            CreateDeltaPackage: CreateDeltaPackage::<Identity, OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap: CreateDeltaPackageUsingBaselineBlockMap::<Identity, OFFSET>,
            UpdatePackage: UpdatePackage::<Identity, OFFSET>,
            UpdateEncryptedPackage: UpdateEncryptedPackage::<Identity, OFFSET>,
            UpdatePackageManifest: UpdatePackageManifest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageEditor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageEditor {}
windows_core::imp::define_interface!(IAppxPackageReader, IAppxPackageReader_Vtbl, 0xb5c49650_99bc_481c_9a34_3d53a4106708);
windows_core::imp::interface_hierarchy!(IAppxPackageReader, windows_core::IUnknown);
impl IAppxPackageReader {
    pub unsafe fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlockMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFootprintFile)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPayloadFile<P0>(&self, filename: P0) -> windows_core::Result<IAppxFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPayloadFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPayloadFiles(&self) -> windows_core::Result<IAppxFilesEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPayloadFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetManifest(&self) -> windows_core::Result<IAppxManifestReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManifest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBlockMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFootprintFile: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_FOOTPRINT_FILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxPackageReader_Impl: windows_core::IUnknownImpl {
    fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader>;
    fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile>;
    fn GetPayloadFile(&self, filename: &windows_core::PCWSTR) -> windows_core::Result<IAppxFile>;
    fn GetPayloadFiles(&self) -> windows_core::Result<IAppxFilesEnumerator>;
    fn GetManifest(&self) -> windows_core::Result<IAppxManifestReader>;
}
impl IAppxPackageReader_Vtbl {
    pub const fn new<Identity: IAppxPackageReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBlockMap<Identity: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockmapreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxPackageReader_Impl::GetBlockMap(this) {
                    Ok(ok__) => {
                        blockmapreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFootprintFile<Identity: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxPackageReader_Impl::GetFootprintFile(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPayloadFile<Identity: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, file: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxPackageReader_Impl::GetPayloadFile(this, core::mem::transmute(&filename)) {
                    Ok(ok__) => {
                        file.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPayloadFiles<Identity: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxPackageReader_Impl::GetPayloadFiles(this) {
                    Ok(ok__) => {
                        filesenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetManifest<Identity: IAppxPackageReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifestreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxPackageReader_Impl::GetManifest(this) {
                    Ok(ok__) => {
                        manifestreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBlockMap: GetBlockMap::<Identity, OFFSET>,
            GetFootprintFile: GetFootprintFile::<Identity, OFFSET>,
            GetPayloadFile: GetPayloadFile::<Identity, OFFSET>,
            GetPayloadFiles: GetPayloadFiles::<Identity, OFFSET>,
            GetManifest: GetManifest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxPackageReader {}
windows_core::imp::define_interface!(IAppxPackageWriter, IAppxPackageWriter_Vtbl, 0x9099e33b_246f_41e4_881a_008eb613f858);
windows_core::imp::interface_hierarchy!(IAppxPackageWriter, windows_core::IUnknown);
impl IAppxPackageWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFile<P0, P1, P3>(&self, filename: P0, contenttype: P1, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadFile)(windows_core::Interface::as_raw(self), filename.param().abi(), contenttype.param().abi(), compressionoption, inputstream.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0>(&self, manifest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), manifest.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, APPX_COMPRESSION_OPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter_Impl: windows_core::IUnknownImpl {
    fn AddPayloadFile(&self, filename: &windows_core::PCWSTR, contenttype: &windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Close(&self, manifest: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter_Vtbl {
    pub const fn new<Identity: IAppxPackageWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadFile<Identity: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: windows_core::PCWSTR, contenttype: windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageWriter_Impl::AddPayloadFile(this, core::mem::transmute(&filename), core::mem::transmute(&contenttype), core::mem::transmute_copy(&compressionoption), core::mem::transmute_copy(&inputstream)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAppxPackageWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageWriter_Impl::Close(this, core::mem::transmute_copy(&manifest)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddPayloadFile: AddPayloadFile::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter {}
windows_core::imp::define_interface!(IAppxPackageWriter2, IAppxPackageWriter2_Vtbl, 0x2cf5c4fd_e54c_4ea5_ba4e_f8c4b105a8c8);
windows_core::imp::interface_hierarchy!(IAppxPackageWriter2, windows_core::IUnknown);
impl IAppxPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0, P1>(&self, manifest: P0, contentgroupmap: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), manifest.param().abi(), contentgroupmap.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter2_Impl: windows_core::IUnknownImpl {
    fn Close(&self, manifest: windows_core::Ref<super::super::super::System::Com::IStream>, contentgroupmap: windows_core::Ref<super::super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter2_Vtbl {
    pub const fn new<Identity: IAppxPackageWriter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IAppxPackageWriter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manifest: *mut core::ffi::c_void, contentgroupmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageWriter2_Impl::Close(this, core::mem::transmute_copy(&manifest), core::mem::transmute_copy(&contentgroupmap)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter2 {}
windows_core::imp::define_interface!(IAppxPackageWriter3, IAppxPackageWriter3_Vtbl, 0xa83aacd3_41c0_4501_b8a3_74164f50b2fd);
windows_core::imp::interface_hierarchy!(IAppxPackageWriter3, windows_core::IUnknown);
impl IAppxPackageWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFiles(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPayloadFiles)(windows_core::Interface::as_raw(self), payloadfiles.len().try_into().unwrap(), core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFiles: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter3_Impl: windows_core::IUnknownImpl {
    fn AddPayloadFiles(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAppxPackageWriter3_Vtbl {
    pub const fn new<Identity: IAppxPackageWriter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPayloadFiles<Identity: IAppxPackageWriter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackageWriter3_Impl::AddPayloadFiles(this, core::mem::transmute_copy(&filecount), core::mem::transmute_copy(&payloadfiles), core::mem::transmute_copy(&memorylimit)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPayloadFiles: AddPayloadFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackageWriter3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppxPackageWriter3 {}
windows_core::imp::define_interface!(IAppxPackagingDiagnosticEventSink, IAppxPackagingDiagnosticEventSink_Vtbl, 0x17239d47_6adb_45d2_80f6_f9cbc3bf059d);
windows_core::imp::interface_hierarchy!(IAppxPackagingDiagnosticEventSink, windows_core::IUnknown);
impl IAppxPackagingDiagnosticEventSink {
    pub unsafe fn ReportContextChange<P2, P3, P4>(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: P2, contextmessage: P3, detailsmessage: P4) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportContextChange)(windows_core::Interface::as_raw(self), changetype, contextid, contextname.param().abi(), contextmessage.param().abi(), detailsmessage.param().abi()).ok() }
    }
    pub unsafe fn ReportError<P0>(&self, errormessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportError)(windows_core::Interface::as_raw(self), errormessage.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportContextChange: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_PACKAGING_CONTEXT_CHANGE_TYPE, i32, windows_core::PCSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAppxPackagingDiagnosticEventSink_Impl: windows_core::IUnknownImpl {
    fn ReportContextChange(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: &windows_core::PCSTR, contextmessage: &windows_core::PCWSTR, detailsmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReportError(&self, errormessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAppxPackagingDiagnosticEventSink_Vtbl {
    pub const fn new<Identity: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportContextChange<Identity: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: windows_core::PCSTR, contextmessage: windows_core::PCWSTR, detailsmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackagingDiagnosticEventSink_Impl::ReportContextChange(this, core::mem::transmute_copy(&changetype), core::mem::transmute_copy(&contextid), core::mem::transmute(&contextname), core::mem::transmute(&contextmessage), core::mem::transmute(&detailsmessage)).into()
            }
        }
        unsafe extern "system" fn ReportError<Identity: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errormessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackagingDiagnosticEventSink_Impl::ReportError(this, core::mem::transmute(&errormessage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportContextChange: ReportContextChange::<Identity, OFFSET>,
            ReportError: ReportError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxPackagingDiagnosticEventSink {}
windows_core::imp::define_interface!(IAppxPackagingDiagnosticEventSinkManager, IAppxPackagingDiagnosticEventSinkManager_Vtbl, 0x369648fa_a7eb_4909_a15d_6954a078f18a);
windows_core::imp::interface_hierarchy!(IAppxPackagingDiagnosticEventSinkManager, windows_core::IUnknown);
impl IAppxPackagingDiagnosticEventSinkManager {
    pub unsafe fn SetSinkForProcess<P0>(&self, sink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAppxPackagingDiagnosticEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSinkForProcess)(windows_core::Interface::as_raw(self), sink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSinkForProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxPackagingDiagnosticEventSinkManager_Impl: windows_core::IUnknownImpl {
    fn SetSinkForProcess(&self, sink: windows_core::Ref<IAppxPackagingDiagnosticEventSink>) -> windows_core::Result<()>;
}
impl IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub const fn new<Identity: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSinkForProcess<Identity: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppxPackagingDiagnosticEventSinkManager_Impl::SetSinkForProcess(this, core::mem::transmute_copy(&sink)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSinkForProcess: SetSinkForProcess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxPackagingDiagnosticEventSinkManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxPackagingDiagnosticEventSinkManager {}
windows_core::imp::define_interface!(IAppxSourceContentGroupMapReader, IAppxSourceContentGroupMapReader_Vtbl, 0xf329791d_540b_4a9f_bc75_3282b7d73193);
windows_core::imp::interface_hierarchy!(IAppxSourceContentGroupMapReader, windows_core::IUnknown);
impl IAppxSourceContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequiredGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAutomaticGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxSourceContentGroupMapReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAppxSourceContentGroupMapReader_Impl: windows_core::IUnknownImpl {
    fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl IAppxSourceContentGroupMapReader_Vtbl {
    pub const fn new<Identity: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRequiredGroup<Identity: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxSourceContentGroupMapReader_Impl::GetRequiredGroup(this) {
                    Ok(ok__) => {
                        requiredgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: IAppxSourceContentGroupMapReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, automaticgroupsenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAppxSourceContentGroupMapReader_Impl::GetAutomaticGroups(this) {
                    Ok(ok__) => {
                        automaticgroupsenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRequiredGroup: GetRequiredGroup::<Identity, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppxSourceContentGroupMapReader as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAppxSourceContentGroupMapReader {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACKAGEDEPENDENCY_CONTEXT(pub *mut core::ffi::c_void);
impl PACKAGEDEPENDENCY_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACKAGEDEPENDENCY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PACKAGE_APPLICATIONS_MAX_COUNT: u32 = 100u32;
pub const PACKAGE_APPLICATIONS_MIN_COUNT: u32 = 0u32;
pub const PACKAGE_ARCHITECTURE_MAX_LENGTH: u32 = 7u32;
pub const PACKAGE_ARCHITECTURE_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
pub const PACKAGE_FAMILY_MAX_RESOURCE_PACKAGES: u32 = 512u32;
pub const PACKAGE_FAMILY_MIN_RESOURCE_PACKAGES: u32 = 0u32;
pub const PACKAGE_FAMILY_NAME_MAX_LENGTH: u32 = 64u32;
pub const PACKAGE_FAMILY_NAME_MIN_LENGTH: u32 = 17u32;
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
pub const PACKAGE_FULL_NAME_MAX_LENGTH: u32 = 127u32;
pub const PACKAGE_FULL_NAME_MIN_LENGTH: u32 = 30u32;
pub const PACKAGE_GRAPH_MAX_SIZE: u32 = 641u32;
pub const PACKAGE_GRAPH_MIN_SIZE: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
pub const PACKAGE_MAX_DEPENDENCIES: u32 = 128u32;
pub const PACKAGE_MIN_DEPENDENCIES: u32 = 0u32;
pub const PACKAGE_NAME_MAX_LENGTH: u32 = 50u32;
pub const PACKAGE_NAME_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
pub const PACKAGE_PUBLISHERID_MAX_LENGTH: u32 = 13u32;
pub const PACKAGE_PUBLISHERID_MIN_LENGTH: u32 = 13u32;
pub const PACKAGE_PUBLISHER_MAX_LENGTH: u32 = 8192u32;
pub const PACKAGE_PUBLISHER_MIN_LENGTH: u32 = 3u32;
pub const PACKAGE_RELATIVE_APPLICATION_ID_MAX_LENGTH: u32 = 65u32;
pub const PACKAGE_RELATIVE_APPLICATION_ID_MIN_LENGTH: u32 = 2u32;
pub const PACKAGE_RESOURCEID_MAX_LENGTH: u32 = 30u32;
pub const PACKAGE_RESOURCEID_MIN_LENGTH: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
pub const PACKAGE_VERSION_MAX_LENGTH: u32 = 23u32;
pub const PACKAGE_VERSION_MIN_LENGTH: u32 = 7u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE(pub *mut core::ffi::c_void);
impl PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageDependencyLifetimeKind(pub i32);
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
impl PackageDependencyProcessorArchitectures {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PackageDependencyProcessorArchitectures {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PackageDependencyProcessorArchitectures {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageInfo3Type(pub i32);
pub const PackageInfo3Type_PackageInfoGeneration: PackageInfo3Type = PackageInfo3Type(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageOrigin(pub i32);
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackagePathType(pub i32);
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut core::ffi::c_void,
}
impl Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
