#[inline]
pub unsafe fn ActivatePackageVirtualizationContext<P0>(context: P0) -> windows_core::Result<usize>
where
    P0: windows_core::Param<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ActivatePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, cookie : *mut usize) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ActivatePackageVirtualizationContext(context.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn AddPackageDependency<P0>(packagedependencyid: P0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernelbase.dll" "system" fn AddPackageDependency(packagedependencyid : windows_core::PCWSTR, rank : i32, options : AddPackageDependencyOptions, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    AddPackageDependency(packagedependencyid.param().abi(), rank, options, packagedependencycontext, core::mem::transmute(packagefullname.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn AppPolicyGetClrCompat<P0>(processtoken: P0, policy: *mut AppPolicyClrCompat) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetClrCompat(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyClrCompat) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetClrCompat(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess<P0>(processtoken: P0, policy: *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetCreateFileAccess(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetCreateFileAccess(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement<P0>(processtoken: P0, policy: *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetLifecycleManagement(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetLifecycleManagement(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading<P0>(processtoken: P0, policy: *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetMediaFoundationCodecLoading(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetMediaFoundationCodecLoading(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod<P0>(processtoken: P0, policy: *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetProcessTerminationMethod(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetProcessTerminationMethod(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic<P0>(processtoken: P0, policy: *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetShowDeveloperDiagnostic(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetShowDeveloperDiagnostic(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType<P0>(processtoken: P0, policy: *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetThreadInitializationType(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetThreadInitializationType(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn AppPolicyGetWindowingModel<P0>(processtoken: P0, policy: *mut AppPolicyWindowingModel) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AppPolicyGetWindowingModel(processtoken : super::super::super::Foundation:: HANDLE, policy : *mut AppPolicyWindowingModel) -> super::super::super::Foundation:: WIN32_ERROR);
    AppPolicyGetWindowingModel(processtoken.param().abi(), policy)
}
#[inline]
pub unsafe fn CheckIsMSIXPackage<P0>(packagefullname: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CheckIsMSIXPackage(packagefullname : windows_core::PCWSTR, ismsixpackage : *mut super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CheckIsMSIXPackage(packagefullname.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn ClosePackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    ClosePackageInfo(packageinforeference)
}
#[inline]
pub unsafe fn CreatePackageVirtualizationContext<P0>(packagefamilyname: P0) -> windows_core::Result<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreatePackageVirtualizationContext(packagefamilyname : windows_core::PCWSTR, context : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreatePackageVirtualizationContext(packagefamilyname.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DeactivatePackageVirtualizationContext(cookie: usize) {
    windows_targets::link!("kernel32.dll" "system" fn DeactivatePackageVirtualizationContext(cookie : usize));
    DeactivatePackageVirtualizationContext(cookie)
}
#[inline]
pub unsafe fn DeletePackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernelbase.dll" "system" fn DeletePackageDependency(packagedependencyid : windows_core::PCWSTR) -> windows_core::HRESULT);
    DeletePackageDependency(packagedependencyid.param().abi()).ok()
}
#[inline]
pub unsafe fn DuplicatePackageVirtualizationContext<P0>(sourcecontext: P0) -> windows_core::Result<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>
where
    P0: windows_core::Param<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DuplicatePackageVirtualizationContext(sourcecontext : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE, destcontext : *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DuplicatePackageVirtualizationContext(sourcecontext.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FindPackagesByPackageFamily<P0>(packagefamilyname: P0, packagefilters: u32, count: *mut u32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut u32, buffer: windows_core::PWSTR, packageproperties: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FindPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, packagefilters : u32, count : *mut u32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut u32, buffer : windows_core::PWSTR, packageproperties : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    FindPackagesByPackageFamily(packagefamilyname.param().abi(), packagefilters, count, core::mem::transmute(packagefullnames.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(buffer), core::mem::transmute(packageproperties.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FormatApplicationUserModelId<P0, P1>(packagefamilyname: P0, packagerelativeapplicationid: P1, applicationusermodelidlength: *mut u32, applicationusermodelid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn FormatApplicationUserModelId(packagefamilyname : windows_core::PCWSTR, packagerelativeapplicationid : windows_core::PCWSTR, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    FormatApplicationUserModelId(packagefamilyname.param().abi(), packagerelativeapplicationid.param().abi(), applicationusermodelidlength, core::mem::transmute(applicationusermodelid))
}
#[inline]
pub unsafe fn GetApplicationUserModelId<P0>(hprocess: P0, applicationusermodelidlength: *mut u32, applicationusermodelid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetApplicationUserModelId(hprocess : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetApplicationUserModelId(hprocess.param().abi(), applicationusermodelidlength, core::mem::transmute(applicationusermodelid))
}
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken<P0>(token: P0, applicationusermodelidlength: *mut u32, applicationusermodelid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetApplicationUserModelIdFromToken(token : super::super::super::Foundation:: HANDLE, applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetApplicationUserModelIdFromToken(token.param().abi(), applicationusermodelidlength, core::mem::transmute(applicationusermodelid))
}
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentApplicationUserModelId(applicationusermodelidlength, core::mem::transmute(applicationusermodelid))
}
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageFamilyName(packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackageFamilyName(packagefamilynamelength, core::mem::transmute(packagefamilyname))
}
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageFullName(packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackageFullName(packagefullnamelength, core::mem::transmute(packagefullname))
}
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageId(bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackageId(bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackageInfo(flags, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackageInfo2(flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackageInfo2(flags, packagepathtype, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetCurrentPackageInfo3(flags: u32, packageinfotype: PackageInfo3Type, bufferlength: *mut u32, buffer: Option<*mut core::ffi::c_void>, count: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageInfo3(flags : u32, packageinfotype : PackageInfo3Type, bufferlength : *mut u32, buffer : *mut core::ffi::c_void, count : *mut u32) -> windows_core::HRESULT);
    GetCurrentPackageInfo3(flags, packageinfotype, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackagePath(pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackagePath(pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackagePath2(packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetCurrentPackagePath2(packagepathtype, pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetCurrentPackageVirtualizationContext() -> PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentPackageVirtualizationContext() -> PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE);
    GetCurrentPackageVirtualizationContext()
}
#[inline]
pub unsafe fn GetIdForPackageDependencyContext<P0>(packagedependencycontext: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<PACKAGEDEPENDENCY_CONTEXT>,
{
    windows_targets::link!("kernelbase.dll" "system" fn GetIdForPackageDependencyContext(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetIdForPackageDependencyContext(packagedependencycontext.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetPackageApplicationIds(packageinforeference : *const _PACKAGE_INFO_REFERENCE, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageApplicationIds(packageinforeference, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetPackageFamilyName<P0>(hprocess: P0, packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPackageFamilyName(hprocess : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageFamilyName(hprocess.param().abi(), packagefamilynamelength, core::mem::transmute(packagefamilyname))
}
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken<P0>(token: P0, packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFamilyNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageFamilyNameFromToken(token.param().abi(), packagefamilynamelength, core::mem::transmute(packagefamilyname))
}
#[inline]
pub unsafe fn GetPackageFullName<P0>(hprocess: P0, packagefullnamelength: *mut u32, packagefullname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPackageFullName(hprocess : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageFullName(hprocess.param().abi(), packagefullnamelength, core::mem::transmute(packagefullname))
}
#[inline]
pub unsafe fn GetPackageFullNameFromToken<P0>(token: P0, packagefullnamelength: *mut u32, packagefullname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFullNameFromToken(token : super::super::super::Foundation:: HANDLE, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageFullNameFromToken(token.param().abi(), packagefullnamelength, core::mem::transmute(packagefullname))
}
#[inline]
pub unsafe fn GetPackageGraphRevisionId() -> u32 {
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-6.dll" "system" fn GetPackageGraphRevisionId() -> u32);
    GetPackageGraphRevisionId()
}
#[inline]
pub unsafe fn GetPackageId<P0>(hprocess: P0, bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPackageId(hprocess : super::super::super::Foundation:: HANDLE, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageId(hprocess.param().abi(), bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetPackageInfo(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageInfo(packageinforeference, flags, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackageInfo2(packageinforeference : *const _PACKAGE_INFO_REFERENCE, flags : u32, packagepathtype : PackagePathType, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackageInfo2(packageinforeference, flags, packagepathtype, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(count.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn GetPackagePath(packageid : *const PACKAGE_ID, reserved : u32, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackagePath(packageid, reserved, pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackagePathByFullName(packagefullname.param().abi(), pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetPackagesByPackageFamily<P0>(packagefamilyname: P0, count: *mut u32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut u32, buffer: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, count : *mut u32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut u32, buffer : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetPackagesByPackageFamily(packagefamilyname.param().abi(), count, core::mem::transmute(packagefullnames.unwrap_or(std::ptr::null_mut())), bufferlength, core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn GetProcessesInVirtualizationContext<P0>(packagefamilyname: P0, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessesInVirtualizationContext(packagefamilyname : windows_core::PCWSTR, count : *mut u32, processes : *mut *mut super::super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    GetProcessesInVirtualizationContext(packagefamilyname.param().abi(), count, processes).ok()
}
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernelbase.dll" "system" fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid : windows_core::PCWSTR, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetResolvedPackageFullNameForPackageDependency(packagedependencyid.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetStagedPackageOrigin<P0>(packagefullname: P0, origin: *mut PackageOrigin) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetStagedPackageOrigin(packagefullname : windows_core::PCWSTR, origin : *mut PackageOrigin) -> super::super::super::Foundation:: WIN32_ERROR);
    GetStagedPackageOrigin(packagefullname.param().abi(), origin)
}
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetStagedPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetStagedPackagePathByFullName(packagefullname.param().abi(), pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetStagedPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut u32, path : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    GetStagedPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength, core::mem::transmute(path))
}
#[inline]
pub unsafe fn OpenPackageInfoByFullName<P0>(packagefullname: P0, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenPackageInfoByFullName(packagefullname : windows_core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    OpenPackageInfoByFullName(packagefullname.param().abi(), reserved, packageinforeference)
}
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<P0, P1>(usersid: P0, packagefullname: P1, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::super::Foundation::PSID>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn OpenPackageInfoByFullNameForUser(usersid : super::super::super::Foundation:: PSID, packagefullname : windows_core::PCWSTR, reserved : u32, packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation:: WIN32_ERROR);
    OpenPackageInfoByFullNameForUser(usersid.param().abi(), packagefullname.param().abi(), reserved, packageinforeference)
}
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<P0>(packagefullname: P0, packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn PackageFamilyNameFromFullName(packagefullname : windows_core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    PackageFamilyNameFromFullName(packagefullname.param().abi(), packagefamilynamelength, core::mem::transmute(packagefamilyname))
}
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn PackageFamilyNameFromId(packageid : *const PACKAGE_ID, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    PackageFamilyNameFromId(packageid, packagefamilynamelength, core::mem::transmute(packagefamilyname))
}
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("kernel32.dll" "system" fn PackageFullNameFromId(packageid : *const PACKAGE_ID, packagefullnamelength : *mut u32, packagefullname : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    PackageFullNameFromId(packageid, packagefullnamelength, core::mem::transmute(packagefullname))
}
#[inline]
pub unsafe fn PackageIdFromFullName<P0>(packagefullname: P0, flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn PackageIdFromFullName(packagefullname : windows_core::PCWSTR, flags : u32, bufferlength : *mut u32, buffer : *mut u8) -> super::super::super::Foundation:: WIN32_ERROR);
    PackageIdFromFullName(packagefullname.param().abi(), flags, bufferlength, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<P0>(packagefamilyname: P0, packagenamelength: *mut u32, packagename: windows_core::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname : windows_core::PCWSTR, packagenamelength : *mut u32, packagename : windows_core::PWSTR, packagepublisheridlength : *mut u32, packagepublisherid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    PackageNameAndPublisherIdFromFamilyName(packagefamilyname.param().abi(), packagenamelength, core::mem::transmute(packagename), packagepublisheridlength, core::mem::transmute(packagepublisherid))
}
#[inline]
pub unsafe fn ParseApplicationUserModelId<P0>(applicationusermodelid: P0, packagefamilynamelength: *mut u32, packagefamilyname: windows_core::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn ParseApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR, packagefamilynamelength : *mut u32, packagefamilyname : windows_core::PWSTR, packagerelativeapplicationidlength : *mut u32, packagerelativeapplicationid : windows_core::PWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    ParseApplicationUserModelId(applicationusermodelid.param().abi(), packagefamilynamelength, core::mem::transmute(packagefamilyname), packagerelativeapplicationidlength, core::mem::transmute(packagerelativeapplicationid))
}
#[inline]
pub unsafe fn ReleasePackageVirtualizationContext<P0>(context: P0)
where
    P0: windows_core::Param<PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReleasePackageVirtualizationContext(context : PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE));
    ReleasePackageVirtualizationContext(context.param().abi())
}
#[inline]
pub unsafe fn RemovePackageDependency<P0>(packagedependencycontext: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<PACKAGEDEPENDENCY_CONTEXT>,
{
    windows_targets::link!("kernelbase.dll" "system" fn RemovePackageDependency(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT) -> windows_core::HRESULT);
    RemovePackageDependency(packagedependencycontext.param().abi()).ok()
}
#[inline]
pub unsafe fn TryCreatePackageDependency<P0, P1, P2>(user: P0, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P2, options: CreatePackageDependencyOptions) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<super::super::super::Foundation::PSID>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernelbase.dll" "system" fn TryCreatePackageDependency(user : super::super::super::Foundation:: PSID, packagefamilyname : windows_core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_core::PCWSTR, options : CreatePackageDependencyOptions, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    TryCreatePackageDependency(user.param().abi(), packagefamilyname.param().abi(), core::mem::transmute(minversion), packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VerifyApplicationUserModelId<P0>(applicationusermodelid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    VerifyApplicationUserModelId(applicationusermodelid.param().abi())
}
#[inline]
pub unsafe fn VerifyPackageFamilyName<P0>(packagefamilyname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFamilyName(packagefamilyname : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    VerifyPackageFamilyName(packagefamilyname.param().abi())
}
#[inline]
pub unsafe fn VerifyPackageFullName<P0>(packagefullname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFullName(packagefullname : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    VerifyPackageFullName(packagefullname.param().abi())
}
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> super::super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageId(packageid : *const PACKAGE_ID) -> super::super::super::Foundation:: WIN32_ERROR);
    VerifyPackageId(packageid)
}
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<P0>(packagerelativeapplicationid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid : windows_core::PCWSTR) -> super::super::super::Foundation:: WIN32_ERROR);
    VerifyPackageRelativeApplicationId(packagerelativeapplicationid.param().abi())
}
windows_core::imp::define_interface!(IAppxAppInstallerReader, IAppxAppInstallerReader_Vtbl, 0xf35bc38c_1d2f_43db_a1f4_586430d1fed2);
impl core::ops::Deref for IAppxAppInstallerReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxAppInstallerReader, windows_core::IUnknown);
impl IAppxAppInstallerReader {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetXmlDom(&self) -> windows_core::Result<super::super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetXmlDom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxAppInstallerReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetXmlDom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetXmlDom: usize,
}
windows_core::imp::define_interface!(IAppxBlockMapBlock, IAppxBlockMapBlock_Vtbl, 0x75cf3930_3244_4fe0_a8c8_e0bcb270b889);
impl core::ops::Deref for IAppxBlockMapBlock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBlockMapBlock, windows_core::IUnknown);
impl IAppxBlockMapBlock {
    pub unsafe fn GetHash(&self, buffersize: *mut u32) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHash)(windows_core::Interface::as_raw(self), buffersize, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCompressedSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCompressedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBlockMapBlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetCompressedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBlockMapBlocksEnumerator, IAppxBlockMapBlocksEnumerator_Vtbl, 0x6b429b5b_36ef_479e_b9eb_0c1482b49e16);
impl core::ops::Deref for IAppxBlockMapBlocksEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBlockMapBlocksEnumerator, windows_core::IUnknown);
impl IAppxBlockMapBlocksEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapBlock> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBlockMapBlocksEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBlockMapFile, IAppxBlockMapFile_Vtbl, 0x277672ac_4f63_42c1_8abc_beae3600eb59);
impl core::ops::Deref for IAppxBlockMapFile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBlockMapFile, windows_core::IUnknown);
impl IAppxBlockMapFile {
    pub unsafe fn GetBlocks(&self) -> windows_core::Result<IAppxBlockMapBlocksEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalFileHeaderSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalFileHeaderSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUncompressedSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUncompressedSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValidateFileHash<P0>(&self, filestream: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValidateFileHash)(windows_core::Interface::as_raw(self), filestream.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBlockMapFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalFileHeaderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetUncompressedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValidateFileHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValidateFileHash: usize,
}
windows_core::imp::define_interface!(IAppxBlockMapFilesEnumerator, IAppxBlockMapFilesEnumerator_Vtbl, 0x02b856a2_4262_4070_bacb_1a8cbbc42305);
impl core::ops::Deref for IAppxBlockMapFilesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBlockMapFilesEnumerator, windows_core::IUnknown);
impl IAppxBlockMapFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBlockMapFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBlockMapFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBlockMapReader, IAppxBlockMapReader_Vtbl, 0x5efec991_bca3_42d1_9ec2_e92d609ec22a);
impl core::ops::Deref for IAppxBlockMapReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBlockMapReader, windows_core::IUnknown);
impl IAppxBlockMapReader {
    pub unsafe fn GetFile<P0>(&self, filename: P0) -> windows_core::Result<IAppxBlockMapFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFiles(&self) -> windows_core::Result<IAppxBlockMapFilesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHashMethod(&self) -> windows_core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHashMethod)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxBundleFactory, IAppxBundleFactory_Vtbl, 0xbba65864_965f_4a5f_855f_f074bdbf3a7b);
impl core::ops::Deref for IAppxBundleFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleFactory, windows_core::IUnknown);
impl IAppxBundleFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64) -> windows_core::Result<IAppxBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBundleReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleManifestReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBundleManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBundleManifestReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxBundleFactory2, IAppxBundleFactory2_Vtbl, 0x7325b83d_0185_42c4_82ac_be34ab1a2a8a);
impl core::ops::Deref for IAppxBundleFactory2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleFactory2, windows_core::IUnknown);
impl IAppxBundleFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBundleReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleReader2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleReader2: usize,
}
windows_core::imp::define_interface!(IAppxBundleManifestOptionalBundleInfo, IAppxBundleManifestOptionalBundleInfo_Vtbl, 0x515bf2e8_bcb0_4d69_8c48_e383147b6e12);
impl core::ops::Deref for IAppxBundleManifestOptionalBundleInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfo, windows_core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfo {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageInfoItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestOptionalBundleInfoEnumerator, IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl, 0x9a178793_f97e_46ac_aaca_dd5ba4c177c8);
impl core::ops::Deref for IAppxBundleManifestOptionalBundleInfoEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfoEnumerator, windows_core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo, IAppxBundleManifestPackageInfo_Vtbl, 0x54cd06c1_268f_40bb_8ed2_757a9ebaec8d);
impl core::ops::Deref for IAppxBundleManifestPackageInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo {
    pub unsafe fn GetPackageType(&self) -> windows_core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOffset(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetResources(&self) -> windows_core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> windows_core::HRESULT,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo2, IAppxBundleManifestPackageInfo2_Vtbl, 0x44c2acbc_b2cf_4ccb_bbdb_9c6da8c3bc9e);
impl core::ops::Deref for IAppxBundleManifestPackageInfo2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo2, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo2 {
    pub unsafe fn GetIsPackageReference(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsPackageReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsNonQualifiedResourcePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIsDefaultApplicablePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsDefaultApplicablePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetIsDefaultApplicablePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo3, IAppxBundleManifestPackageInfo3_Vtbl, 0x6ba74b98_bb74_4296_80d0_5f4256a99675);
impl core::ops::Deref for IAppxBundleManifestPackageInfo3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo3, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo3 {
    pub unsafe fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetDeviceFamilies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfo4, IAppxBundleManifestPackageInfo4_Vtbl, 0x5da6f13d_a8a7_4532_857c_1393d659371d);
impl core::ops::Deref for IAppxBundleManifestPackageInfo4 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfo4, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfo4 {
    pub unsafe fn GetIsStub(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsStub)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfo4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestPackageInfoEnumerator, IAppxBundleManifestPackageInfoEnumerator_Vtbl, 0xf9b856ee_49a6_4e19_b2b0_6a2406d63a32);
impl core::ops::Deref for IAppxBundleManifestPackageInfoEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestPackageInfoEnumerator, windows_core::IUnknown);
impl IAppxBundleManifestPackageInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxBundleManifestPackageInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleManifestReader, IAppxBundleManifestReader_Vtbl, 0xcf0ebbc1_cc99_4106_91eb_e67462e04fb0);
impl core::ops::Deref for IAppxBundleManifestReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestReader, windows_core::IUnknown);
impl IAppxBundleManifestReader {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPackageInfoItems(&self) -> windows_core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageInfoItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleManifestReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
windows_core::imp::define_interface!(IAppxBundleManifestReader2, IAppxBundleManifestReader2_Vtbl, 0x5517df70_033f_4af2_8213_87d766805c02);
impl core::ops::Deref for IAppxBundleManifestReader2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleManifestReader2, windows_core::IUnknown);
impl IAppxBundleManifestReader2 {
    pub unsafe fn GetOptionalBundles(&self) -> windows_core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionalBundles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleManifestReader2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOptionalBundles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleReader, IAppxBundleReader_Vtbl, 0xdd75b8c0_ba76_43b0_ae0f_68656a1dc5c8);
impl core::ops::Deref for IAppxBundleReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleReader, windows_core::IUnknown);
impl IAppxBundleReader {
    pub unsafe fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFootprintFile)(windows_core::Interface::as_raw(self), filetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBlockMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetManifest(&self) -> windows_core::Result<IAppxBundleManifestReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetManifest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPayloadPackages(&self) -> windows_core::Result<IAppxFilesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPayloadPackages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPayloadPackage<P0>(&self, filename: P0) -> windows_core::Result<IAppxFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxBundleReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFootprintFile: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_BUNDLE_FOOTPRINT_FILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBlockMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleWriter, IAppxBundleWriter_Vtbl, 0xec446fe8_bfec_4c64_ab4f_49f038f0c6d2);
impl core::ops::Deref for IAppxBundleWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleWriter, windows_core::IUnknown);
impl IAppxBundleWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackage<P0, P1>(&self, filename: P0, packagestream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAppxBundleWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleWriter2, IAppxBundleWriter2_Vtbl, 0x6d8fe971_01cc_49a0_b685_233851279962);
impl core::ops::Deref for IAppxBundleWriter2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleWriter2, windows_core::IUnknown);
impl IAppxBundleWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxBundleWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
windows_core::imp::define_interface!(IAppxBundleWriter3, IAppxBundleWriter3_Vtbl, 0xad711152_f969_4193_82d5_9ddf2786d21a);
impl core::ops::Deref for IAppxBundleWriter3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleWriter3, windows_core::IUnknown);
impl IAppxBundleWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok()
    }
    pub unsafe fn Close<P0>(&self, hashmethodstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), hashmethodstring.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxBundleWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxBundleWriter4, IAppxBundleWriter4_Vtbl, 0x9cd9d523_5009_4c01_9882_dc029fbd47a3);
impl core::ops::Deref for IAppxBundleWriter4 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxBundleWriter4, windows_core::IUnknown);
impl IAppxBundleWriter4 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackage<P0, P1, P2>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddPayloadPackage)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi(), isdefaultapplicablepackage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxBundleWriter4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
windows_core::imp::define_interface!(IAppxContentGroup, IAppxContentGroup_Vtbl, 0x328f6468_c04f_4e3c_b6fa_6b8d27f3003a);
impl core::ops::Deref for IAppxContentGroup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxContentGroup, windows_core::IUnknown);
impl IAppxContentGroup {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFiles(&self) -> windows_core::Result<IAppxContentGroupFilesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxContentGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxContentGroupFilesEnumerator, IAppxContentGroupFilesEnumerator_Vtbl, 0x1a09a2fd_7440_44eb_8c84_848205a6a1cc);
impl core::ops::Deref for IAppxContentGroupFilesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxContentGroupFilesEnumerator, windows_core::IUnknown);
impl IAppxContentGroupFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxContentGroupFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxContentGroupMapReader, IAppxContentGroupMapReader_Vtbl, 0x418726d8_dd99_4f5d_9886_157add20de01);
impl core::ops::Deref for IAppxContentGroupMapReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxContentGroupMapReader, windows_core::IUnknown);
impl IAppxContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequiredGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutomaticGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxContentGroupMapReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxContentGroupMapWriter, IAppxContentGroupMapWriter_Vtbl, 0xd07ab776_a9de_4798_8c14_3db31e687c78);
impl core::ops::Deref for IAppxContentGroupMapWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxContentGroupMapWriter, windows_core::IUnknown);
impl IAppxContentGroupMapWriter {
    pub unsafe fn AddAutomaticGroup<P0>(&self, groupname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAutomaticGroup)(windows_core::Interface::as_raw(self), groupname.param().abi()).ok()
    }
    pub unsafe fn AddAutomaticFile<P0>(&self, filename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddAutomaticFile)(windows_core::Interface::as_raw(self), filename.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAppxContentGroupMapWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAutomaticGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddAutomaticFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxContentGroupsEnumerator, IAppxContentGroupsEnumerator_Vtbl, 0x3264e477_16d1_4d63_823e_7d2984696634);
impl core::ops::Deref for IAppxContentGroupsEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxContentGroupsEnumerator, windows_core::IUnknown);
impl IAppxContentGroupsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxContentGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxContentGroupsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxDigestProvider, IAppxDigestProvider_Vtbl, 0x9fe2702b_7640_4659_8e6c_349e43c4cdbd);
impl core::ops::Deref for IAppxDigestProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxDigestProvider, windows_core::IUnknown);
impl IAppxDigestProvider {
    pub unsafe fn GetDigest(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDigest)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxDigestProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter, IAppxEncryptedBundleWriter_Vtbl, 0x80b0902f_7bf0_4117_b8c6_4279ef81ee77);
impl core::ops::Deref for IAppxEncryptedBundleWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1>(&self, filename: P0, packagestream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPayloadPackageEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter2, IAppxEncryptedBundleWriter2_Vtbl, 0xe644be82_f0fa_42b8_a956_8d1cb48ee379);
impl core::ops::Deref for IAppxEncryptedBundleWriter2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter2, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
windows_core::imp::define_interface!(IAppxEncryptedBundleWriter3, IAppxEncryptedBundleWriter3_Vtbl, 0x0d34deb3_5cae_4dd3_977c_504932a51d31);
impl core::ops::Deref for IAppxEncryptedBundleWriter3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptedBundleWriter3, windows_core::IUnknown);
impl IAppxEncryptedBundleWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1, P2>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddPayloadPackageEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), packagestream.param().abi(), isdefaultapplicablepackage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).AddExternalPackageReference)(windows_core::Interface::as_raw(self), filename.param().abi(), inputstream.param().abi(), isdefaultapplicablepackage.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptedBundleWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
windows_core::imp::define_interface!(IAppxEncryptedPackageWriter, IAppxEncryptedPackageWriter_Vtbl, 0xf43d0b0b_1379_40e2_9b29_682ea2bf42af);
impl core::ops::Deref for IAppxEncryptedPackageWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptedPackageWriter, windows_core::IUnknown);
impl IAppxEncryptedPackageWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFileEncrypted<P0, P1>(&self, filename: P0, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPayloadFileEncrypted)(windows_core::Interface::as_raw(self), filename.param().abi(), compressionoption, inputstream.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptedPackageWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFileEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, APPX_COMPRESSION_OPTION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFileEncrypted: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxEncryptedPackageWriter2, IAppxEncryptedPackageWriter2_Vtbl, 0x3e475447_3a25_40b5_8ad2_f953ae50c92d);
impl core::ops::Deref for IAppxEncryptedPackageWriter2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptedPackageWriter2, windows_core::IUnknown);
impl IAppxEncryptedPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFilesEncrypted(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPayloadFilesEncrypted)(windows_core::Interface::as_raw(self), payloadfiles.len().try_into().unwrap(), core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptedPackageWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFilesEncrypted: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFilesEncrypted: usize,
}
windows_core::imp::define_interface!(IAppxEncryptionFactory, IAppxEncryptionFactory_Vtbl, 0x80e8e04d_8c88_44ae_a011_7cadf6fb2e72);
impl core::ops::Deref for IAppxEncryptionFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory, windows_core::IUnknown);
impl IAppxEncryptionFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).DecryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), keyinfo).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1>(&self, outputstream: P0, manifeststream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), settings, keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedPackageReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).EncryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).DecryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), keyinfo).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, settings, keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedBundleReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxEncryptionFactory2, IAppxEncryptionFactory2_Vtbl, 0xc1b11eee_c4ba_4ab2_a55d_d015fe8ff64f);
impl core::ops::Deref for IAppxEncryptionFactory2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory2, windows_core::IUnknown);
impl IAppxEncryptionFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), contentgroupmapstream.param().abi(), settings, keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxEncryptionFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
}
windows_core::imp::define_interface!(IAppxEncryptionFactory3, IAppxEncryptionFactory3_Vtbl, 0x09edca37_cd64_47d6_b7e8_1cb11d4f7e05);
impl core::ops::Deref for IAppxEncryptionFactory3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory3, windows_core::IUnknown);
impl IAppxEncryptionFactory3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedPackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), manifeststream.param().abi(), contentgroupmapstream.param().abi(), settings, keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).EncryptBundle)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> windows_core::Result<IAppxEncryptedBundleWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedBundleWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), bundleversion, settings, keyinfo, exemptedfiles, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxEncryptionFactory4, IAppxEncryptionFactory4_Vtbl, 0xa879611f_12fd_41fe_85d5_06ae779bbaf5);
impl core::ops::Deref for IAppxEncryptionFactory4 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory4, windows_core::IUnknown);
impl IAppxEncryptionFactory4 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).EncryptPackage)(windows_core::Interface::as_raw(self), inputstream.param().abi(), outputstream.param().abi(), settings, keyinfo, exemptedfiles, memorylimit).ok()
    }
}
#[repr(C)]
pub struct IAppxEncryptionFactory4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, *const APPX_KEY_INFO, *const APPX_ENCRYPTED_EXEMPTIONS, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
}
windows_core::imp::define_interface!(IAppxEncryptionFactory5, IAppxEncryptionFactory5_Vtbl, 0x68d6e77a_f446_480f_b0f0_d91a24c60746);
impl core::ops::Deref for IAppxEncryptionFactory5 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxEncryptionFactory5, windows_core::IUnknown);
impl IAppxEncryptionFactory5 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader2<P0, P1>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO, expecteddigest: P1) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedPackageReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader2<P0, P1>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO, expecteddigest: P1) -> windows_core::Result<IAppxBundleReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEncryptedBundleReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), keyinfo, expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxFactory, IAppxFactory_Vtbl, 0xbeb94909_e451_438b_b5a7_d79e767b75d8);
impl core::ops::Deref for IAppxFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxFactory, windows_core::IUnknown);
impl IAppxFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageWriter<P0>(&self, outputstream: P0, settings: *const APPX_PACKAGE_SETTINGS) -> windows_core::Result<IAppxPackageWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageWriter)(windows_core::Interface::as_raw(self), outputstream.param().abi(), settings, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateManifestReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlockMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxBlockMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBlockMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateValidatedBlockMapReader<P0, P1>(&self, blockmapstream: P0, signaturefilename: P1) -> windows_core::Result<IAppxBlockMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateValidatedBlockMapReader)(windows_core::Interface::as_raw(self), blockmapstream.param().abi(), signaturefilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxFactory2, IAppxFactory2_Vtbl, 0xf1346df2_c282_4e22_b918_743a929a8d55);
impl core::ops::Deref for IAppxFactory2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxFactory2, windows_core::IUnknown);
impl IAppxFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxContentGroupMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateContentGroupMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSourceContentGroupMapReader<P0>(&self, inputstream: P0) -> windows_core::Result<IAppxSourceContentGroupMapReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSourceContentGroupMapReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapWriter<P0>(&self, stream: P0) -> windows_core::Result<IAppxContentGroupMapWriter>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateContentGroupMapWriter)(windows_core::Interface::as_raw(self), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxFactory3, IAppxFactory3_Vtbl, 0x776b2c05_e21d_4e24_ba1a_cd529a8bfdbb);
impl core::ops::Deref for IAppxFactory3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxFactory3, windows_core::IUnknown);
impl IAppxFactory3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxPackageReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePackageReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader2<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxManifestReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateManifestReader2)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAppInstallerReader<P0, P1>(&self, inputstream: P0, expecteddigest: P1) -> windows_core::Result<IAppxAppInstallerReader>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAppInstallerReader)(windows_core::Interface::as_raw(self), inputstream.param().abi(), expecteddigest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxFile, IAppxFile_Vtbl, 0x91df827b_94fd_468f_827b_57f41b2f6f2e);
impl core::ops::Deref for IAppxFile {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxFile, windows_core::IUnknown);
impl IAppxFile {
    pub unsafe fn GetCompressionOption(&self) -> windows_core::Result<APPX_COMPRESSION_OPTION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCompressionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetContentType(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxFilesEnumerator, IAppxFilesEnumerator_Vtbl, 0xf007eeaf_9831_411c_9847_917cdc62d1fe);
impl core::ops::Deref for IAppxFilesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxFilesEnumerator, windows_core::IUnknown);
impl IAppxFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxFilesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestApplication, IAppxManifestApplication_Vtbl, 0x5da89bf4_3773_46be_b650_7e744863b7e8);
impl core::ops::Deref for IAppxManifestApplication {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestApplication, windows_core::IUnknown);
impl IAppxManifestApplication {
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAppUserModelId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAppUserModelId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestApplication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetAppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestApplicationsEnumerator, IAppxManifestApplicationsEnumerator_Vtbl, 0x9eb8a55a_f04b_4d0d_808d_686185d4847a);
impl core::ops::Deref for IAppxManifestApplicationsEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestApplicationsEnumerator, windows_core::IUnknown);
impl IAppxManifestApplicationsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestApplication> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestApplicationsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestCapabilitiesEnumerator, IAppxManifestCapabilitiesEnumerator_Vtbl, 0x11d22258_f470_42c1_b291_8361c5437e41);
impl core::ops::Deref for IAppxManifestCapabilitiesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestCapabilitiesEnumerator, windows_core::IUnknown);
impl IAppxManifestCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestDeviceCapabilitiesEnumerator, IAppxManifestDeviceCapabilitiesEnumerator_Vtbl, 0x30204541_427b_4a1c_bacf_655bf463a540);
impl core::ops::Deref for IAppxManifestDeviceCapabilitiesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestDeviceCapabilitiesEnumerator, windows_core::IUnknown);
impl IAppxManifestDeviceCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestDriverConstraint, IAppxManifestDriverConstraint_Vtbl, 0xc031bee4_bbcc_48ea_a237_c34045c80a07);
impl core::ops::Deref for IAppxManifestDriverConstraint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestDriverConstraint, windows_core::IUnknown);
impl IAppxManifestDriverConstraint {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinDate(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestDriverConstraint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetMinDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestDriverConstraintsEnumerator, IAppxManifestDriverConstraintsEnumerator_Vtbl, 0xd402b2d1_f600_49e0_95e6_975d8da13d89);
impl core::ops::Deref for IAppxManifestDriverConstraintsEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestDriverConstraintsEnumerator, windows_core::IUnknown);
impl IAppxManifestDriverConstraintsEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverConstraint> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestDriverDependenciesEnumerator, IAppxManifestDriverDependenciesEnumerator_Vtbl, 0xfe039db2_467f_4755_8404_8f5eb6865b33);
impl core::ops::Deref for IAppxManifestDriverDependenciesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestDriverDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestDriverDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestDriverDependency> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestDriverDependency, IAppxManifestDriverDependency_Vtbl, 0x1210cb94_5a92_4602_be24_79f318af4af9);
impl core::ops::Deref for IAppxManifestDriverDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestDriverDependency, windows_core::IUnknown);
impl IAppxManifestDriverDependency {
    pub unsafe fn GetDriverConstraints(&self) -> windows_core::Result<IAppxManifestDriverConstraintsEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDriverConstraints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestDriverDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDriverConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependenciesEnumerator, IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl, 0x6427a646_7f49_433e_b1a6_0da309f6885a);
impl core::ops::Deref for IAppxManifestHostRuntimeDependenciesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependency> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependency, IAppxManifestHostRuntimeDependency_Vtbl, 0x3455d234_8414_410d_95c7_7b35255b8391);
impl core::ops::Deref for IAppxManifestHostRuntimeDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependency, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestHostRuntimeDependency2, IAppxManifestHostRuntimeDependency2_Vtbl, 0xc26f23a8_ee10_4ad6_b898_2b4d7aebfe6a);
impl core::ops::Deref for IAppxManifestHostRuntimeDependency2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestHostRuntimeDependency2, windows_core::IUnknown);
impl IAppxManifestHostRuntimeDependency2 {
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestHostRuntimeDependency2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestMainPackageDependenciesEnumerator, IAppxManifestMainPackageDependenciesEnumerator_Vtbl, 0xa99c4f00_51d2_4f0f_ba46_7ed5255ebdff);
impl core::ops::Deref for IAppxManifestMainPackageDependenciesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestMainPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestMainPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestMainPackageDependency> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestMainPackageDependency, IAppxManifestMainPackageDependency_Vtbl, 0x05d0611c_bc29_46d5_97e2_84b9c79bd8ae);
impl core::ops::Deref for IAppxManifestMainPackageDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestMainPackageDependency, windows_core::IUnknown);
impl IAppxManifestMainPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestMainPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestOSPackageDependenciesEnumerator, IAppxManifestOSPackageDependenciesEnumerator_Vtbl, 0xb84e2fc3_f8ec_4bc1_8ae2_156346f5ffea);
impl core::ops::Deref for IAppxManifestOSPackageDependenciesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestOSPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestOSPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestOSPackageDependency> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestOSPackageDependency, IAppxManifestOSPackageDependency_Vtbl, 0x154995ee_54a6_4f14_ac97_d8cf0519644b);
impl core::ops::Deref for IAppxManifestOSPackageDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestOSPackageDependency, windows_core::IUnknown);
impl IAppxManifestOSPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestOSPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestOptionalPackageInfo, IAppxManifestOptionalPackageInfo_Vtbl, 0x2634847d_5b5d_4fe5_a243_002ff95edc7e);
impl core::ops::Deref for IAppxManifestOptionalPackageInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestOptionalPackageInfo, windows_core::IUnknown);
impl IAppxManifestOptionalPackageInfo {
    pub unsafe fn GetIsOptionalPackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsOptionalPackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMainPackageName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMainPackageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestOptionalPackageInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsOptionalPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetMainPackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestPackageDependenciesEnumerator, IAppxManifestPackageDependenciesEnumerator_Vtbl, 0xb43bbcf9_65a6_42dd_bac0_8c6741e7f5a4);
impl core::ops::Deref for IAppxManifestPackageDependenciesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependenciesEnumerator, windows_core::IUnknown);
impl IAppxManifestPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestPackageDependency> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestPackageDependency, IAppxManifestPackageDependency_Vtbl, 0xe4946b59_733e_43f0_a724_3bde4c1285a0);
impl core::ops::Deref for IAppxManifestPackageDependency {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependency, windows_core::IUnknown);
impl IAppxManifestPackageDependency {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxMajorVersionTested)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageDependency2_Vtbl {
    pub base__: IAppxManifestPackageDependency_Vtbl,
    pub GetMaxMajorVersionTested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestPackageDependency3, IAppxManifestPackageDependency3_Vtbl, 0x1ac56374_6198_4d6b_92e4_749d5ab8a895);
impl core::ops::Deref for IAppxManifestPackageDependency3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageDependency3, windows_core::IUnknown);
impl IAppxManifestPackageDependency3 {
    pub unsafe fn GetIsOptional(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsOptional)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageDependency3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsOptional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestPackageId, IAppxManifestPackageId_Vtbl, 0x283ce2d7_7153_4a91_9649_7a0f7240945f);
impl core::ops::Deref for IAppxManifestPackageId {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestPackageId, windows_core::IUnknown);
impl IAppxManifestPackageId {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetArchitecture(&self) -> windows_core::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetArchitecture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPublisher(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisher)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetResourceId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ComparePublisher<P0>(&self, other: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComparePublisher)(windows_core::Interface::as_raw(self), other.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageFullName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageFullName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetArchitecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_PACKAGE_ARCHITECTURE) -> windows_core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ComparePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetArchitecture2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestPackageId2_Vtbl {
    pub base__: IAppxManifestPackageId_Vtbl,
    pub GetArchitecture2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APPX_PACKAGE_ARCHITECTURE2) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestProperties, IAppxManifestProperties_Vtbl, 0x03faf64d_f26f_4b2c_aaf7_8fe7789b8bca);
impl core::ops::Deref for IAppxManifestProperties {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestProperties, windows_core::IUnknown);
impl IAppxManifestProperties {
    pub unsafe fn GetBoolValue<P0>(&self, name: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoolValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBoolValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestQualifiedResource, IAppxManifestQualifiedResource_Vtbl, 0x3b53a497_3c5c_48d1_9ea3_bb7eac8cd7d4);
impl core::ops::Deref for IAppxManifestQualifiedResource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestQualifiedResource, windows_core::IUnknown);
impl IAppxManifestQualifiedResource {
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetScale(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDXFeatureLevel(&self) -> windows_core::Result<DX_FEATURE_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDXFeatureLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestQualifiedResource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDXFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DX_FEATURE_LEVEL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestQualifiedResourcesEnumerator, IAppxManifestQualifiedResourcesEnumerator_Vtbl, 0x8ef6adfe_3762_4a8f_9373_2fc5d444c8d2);
impl core::ops::Deref for IAppxManifestQualifiedResourcesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestQualifiedResourcesEnumerator, windows_core::IUnknown);
impl IAppxManifestQualifiedResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestQualifiedResource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestReader, IAppxManifestReader_Vtbl, 0x4e1bd148_55a0_4480_a3d1_15544710637c);
impl core::ops::Deref for IAppxManifestReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader, windows_core::IUnknown);
impl IAppxManifestReader {
    pub unsafe fn GetPackageId(&self) -> windows_core::Result<IAppxManifestPackageId> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<IAppxManifestProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPackageDependencies(&self) -> windows_core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCapabilities(&self) -> windows_core::Result<APPX_CAPABILITIES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetResources(&self) -> windows_core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> windows_core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceCapabilities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrerequisite)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetApplications(&self) -> windows_core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> windows_core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQualifiedResources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestReader2_Vtbl {
    pub base__: IAppxManifestReader_Vtbl,
    pub GetQualifiedResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilitiesByCapabilityClass)(windows_core::Interface::as_raw(self), capabilityclass, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetDeviceFamilies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestReader3_Vtbl {
    pub base__: IAppxManifestReader2_Vtbl,
    pub GetCapabilitiesByCapabilityClass: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_CAPABILITY_CLASS_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOptionalPackageInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestReader4_Vtbl {
    pub base__: IAppxManifestReader3_Vtbl,
    pub GetOptionalPackageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestReader5, IAppxManifestReader5_Vtbl, 0x8d7ae132_a690_4c00_b75a_6aae1feaac80);
impl core::ops::Deref for IAppxManifestReader5 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader5, windows_core::IUnknown);
impl IAppxManifestReader5 {
    pub unsafe fn GetMainPackageDependencies(&self) -> windows_core::Result<IAppxManifestMainPackageDependenciesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMainPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestReader5_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMainPackageDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestReader6, IAppxManifestReader6_Vtbl, 0x34deaca4_d3c0_4e3e_b312_e42625e3807e);
impl core::ops::Deref for IAppxManifestReader6 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader6, windows_core::IUnknown);
impl IAppxManifestReader6 {
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIsNonQualifiedResourcePackage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestReader6_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestReader7, IAppxManifestReader7_Vtbl, 0x8efe6f27_0ce0_4988_b32d_738eb63db3b7);
impl core::ops::Deref for IAppxManifestReader7 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestReader7, windows_core::IUnknown);
impl IAppxManifestReader7 {
    pub unsafe fn GetDriverDependencies(&self) -> windows_core::Result<IAppxManifestDriverDependenciesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDriverDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOSPackageDependencies(&self) -> windows_core::Result<IAppxManifestOSPackageDependenciesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOSPackageDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHostRuntimeDependencies(&self) -> windows_core::Result<IAppxManifestHostRuntimeDependenciesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHostRuntimeDependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxManifestReader7_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDriverDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOSPackageDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHostRuntimeDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestResourcesEnumerator, IAppxManifestResourcesEnumerator_Vtbl, 0xde4dfbbd_881a_48bb_858c_d6f2baeae6ed);
impl core::ops::Deref for IAppxManifestResourcesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestResourcesEnumerator, windows_core::IUnknown);
impl IAppxManifestResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestResourcesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestTargetDeviceFamiliesEnumerator, IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl, 0x36537f36_27a4_4788_88c0_733819575017);
impl core::ops::Deref for IAppxManifestTargetDeviceFamiliesEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestTargetDeviceFamiliesEnumerator, windows_core::IUnknown);
impl IAppxManifestTargetDeviceFamiliesEnumerator {
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IAppxManifestTargetDeviceFamily> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHasCurrent(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHasCurrent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MoveNext(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveNext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxManifestTargetDeviceFamily, IAppxManifestTargetDeviceFamily_Vtbl, 0x9091b09b_c8d5_4f31_8687_a338259faefb);
impl core::ops::Deref for IAppxManifestTargetDeviceFamily {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxManifestTargetDeviceFamily, windows_core::IUnknown);
impl IAppxManifestTargetDeviceFamily {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMinVersion(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMinVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxVersionTested(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxVersionTested)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAppxManifestTargetDeviceFamily_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetMaxVersionTested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxPackageEditor, IAppxPackageEditor_Vtbl, 0xe2adb6dc_5e71_4416_86b6_86e5f5291a6b);
impl core::ops::Deref for IAppxPackageEditor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackageEditor, windows_core::IUnknown);
impl IAppxPackageEditor {
    pub unsafe fn SetWorkingDirectory<P0>(&self, workingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), workingdirectory.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackage<P0, P1, P2>(&self, updatedpackagestream: P0, baselinepackagestream: P1, deltapackagestream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).CreateDeltaPackage)(windows_core::Interface::as_raw(self), updatedpackagestream.param().abi(), baselinepackagestream.param().abi(), deltapackagestream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackageUsingBaselineBlockMap<P0, P1, P2, P3>(&self, updatedpackagestream: P0, baselineblockmapstream: P1, baselinepackagefullname: P2, deltapackagestream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).CreateDeltaPackageUsingBaselineBlockMap)(windows_core::Interface::as_raw(self), updatedpackagestream.param().abi(), baselineblockmapstream.param().abi(), baselinepackagefullname.param().abi(), deltapackagestream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackage<P0, P1>(&self, baselinepackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).UpdatePackage)(windows_core::Interface::as_raw(self), baselinepackagestream.param().abi(), deltapackagestream.param().abi(), updateoption).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateEncryptedPackage<P0, P1>(&self, baselineencryptedpackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).UpdateEncryptedPackage)(windows_core::Interface::as_raw(self), baselineencryptedpackagestream.param().abi(), deltapackagestream.param().abi(), updateoption, settings, keyinfo).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackageManifest<P0, P1, P2>(&self, packagestream: P0, updatedmanifeststream: P1, ispackageencrypted: P2, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UpdatePackageManifest)(windows_core::Interface::as_raw(self), packagestream.param().abi(), updatedmanifeststream.param().abi(), ispackageencrypted.param().abi(), options).ok()
    }
}
#[repr(C)]
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
    pub UpdatePackageManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL, APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdatePackageManifest: usize,
}
windows_core::imp::define_interface!(IAppxPackageReader, IAppxPackageReader_Vtbl, 0xb5c49650_99bc_481c_9a34_3d53a4106708);
impl core::ops::Deref for IAppxPackageReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackageReader, windows_core::IUnknown);
impl IAppxPackageReader {
    pub unsafe fn GetBlockMap(&self) -> windows_core::Result<IAppxBlockMapReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBlockMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> windows_core::Result<IAppxFile> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFootprintFile)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPayloadFile<P0>(&self, filename: P0) -> windows_core::Result<IAppxFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPayloadFile)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPayloadFiles(&self) -> windows_core::Result<IAppxFilesEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPayloadFiles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetManifest(&self) -> windows_core::Result<IAppxManifestReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetManifest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxPackageReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBlockMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFootprintFile: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_FOOTPRINT_FILE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPayloadFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxPackageWriter, IAppxPackageWriter_Vtbl, 0x9099e33b_246f_41e4_881a_008eb613f858);
impl core::ops::Deref for IAppxPackageWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackageWriter, windows_core::IUnknown);
impl IAppxPackageWriter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFile<P0, P1, P2>(&self, filename: P0, contenttype: P1, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).AddPayloadFile)(windows_core::Interface::as_raw(self), filename.param().abi(), contenttype.param().abi(), compressionoption, inputstream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0>(&self, manifest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), manifest.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAppxPackageWriter2, IAppxPackageWriter2_Vtbl, 0x2cf5c4fd_e54c_4ea5_ba4e_f8c4b105a8c8);
impl core::ops::Deref for IAppxPackageWriter2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackageWriter2, windows_core::IUnknown);
impl IAppxPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0, P1>(&self, manifest: P0, contentgroupmap: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), manifest.param().abi(), contentgroupmap.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxPackageWriter2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
windows_core::imp::define_interface!(IAppxPackageWriter3, IAppxPackageWriter3_Vtbl, 0xa83aacd3_41c0_4501_b8a3_74164f50b2fd);
impl core::ops::Deref for IAppxPackageWriter3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackageWriter3, windows_core::IUnknown);
impl IAppxPackageWriter3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFiles(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPayloadFiles)(windows_core::Interface::as_raw(self), payloadfiles.len().try_into().unwrap(), core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok()
    }
}
#[repr(C)]
pub struct IAppxPackageWriter3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFiles: usize,
}
windows_core::imp::define_interface!(IAppxPackagingDiagnosticEventSink, IAppxPackagingDiagnosticEventSink_Vtbl, 0x17239d47_6adb_45d2_80f6_f9cbc3bf059d);
impl core::ops::Deref for IAppxPackagingDiagnosticEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackagingDiagnosticEventSink, windows_core::IUnknown);
impl IAppxPackagingDiagnosticEventSink {
    pub unsafe fn ReportContextChange<P0, P1, P2>(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: P0, contextmessage: P1, detailsmessage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReportContextChange)(windows_core::Interface::as_raw(self), changetype, contextid, contextname.param().abi(), contextmessage.param().abi(), detailsmessage.param().abi()).ok()
    }
    pub unsafe fn ReportError<P0>(&self, errormessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReportError)(windows_core::Interface::as_raw(self), errormessage.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxPackagingDiagnosticEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportContextChange: unsafe extern "system" fn(*mut core::ffi::c_void, APPX_PACKAGING_CONTEXT_CHANGE_TYPE, i32, windows_core::PCSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxPackagingDiagnosticEventSinkManager, IAppxPackagingDiagnosticEventSinkManager_Vtbl, 0x369648fa_a7eb_4909_a15d_6954a078f18a);
impl core::ops::Deref for IAppxPackagingDiagnosticEventSinkManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxPackagingDiagnosticEventSinkManager, windows_core::IUnknown);
impl IAppxPackagingDiagnosticEventSinkManager {
    pub unsafe fn SetSinkForProcess<P0>(&self, sink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAppxPackagingDiagnosticEventSink>,
    {
        (windows_core::Interface::vtable(self).SetSinkForProcess)(windows_core::Interface::as_raw(self), sink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSinkForProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppxSourceContentGroupMapReader, IAppxSourceContentGroupMapReader_Vtbl, 0xf329791d_540b_4a9f_bc75_3282b7d73193);
impl core::ops::Deref for IAppxSourceContentGroupMapReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAppxSourceContentGroupMapReader, windows_core::IUnknown);
impl IAppxSourceContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> windows_core::Result<IAppxContentGroup> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRequiredGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAutomaticGroups(&self) -> windows_core::Result<IAppxContentGroupsEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAutomaticGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAppxSourceContentGroupMapReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const APPLICATION_USER_MODEL_ID_MAX_LENGTH: u32 = 130u32;
pub const APPLICATION_USER_MODEL_ID_MIN_LENGTH: u32 = 20u32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024i32);
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
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
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1i32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0i32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2i32);
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
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
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
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
pub const PACKAGE_VERSION_MAX_LENGTH: u32 = 23u32;
pub const PACKAGE_VERSION_MIN_LENGTH: u32 = 7u32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
pub const PackageInfo3Type_PackageInfoGeneration: PackageInfo3Type = PackageInfo3Type(16i32);
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
impl windows_core::TypeKind for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_BUNDLE_FOOTPRINT_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
impl windows_core::TypeKind for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_CAPABILITIES(pub i32);
impl windows_core::TypeKind for APPX_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_CAPABILITIES").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
impl windows_core::TypeKind for APPX_CAPABILITY_CLASS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_CAPABILITY_CLASS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_CAPABILITY_CLASS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
impl windows_core::TypeKind for APPX_COMPRESSION_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_COMPRESSION_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_COMPRESSION_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub i32);
impl windows_core::TypeKind for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_ENCRYPTED_PACKAGE_OPTIONS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
impl windows_core::TypeKind for APPX_FOOTPRINT_FILE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_FOOTPRINT_FILE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_FOOTPRINT_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
impl windows_core::TypeKind for APPX_PACKAGE_ARCHITECTURE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_PACKAGE_ARCHITECTURE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_ARCHITECTURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
impl windows_core::TypeKind for APPX_PACKAGE_ARCHITECTURE2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_PACKAGE_ARCHITECTURE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_ARCHITECTURE2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub i32);
impl windows_core::TypeKind for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
impl windows_core::TypeKind for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPX_PACKAGING_CONTEXT_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AddPackageDependencyOptions(pub i32);
impl windows_core::TypeKind for AddPackageDependencyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AddPackageDependencyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AddPackageDependencyOptions").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyClrCompat(pub i32);
impl windows_core::TypeKind for AppPolicyClrCompat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyClrCompat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyClrCompat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyCreateFileAccess(pub i32);
impl windows_core::TypeKind for AppPolicyCreateFileAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyCreateFileAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyCreateFileAccess").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyLifecycleManagement(pub i32);
impl windows_core::TypeKind for AppPolicyLifecycleManagement {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyLifecycleManagement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyLifecycleManagement").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
impl windows_core::TypeKind for AppPolicyMediaFoundationCodecLoading {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyMediaFoundationCodecLoading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyMediaFoundationCodecLoading").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
impl windows_core::TypeKind for AppPolicyProcessTerminationMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyProcessTerminationMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyProcessTerminationMethod").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
impl windows_core::TypeKind for AppPolicyShowDeveloperDiagnostic {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyShowDeveloperDiagnostic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyShowDeveloperDiagnostic").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyThreadInitializationType(pub i32);
impl windows_core::TypeKind for AppPolicyThreadInitializationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyThreadInitializationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyThreadInitializationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppPolicyWindowingModel(pub i32);
impl windows_core::TypeKind for AppPolicyWindowingModel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppPolicyWindowingModel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppPolicyWindowingModel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CreatePackageDependencyOptions(pub i32);
impl windows_core::TypeKind for CreatePackageDependencyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CreatePackageDependencyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CreatePackageDependencyOptions").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DX_FEATURE_LEVEL(pub i32);
impl windows_core::TypeKind for DX_FEATURE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DX_FEATURE_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DX_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageDependencyLifetimeKind(pub i32);
impl windows_core::TypeKind for PackageDependencyLifetimeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageDependencyLifetimeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageDependencyLifetimeKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
impl windows_core::TypeKind for PackageDependencyProcessorArchitectures {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageDependencyProcessorArchitectures {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageDependencyProcessorArchitectures").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageInfo3Type(pub i32);
impl windows_core::TypeKind for PackageInfo3Type {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageInfo3Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageInfo3Type").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageOrigin(pub i32);
impl windows_core::TypeKind for PackageOrigin {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageOrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageOrigin").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackagePathType(pub i32);
impl windows_core::TypeKind for PackagePathType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackagePathType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackagePathType").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *const windows_core::PCWSTR,
}
impl windows_core::TypeKind for APPX_ENCRYPTED_EXEMPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_core::PCWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: windows_core::PCWSTR,
    pub blockMapHashAlgorithm: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
    pub options: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl windows_core::TypeKind for APPX_KEY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IUri>>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for APPX_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for APPX_PACKAGE_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug, Eq, PartialEq)]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IStream>>,
    pub fileName: windows_core::PCWSTR,
    pub contentType: windows_core::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AppxBundleFactory: windows_core::GUID = windows_core::GUID::from_u128(0x378e0446_5384_43b7_8877_e7dbdd883446);
pub const AppxEncryptionFactory: windows_core::GUID = windows_core::GUID::from_u128(0xdc664fdd_d868_46ee_8780_8d196cb739f7);
pub const AppxFactory: windows_core::GUID = windows_core::GUID::from_u128(0x5842a140_ff9f_4166_8f5c_62f5b7b0c781);
pub const AppxPackageEditor: windows_core::GUID = windows_core::GUID::from_u128(0xf004f2ca_aebc_4b0d_bf58_e516d5bcc0ab);
pub const AppxPackagingDiagnosticEventSinkManager: windows_core::GUID = windows_core::GUID::from_u128(0x50ca0a46_1588_4161_8ed2_ef9e469ced5d);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACKAGEDEPENDENCY_CONTEXT(pub isize);
impl Default for PACKAGEDEPENDENCY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PACKAGEDEPENDENCY_CONTEXT {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PACKAGE_ID {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PACKAGE_ID {
    type TypeKind = windows_core::CopyType;
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
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for PACKAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_INFO {
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
impl windows_core::TypeKind for PACKAGE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl windows_core::TypeKind for PACKAGE_VERSION {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for PACKAGE_VERSION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl windows_core::TypeKind for PACKAGE_VERSION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PACKAGE_VERSION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE(pub isize);
impl Default for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for _PACKAGE_INFO_REFERENCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
