#[inline]
pub unsafe fn CoCreateActivity<P0>(piunknown: P0, riid: *const windows_core::GUID, ppobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("comsvcs.dll" "system" fn CoCreateActivity(piunknown : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoCreateActivity(piunknown.param().abi(), riid, ppobj as _).ok() }
}
#[inline]
pub unsafe fn CoEnterServiceDomain<P0>(pconfigobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("comsvcs.dll" "system" fn CoEnterServiceDomain(pconfigobject : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoEnterServiceDomain(pconfigobject.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn CoGetDefaultContext(apttype : super::Com:: APTTYPE, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetDefaultContext(apttype, riid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CoLeaveServiceDomain<P0>(punkstatus: P0)
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("comsvcs.dll" "system" fn CoLeaveServiceDomain(punkstatus : * mut core::ffi::c_void));
    unsafe { CoLeaveServiceDomain(punkstatus.param().abi()) }
}
#[inline]
pub unsafe fn GetDispenserManager() -> windows_core::Result<IDispenserManager> {
    windows_core::link!("mtxdm.dll" "C" fn GetDispenserManager(param0 : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetDispenserManager(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetManagedExtensions(dwexts: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("comsvcs.dll" "system" fn GetManagedExtensions(dwexts : *mut u32) -> windows_core::HRESULT);
    unsafe { GetManagedExtensions(dwexts as _).ok() }
}
#[inline]
pub unsafe fn MTSCreateActivity(riid: *const windows_core::GUID, ppobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("comsvcs.dll" "system" fn MTSCreateActivity(riid : *const windows_core::GUID, ppobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MTSCreateActivity(riid, ppobj as _).ok() }
}
#[inline]
pub unsafe fn RecycleSurrogate(lreasoncode: i32) -> windows_core::Result<()> {
    windows_core::link!("comsvcs.dll" "C" fn RecycleSurrogate(lreasoncode : i32) -> windows_core::HRESULT);
    unsafe { RecycleSurrogate(lreasoncode).ok() }
}
#[inline]
pub unsafe fn SafeRef<P1>(rid: *const windows_core::GUID, punk: P1) -> *mut core::ffi::c_void
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("comsvcs.dll" "C" fn SafeRef(rid : *const windows_core::GUID, punk : * mut core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { SafeRef(rid, punk.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPDATA {
    pub m_idApp: u32,
    pub m_szAppGuid: [u16; 40],
    pub m_dwAppProcessId: u32,
    pub m_AppStatistics: APPSTATISTICS,
}
impl Default for APPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPSTATISTICS {
    pub m_cTotalCalls: u32,
    pub m_cTotalInstances: u32,
    pub m_cTotalClasses: u32,
    pub m_cCallsPerSecond: u32,
}
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = COMPLUS_APPTYPE(0i32);
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = COMPLUS_APPTYPE(1i32);
pub const APPTYPE_SWC: COMPLUS_APPTYPE = COMPLUS_APPTYPE(2i32);
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = COMPLUS_APPTYPE(-1i32);
pub const AppDomainHelper: windows_core::GUID = windows_core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: windows_core::BOOL,
    pub IsRecycled: windows_core::BOOL,
    pub TimeRecycled: super::super::Foundation::FILETIME,
    pub TimeToTerminate: super::super::Foundation::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: windows_core::BOOL,
    pub HasAutomaticLifetimeRecycling: windows_core::BOOL,
    pub TimeForAutomaticRecycling: super::super::Foundation::FILETIME,
    pub MemoryLimitInKB: u32,
    pub MemoryUsageInKBLastCheck: u32,
    pub ActivationLimit: u32,
    pub NumActivationsLastReported: u32,
    pub CallLimit: u32,
    pub NumCallsLastReported: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: u32,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
    pub AvgCallsPerSecond: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: windows_core::GUID,
    pub ApplicationIdPrimaryApplication: windows_core::GUID,
    pub ApplicationInstanceId: windows_core::GUID,
    pub ProcessId: u32,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: windows_core::PWSTR,
    pub IsService: windows_core::BOOL,
    pub IsPaused: windows_core::BOOL,
    pub IsRecycled: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub ApplicationId: windows_core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: windows_core::PWSTR,
    pub NumTrackedComponents: u32,
    pub NumComponentInstances: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutoSvcs_Error_Constants(pub u32);
pub const ByotServerEx: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLSIDDATA {
    pub m_clsid: windows_core::GUID,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLSIDDATA2 {
    pub m_clsid: windows_core::GUID,
    pub m_appid: windows_core::GUID,
    pub m_partid: windows_core::GUID,
    pub m_pwszAppName: windows_core::PWSTR,
    pub m_pwszCtxName: windows_core::PWSTR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: u32,
    pub m_cBound: u32,
    pub m_cPooled: u32,
    pub m_cInCall: u32,
    pub m_dwRespTime: u32,
    pub m_cCallsCompleted: u32,
    pub m_cCallsFailed: u32,
}
pub const COMAdmin32BitComponent: COMAdminComponentType = COMAdminComponentType(1i32);
pub const COMAdmin64BitComponent: COMAdminComponentType = COMAdminComponentType(2i32);
pub const COMAdminAccessChecksApplicationComponentLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(1i32);
pub const COMAdminAccessChecksApplicationLevel: COMAdminAccessChecksLevelOptions = COMAdminAccessChecksLevelOptions(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminAccessChecksLevelOptions(pub i32);
pub const COMAdminActivationInproc: COMAdminActivationOptions = COMAdminActivationOptions(0i32);
pub const COMAdminActivationLocal: COMAdminActivationOptions = COMAdminActivationOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminActivationOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminApplicationExportOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminApplicationInstallOptions(pub i32);
pub const COMAdminAuthenticationCall: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(3i32);
pub const COMAdminAuthenticationCapabilitiesDynamicCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(64i32);
pub const COMAdminAuthenticationCapabilitiesNone: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminAuthenticationCapabilitiesOptions(pub i32);
pub const COMAdminAuthenticationCapabilitiesSecureReference: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(2i32);
pub const COMAdminAuthenticationCapabilitiesStaticCloaking: COMAdminAuthenticationCapabilitiesOptions = COMAdminAuthenticationCapabilitiesOptions(32i32);
pub const COMAdminAuthenticationConnect: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(2i32);
pub const COMAdminAuthenticationDefault: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(0i32);
pub const COMAdminAuthenticationIntegrity: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminAuthenticationLevelOptions(pub i32);
pub const COMAdminAuthenticationNone: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(1i32);
pub const COMAdminAuthenticationPacket: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(4i32);
pub const COMAdminAuthenticationPrivacy: COMAdminAuthenticationLevelOptions = COMAdminAuthenticationLevelOptions(6i32);
pub const COMAdminCatalog: windows_core::GUID = windows_core::GUID::from_u128(0xf618c514_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogCollection: windows_core::GUID = windows_core::GUID::from_u128(0xf618c516_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCatalogObject: windows_core::GUID = windows_core::GUID::from_u128(0xf618c515_dfb8_11d1_a2cf_00805fc79235);
pub const COMAdminCompFlagAlreadyInstalled: COMAdminComponentFlags = COMAdminComponentFlags(16i32);
pub const COMAdminCompFlagCOMPlusPropertiesFound: COMAdminComponentFlags = COMAdminComponentFlags(2i32);
pub const COMAdminCompFlagInterfacesFound: COMAdminComponentFlags = COMAdminComponentFlags(8i32);
pub const COMAdminCompFlagNotInApplication: COMAdminComponentFlags = COMAdminComponentFlags(32i32);
pub const COMAdminCompFlagProxyFound: COMAdminComponentFlags = COMAdminComponentFlags(4i32);
pub const COMAdminCompFlagTypeInfoFound: COMAdminComponentFlags = COMAdminComponentFlags(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminComponentFlags(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminComponentType(pub i32);
pub const COMAdminErrAlreadyInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368508i32);
pub const COMAdminErrAppDirNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368481i32);
pub const COMAdminErrAppFileReadFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368504i32);
pub const COMAdminErrAppFileVersion: COMAdminErrorCodes = COMAdminErrorCodes(-2146368503i32);
pub const COMAdminErrAppFileWriteFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368505i32);
pub const COMAdminErrAppNotRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146367478i32);
pub const COMAdminErrApplicationExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368501i32);
pub const COMAdminErrApplidMatchesClsid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368442i32);
pub const COMAdminErrAuthenticationLevel: COMAdminErrorCodes = COMAdminErrorCodes(-2146368493i32);
pub const COMAdminErrBadPath: COMAdminErrorCodes = COMAdminErrorCodes(-2146368502i32);
pub const COMAdminErrBadRegistryLibID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368482i32);
pub const COMAdminErrBadRegistryProgID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368494i32);
pub const COMAdminErrBasePartitionOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146368432i32);
pub const COMAdminErrCLSIDOrIIDMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368488i32);
pub const COMAdminErrCanNotExportAppProxy: COMAdminErrorCodes = COMAdminErrorCodes(-2146368438i32);
pub const COMAdminErrCanNotExportSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368436i32);
pub const COMAdminErrCanNotStartApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368437i32);
pub const COMAdminErrCanNotSubscribeToComponent: COMAdminErrorCodes = COMAdminErrorCodes(-2146368435i32);
pub const COMAdminErrCannotCopyEventClass: COMAdminErrorCodes = COMAdminErrorCodes(-2146367456i32);
pub const COMAdminErrCantCopyFile: COMAdminErrorCodes = COMAdminErrorCodes(-2146368499i32);
pub const COMAdminErrCantRecycleLibraryApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367473i32);
pub const COMAdminErrCantRecycleServiceApps: COMAdminErrorCodes = COMAdminErrorCodes(-2146367471i32);
pub const COMAdminErrCatBitnessMismatch: COMAdminErrorCodes = COMAdminErrorCodes(-2146368382i32);
pub const COMAdminErrCatPauseResumeNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368379i32);
pub const COMAdminErrCatServerFault: COMAdminErrorCodes = COMAdminErrorCodes(-2146368378i32);
pub const COMAdminErrCatUnacceptableBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368381i32);
pub const COMAdminErrCatWrongAppBitnessBitness: COMAdminErrorCodes = COMAdminErrorCodes(-2146368380i32);
pub const COMAdminErrCoReqCompInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368459i32);
pub const COMAdminErrCompFileBadTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368472i32);
pub const COMAdminErrCompFileClassNotAvail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368473i32);
pub const COMAdminErrCompFileDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368476i32);
pub const COMAdminErrCompFileGetClassObj: COMAdminErrorCodes = COMAdminErrorCodes(-2146368474i32);
pub const COMAdminErrCompFileLoadDLLFail: COMAdminErrorCodes = COMAdminErrorCodes(-2146368475i32);
pub const COMAdminErrCompFileNoRegistrar: COMAdminErrorCodes = COMAdminErrorCodes(-2146368460i32);
pub const COMAdminErrCompFileNotInstallable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368471i32);
pub const COMAdminErrCompMoveBadDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146368466i32);
pub const COMAdminErrCompMoveDest: COMAdminErrorCodes = COMAdminErrorCodes(-2146367459i32);
pub const COMAdminErrCompMoveLocked: COMAdminErrorCodes = COMAdminErrorCodes(-2146368467i32);
pub const COMAdminErrCompMovePrivate: COMAdminErrorCodes = COMAdminErrorCodes(-2146367458i32);
pub const COMAdminErrCompMoveSource: COMAdminErrorCodes = COMAdminErrorCodes(-2146367460i32);
pub const COMAdminErrComponentExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368455i32);
pub const COMAdminErrDllLoadFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368483i32);
pub const COMAdminErrDllRegisterServer: COMAdminErrorCodes = COMAdminErrorCodes(-2146368486i32);
pub const COMAdminErrDuplicatePartitionName: COMAdminErrorCodes = COMAdminErrorCodes(-2146368425i32);
pub const COMAdminErrEventClassCannotBeSubscriber: COMAdminErrorCodes = COMAdminErrorCodes(-2146368434i32);
pub const COMAdminErrImportedComponentsNotAllowed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368421i32);
pub const COMAdminErrInvalidPartition: COMAdminErrorCodes = COMAdminErrorCodes(-2146367477i32);
pub const COMAdminErrInvalidUserids: COMAdminErrorCodes = COMAdminErrorCodes(-2146368496i32);
pub const COMAdminErrKeyMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146368509i32);
pub const COMAdminErrLibAppProxyIncompatible: COMAdminErrorCodes = COMAdminErrorCodes(-2146368433i32);
pub const COMAdminErrMigSchemaNotFound: COMAdminErrorCodes = COMAdminErrorCodes(-2146368383i32);
pub const COMAdminErrMigVersionNotSupported: COMAdminErrorCodes = COMAdminErrorCodes(-2146368384i32);
pub const COMAdminErrNoRegistryCLSID: COMAdminErrorCodes = COMAdminErrorCodes(-2146368495i32);
pub const COMAdminErrNoServerShare: COMAdminErrorCodes = COMAdminErrorCodes(-2146368485i32);
pub const COMAdminErrNoUser: COMAdminErrorCodes = COMAdminErrorCodes(-2146368497i32);
pub const COMAdminErrNotChangeable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368470i32);
pub const COMAdminErrNotDeletable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368469i32);
pub const COMAdminErrNotInRegistry: COMAdminErrorCodes = COMAdminErrorCodes(-2146368450i32);
pub const COMAdminErrObjectDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146367479i32);
pub const COMAdminErrObjectErrors: COMAdminErrorCodes = COMAdminErrorCodes(-2146368511i32);
pub const COMAdminErrObjectExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368456i32);
pub const COMAdminErrObjectInvalid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368510i32);
pub const COMAdminErrObjectNotPoolable: COMAdminErrorCodes = COMAdminErrorCodes(-2146368449i32);
pub const COMAdminErrObjectParentMissing: COMAdminErrorCodes = COMAdminErrorCodes(-2146367480i32);
pub const COMAdminErrPartitionInUse: COMAdminErrorCodes = COMAdminErrorCodes(-2146368423i32);
pub const COMAdminErrPartitionMsiOnly: COMAdminErrorCodes = COMAdminErrorCodes(-2146367463i32);
pub const COMAdminErrPausedProcessMayNotBeRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367469i32);
pub const COMAdminErrProcessAlreadyRecycled: COMAdminErrorCodes = COMAdminErrorCodes(-2146367470i32);
pub const COMAdminErrPropertyOverflow: COMAdminErrorCodes = COMAdminErrorCodes(-2146368452i32);
pub const COMAdminErrPropertySaveFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368457i32);
pub const COMAdminErrQueuingServiceNotAvailable: COMAdminErrorCodes = COMAdminErrorCodes(-2146367998i32);
pub const COMAdminErrRegFileCorrupt: COMAdminErrorCodes = COMAdminErrorCodes(-2146368453i32);
pub const COMAdminErrRegdbAlreadyRunning: COMAdminErrorCodes = COMAdminErrorCodes(-2146368395i32);
pub const COMAdminErrRegdbNotInitialized: COMAdminErrorCodes = COMAdminErrorCodes(-2146368398i32);
pub const COMAdminErrRegdbNotOpen: COMAdminErrorCodes = COMAdminErrorCodes(-2146368397i32);
pub const COMAdminErrRegdbSystemErr: COMAdminErrorCodes = COMAdminErrorCodes(-2146368396i32);
pub const COMAdminErrRegisterTLB: COMAdminErrorCodes = COMAdminErrorCodes(-2146368464i32);
pub const COMAdminErrRegistrarFailed: COMAdminErrorCodes = COMAdminErrorCodes(-2146368477i32);
pub const COMAdminErrRemoteInterface: COMAdminErrorCodes = COMAdminErrorCodes(-2146368487i32);
pub const COMAdminErrRequiresDifferentPlatform: COMAdminErrorCodes = COMAdminErrorCodes(-2146368439i32);
pub const COMAdminErrRoleDoesNotExist: COMAdminErrorCodes = COMAdminErrorCodes(-2146368441i32);
pub const COMAdminErrRoleExists: COMAdminErrorCodes = COMAdminErrorCodes(-2146368500i32);
pub const COMAdminErrServiceNotInstalled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368458i32);
pub const COMAdminErrSession: COMAdminErrorCodes = COMAdminErrorCodes(-2146368468i32);
pub const COMAdminErrStartAppDisabled: COMAdminErrorCodes = COMAdminErrorCodes(-2146368431i32);
pub const COMAdminErrStartAppNeedsComponents: COMAdminErrorCodes = COMAdminErrorCodes(-2146368440i32);
pub const COMAdminErrSystemApp: COMAdminErrorCodes = COMAdminErrorCodes(-2146368461i32);
pub const COMAdminErrUserPasswdNotValid: COMAdminErrorCodes = COMAdminErrorCodes(-2146368492i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminErrorCodes(pub i32);
pub const COMAdminExportApplicationProxy: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(2i32);
pub const COMAdminExportForceOverwriteOfFiles: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(4i32);
pub const COMAdminExportIn10Format: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(16i32);
pub const COMAdminExportNoUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(0i32);
pub const COMAdminExportUsers: COMAdminApplicationExportOptions = COMAdminApplicationExportOptions(1i32);
pub const COMAdminFileFlagAlreadyInstalled: COMAdminFileFlags = COMAdminFileFlags(512i32);
pub const COMAdminFileFlagBadTLB: COMAdminFileFlags = COMAdminFileFlags(1024i32);
pub const COMAdminFileFlagCOM: COMAdminFileFlags = COMAdminFileFlags(2i32);
pub const COMAdminFileFlagClassNotAvailable: COMAdminFileFlags = COMAdminFileFlags(4096i32);
pub const COMAdminFileFlagContainsComp: COMAdminFileFlags = COMAdminFileFlags(8i32);
pub const COMAdminFileFlagContainsPS: COMAdminFileFlags = COMAdminFileFlags(4i32);
pub const COMAdminFileFlagContainsTLB: COMAdminFileFlags = COMAdminFileFlags(16i32);
pub const COMAdminFileFlagDLLRegsvrFailed: COMAdminFileFlags = COMAdminFileFlags(32768i32);
pub const COMAdminFileFlagDoesNotExist: COMAdminFileFlags = COMAdminFileFlags(256i32);
pub const COMAdminFileFlagError: COMAdminFileFlags = COMAdminFileFlags(262144i32);
pub const COMAdminFileFlagGetClassObjFailed: COMAdminFileFlags = COMAdminFileFlags(2048i32);
pub const COMAdminFileFlagLoadable: COMAdminFileFlags = COMAdminFileFlags(1i32);
pub const COMAdminFileFlagNoRegistrar: COMAdminFileFlags = COMAdminFileFlags(16384i32);
pub const COMAdminFileFlagRegTLBFailed: COMAdminFileFlags = COMAdminFileFlags(65536i32);
pub const COMAdminFileFlagRegistrar: COMAdminFileFlags = COMAdminFileFlags(8192i32);
pub const COMAdminFileFlagRegistrarFailed: COMAdminFileFlags = COMAdminFileFlags(131072i32);
pub const COMAdminFileFlagSelfReg: COMAdminFileFlags = COMAdminFileFlags(32i32);
pub const COMAdminFileFlagSelfUnReg: COMAdminFileFlags = COMAdminFileFlags(64i32);
pub const COMAdminFileFlagUnloadableDLL: COMAdminFileFlags = COMAdminFileFlags(128i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminFileFlags(pub i32);
pub const COMAdminImpersonationAnonymous: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(1i32);
pub const COMAdminImpersonationDelegate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(4i32);
pub const COMAdminImpersonationIdentify: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(2i32);
pub const COMAdminImpersonationImpersonate: COMAdminImpersonationLevelOptions = COMAdminImpersonationLevelOptions(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminImpersonationLevelOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminInUse(pub i32);
pub const COMAdminInUseByCatalog: COMAdminInUse = COMAdminInUse(1i32);
pub const COMAdminInUseByRegistryClsid: COMAdminInUse = COMAdminInUse(5i32);
pub const COMAdminInUseByRegistryProxyStub: COMAdminInUse = COMAdminInUse(3i32);
pub const COMAdminInUseByRegistryTypeLib: COMAdminInUse = COMAdminInUse(4i32);
pub const COMAdminInUseByRegistryUnknown: COMAdminInUse = COMAdminInUse(2i32);
pub const COMAdminInstallForceOverwriteOfFiles: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(2i32);
pub const COMAdminInstallNoUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(0i32);
pub const COMAdminInstallUsers: COMAdminApplicationInstallOptions = COMAdminApplicationInstallOptions(1i32);
pub const COMAdminNotInUse: COMAdminInUse = COMAdminInUse(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminOS(pub i32);
pub const COMAdminOSNotInitialized: COMAdminOS = COMAdminOS(0i32);
pub const COMAdminOSUnknown: COMAdminOS = COMAdminOS(6i32);
pub const COMAdminOSWindows2000: COMAdminOS = COMAdminOS(3i32);
pub const COMAdminOSWindows2000AdvancedServer: COMAdminOS = COMAdminOS(4i32);
pub const COMAdminOSWindows2000Unknown: COMAdminOS = COMAdminOS(5i32);
pub const COMAdminOSWindows3_1: COMAdminOS = COMAdminOS(1i32);
pub const COMAdminOSWindows7DatacenterServer: COMAdminOS = COMAdminOS(27i32);
pub const COMAdminOSWindows7EnterpriseServer: COMAdminOS = COMAdminOS(26i32);
pub const COMAdminOSWindows7Personal: COMAdminOS = COMAdminOS(23i32);
pub const COMAdminOSWindows7Professional: COMAdminOS = COMAdminOS(24i32);
pub const COMAdminOSWindows7StandardServer: COMAdminOS = COMAdminOS(25i32);
pub const COMAdminOSWindows7WebServer: COMAdminOS = COMAdminOS(28i32);
pub const COMAdminOSWindows8DatacenterServer: COMAdminOS = COMAdminOS(33i32);
pub const COMAdminOSWindows8EnterpriseServer: COMAdminOS = COMAdminOS(32i32);
pub const COMAdminOSWindows8Personal: COMAdminOS = COMAdminOS(29i32);
pub const COMAdminOSWindows8Professional: COMAdminOS = COMAdminOS(30i32);
pub const COMAdminOSWindows8StandardServer: COMAdminOS = COMAdminOS(31i32);
pub const COMAdminOSWindows8WebServer: COMAdminOS = COMAdminOS(34i32);
pub const COMAdminOSWindows9x: COMAdminOS = COMAdminOS(2i32);
pub const COMAdminOSWindowsBlueDatacenterServer: COMAdminOS = COMAdminOS(39i32);
pub const COMAdminOSWindowsBlueEnterpriseServer: COMAdminOS = COMAdminOS(38i32);
pub const COMAdminOSWindowsBluePersonal: COMAdminOS = COMAdminOS(35i32);
pub const COMAdminOSWindowsBlueProfessional: COMAdminOS = COMAdminOS(36i32);
pub const COMAdminOSWindowsBlueStandardServer: COMAdminOS = COMAdminOS(37i32);
pub const COMAdminOSWindowsBlueWebServer: COMAdminOS = COMAdminOS(40i32);
pub const COMAdminOSWindowsLonghornDatacenterServer: COMAdminOS = COMAdminOS(21i32);
pub const COMAdminOSWindowsLonghornEnterpriseServer: COMAdminOS = COMAdminOS(20i32);
pub const COMAdminOSWindowsLonghornPersonal: COMAdminOS = COMAdminOS(17i32);
pub const COMAdminOSWindowsLonghornProfessional: COMAdminOS = COMAdminOS(18i32);
pub const COMAdminOSWindowsLonghornStandardServer: COMAdminOS = COMAdminOS(19i32);
pub const COMAdminOSWindowsLonghornWebServer: COMAdminOS = COMAdminOS(22i32);
pub const COMAdminOSWindowsNETDatacenterServer: COMAdminOS = COMAdminOS(15i32);
pub const COMAdminOSWindowsNETEnterpriseServer: COMAdminOS = COMAdminOS(14i32);
pub const COMAdminOSWindowsNETStandardServer: COMAdminOS = COMAdminOS(13i32);
pub const COMAdminOSWindowsNETWebServer: COMAdminOS = COMAdminOS(16i32);
pub const COMAdminOSWindowsXPPersonal: COMAdminOS = COMAdminOS(11i32);
pub const COMAdminOSWindowsXPProfessional: COMAdminOS = COMAdminOS(12i32);
pub const COMAdminQCMessageAuthenticateOff: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(1i32);
pub const COMAdminQCMessageAuthenticateOn: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminQCMessageAuthenticateOptions(pub i32);
pub const COMAdminQCMessageAuthenticateSecureApps: COMAdminQCMessageAuthenticateOptions = COMAdminQCMessageAuthenticateOptions(0i32);
pub const COMAdminServiceContinuePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(4i32);
pub const COMAdminServiceLoadBalanceRouter: COMAdminServiceOptions = COMAdminServiceOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminServiceOptions(pub i32);
pub const COMAdminServicePausePending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(5i32);
pub const COMAdminServicePaused: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(6i32);
pub const COMAdminServiceRunning: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(3i32);
pub const COMAdminServiceStartPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminServiceStatusOptions(pub i32);
pub const COMAdminServiceStopPending: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(2i32);
pub const COMAdminServiceStopped: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(0i32);
pub const COMAdminServiceUnknownState: COMAdminServiceStatusOptions = COMAdminServiceStatusOptions(7i32);
pub const COMAdminSynchronizationIgnored: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(0i32);
pub const COMAdminSynchronizationNone: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminSynchronizationOptions(pub i32);
pub const COMAdminSynchronizationRequired: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(3i32);
pub const COMAdminSynchronizationRequiresNew: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(4i32);
pub const COMAdminSynchronizationSupported: COMAdminSynchronizationOptions = COMAdminSynchronizationOptions(2i32);
pub const COMAdminThreadingModelApartment: COMAdminThreadingModels = COMAdminThreadingModels(0i32);
pub const COMAdminThreadingModelBoth: COMAdminThreadingModels = COMAdminThreadingModels(3i32);
pub const COMAdminThreadingModelFree: COMAdminThreadingModels = COMAdminThreadingModels(1i32);
pub const COMAdminThreadingModelMain: COMAdminThreadingModels = COMAdminThreadingModels(2i32);
pub const COMAdminThreadingModelNeutral: COMAdminThreadingModels = COMAdminThreadingModels(4i32);
pub const COMAdminThreadingModelNotSpecified: COMAdminThreadingModels = COMAdminThreadingModels(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminThreadingModels(pub i32);
pub const COMAdminTransactionIgnored: COMAdminTransactionOptions = COMAdminTransactionOptions(0i32);
pub const COMAdminTransactionNone: COMAdminTransactionOptions = COMAdminTransactionOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminTransactionOptions(pub i32);
pub const COMAdminTransactionRequired: COMAdminTransactionOptions = COMAdminTransactionOptions(3i32);
pub const COMAdminTransactionRequiresNew: COMAdminTransactionOptions = COMAdminTransactionOptions(4i32);
pub const COMAdminTransactionSupported: COMAdminTransactionOptions = COMAdminTransactionOptions(2i32);
pub const COMAdminTxIsolationLevelAny: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMAdminTxIsolationLevelOptions(pub i32);
pub const COMAdminTxIsolationLevelReadCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(2i32);
pub const COMAdminTxIsolationLevelReadUnCommitted: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(1i32);
pub const COMAdminTxIsolationLevelRepeatableRead: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(3i32);
pub const COMAdminTxIsolationLevelSerializable: COMAdminTxIsolationLevelOptions = COMAdminTxIsolationLevelOptions(4i32);
pub const COMEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPLUS_APPTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMSVCSEVENTINFO {
    pub cbSize: u32,
    pub dwPid: u32,
    pub lTime: i64,
    pub lMicroTime: i32,
    pub perfCount: i64,
    pub guidApp: windows_core::GUID,
    pub sMachineName: windows_core::PWSTR,
}
pub const CRMClerk: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRMFLAGS(pub i32);
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = CRMFLAGS(1i32);
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = CRMFLAGS(64i32);
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = CRMFLAGS(8i32);
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = CRMFLAGS(4i32);
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = CRMFLAGS(2i32);
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = CRMFLAGS(16i32);
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = CRMFLAGS(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRMREGFLAGS(pub i32);
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = CRMREGFLAGS(4i32);
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = CRMREGFLAGS(7i32);
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = CRMREGFLAGS(2i32);
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = CRMREGFLAGS(16i32);
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = CRMREGFLAGS(1i32);
pub const CRMRecoveryClerk: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
pub const CSC_BindToPoolThread: CSC_Binding = CSC_Binding(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_Binding(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_COMTIIntrinsicsConfig(pub i32);
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = CSC_TransactionConfig(2i32);
pub const CSC_DontUseTracker: CSC_TrackerConfig = CSC_TrackerConfig(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_IISIntrinsicsConfig(pub i32);
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = CSC_SynchronizationConfig(1i32);
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = CSC_TransactionConfig(1i32);
pub const CSC_Ignore: CSC_InheritanceConfig = CSC_InheritanceConfig(1i32);
pub const CSC_Inherit: CSC_InheritanceConfig = CSC_InheritanceConfig(0i32);
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(1i32);
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(1i32);
pub const CSC_InheritPartition: CSC_PartitionConfig = CSC_PartitionConfig(1i32);
pub const CSC_InheritSxs: CSC_SxsConfig = CSC_SxsConfig(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_InheritanceConfig(pub i32);
pub const CSC_MTAThreadPool: CSC_ThreadPool = CSC_ThreadPool(3i32);
pub const CSC_NewPartition: CSC_PartitionConfig = CSC_PartitionConfig(2i32);
pub const CSC_NewSxs: CSC_SxsConfig = CSC_SxsConfig(2i32);
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(3i32);
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = CSC_SynchronizationConfig(2i32);
pub const CSC_NewTransaction: CSC_TransactionConfig = CSC_TransactionConfig(3i32);
pub const CSC_NoBinding: CSC_Binding = CSC_Binding(0i32);
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = CSC_COMTIIntrinsicsConfig(0i32);
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = CSC_IISIntrinsicsConfig(0i32);
pub const CSC_NoPartition: CSC_PartitionConfig = CSC_PartitionConfig(0i32);
pub const CSC_NoSxs: CSC_SxsConfig = CSC_SxsConfig(0i32);
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = CSC_SynchronizationConfig(0i32);
pub const CSC_NoTransaction: CSC_TransactionConfig = CSC_TransactionConfig(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_PartitionConfig(pub i32);
pub const CSC_STAThreadPool: CSC_ThreadPool = CSC_ThreadPool(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_SxsConfig(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_SynchronizationConfig(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_ThreadPool(pub i32);
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = CSC_ThreadPool(1i32);
pub const CSC_ThreadPoolNone: CSC_ThreadPool = CSC_ThreadPool(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_TrackerConfig(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSC_TransactionConfig(pub i32);
pub const CSC_UseTracker: CSC_TrackerConfig = CSC_TrackerConfig(1i32);
pub const CServiceConfig: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: windows_core::GUID = windows_core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
pub const CoMTSLocator: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: windows_core::BOOL,
    pub TerminateOnHang: windows_core::BOOL,
    pub AvgCallThresholdInMs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComponentStatistics {
    pub NumInstances: u32,
    pub NumBoundReferences: u32,
    pub NumPooledObjects: u32,
    pub NumObjectsInCall: u32,
    pub AvgResponseTimeInMs: u32,
    pub NumCallsCompletedRecent: u32,
    pub NumCallsFailedRecent: u32,
    pub NumCallsCompletedTotal: u32,
    pub NumCallsFailedTotal: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ComponentSummary {
    pub ApplicationInstanceId: windows_core::GUID,
    pub PartitionId: windows_core::GUID,
    pub ApplicationId: windows_core::GUID,
    pub Clsid: windows_core::GUID,
    pub ClassName: windows_core::PWSTR,
    pub ApplicationName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ContextInfo, ContextInfo_Vtbl, 0x19a5a02c_0ac8_11d2_b286_00c04f8ef934);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ContextInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ContextInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo {
    pub unsafe fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransactionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransactionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetActivityId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetContextId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo_Impl: super::Com::IDispatch_Impl {
    fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTransactionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetActivityId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetContextId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextInfo_Vtbl {
    pub const fn new<Identity: ContextInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsInTransaction<Identity: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo_Impl::IsInTransaction(this) {
                    Ok(ok__) => {
                        pbisintx.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptx: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo_Impl::GetTransaction(this) {
                    Ok(ok__) => {
                        pptx.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtxid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo_Impl::GetTransactionId(this) {
                    Ok(ok__) => {
                        pbstrtxid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstractivityid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo_Impl::GetActivityId(this) {
                    Ok(ok__) => {
                        pbstractivityid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextId<Identity: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrctxid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo_Impl::GetContextId(this) {
                    Ok(ok__) => {
                        pbstrctxid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, OFFSET>,
            GetActivityId: GetActivityId::<Identity, OFFSET>,
            GetContextId: GetContextId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ContextInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ContextInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ContextInfo2, ContextInfo2_Vtbl, 0xc99d6e75_2375_11d4_8331_00c04f605588);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ContextInfo2 {
    type Target = ContextInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ContextInfo2, windows_core::IUnknown, super::Com::IDispatch, ContextInfo);
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2 {
    pub unsafe fn GetPartitionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartitionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetApplicationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetApplicationInstanceId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextInfo2_Vtbl {
    pub base__: ContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo2_Impl: ContextInfo_Impl {
    fn GetPartitionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetApplicationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetApplicationInstanceId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextInfo2_Vtbl {
    pub const fn new<Identity: ContextInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPartitionId<Identity: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20000: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo2_Impl::GetPartitionId(this) {
                    Ok(ok__) => {
                        __midl__contextinfo20000.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20001: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo2_Impl::GetApplicationId(this) {
                    Ok(ok__) => {
                        __midl__contextinfo20001.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20002: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ContextInfo2_Impl::GetApplicationInstanceId(this) {
                    Ok(ok__) => {
                        __midl__contextinfo20002.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ContextInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ContextInfo2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ContextInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ContextInfo2 {}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: u32,
    pub dwSequenceNumber: u32,
    pub blobUserData: super::Com::BLOB,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CrmTransactionState(pub i32);
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DUMPTYPE(pub i32);
pub const DUMPTYPE_FULL: DUMPTYPE = DUMPTYPE(0i32);
pub const DUMPTYPE_MINI: DUMPTYPE = DUMPTYPE(1i32);
pub const DUMPTYPE_NONE: DUMPTYPE = DUMPTYPE(2i32);
pub const DispenserManager: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub const EventServer: windows_core::GUID = windows_core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(16i32);
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(8i32);
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = GetAppTrackerDataFlags(2i32);
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = GetAppTrackerDataFlags(1i32);
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = GetAppTrackerDataFlags(4i32);
pub const GUID_STRING_SIZE: u32 = 40u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GetAppTrackerDataFlags(pub i32);
pub const GetSecurityCallContextAppObject: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: windows_core::BOOL,
    pub fTerminateOnHang: windows_core::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: u32,
    pub dwDumpCount: u32,
    pub dwInfoMsgCount: u32,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAppDomainHelper, IAppDomainHelper_Vtbl, 0xc7b67079_8255_42c6_9ec0_6994a3548780);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAppDomainHelper {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAppDomainHelper, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelper {
    pub unsafe fn Initialize<P0>(&self, punkad: P0, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), punkad.param().abi(), __midl__iappdomainhelper0000, ppool as _).ok() }
    }
    pub unsafe fn DoCallback<P0>(&self, punkad: P0, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoCallback)(windows_core::Interface::as_raw(self), punkad.param().abi(), __midl__iappdomainhelper0001, ppool as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAppDomainHelper_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, isize, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAppDomainHelper_Impl: super::Com::IDispatch_Impl {
    fn Initialize(&self, punkad: windows_core::Ref<windows_core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DoCallback(&self, punkad: windows_core::Ref<windows_core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAppDomainHelper_Vtbl {
    pub const fn new<Identity: IAppDomainHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppDomainHelper_Impl::Initialize(this, core::mem::transmute_copy(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0000), core::mem::transmute_copy(&ppool)).into()
            }
        }
        unsafe extern "system" fn DoCallback<Identity: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAppDomainHelper_Impl::DoCallback(this, core::mem::transmute_copy(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0001), core::mem::transmute_copy(&ppool)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            DoCallback: DoCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppDomainHelper as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAppDomainHelper {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAssemblyLocator, IAssemblyLocator_Vtbl, 0x391ffbb9_a8ee_432a_abc8_baa238dab90f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAssemblyLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAssemblyLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocator {
    pub unsafe fn GetModules(&self, applicationdir: &windows_core::BSTR, applicationname: &windows_core::BSTR, assemblyname: &windows_core::BSTR) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModules)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(applicationdir), core::mem::transmute_copy(applicationname), core::mem::transmute_copy(assemblyname), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAssemblyLocator_Impl: super::Com::IDispatch_Impl {
    fn GetModules(&self, applicationdir: &windows_core::BSTR, applicationname: &windows_core::BSTR, assemblyname: &windows_core::BSTR) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAssemblyLocator_Vtbl {
    pub const fn new<Identity: IAssemblyLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetModules<Identity: IAssemblyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationdir: *mut core::ffi::c_void, applicationname: *mut core::ffi::c_void, assemblyname: *mut core::ffi::c_void, pmodules: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAssemblyLocator_Impl::GetModules(this, core::mem::transmute(&applicationdir), core::mem::transmute(&applicationname), core::mem::transmute(&assemblyname)) {
                    Ok(ok__) => {
                        pmodules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetModules: GetModules::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAssemblyLocator {}
windows_core::imp::define_interface!(IAsyncErrorNotify, IAsyncErrorNotify_Vtbl, 0xfe6777fb_a674_4177_8f32_6d707e113484);
windows_core::imp::interface_hierarchy!(IAsyncErrorNotify, windows_core::IUnknown);
impl IAsyncErrorNotify {
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncErrorNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IAsyncErrorNotify_Impl: windows_core::IUnknownImpl {
    fn OnError(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IAsyncErrorNotify_Vtbl {
    pub const fn new<Identity: IAsyncErrorNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnError<Identity: IAsyncErrorNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncErrorNotify_Impl::OnError(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncErrorNotify {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICOMAdminCatalog, ICOMAdminCatalog_Vtbl, 0xdd662187_dfc2_11d1_a2cf_00805fc79235);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICOMAdminCatalog {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog {
    pub unsafe fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Connect(&self, bstrcatalogservername: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcatalogservername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCollectionByQuery(&self, bstrcollname: &windows_core::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionByQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), ppsavarquery, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ImportComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrclsidorprogid)).ok() }
    }
    pub unsafe fn InstallComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrdll), core::mem::transmute_copy(bstrtlb), core::mem::transmute_copy(bstrpsdll)).ok() }
    }
    pub unsafe fn ShutdownApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShutdownApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname)).ok() }
    }
    pub unsafe fn ExportApplication(&self, bstrapplidorname: &windows_core::BSTR, bstrapplicationfile: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrapplicationfile), loptions).ok() }
    }
    pub unsafe fn InstallApplication(&self, bstrapplicationfile: &windows_core::BSTR, bstrdestinationdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), core::mem::transmute_copy(bstrdestinationdirectory), loptions, core::mem::transmute_copy(bstruserid), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrrsn)).ok() }
    }
    pub unsafe fn StopRouter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopRouter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RefreshRouter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshRouter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StartRouter(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartRouter)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reserved1(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reserved2(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InstallMultipleComponents(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallMultipleComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids).ok() }
    }
    pub unsafe fn GetMultipleComponentsInfo(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMultipleComponentsInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids as _, ppsavarclassnames as _, ppsavarfileflags as _, ppsavarcomponentflags as _).ok() }
    }
    pub unsafe fn RefreshComponents(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RefreshComponents)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BackupREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BackupREGDB)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupfilepath)).ok() }
    }
    pub unsafe fn RestoreREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestoreREGDB)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbackupfilepath)).ok() }
    }
    pub unsafe fn QueryApplicationFile(&self, bstrapplicationfile: &windows_core::BSTR, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryApplicationFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), core::mem::transmute(pbstrapplicationname), core::mem::transmute(pbstrapplicationdescription), pbhasusers as _, pbisproxy as _, ppsavarfilenames as _).ok() }
    }
    pub unsafe fn StartApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname)).ok() }
    }
    pub unsafe fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceCheck)(windows_core::Interface::as_raw(self), lservice, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstallMultipleEventClasses(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallMultipleEventClasses)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), ppsavarfilenames, ppsavarclsids).ok() }
    }
    pub unsafe fn InstallEventClass(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallEventClass)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplidorname), core::mem::transmute_copy(bstrdll), core::mem::transmute_copy(bstrtlb), core::mem::transmute_copy(bstrpsdll)).ok() }
    }
    pub unsafe fn GetEventClassesForIID(&self, bstriid: &windows_core::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEventClassesForIID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstriid), ppsavarclsids as _, ppsavarprogids as _, ppsavardescriptions as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCollectionByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::Com::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExportApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, COMAdminApplicationExportOptions) -> windows_core::HRESULT,
    pub InstallApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, COMAdminApplicationInstallOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartRouter: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallMultipleComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::Com::SAFEARRAY, *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub GetMultipleComponentsInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub RefreshComponents: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BackupREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestoreREGDB: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryApplicationFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub StartApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceCheck: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub InstallMultipleEventClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const *const super::Com::SAFEARRAY, *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub InstallEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEventClassesForIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog_Impl: super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrcatalogservername: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn GetCollectionByQuery(&self, bstrcollname: &windows_core::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> windows_core::Result<super::Com::IDispatch>;
    fn ImportComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShutdownApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExportApplication(&self, bstrapplidorname: &windows_core::BSTR, bstrapplicationfile: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>;
    fn InstallApplication(&self, bstrapplicationfile: &windows_core::BSTR, bstrdestinationdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopRouter(&self) -> windows_core::Result<()>;
    fn RefreshRouter(&self) -> windows_core::Result<()>;
    fn StartRouter(&self) -> windows_core::Result<()>;
    fn Reserved1(&self) -> windows_core::Result<()>;
    fn Reserved2(&self) -> windows_core::Result<()>;
    fn InstallMultipleComponents(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GetMultipleComponentsInfo(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RefreshComponents(&self) -> windows_core::Result<()>;
    fn BackupREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RestoreREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile(&self, bstrapplicationfile: &windows_core::BSTR, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn StartApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32>;
    fn InstallMultipleEventClasses(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn InstallEventClass(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetEventClassesForIID(&self, bstriid: &windows_core::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCollection<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcatalogservername: *mut core::ffi::c_void, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::Connect(this, core::mem::transmute(&bstrcatalogservername)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        plminorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::GetCollectionByQuery(this, core::mem::transmute(&bstrcollname), core::mem::transmute_copy(&ppsavarquery)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ImportComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrclsidorprogid)).into()
            }
        }
        unsafe extern "system" fn InstallComponent<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrdll: *mut core::ffi::c_void, bstrtlb: *mut core::ffi::c_void, bstrpsdll: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
            }
        }
        unsafe extern "system" fn ShutdownApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ShutdownApplication(this, core::mem::transmute(&bstrapplidorname)).into()
            }
        }
        unsafe extern "system" fn ExportApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::ExportApplication(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn InstallApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, bstrdestinationdirectory: *mut core::ffi::c_void, loptions: COMAdminApplicationInstallOptions, bstruserid: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrrsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallApplication(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute(&bstrdestinationdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
            }
        }
        unsafe extern "system" fn StopRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StopRouter(this).into()
            }
        }
        unsafe extern "system" fn RefreshRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RefreshRouter(this).into()
            }
        }
        unsafe extern "system" fn StartRouter<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StartRouter(this).into()
            }
        }
        unsafe extern "system" fn Reserved1<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::Reserved1(this).into()
            }
        }
        unsafe extern "system" fn Reserved2<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::Reserved2(this).into()
            }
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallMultipleComponents(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
            }
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::GetMultipleComponentsInfo(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarclassnames), core::mem::transmute_copy(&ppsavarfileflags), core::mem::transmute_copy(&ppsavarcomponentflags)).into()
            }
        }
        unsafe extern "system" fn RefreshComponents<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RefreshComponents(this).into()
            }
        }
        unsafe extern "system" fn BackupREGDB<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::BackupREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
            }
        }
        unsafe extern "system" fn RestoreREGDB<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::RestoreREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
            }
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, pbstrapplicationname: *mut *mut core::ffi::c_void, pbstrapplicationdescription: *mut *mut core::ffi::c_void, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::QueryApplicationFile(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&pbstrapplicationname), core::mem::transmute_copy(&pbstrapplicationdescription), core::mem::transmute_copy(&pbhasusers), core::mem::transmute_copy(&pbisproxy), core::mem::transmute_copy(&ppsavarfilenames)).into()
            }
        }
        unsafe extern "system" fn StartApplication<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::StartApplication(this, core::mem::transmute(&bstrapplidorname)).into()
            }
        }
        unsafe extern "system" fn ServiceCheck<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog_Impl::ServiceCheck(this, core::mem::transmute_copy(&lservice)) {
                    Ok(ok__) => {
                        plstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallMultipleEventClasses(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
            }
        }
        unsafe extern "system" fn InstallEventClass<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: *mut core::ffi::c_void, bstrdll: *mut core::ffi::c_void, bstrtlb: *mut core::ffi::c_void, bstrpsdll: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::InstallEventClass(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
            }
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstriid: *mut core::ffi::c_void, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog_Impl::GetEventClassesForIID(this, core::mem::transmute(&bstriid), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarprogids), core::mem::transmute_copy(&ppsavardescriptions)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetCollection: GetCollection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, OFFSET>,
            ImportComponent: ImportComponent::<Identity, OFFSET>,
            InstallComponent: InstallComponent::<Identity, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, OFFSET>,
            ExportApplication: ExportApplication::<Identity, OFFSET>,
            InstallApplication: InstallApplication::<Identity, OFFSET>,
            StopRouter: StopRouter::<Identity, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, OFFSET>,
            StartRouter: StartRouter::<Identity, OFFSET>,
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, OFFSET>,
            StartApplication: StartApplication::<Identity, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICOMAdminCatalog {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICOMAdminCatalog2, ICOMAdminCatalog2_Vtbl, 0x790c6e0b_9194_4cc9_9426_a48a63185696);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICOMAdminCatalog2 {
    type Target = ICOMAdminCatalog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICOMAdminCatalog2, windows_core::IUnknown, super::Com::IDispatch, ICOMAdminCatalog);
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2 {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCollectionByQuery2(&self, bstrcollectionname: &windows_core::BSTR, pvarquerystrings: *const super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionByQuery2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollectionname), core::mem::transmute(pvarquerystrings), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationInstanceIDFromProcessID)(windows_core::Interface::as_raw(self), lprocessid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShutdownApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PauseApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResumeApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecycleApplicationInstances)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), lreasoncode).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreApplicationInstancesPaused)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarapplicationinstanceid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &windows_core::BSTR, bstrdirectory: &windows_core::BSTR, lmaximages: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpApplicationInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationinstanceid), core::mem::transmute_copy(bstrdirectory), lmaximages, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsApplicationInstanceDumpSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR, bstrservicename: &windows_core::BSTR, bstrstarttype: &windows_core::BSTR, bstrerrorcontrol: &windows_core::BSTR, bstrdependencies: &windows_core::BSTR, bstrrunas: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateServiceForApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute_copy(bstrservicename), core::mem::transmute_copy(bstrstarttype), core::mem::transmute_copy(bstrerrorcontrol), core::mem::transmute_copy(bstrdependencies), core::mem::transmute_copy(bstrrunas), core::mem::transmute_copy(bstrpassword), bdesktopok).ok() }
    }
    pub unsafe fn DeleteServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteServiceForApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname)).ok() }
    }
    pub unsafe fn GetPartitionID(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartitionID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPartitionName(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartitionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCurrentPartition(&self, bstrpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpartitionidorname)).ok() }
    }
    pub unsafe fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPartitionID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPartitionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GlobalPartitionID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FlushPartitionCache(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FlushPartitionCache)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyApplications(&self, bstrsourcepartitionidorname: &windows_core::BSTR, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyApplications)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourcepartitionidorname), core::mem::transmute(pvarapplicationid), core::mem::transmute_copy(bstrdestinationpartitionidorname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourceapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute_copy(bstrdestinationapplicationidorname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsourceapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute_copy(bstrdestinationapplicationidorname)).ok() }
    }
    pub unsafe fn AliasComponent(&self, bstrsrcapplicationidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR, bstrdestapplicationidorname: &windows_core::BSTR, bstrnewprogid: &windows_core::BSTR, bstrnewclsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AliasComponent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsrcapplicationidorname), core::mem::transmute_copy(bstrclsidorprogid), core::mem::transmute_copy(bstrdestapplicationidorname), core::mem::transmute_copy(bstrnewprogid), core::mem::transmute_copy(bstrnewclsid)).ok() }
    }
    pub unsafe fn IsSafeToDelete(&self, bstrdllname: &windows_core::BSTR) -> windows_core::Result<COMAdminInUse> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSafeToDelete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdllname), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportUnconfiguredComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PromoteUnconfiguredComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ImportComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImportComponents)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationidorname), core::mem::transmute(pvarclsidorprogid), core::mem::transmute(pvarcomponenttype)).ok() }
    }
    pub unsafe fn Is64BitCatalogServer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Is64BitCatalogServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExportPartition(&self, bstrpartitionidorname: &windows_core::BSTR, bstrpartitionfilename: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExportPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpartitionidorname), core::mem::transmute_copy(bstrpartitionfilename), loptions).ok() }
    }
    pub unsafe fn InstallPartition(&self, bstrfilename: &windows_core::BSTR, bstrdestdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InstallPartition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfilename), core::mem::transmute_copy(bstrdestdirectory), loptions, core::mem::transmute_copy(bstruserid), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrrsn)).ok() }
    }
    pub unsafe fn QueryApplicationFile2(&self, bstrapplicationfile: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryApplicationFile2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrapplicationfile), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetComponentVersionCount(&self, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentVersionCount)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsidorprogid), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICOMAdminCatalog2_Vtbl {
    pub base__: ICOMAdminCatalog_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCollectionByQuery2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCollectionByQuery2: usize,
    pub GetApplicationInstanceIDFromProcessID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ShutdownApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ShutdownApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PauseApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PauseApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResumeApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResumeApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RecycleApplicationInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RecycleApplicationInstances: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AreApplicationInstancesPaused: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AreApplicationInstancesPaused: usize,
    pub DumpApplicationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsApplicationInstanceDumpSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CreateServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteServiceForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrentPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentPartitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GlobalPartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushPartitionCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyApplications: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyComponents: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveComponents: usize,
    pub AliasComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSafeToDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut COMAdminInUse) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PromoteUnconfiguredComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PromoteUnconfiguredComponents: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ImportComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ImportComponents: usize,
    pub Is64BitCatalogServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExportPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, COMAdminApplicationExportOptions) -> windows_core::HRESULT,
    pub InstallPartition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, COMAdminApplicationInstallOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryApplicationFile2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentVersionCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog2_Impl: ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&self, bstrcollectionname: &windows_core::BSTR, pvarquerystrings: *const super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> windows_core::Result<()>;
    fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &windows_core::BSTR, bstrdirectory: &windows_core::BSTR, lmaximages: i32) -> windows_core::Result<windows_core::BSTR>;
    fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CreateServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR, bstrservicename: &windows_core::BSTR, bstrstarttype: &windows_core::BSTR, bstrerrorcontrol: &windows_core::BSTR, bstrdependencies: &windows_core::BSTR, bstrrunas: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPartitionID(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetPartitionName(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentPartition(&self, bstrpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FlushPartitionCache(&self) -> windows_core::Result<()>;
    fn CopyApplications(&self, bstrsourcepartitionidorname: &windows_core::BSTR, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MoveComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AliasComponent(&self, bstrsrcapplicationidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR, bstrdestapplicationidorname: &windows_core::BSTR, bstrnewprogid: &windows_core::BSTR, bstrnewclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSafeToDelete(&self, bstrdllname: &windows_core::BSTR) -> windows_core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ImportComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Is64BitCatalogServer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExportPartition(&self, bstrpartitionidorname: &windows_core::BSTR, bstrpartitionfilename: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>;
    fn InstallPartition(&self, bstrfilename: &windows_core::BSTR, bstrdestdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile2(&self, bstrapplicationfile: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(&self, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollectionname: *mut core::ffi::c_void, pvarquerystrings: *const super::Variant::VARIANT, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetCollectionByQuery2(this, core::mem::transmute(&bstrcollectionname), core::mem::transmute_copy(&pvarquerystrings)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetApplicationInstanceIDFromProcessID(this, core::mem::transmute_copy(&lprocessid)) {
                    Ok(ok__) => {
                        pbstrapplicationinstanceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ShutdownApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::PauseApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ResumeApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
            }
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::RecycleApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid), core::mem::transmute_copy(&lreasoncode)).into()
            }
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, pvarboolpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::AreApplicationInstancesPaused(this, core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                    Ok(ok__) => {
                        pvarboolpaused.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationinstanceid: *mut core::ffi::c_void, bstrdirectory: *mut core::ffi::c_void, lmaximages: i32, pbstrdumpfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::DumpApplicationInstance(this, core::mem::transmute(&bstrapplicationinstanceid), core::mem::transmute(&bstrdirectory), core::mem::transmute_copy(&lmaximages)) {
                    Ok(ok__) => {
                        pbstrdumpfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbooldumpsupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::IsApplicationInstanceDumpSupported(this) {
                    Ok(ok__) => {
                        pvarbooldumpsupported.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, bstrservicename: *mut core::ffi::c_void, bstrstarttype: *mut core::ffi::c_void, bstrerrorcontrol: *mut core::ffi::c_void, bstrdependencies: *mut core::ffi::c_void, bstrrunas: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CreateServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute(&bstrservicename), core::mem::transmute(&bstrstarttype), core::mem::transmute(&bstrerrorcontrol), core::mem::transmute(&bstrdependencies), core::mem::transmute(&bstrrunas), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&bdesktopok)).into()
            }
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::DeleteServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn GetPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pbstrpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetPartitionID(this, core::mem::transmute(&bstrapplicationidorname)) {
                    Ok(ok__) => {
                        pbstrpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pbstrpartitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetPartitionName(this, core::mem::transmute(&bstrapplicationidorname)) {
                    Ok(ok__) => {
                        pbstrpartitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::SetCurrentPartition(this, core::mem::transmute(&bstrpartitionidorname)).into()
            }
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::CurrentPartitionID(this) {
                    Ok(ok__) => {
                        pbstrpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::CurrentPartitionName(this) {
                    Ok(ok__) => {
                        pbstrpartitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrglobalpartitionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GlobalPartitionID(this) {
                    Ok(ok__) => {
                        pbstrglobalpartitionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::FlushPartitionCache(this).into()
            }
        }
        unsafe extern "system" fn CopyApplications<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourcepartitionidorname: *mut core::ffi::c_void, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CopyApplications(this, core::mem::transmute(&bstrsourcepartitionidorname), core::mem::transmute_copy(&pvarapplicationid), core::mem::transmute(&bstrdestinationpartitionidorname)).into()
            }
        }
        unsafe extern "system" fn CopyComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::CopyComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn MoveComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::MoveComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
            }
        }
        unsafe extern "system" fn AliasComponent<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsrcapplicationidorname: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void, bstrdestapplicationidorname: *mut core::ffi::c_void, bstrnewprogid: *mut core::ffi::c_void, bstrnewclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::AliasComponent(this, core::mem::transmute(&bstrsrcapplicationidorname), core::mem::transmute(&bstrclsidorprogid), core::mem::transmute(&bstrdestapplicationidorname), core::mem::transmute(&bstrnewprogid), core::mem::transmute(&bstrnewclsid)).into()
            }
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllname: *mut core::ffi::c_void, pcomadmininuse: *mut COMAdminInUse) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::IsSafeToDelete(this, core::mem::transmute(&bstrdllname)) {
                    Ok(ok__) => {
                        pcomadmininuse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ImportUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::PromoteUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn ImportComponents<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: *mut core::ffi::c_void, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ImportComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
            }
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbis64bit: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::Is64BitCatalogServer(this) {
                    Ok(ok__) => {
                        pbis64bit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: *mut core::ffi::c_void, bstrpartitionfilename: *mut core::ffi::c_void, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::ExportPartition(this, core::mem::transmute(&bstrpartitionidorname), core::mem::transmute(&bstrpartitionfilename), core::mem::transmute_copy(&loptions)).into()
            }
        }
        unsafe extern "system" fn InstallPartition<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, bstrdestdirectory: *mut core::ffi::c_void, loptions: COMAdminApplicationInstallOptions, bstruserid: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrrsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMAdminCatalog2_Impl::InstallPartition(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrdestdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
            }
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: *mut core::ffi::c_void, ppfilesforimport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::QueryApplicationFile2(this, core::mem::transmute(&bstrapplicationfile)) {
                    Ok(ok__) => {
                        ppfilesforimport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsidorprogid: *mut core::ffi::c_void, plversioncount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICOMAdminCatalog2_Impl::GetComponentVersionCount(this, core::mem::transmute(&bstrclsidorprogid)) {
                    Ok(ok__) => {
                        plversioncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICOMAdminCatalog_Vtbl::new::<Identity, OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, OFFSET>,
            CopyApplications: CopyApplications::<Identity, OFFSET>,
            CopyComponents: CopyComponents::<Identity, OFFSET>,
            MoveComponents: MoveComponents::<Identity, OFFSET>,
            AliasComponent: AliasComponent::<Identity, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, OFFSET>,
            ImportComponents: ImportComponents::<Identity, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, OFFSET>,
            ExportPartition: ExportPartition::<Identity, OFFSET>,
            InstallPartition: InstallPartition::<Identity, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICOMAdminCatalog as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICOMAdminCatalog2 {}
windows_core::imp::define_interface!(ICOMLBArguments, ICOMLBArguments_Vtbl, 0x3a0f150f_8ee5_4b94_b40e_aef2f9e42ed2);
windows_core::imp::interface_hierarchy!(ICOMLBArguments, windows_core::IUnknown);
impl ICOMLBArguments {
    pub unsafe fn GetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCLSID)(windows_core::Interface::as_raw(self), pclsid as _).ok() }
    }
    pub unsafe fn SetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCLSID)(windows_core::Interface::as_raw(self), pclsid as _).ok() }
    }
    pub unsafe fn GetMachineName(&self, szservername: &mut [u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMachineName)(windows_core::Interface::as_raw(self), szservername.len().try_into().unwrap(), core::mem::transmute(szservername.as_ptr())).ok() }
    }
    pub unsafe fn SetMachineName(&self, szservername: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMachineName)(windows_core::Interface::as_raw(self), szservername.len().try_into().unwrap(), core::mem::transmute(szservername.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICOMLBArguments_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICOMLBArguments_Impl: windows_core::IUnknownImpl {
    fn GetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetMachineName(&self, cchsvr: u32, szservername: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetMachineName(&self, cchsvr: u32, szservername: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICOMLBArguments_Vtbl {
    pub const fn new<Identity: ICOMLBArguments_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCLSID<Identity: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMLBArguments_Impl::GetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
            }
        }
        unsafe extern "system" fn SetCLSID<Identity: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMLBArguments_Impl::SetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
            }
        }
        unsafe extern "system" fn GetMachineName<Identity: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMLBArguments_Impl::GetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute_copy(&szservername)).into()
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICOMLBArguments_Impl::SetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute(&szservername)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            SetCLSID: SetCLSID::<Identity, OFFSET>,
            GetMachineName: GetMachineName::<Identity, OFFSET>,
            SetMachineName: SetMachineName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMLBArguments as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICOMLBArguments {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICatalogCollection, ICatalogCollection_Vtbl, 0x6eb22872_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICatalogCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICatalogCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self, lindex: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), lindex).ok() }
    }
    pub unsafe fn Add(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Populate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Populate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCollection(&self, bstrcollname: &windows_core::BSTR, varobjectkey: &super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcollname), core::mem::transmute_copy(varobjectkey), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Name(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoveEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUtilInterface(&self) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUtilInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DataStoreMajorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataStoreMajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataStoreMinorVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataStoreMinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PopulateByKey)(windows_core::Interface::as_raw(self), psakeys).ok() }
    }
    pub unsafe fn PopulateByQuery(&self, bstrquerystring: &windows_core::BSTR, lquerytype: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PopulateByQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrquerystring), lquerytype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Populate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCollection: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Name: usize,
    pub AddEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RemoveEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetUtilInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataStoreMajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataStoreMinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PopulateByKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub PopulateByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogCollection_Impl: super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Com::IDispatch>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn Add(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Populate(&self) -> windows_core::Result<()>;
    fn SaveChanges(&self) -> windows_core::Result<i32>;
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR, varobjectkey: &super::Variant::VARIANT) -> windows_core::Result<super::Com::IDispatch>;
    fn Name(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn AddEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RemoveEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetUtilInterface(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(&self) -> windows_core::Result<i32>;
    fn DataStoreMinorVersion(&self) -> windows_core::Result<i32>;
    fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn PopulateByQuery(&self, bstrquerystring: &windows_core::BSTR, lquerytype: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: ICatalogCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenumvariant.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppcatalogobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plobjectcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        plobjectcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Add(this) {
                    Ok(ok__) => {
                        ppcatalogobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Populate<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::Populate(this).into()
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchanges: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::SaveChanges(this) {
                    Ok(ok__) => {
                        pcchanges.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollection<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: *mut core::ffi::c_void, varobjectkey: super::Variant::VARIANT, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::GetCollection(this, core::mem::transmute(&bstrcollname), core::mem::transmute(&varobjectkey)) {
                    Ok(ok__) => {
                        ppcatalogcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnamel: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::Name(this) {
                    Ok(ok__) => {
                        pvarnamel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::AddEnabled(this) {
                    Ok(ok__) => {
                        pvarbool.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::RemoveEnabled(this) {
                    Ok(ok__) => {
                        pvarbool.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::GetUtilInterface(this) {
                    Ok(ok__) => {
                        ppidispatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::DataStoreMajorVersion(this) {
                    Ok(ok__) => {
                        plmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversionl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogCollection_Impl::DataStoreMinorVersion(this) {
                    Ok(ok__) => {
                        plminorversionl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::PopulateByKey(this, core::mem::transmute_copy(&psakeys)).into()
            }
        }
        unsafe extern "system" fn PopulateByQuery<Identity: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquerystring: *mut core::ffi::c_void, lquerytype: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogCollection_Impl::PopulateByQuery(this, core::mem::transmute(&bstrquerystring), core::mem::transmute_copy(&lquerytype)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Populate: Populate::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            GetCollection: GetCollection::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            AddEnabled: AddEnabled::<Identity, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICatalogCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICatalogObject, ICatalogObject_Vtbl, 0x6eb22871_8a19_11d0_81b6_00a0c9231c29);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICatalogObject {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICatalogObject, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObject {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Value(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_Value(&self, bstrpropname: &windows_core::BSTR, val: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), core::mem::transmute_copy(val)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Key(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Name(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsPropertyReadOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPropertyReadOnly)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Valid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Valid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsPropertyWriteOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPropertyWriteOnly)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Key: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Name: usize,
    pub IsPropertyReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Valid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsPropertyWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogObject_Impl: super::Com::IDispatch_Impl {
    fn get_Value(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn put_Value(&self, bstrpropname: &windows_core::BSTR, val: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn Name(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn IsPropertyReadOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Valid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPropertyWriteOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: ICatalogObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::get_Value(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, val: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICatalogObject_Impl::put_Value(this, core::mem::transmute(&bstrpropname), core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn Key<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Key(this) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Name(this) {
                    Ok(ok__) => {
                        pvarretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::IsPropertyReadOnly(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pbretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Valid<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::Valid(this) {
                    Ok(ok__) => {
                        pbretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICatalogObject_Impl::IsPropertyWriteOnly(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pbretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, OFFSET>,
            Valid: Valid::<Identity, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICatalogObject {}
windows_core::imp::define_interface!(ICheckSxsConfig, ICheckSxsConfig_Vtbl, 0x0ff5a96f_11fc_47d1_baa6_25dd347e7242);
windows_core::imp::interface_hierarchy!(ICheckSxsConfig, windows_core::IUnknown);
impl ICheckSxsConfig {
    pub unsafe fn IsSameSxsConfig<P0, P1, P2>(&self, wszsxsname: P0, wszsxsdirectory: P1, wszsxsappname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsSameSxsConfig)(windows_core::Interface::as_raw(self), wszsxsname.param().abi(), wszsxsdirectory.param().abi(), wszsxsappname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICheckSxsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSameSxsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ICheckSxsConfig_Impl: windows_core::IUnknownImpl {
    fn IsSameSxsConfig(&self, wszsxsname: &windows_core::PCWSTR, wszsxsdirectory: &windows_core::PCWSTR, wszsxsappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ICheckSxsConfig_Vtbl {
    pub const fn new<Identity: ICheckSxsConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSameSxsConfig<Identity: ICheckSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsxsname: windows_core::PCWSTR, wszsxsdirectory: windows_core::PCWSTR, wszsxsappname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICheckSxsConfig_Impl::IsSameSxsConfig(this, core::mem::transmute(&wszsxsname), core::mem::transmute(&wszsxsdirectory), core::mem::transmute(&wszsxsappname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICheckSxsConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICheckSxsConfig {}
windows_core::imp::define_interface!(IComActivityEvents, IComActivityEvents_Vtbl, 0x683130b0_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComActivityEvents, windows_core::IUnknown);
impl IComActivityEvents {
    pub unsafe fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityCreate)(windows_core::Interface::as_raw(self), pinfo, guidactivity).ok() }
    }
    pub unsafe fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityDestroy)(windows_core::Interface::as_raw(self), pinfo, guidactivity).ok() }
    }
    pub unsafe fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityEnter)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidentered, dwthread).ok() }
    }
    pub unsafe fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityTimeout)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidentered, dwthread, dwtimeout).ok() }
    }
    pub unsafe fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityReenter)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, dwthread, dwcalldepth).ok() }
    }
    pub unsafe fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityLeave)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, guidleft).ok() }
    }
    pub unsafe fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnActivityLeaveSame)(windows_core::Interface::as_raw(self), pinfo, guidcurrent, dwcalldepth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComActivityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnActivityCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnActivityTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnActivityReenter: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnActivityLeave: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnActivityLeaveSame: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IComActivityEvents_Impl: windows_core::IUnknownImpl {
    fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::Result<()>;
    fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::Result<()>;
    fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::Result<()>;
    fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::Result<()>;
}
impl IComActivityEvents_Vtbl {
    pub const fn new<Identity: IComActivityEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivityCreate<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
            }
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
            }
        }
        unsafe extern "system" fn OnActivityEnter<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityEnter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread)).into()
            }
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn OnActivityReenter<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityReenter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwcalldepth)).into()
            }
        }
        unsafe extern "system" fn OnActivityLeave<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityLeave(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidleft)).into()
            }
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComActivityEvents_Impl::OnActivityLeaveSame(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwcalldepth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Identity, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComActivityEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComActivityEvents {}
windows_core::imp::define_interface!(IComApp2Events, IComApp2Events_Vtbl, 0x1290bc1a_b219_418d_b078_5934ded08242);
windows_core::imp::interface_hierarchy!(IComApp2Events, windows_core::IUnknown);
impl IComApp2Events {
    pub unsafe fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppActivation2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), core::mem::transmute(guidprocess)).ok() }
    }
    pub unsafe fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppShutdown2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppForceShutdown2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnAppPaused2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, bpaused: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppPaused2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), bpaused.into()).ok() }
    }
    pub unsafe fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID, lreason: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppRecycle2)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp), core::mem::transmute(guidprocess), lreason).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComApp2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppActivation2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppShutdown2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppForceShutdown2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppPaused2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnAppRecycle2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, i32) -> windows_core::HRESULT,
}
pub trait IComApp2Events_Impl: windows_core::IUnknownImpl {
    fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, guidprocess: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppPaused2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, bpaused: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, guidprocess: &windows_core::GUID, lreason: i32) -> windows_core::Result<()>;
}
impl IComApp2Events_Vtbl {
    pub const fn new<Identity: IComApp2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAppActivation2<Identity: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComApp2Events_Impl::OnAppActivation2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess)).into()
            }
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComApp2Events_Impl::OnAppShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComApp2Events_Impl::OnAppForceShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnAppPaused2<Identity: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, bpaused: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComApp2Events_Impl::OnAppPaused2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute_copy(&bpaused)).into()
            }
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID, lreason: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComApp2Events_Impl::OnAppRecycle2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess), core::mem::transmute_copy(&lreason)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Identity, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComApp2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComApp2Events {}
windows_core::imp::define_interface!(IComAppEvents, IComAppEvents_Vtbl, 0x683130a6_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComAppEvents, windows_core::IUnknown);
impl IComAppEvents {
    pub unsafe fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppActivation)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppShutdown)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAppForceShutdown)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComAppEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnAppForceShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComAppEvents_Impl: windows_core::IUnknownImpl {
    fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IComAppEvents_Vtbl {
    pub const fn new<Identity: IComAppEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAppActivation<Identity: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComAppEvents_Impl::OnAppActivation(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnAppShutdown<Identity: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComAppEvents_Impl::OnAppShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComAppEvents_Impl::OnAppForceShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation: OnAppActivation::<Identity, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComAppEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComAppEvents {}
windows_core::imp::define_interface!(IComCRMEvents, IComCRMEvents_Vtbl, 0x683130b5_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComCRMEvents, windows_core::IUnknown);
impl IComCRMEvents {
    pub unsafe fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMRecoveryStart)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMRecoveryDone)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMCheckpoint)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidapp)).ok() }
    }
    pub unsafe fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, guidactivity: windows_core::GUID, guidtx: windows_core::GUID, szprogidcompensator: &[u16; 64], szdescription: &[u16; 64]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMBegin)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), core::mem::transmute(guidactivity), core::mem::transmute(guidtx), core::mem::transmute(szprogidcompensator.as_ptr()), core::mem::transmute(szdescription.as_ptr())).ok() }
    }
    pub unsafe fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMPrepare)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMCommit)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMAbort)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMIndoubt)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMDone)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMRelease)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMAnalyze)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), dwcrmrecordtype, dwrecordsize).ok() }
    }
    pub unsafe fn OnCRMWrite(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: bool, dwrecordsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMWrite)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), fvariants.into(), dwrecordsize).ok() }
    }
    pub unsafe fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMForget)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMForce)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid)).ok() }
    }
    pub unsafe fn OnCRMDeliver(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: bool, dwrecordsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCRMDeliver)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidclerkclsid), fvariants.into(), dwrecordsize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComCRMEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCRMRecoveryStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMRecoveryDone: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMCheckpoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMBegin: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnCRMPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMIndoubt: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMDone: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMAnalyze: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnCRMWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub OnCRMForget: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMForce: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnCRMDeliver: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::BOOL, u32) -> windows_core::HRESULT,
}
pub trait IComCRMEvents_Impl: windows_core::IUnknownImpl {
    fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, guidactivity: &windows_core::GUID, guidtx: &windows_core::GUID, szprogidcompensator: &windows_core::PCWSTR, szdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::Result<()>;
    fn OnCRMWrite(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, fvariants: windows_core::BOOL, dwrecordsize: u32) -> windows_core::Result<()>;
    fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMDeliver(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, fvariants: windows_core::BOOL, dwrecordsize: u32) -> windows_core::Result<()>;
}
impl IComCRMEvents_Vtbl {
    pub const fn new<Identity: IComCRMEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMRecoveryStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMRecoveryDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMCheckpoint(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
            }
        }
        unsafe extern "system" fn OnCRMBegin<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, guidactivity: windows_core::GUID, guidtx: windows_core::GUID, szprogidcompensator: windows_core::PCWSTR, szdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMBegin(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute(&guidactivity), core::mem::transmute(&guidtx), core::mem::transmute(&szprogidcompensator), core::mem::transmute(&szdescription)).into()
            }
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMCommit<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMAbort<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMIndoubt(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMDone<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMRelease<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMRelease(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMAnalyze(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&dwcrmrecordtype), core::mem::transmute_copy(&dwrecordsize)).into()
            }
        }
        unsafe extern "system" fn OnCRMWrite<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: windows_core::BOOL, dwrecordsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMWrite(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
            }
        }
        unsafe extern "system" fn OnCRMForget<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMForget(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMForce<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMForce(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
            }
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: windows_core::BOOL, dwrecordsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComCRMEvents_Impl::OnCRMDeliver(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Identity, OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Identity, OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Identity, OFFSET>,
            OnCRMBegin: OnCRMBegin::<Identity, OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Identity, OFFSET>,
            OnCRMCommit: OnCRMCommit::<Identity, OFFSET>,
            OnCRMAbort: OnCRMAbort::<Identity, OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Identity, OFFSET>,
            OnCRMDone: OnCRMDone::<Identity, OFFSET>,
            OnCRMRelease: OnCRMRelease::<Identity, OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Identity, OFFSET>,
            OnCRMWrite: OnCRMWrite::<Identity, OFFSET>,
            OnCRMForget: OnCRMForget::<Identity, OFFSET>,
            OnCRMForce: OnCRMForce::<Identity, OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComCRMEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComCRMEvents {}
windows_core::imp::define_interface!(IComExceptionEvents, IComExceptionEvents_Vtbl, 0x683130b3_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComExceptionEvents, windows_core::IUnknown);
impl IComExceptionEvents {
    pub unsafe fn OnExceptionUser<P3>(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnExceptionUser)(windows_core::Interface::as_raw(self), pinfo, code, address, pszstacktrace.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComExceptionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnExceptionUser: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u32, u64, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IComExceptionEvents_Impl: windows_core::IUnknownImpl {
    fn OnExceptionUser(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IComExceptionEvents_Vtbl {
    pub const fn new<Identity: IComExceptionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnExceptionUser<Identity: IComExceptionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComExceptionEvents_Impl::OnExceptionUser(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&code), core::mem::transmute_copy(&address), core::mem::transmute(&pszstacktrace)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnExceptionUser: OnExceptionUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComExceptionEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComExceptionEvents {}
windows_core::imp::define_interface!(IComIdentityEvents, IComIdentityEvents_Vtbl, 0x683130b1_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComIdentityEvents, windows_core::IUnknown);
impl IComIdentityEvents {
    pub unsafe fn OnIISRequestInfo<P2, P3, P4>(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: P2, pszserverip: P3, pszurl: P4) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIISRequestInfo)(windows_core::Interface::as_raw(self), pinfo, objid, pszclientip.param().abi(), pszserverip.param().abi(), pszurl.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComIdentityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnIISRequestInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IComIdentityEvents_Impl: windows_core::IUnknownImpl {
    fn OnIISRequestInfo(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: &windows_core::PCWSTR, pszserverip: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IComIdentityEvents_Vtbl {
    pub const fn new<Identity: IComIdentityEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnIISRequestInfo<Identity: IComIdentityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: windows_core::PCWSTR, pszserverip: windows_core::PCWSTR, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComIdentityEvents_Impl::OnIISRequestInfo(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&pszclientip), core::mem::transmute(&pszserverip), core::mem::transmute(&pszurl)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComIdentityEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComIdentityEvents {}
windows_core::imp::define_interface!(IComInstance2Events, IComInstance2Events_Vtbl, 0x20e3bf07_b506_4ad5_a50c_d2ca5b9c158e);
windows_core::imp::interface_hierarchy!(IComInstance2Events, windows_core::IUnknown);
impl IComInstance2Events {
    pub unsafe fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectCreate2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, clsid, tsid, ctxtid, objectid, guidpartition).ok() }
    }
    pub unsafe fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectDestroy2)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstance2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectCreate2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, u64, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnObjectDestroy2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
pub trait IComInstance2Events_Impl: windows_core::IUnknownImpl {
    fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl IComInstance2Events_Vtbl {
    pub const fn new<Identity: IComInstance2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjectCreate2<Identity: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComInstance2Events_Impl::OnObjectCreate2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidpartition)).into()
            }
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComInstance2Events_Impl::OnObjectDestroy2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Identity, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComInstance2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComInstance2Events {}
windows_core::imp::define_interface!(IComInstanceEvents, IComInstanceEvents_Vtbl, 0x683130a7_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComInstanceEvents, windows_core::IUnknown);
impl IComInstanceEvents {
    pub unsafe fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectCreate)(windows_core::Interface::as_raw(self), pinfo, guidactivity, clsid, tsid, ctxtid, objectid).ok() }
    }
    pub unsafe fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectDestroy)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComInstanceEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, u64) -> windows_core::HRESULT,
    pub OnObjectDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
pub trait IComInstanceEvents_Impl: windows_core::IUnknownImpl {
    fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl IComInstanceEvents_Vtbl {
    pub const fn new<Identity: IComInstanceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjectCreate<Identity: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComInstanceEvents_Impl::OnObjectCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
            }
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComInstanceEvents_Impl::OnObjectDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Identity, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComInstanceEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComInstanceEvents {}
windows_core::imp::define_interface!(IComLTxEvents, IComLTxEvents_Vtbl, 0x605cf82c_578e_4298_975d_82babcd9e053);
windows_core::imp::interface_hierarchy!(IComLTxEvents, windows_core::IUnknown);
impl IComLTxEvents {
    pub unsafe fn OnLtxTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, tsid: windows_core::GUID, froot: bool, nisolationlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLtxTransactionStart)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), core::mem::transmute(tsid), froot.into(), nisolationlevel).ok() }
    }
    pub unsafe fn OnLtxTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, fvote: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLtxTransactionPrepare)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), fvote.into()).ok() }
    }
    pub unsafe fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLtxTransactionAbort)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx)).ok() }
    }
    pub unsafe fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLtxTransactionCommit)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx)).ok() }
    }
    pub unsafe fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, txnid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLtxTransactionPromote)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(guidltx), core::mem::transmute(txnid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComLTxEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLtxTransactionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID, windows_core::BOOL, i32) -> windows_core::HRESULT,
    pub OnLtxTransactionPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnLtxTransactionAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnLtxTransactionCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID) -> windows_core::HRESULT,
    pub OnLtxTransactionPromote: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::GUID, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComLTxEvents_Impl: windows_core::IUnknownImpl {
    fn OnLtxTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, tsid: &windows_core::GUID, froot: windows_core::BOOL, nisolationlevel: i32) -> windows_core::Result<()>;
    fn OnLtxTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, fvote: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, txnid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IComLTxEvents_Vtbl {
    pub const fn new<Identity: IComLTxEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, tsid: windows_core::GUID, froot: windows_core::BOOL, nisolationlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComLTxEvents_Impl::OnLtxTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
            }
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, fvote: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComLTxEvents_Impl::OnLtxTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute_copy(&fvote)).into()
            }
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComLTxEvents_Impl::OnLtxTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
            }
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComLTxEvents_Impl::OnLtxTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
            }
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, txnid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComLTxEvents_Impl::OnLtxTransactionPromote(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&txnid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComLTxEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComLTxEvents {}
windows_core::imp::define_interface!(IComMethod2Events, IComMethod2Events_Vtbl, 0xfb388aaa_567d_4024_af8e_6e93ee748573);
windows_core::imp::interface_hierarchy!(IComMethod2Events, windows_core::IUnknown);
impl IComMethod2Events {
    pub unsafe fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodCall2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth).ok() }
    }
    pub unsafe fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodReturn2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth, hresult).ok() }
    }
    pub unsafe fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodException2)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, dwthread, imeth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethod2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMethodCall2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub OnMethodReturn2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnMethodException2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
}
pub trait IComMethod2Events_Impl: windows_core::IUnknownImpl {
    fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()>;
    fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()>;
}
impl IComMethod2Events_Vtbl {
    pub const fn new<Identity: IComMethod2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMethodCall2<Identity: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethod2Events_Impl::OnMethodCall2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
            }
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethod2Events_Impl::OnMethodReturn2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn OnMethodException2<Identity: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethod2Events_Impl::OnMethodException2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Identity, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMethod2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComMethod2Events {}
windows_core::imp::define_interface!(IComMethodEvents, IComMethodEvents_Vtbl, 0x683130a9_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComMethodEvents, windows_core::IUnknown);
impl IComMethodEvents {
    pub unsafe fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodCall)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth).ok() }
    }
    pub unsafe fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodReturn)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth, hresult).ok() }
    }
    pub unsafe fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMethodException)(windows_core::Interface::as_raw(self), pinfo, oid, guidcid, guidrid, imeth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMethodEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMethodCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnMethodReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnMethodException: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IComMethodEvents_Impl: windows_core::IUnknownImpl {
    fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()>;
    fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()>;
}
impl IComMethodEvents_Vtbl {
    pub const fn new<Identity: IComMethodEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMethodCall<Identity: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethodEvents_Impl::OnMethodCall(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
            }
        }
        unsafe extern "system" fn OnMethodReturn<Identity: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethodEvents_Impl::OnMethodReturn(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn OnMethodException<Identity: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMethodEvents_Impl::OnMethodException(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall: OnMethodCall::<Identity, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, OFFSET>,
            OnMethodException: OnMethodException::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMethodEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComMethodEvents {}
windows_core::imp::define_interface!(IComMtaThreadPoolKnobs, IComMtaThreadPoolKnobs_Vtbl, 0xf9a76d2e_76a5_43eb_a0c4_49bec8e48480);
windows_core::imp::interface_hierarchy!(IComMtaThreadPoolKnobs, windows_core::IUnknown);
impl IComMtaThreadPoolKnobs {
    pub unsafe fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MTASetMaxThreadCount)(windows_core::Interface::as_raw(self), dwmaxthreads).ok() }
    }
    pub unsafe fn MTAGetMaxThreadCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MTAGetMaxThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MTASetThrottleValue(&self, dwthrottle: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MTASetThrottleValue)(windows_core::Interface::as_raw(self), dwthrottle).ok() }
    }
    pub unsafe fn MTAGetThrottleValue(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MTAGetThrottleValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComMtaThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MTASetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MTAGetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MTASetThrottleValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MTAGetThrottleValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IComMtaThreadPoolKnobs_Impl: windows_core::IUnknownImpl {
    fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> windows_core::Result<()>;
    fn MTAGetMaxThreadCount(&self) -> windows_core::Result<u32>;
    fn MTASetThrottleValue(&self, dwthrottle: u32) -> windows_core::Result<()>;
    fn MTAGetThrottleValue(&self) -> windows_core::Result<u32>;
}
impl IComMtaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMtaThreadPoolKnobs_Impl::MTASetMaxThreadCount(this, core::mem::transmute_copy(&dwmaxthreads)).into()
            }
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComMtaThreadPoolKnobs_Impl::MTAGetMaxThreadCount(this) {
                    Ok(ok__) => {
                        pdwmaxthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthrottle: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComMtaThreadPoolKnobs_Impl::MTASetThrottleValue(this, core::mem::transmute_copy(&dwthrottle)).into()
            }
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthrottle: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComMtaThreadPoolKnobs_Impl::MTAGetThrottleValue(this) {
                    Ok(ok__) => {
                        pdwthrottle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComMtaThreadPoolKnobs {}
windows_core::imp::define_interface!(IComObjectConstruction2Events, IComObjectConstruction2Events_Vtbl, 0x4b5a7827_8df2_45c0_8f6f_57ea1f856a9f);
windows_core::imp::interface_hierarchy!(IComObjectConstruction2Events, windows_core::IUnknown);
impl IComObjectConstruction2Events {
    pub unsafe fn OnObjectConstruct2<P2>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: P2, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnObjectConstruct2)(windows_core::Interface::as_raw(self), pinfo, guidobject, sconstructstring.param().abi(), oid, guidpartition).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstruction2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectConstruct2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::PCWSTR, u64, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComObjectConstruction2Events_Impl: windows_core::IUnknownImpl {
    fn OnObjectConstruct2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: &windows_core::PCWSTR, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComObjectConstruction2Events_Vtbl {
    pub const fn new<Identity: IComObjectConstruction2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjectConstruct2<Identity: IComObjectConstruction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectConstruction2Events_Impl::OnObjectConstruct2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectConstruction2Events {}
windows_core::imp::define_interface!(IComObjectConstructionEvents, IComObjectConstructionEvents_Vtbl, 0x683130af_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComObjectConstructionEvents, windows_core::IUnknown);
impl IComObjectConstructionEvents {
    pub unsafe fn OnObjectConstruct<P2>(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: P2, oid: u64) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnObjectConstruct)(windows_core::Interface::as_raw(self), pinfo, guidobject, sconstructstring.param().abi(), oid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectConstructionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectConstruct: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
}
pub trait IComObjectConstructionEvents_Impl: windows_core::IUnknownImpl {
    fn OnObjectConstruct(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: &windows_core::PCWSTR, oid: u64) -> windows_core::Result<()>;
}
impl IComObjectConstructionEvents_Vtbl {
    pub const fn new<Identity: IComObjectConstructionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjectConstruct<Identity: IComObjectConstructionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectConstructionEvents_Impl::OnObjectConstruct(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectConstructionEvents {}
windows_core::imp::define_interface!(IComObjectEvents, IComObjectEvents_Vtbl, 0x683130aa_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComObjectEvents, windows_core::IUnknown);
impl IComObjectEvents {
    pub unsafe fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectActivate)(windows_core::Interface::as_raw(self), pinfo, ctxtid, objectid).ok() }
    }
    pub unsafe fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjectDeactivate)(windows_core::Interface::as_raw(self), pinfo, ctxtid, objectid).ok() }
    }
    pub unsafe fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDisableCommit)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
    pub unsafe fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEnableCommit)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
    pub unsafe fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSetComplete)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
    pub unsafe fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSetAbort)(windows_core::Interface::as_raw(self), pinfo, ctxtid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjectActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnObjectDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnDisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnEnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnSetComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
    pub OnSetAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
pub trait IComObjectEvents_Impl: windows_core::IUnknownImpl {
    fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl IComObjectEvents_Vtbl {
    pub const fn new<Identity: IComObjectEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjectActivate<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnObjectActivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
            }
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnObjectDeactivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
            }
        }
        unsafe extern "system" fn OnDisableCommit<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnDisableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        unsafe extern "system" fn OnEnableCommit<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnEnableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        unsafe extern "system" fn OnSetComplete<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnSetComplete(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        unsafe extern "system" fn OnSetAbort<Identity: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectEvents_Impl::OnSetAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Identity, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectEvents {}
windows_core::imp::define_interface!(IComObjectPool2Events, IComObjectPool2Events_Vtbl, 0x65bf6534_85ea_4f64_8cf4_3d974b2ab1cf);
windows_core::imp::interface_hierarchy!(IComObjectPool2Events, windows_core::IUnknown);
impl IComObjectPool2Events {
    pub unsafe fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolPutObject2)(windows_core::Interface::as_raw(self), pinfo, guidobject, nreason, dwavailable, oid).ok() }
    }
    pub unsafe fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolGetObject2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, dwavailable, oid, guidpartition).ok() }
    }
    pub unsafe fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolRecycleToTx2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok() }
    }
    pub unsafe fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolGetFromTx2)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid, guidpartition).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPool2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolPutObject2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, i32, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetObject2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u64, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnObjPoolRecycleToTx2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetFromTx2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComObjectPool2Events_Impl: windows_core::IUnknownImpl {
    fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComObjectPool2Events_Vtbl {
    pub const fn new<Identity: IComObjectPool2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPool2Events_Impl::OnObjPoolPutObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPool2Events_Impl::OnObjPoolGetObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPool2Events_Impl::OnObjPoolRecycleToTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPool2Events_Impl::OnObjPoolGetFromTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidpartition)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPool2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectPool2Events {}
windows_core::imp::define_interface!(IComObjectPoolEvents, IComObjectPoolEvents_Vtbl, 0x683130ad_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComObjectPoolEvents, windows_core::IUnknown);
impl IComObjectPoolEvents {
    pub unsafe fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolPutObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, nreason, dwavailable, oid).ok() }
    }
    pub unsafe fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolGetObject)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, dwavailable, oid).ok() }
    }
    pub unsafe fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolRecycleToTx)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok() }
    }
    pub unsafe fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolGetFromTx)(windows_core::Interface::as_raw(self), pinfo, guidactivity, guidobject, guidtx, objid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolPutObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, i32, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolRecycleToTx: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnObjPoolGetFromTx: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, *const windows_core::GUID, u64) -> windows_core::HRESULT,
}
pub trait IComObjectPoolEvents_Impl: windows_core::IUnknownImpl {
    fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
}
impl IComObjectPoolEvents_Vtbl {
    pub const fn new<Identity: IComObjectPoolEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents_Impl::OnObjPoolPutObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents_Impl::OnObjPoolGetObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents_Impl::OnObjPoolRecycleToTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents_Impl::OnObjPoolGetFromTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectPoolEvents {}
windows_core::imp::define_interface!(IComObjectPoolEvents2, IComObjectPoolEvents2_Vtbl, 0x683130ae_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComObjectPoolEvents2, windows_core::IUnknown);
impl IComObjectPoolEvents2 {
    pub unsafe fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolCreateObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwobjscreated, oid).ok() }
    }
    pub unsafe fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolDestroyObject)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwobjscreated, oid).ok() }
    }
    pub unsafe fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolCreateDecision)(windows_core::Interface::as_raw(self), pinfo, dwthreadswaiting, dwavail, dwcreated, dwmin, dwmax).ok() }
    }
    pub unsafe fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolTimeout)(windows_core::Interface::as_raw(self), pinfo, guidobject, guidactivity, dwtimeout).ok() }
    }
    pub unsafe fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnObjPoolCreatePool)(windows_core::Interface::as_raw(self), pinfo, guidobject, dwmin, dwmax, dwtimeout).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComObjectPoolEvents2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnObjPoolCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolDestroyObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u64) -> windows_core::HRESULT,
    pub OnObjPoolCreateDecision: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u32, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub OnObjPoolTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnObjPoolCreatePool: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u32, u32, u32) -> windows_core::HRESULT,
}
pub trait IComObjectPoolEvents2_Impl: windows_core::IUnknownImpl {
    fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::Result<()>;
    fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::Result<()>;
    fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::Result<()>;
}
impl IComObjectPoolEvents2_Vtbl {
    pub const fn new<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents2_Impl::OnObjPoolCreateObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents2_Impl::OnObjPoolDestroyObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents2_Impl::OnObjPoolCreateDecision(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&dwthreadswaiting), core::mem::transmute_copy(&dwavail), core::mem::transmute_copy(&dwcreated), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents2_Impl::OnObjPoolTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComObjectPoolEvents2_Impl::OnObjPoolCreatePool(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax), core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComObjectPoolEvents2 {}
windows_core::imp::define_interface!(IComQCEvents, IComQCEvents_Vtbl, 0x683130b2_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComQCEvents, windows_core::IUnknown);
impl IComQCEvents {
    pub unsafe fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &[u16; 60], guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCRecord)(windows_core::Interface::as_raw(self), pinfo, objid, core::mem::transmute(szqueue.as_ptr()), guidmsgid, guidworkflowid, msmqhr).ok() }
    }
    pub unsafe fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &[u16; 60], queueid: u64, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCQueueOpen)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(szqueue.as_ptr()), queueid, hr).ok() }
    }
    pub unsafe fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCReceive)(windows_core::Interface::as_raw(self), pinfo, queueid, guidmsgid, guidworkflowid, hr).ok() }
    }
    pub unsafe fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCReceiveFail)(windows_core::Interface::as_raw(self), pinfo, queueid, msmqhr).ok() }
    }
    pub unsafe fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCMoveToReTryQueue)(windows_core::Interface::as_raw(self), pinfo, guidmsgid, guidworkflowid, retryindex).ok() }
    }
    pub unsafe fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCMoveToDeadQueue)(windows_core::Interface::as_raw(self), pinfo, guidmsgid, guidworkflowid).ok() }
    }
    pub unsafe fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnQCPlayback)(windows_core::Interface::as_raw(self), pinfo, objid, guidmsgid, guidworkflowid, hr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComQCEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnQCRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCQueueOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, windows_core::PCWSTR, u64, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCReceiveFail: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnQCMoveToReTryQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub OnQCMoveToDeadQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnQCPlayback: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IComQCEvents_Impl: windows_core::IUnknownImpl {
    fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &windows_core::PCWSTR, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &windows_core::PCWSTR, queueid: u64, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::Result<()>;
    fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IComQCEvents_Vtbl {
    pub const fn new<Identity: IComQCEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnQCRecord<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: windows_core::PCWSTR, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCRecord(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&szqueue), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&msmqhr)).into()
            }
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: windows_core::PCWSTR, queueid: u64, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCQueueOpen(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&szqueue), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn OnQCReceive<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCReceive(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCReceiveFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&msmqhr)).into()
            }
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCMoveToReTryQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&retryindex)).into()
            }
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCMoveToDeadQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid)).into()
            }
        }
        unsafe extern "system" fn OnQCPlayback<Identity: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComQCEvents_Impl::OnQCPlayback(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQCRecord: OnQCRecord::<Identity, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComQCEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComQCEvents {}
windows_core::imp::define_interface!(IComResourceEvents, IComResourceEvents_Vtbl, 0x683130ab_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComResourceEvents, windows_core::IUnknown);
impl IComResourceEvents {
    pub unsafe fn OnResourceCreate<P2>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P2, resid: u64, enlisted: bool) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResourceCreate)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.into()).ok() }
    }
    pub unsafe fn OnResourceAllocate<P2>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P2, resid: u64, enlisted: bool, numrated: u32, rating: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResourceAllocate)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.into(), numrated, rating).ok() }
    }
    pub unsafe fn OnResourceRecycle<P2>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P2, resid: u64) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResourceRecycle)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid).ok() }
    }
    pub unsafe fn OnResourceDestroy<P3>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: P3, resid: u64) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResourceDestroy)(windows_core::Interface::as_raw(self), pinfo, objectid, hr, psztype.param().abi(), resid).ok() }
    }
    pub unsafe fn OnResourceTrack<P2>(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: P2, resid: u64, enlisted: bool) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResourceTrack)(windows_core::Interface::as_raw(self), pinfo, objectid, psztype.param().abi(), resid, enlisted.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComResourceEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnResourceCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnResourceAllocate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, windows_core::BOOL, u32, u32) -> windows_core::HRESULT,
    pub OnResourceRecycle: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub OnResourceDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::HRESULT, windows_core::PCWSTR, u64) -> windows_core::HRESULT,
    pub OnResourceTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, windows_core::PCWSTR, u64, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IComResourceEvents_Impl: windows_core::IUnknownImpl {
    fn OnResourceCreate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnResourceAllocate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL, numrated: u32, rating: u32) -> windows_core::Result<()>;
    fn OnResourceRecycle(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64) -> windows_core::Result<()>;
    fn OnResourceDestroy(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: &windows_core::PCWSTR, resid: u64) -> windows_core::Result<()>;
    fn OnResourceTrack(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IComResourceEvents_Vtbl {
    pub const fn new<Identity: IComResourceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnResourceCreate<Identity: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComResourceEvents_Impl::OnResourceCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
            }
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL, numrated: u32, rating: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComResourceEvents_Impl::OnResourceAllocate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted), core::mem::transmute_copy(&numrated), core::mem::transmute_copy(&rating)).into()
            }
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComResourceEvents_Impl::OnResourceRecycle(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
            }
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComResourceEvents_Impl::OnResourceDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&hr), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
            }
        }
        unsafe extern "system" fn OnResourceTrack<Identity: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComResourceEvents_Impl::OnResourceTrack(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Identity, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComResourceEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComResourceEvents {}
windows_core::imp::define_interface!(IComSecurityEvents, IComSecurityEvents_Vtbl, 0x683130ac_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComSecurityEvents, windows_core::IUnknown);
impl IComSecurityEvents {
    pub unsafe fn OnAuthenticate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAuthenticate)(windows_core::Interface::as_raw(self), pinfo, guidactivity, objectid, guidiid, imeth, psidoriginaluser.len().try_into().unwrap(), core::mem::transmute(psidoriginaluser.as_ptr()), psidcurrentuser.len().try_into().unwrap(), core::mem::transmute(psidcurrentuser.as_ptr()), bcurrentuserinpersonatinginproc.into()).ok() }
    }
    pub unsafe fn OnAuthenticateFail(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, psidoriginaluser: &[u8], psidcurrentuser: &[u8], bcurrentuserinpersonatinginproc: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAuthenticateFail)(windows_core::Interface::as_raw(self), pinfo, guidactivity, objectid, guidiid, imeth, psidoriginaluser.len().try_into().unwrap(), core::mem::transmute(psidoriginaluser.as_ptr()), psidcurrentuser.len().try_into().unwrap(), core::mem::transmute(psidcurrentuser.as_ptr()), bcurrentuserinpersonatinginproc.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComSecurityEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64, *const windows_core::GUID, u32, u32, *const u8, u32, *const u8, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnAuthenticateFail: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64, *const windows_core::GUID, u32, u32, *const u8, u32, *const u8, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IComSecurityEvents_Impl: windows_core::IUnknownImpl {
    fn OnAuthenticate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnAuthenticateFail(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IComSecurityEvents_Vtbl {
    pub const fn new<Identity: IComSecurityEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAuthenticate<Identity: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComSecurityEvents_Impl::OnAuthenticate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
            }
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComSecurityEvents_Impl::OnAuthenticateFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Identity, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComSecurityEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComSecurityEvents {}
windows_core::imp::define_interface!(IComStaThreadPoolKnobs, IComStaThreadPoolKnobs_Vtbl, 0x324b64fa_33b6_11d2_98b7_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComStaThreadPoolKnobs, windows_core::IUnknown);
impl IComStaThreadPoolKnobs {
    pub unsafe fn SetMinThreadCount(&self, minthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinThreadCount)(windows_core::Interface::as_raw(self), minthreads).ok() }
    }
    pub unsafe fn GetMinThreadCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxThreadCount(&self, maxthreads: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxThreadCount)(windows_core::Interface::as_raw(self), maxthreads).ok() }
    }
    pub unsafe fn GetMaxThreadCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActivityPerThread(&self, activitiesperthread: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActivityPerThread)(windows_core::Interface::as_raw(self), activitiesperthread).ok() }
    }
    pub unsafe fn GetActivityPerThread(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityPerThread)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActivityRatio(&self, activityratio: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActivityRatio)(windows_core::Interface::as_raw(self), activityratio).ok() }
    }
    pub unsafe fn GetActivityRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityRatio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetThreadCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThreadCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetQueueDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQueueDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQueueDepth(&self, dwqdepth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQueueDepth)(windows_core::Interface::as_raw(self), dwqdepth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMinThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMinThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetActivityPerThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetActivityPerThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetActivityRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetActivityRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetThreadCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IComStaThreadPoolKnobs_Impl: windows_core::IUnknownImpl {
    fn SetMinThreadCount(&self, minthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreadCount(&self) -> windows_core::Result<u32>;
    fn SetMaxThreadCount(&self, maxthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreadCount(&self) -> windows_core::Result<u32>;
    fn SetActivityPerThread(&self, activitiesperthread: u32) -> windows_core::Result<()>;
    fn GetActivityPerThread(&self) -> windows_core::Result<u32>;
    fn SetActivityRatio(&self, activityratio: f64) -> windows_core::Result<()>;
    fn GetActivityRatio(&self) -> windows_core::Result<f64>;
    fn GetThreadCount(&self) -> windows_core::Result<u32>;
    fn GetQueueDepth(&self) -> windows_core::Result<u32>;
    fn SetQueueDepth(&self, dwqdepth: i32) -> windows_core::Result<()>;
}
impl IComStaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMinThreadCount<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs_Impl::SetMinThreadCount(this, core::mem::transmute_copy(&minthreads)).into()
            }
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetMinThreadCount(this) {
                    Ok(ok__) => {
                        minthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs_Impl::SetMaxThreadCount(this, core::mem::transmute_copy(&maxthreads)).into()
            }
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetMaxThreadCount(this) {
                    Ok(ok__) => {
                        maxthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs_Impl::SetActivityPerThread(this, core::mem::transmute_copy(&activitiesperthread)).into()
            }
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetActivityPerThread(this) {
                    Ok(ok__) => {
                        activitiesperthread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActivityRatio<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs_Impl::SetActivityRatio(this, core::mem::transmute_copy(&activityratio)).into()
            }
        }
        unsafe extern "system" fn GetActivityRatio<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetActivityRatio(this) {
                    Ok(ok__) => {
                        activityratio.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadCount<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreads: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetThreadCount(this) {
                    Ok(ok__) => {
                        pdwthreads.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQueueDepth<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwqdepth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs_Impl::GetQueueDepth(this) {
                    Ok(ok__) => {
                        pdwqdepth.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqdepth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&dwqdepth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMinThreadCount: SetMinThreadCount::<Identity, OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Identity, OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Identity, OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Identity, OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Identity, OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Identity, OFFSET>,
            SetActivityRatio: SetActivityRatio::<Identity, OFFSET>,
            GetActivityRatio: GetActivityRatio::<Identity, OFFSET>,
            GetThreadCount: GetThreadCount::<Identity, OFFSET>,
            GetQueueDepth: GetQueueDepth::<Identity, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComStaThreadPoolKnobs {}
windows_core::imp::define_interface!(IComStaThreadPoolKnobs2, IComStaThreadPoolKnobs2_Vtbl, 0x73707523_ff9a_4974_bf84_2108dc213740);
impl core::ops::Deref for IComStaThreadPoolKnobs2 {
    type Target = IComStaThreadPoolKnobs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComStaThreadPoolKnobs2, windows_core::IUnknown, IComStaThreadPoolKnobs);
impl IComStaThreadPoolKnobs2 {
    pub unsafe fn GetMaxCPULoad(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCPULoad)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxCPULoad(&self, pdwload: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxCPULoad)(windows_core::Interface::as_raw(self), pdwload).ok() }
    }
    pub unsafe fn GetCPUMetricEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCPUMetricEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCPUMetricEnabled(&self, bmetricenabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCPUMetricEnabled)(windows_core::Interface::as_raw(self), bmetricenabled.into()).ok() }
    }
    pub unsafe fn GetCreateThreadsAggressively(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreateThreadsAggressively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreateThreadsAggressively(&self, bmetricenabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreateThreadsAggressively)(windows_core::Interface::as_raw(self), bmetricenabled.into()).ok() }
    }
    pub unsafe fn GetMaxCSR(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxCSR)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxCSR(&self, dwcsr: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxCSR)(windows_core::Interface::as_raw(self), dwcsr).ok() }
    }
    pub unsafe fn GetWaitTimeForThreadCleanup(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWaitTimeForThreadCleanup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWaitTimeForThreadCleanup)(windows_core::Interface::as_raw(self), dwthreadcleanupwaittime).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComStaThreadPoolKnobs2_Vtbl {
    pub base__: IComStaThreadPoolKnobs_Vtbl,
    pub GetMaxCPULoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxCPULoad: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetCPUMetricEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCPUMetricEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCreateThreadsAggressively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCreateThreadsAggressively: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetMaxCSR: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxCSR: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetWaitTimeForThreadCleanup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetWaitTimeForThreadCleanup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IComStaThreadPoolKnobs2_Impl: IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(&self) -> windows_core::Result<u32>;
    fn SetMaxCPULoad(&self, pdwload: i32) -> windows_core::Result<()>;
    fn GetCPUMetricEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetCPUMetricEnabled(&self, bmetricenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetCreateThreadsAggressively(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetCreateThreadsAggressively(&self, bmetricenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMaxCSR(&self) -> windows_core::Result<u32>;
    fn SetMaxCSR(&self, dwcsr: i32) -> windows_core::Result<()>;
    fn GetWaitTimeForThreadCleanup(&self) -> windows_core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> windows_core::Result<()>;
}
impl IComStaThreadPoolKnobs2_Vtbl {
    pub const fn new<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxCPULoad<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs2_Impl::GetMaxCPULoad(this) {
                    Ok(ok__) => {
                        pdwload.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs2_Impl::SetMaxCPULoad(this, core::mem::transmute_copy(&pdwload)).into()
            }
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs2_Impl::GetCPUMetricEnabled(this) {
                    Ok(ok__) => {
                        pbmetricenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs2_Impl::SetCPUMetricEnabled(this, core::mem::transmute_copy(&bmetricenabled)).into()
            }
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs2_Impl::GetCreateThreadsAggressively(this) {
                    Ok(ok__) => {
                        pbmetricenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs2_Impl::SetCreateThreadsAggressively(this, core::mem::transmute_copy(&bmetricenabled)).into()
            }
        }
        unsafe extern "system" fn GetMaxCSR<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcsr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs2_Impl::GetMaxCSR(this) {
                    Ok(ok__) => {
                        pdwcsr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxCSR<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcsr: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs2_Impl::SetMaxCSR(this, core::mem::transmute_copy(&dwcsr)).into()
            }
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComStaThreadPoolKnobs2_Impl::GetWaitTimeForThreadCleanup(this) {
                    Ok(ok__) => {
                        pdwthreadcleanupwaittime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadcleanupwaittime: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComStaThreadPoolKnobs2_Impl::SetWaitTimeForThreadCleanup(this, core::mem::transmute_copy(&dwthreadcleanupwaittime)).into()
            }
        }
        Self {
            base__: IComStaThreadPoolKnobs_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCPULoad: GetMaxCPULoad::<Identity, OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Identity, OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Identity, OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Identity, OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Identity, OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Identity, OFFSET>,
            GetMaxCSR: GetMaxCSR::<Identity, OFFSET>,
            SetMaxCSR: SetMaxCSR::<Identity, OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Identity, OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as windows_core::Interface>::IID || iid == &<IComStaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComStaThreadPoolKnobs2 {}
windows_core::imp::define_interface!(IComThreadEvents, IComThreadEvents_Vtbl, 0x683130a5_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComThreadEvents, windows_core::IUnknown);
impl IComThreadEvents {
    pub unsafe fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadStart)(windows_core::Interface::as_raw(self), pinfo, threadid, dwthread, dwtheadcnt).ok() }
    }
    pub unsafe fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadTerminate)(windows_core::Interface::as_raw(self), pinfo, threadid, dwthread, dwtheadcnt).ok() }
    }
    pub unsafe fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadBindToApartment)(windows_core::Interface::as_raw(self), pinfo, threadid, aptid, dwactcnt, dwlowcnt).ok() }
    }
    pub unsafe fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadUnBind)(windows_core::Interface::as_raw(self), pinfo, threadid, aptid, dwactcnt).ok() }
    }
    pub unsafe fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadWorkEnque)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok() }
    }
    pub unsafe fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadWorkPrivate)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid).ok() }
    }
    pub unsafe fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadWorkPublic)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok() }
    }
    pub unsafe fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadWorkRedirect)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen, threadnum).ok() }
    }
    pub unsafe fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadWorkReject)(windows_core::Interface::as_raw(self), pinfo, threadid, msgworkid, queuelen).ok() }
    }
    pub unsafe fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadAssignApartment)(windows_core::Interface::as_raw(self), pinfo, guidactivity, aptid).ok() }
    }
    pub unsafe fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadUnassignApartment)(windows_core::Interface::as_raw(self), pinfo, aptid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnThreadStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadBindToApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32, u32) -> windows_core::HRESULT,
    pub OnThreadUnBind: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkEnque: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkPrivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64) -> windows_core::HRESULT,
    pub OnThreadWorkPublic: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadWorkRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32, u64) -> windows_core::HRESULT,
    pub OnThreadWorkReject: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64, u64, u32) -> windows_core::HRESULT,
    pub OnThreadAssignApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub OnThreadUnassignApartment: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, u64) -> windows_core::HRESULT,
}
pub trait IComThreadEvents_Impl: windows_core::IUnknownImpl {
    fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()>;
    fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()>;
    fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::Result<()>;
    fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::Result<()>;
    fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::Result<()>;
    fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::Result<()>;
    fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::Result<()>;
    fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::Result<()>;
}
impl IComThreadEvents_Vtbl {
    pub const fn new<Identity: IComThreadEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnThreadStart<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
            }
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
            }
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadBindToApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt), core::mem::transmute_copy(&dwlowcnt)).into()
            }
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadUnBind(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt)).into()
            }
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadWorkEnque(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
            }
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadWorkPrivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid)).into()
            }
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadWorkPublic(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
            }
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadWorkRedirect(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen), core::mem::transmute_copy(&threadnum)).into()
            }
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadWorkReject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
            }
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadAssignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&aptid)).into()
            }
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadEvents_Impl::OnThreadUnassignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&aptid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnThreadStart: OnThreadStart::<Identity, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Identity, OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Identity, OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Identity, OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Identity, OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Identity, OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Identity, OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Identity, OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Identity, OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComThreadEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComThreadEvents {}
windows_core::imp::define_interface!(IComTrackingInfoCollection, IComTrackingInfoCollection_Vtbl, 0xc266c677_c9ad_49ab_9fd9_d9661078588a);
windows_core::imp::interface_hierarchy!(IComTrackingInfoCollection, windows_core::IUnknown);
impl IComTrackingInfoCollection {
    pub unsafe fn Type(&self) -> windows_core::Result<TRACKING_COLL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ulindex, riid, ppv as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TRACKING_COLL_TYPE) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IComTrackingInfoCollection_Impl: windows_core::IUnknownImpl {
    fn Type(&self) -> windows_core::Result<TRACKING_COLL_TYPE>;
    fn Count(&self) -> windows_core::Result<u32>;
    fn Item(&self, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IComTrackingInfoCollection_Vtbl {
    pub const fn new<Identity: IComTrackingInfoCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComTrackingInfoCollection_Impl::Type(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComTrackingInfoCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTrackingInfoCollection_Impl::Item(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComTrackingInfoCollection {}
windows_core::imp::define_interface!(IComTrackingInfoEvents, IComTrackingInfoEvents_Vtbl, 0x4e6cdcc9_fb25_4fd5_9cc5_c9f4b6559cec);
windows_core::imp::interface_hierarchy!(IComTrackingInfoEvents, windows_core::IUnknown);
impl IComTrackingInfoEvents {
    pub unsafe fn OnNewTrackingInfo<P0>(&self, ptoplevelcollection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNewTrackingInfo)(windows_core::Interface::as_raw(self), ptoplevelcollection.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewTrackingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IComTrackingInfoEvents_Impl: windows_core::IUnknownImpl {
    fn OnNewTrackingInfo(&self, ptoplevelcollection: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IComTrackingInfoEvents_Vtbl {
    pub const fn new<Identity: IComTrackingInfoEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: IComTrackingInfoEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoplevelcollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTrackingInfoEvents_Impl::OnNewTrackingInfo(this, core::mem::transmute_copy(&ptoplevelcollection)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComTrackingInfoEvents {}
windows_core::imp::define_interface!(IComTrackingInfoObject, IComTrackingInfoObject_Vtbl, 0x116e42c5_d8b1_47bf_ab1e_c895ed3e2372);
windows_core::imp::interface_hierarchy!(IComTrackingInfoObject, windows_core::IUnknown);
impl IComTrackingInfoObject {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue<P0>(&self, szpropertyname: P0) -> windows_core::Result<super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), szpropertyname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComTrackingInfoObject_Impl: windows_core::IUnknownImpl {
    fn GetValue(&self, szpropertyname: &windows_core::PCWSTR) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComTrackingInfoObject_Vtbl {
    pub const fn new<Identity: IComTrackingInfoObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetValue<Identity: IComTrackingInfoObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szpropertyname: windows_core::PCWSTR, pvarout: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComTrackingInfoObject_Impl::GetValue(this, core::mem::transmute(&szpropertyname)) {
                    Ok(ok__) => {
                        pvarout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IComTrackingInfoObject {}
windows_core::imp::define_interface!(IComTrackingInfoProperties, IComTrackingInfoProperties_Vtbl, 0x789b42be_6f6b_443a_898e_67abf390aa14);
windows_core::imp::interface_hierarchy!(IComTrackingInfoProperties, windows_core::IUnknown);
impl IComTrackingInfoProperties {
    pub unsafe fn PropCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropName(&self, ulindex: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropName)(windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTrackingInfoProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PropCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IComTrackingInfoProperties_Impl: windows_core::IUnknownImpl {
    fn PropCount(&self) -> windows_core::Result<u32>;
    fn GetPropName(&self, ulindex: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl IComTrackingInfoProperties_Vtbl {
    pub const fn new<Identity: IComTrackingInfoProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropCount<Identity: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComTrackingInfoProperties_Impl::PropCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropName<Identity: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppszpropname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComTrackingInfoProperties_Impl::GetPropName(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppszpropname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropCount: PropCount::<Identity, OFFSET>,
            GetPropName: GetPropName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComTrackingInfoProperties {}
windows_core::imp::define_interface!(IComTransaction2Events, IComTransaction2Events_Vtbl, 0xa136f62a_2f94_4288_86e0_d8a1fa4c0299);
windows_core::imp::interface_hierarchy!(IComTransaction2Events, windows_core::IUnknown);
impl IComTransaction2Events {
    pub unsafe fn OnTransactionStart2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: bool, nisolationlevel: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionStart2)(windows_core::Interface::as_raw(self), pinfo, guidtx, tsid, froot.into(), nisolationlevel).ok() }
    }
    pub unsafe fn OnTransactionPrepare2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionPrepare2)(windows_core::Interface::as_raw(self), pinfo, guidtx, fvoteyes.into()).ok() }
    }
    pub unsafe fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionAbort2)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok() }
    }
    pub unsafe fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionCommit2)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransaction2Events_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransactionStart2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, windows_core::BOOL, i32) -> windows_core::HRESULT,
    pub OnTransactionPrepare2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnTransactionAbort2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnTransactionCommit2: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComTransaction2Events_Impl: windows_core::IUnknownImpl {
    fn OnTransactionStart2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: windows_core::BOOL, nisolationlevel: i32) -> windows_core::Result<()>;
    fn OnTransactionPrepare2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComTransaction2Events_Vtbl {
    pub const fn new<Identity: IComTransaction2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransactionStart2<Identity: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: windows_core::BOOL, nisolationlevel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransaction2Events_Impl::OnTransactionStart2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
            }
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransaction2Events_Impl::OnTransactionPrepare2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
            }
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransaction2Events_Impl::OnTransactionAbort2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
            }
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransaction2Events_Impl::OnTransactionCommit2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Identity, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTransaction2Events as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComTransaction2Events {}
windows_core::imp::define_interface!(IComTransactionEvents, IComTransactionEvents_Vtbl, 0x683130a8_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComTransactionEvents, windows_core::IUnknown);
impl IComTransactionEvents {
    pub unsafe fn OnTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionStart)(windows_core::Interface::as_raw(self), pinfo, guidtx, tsid, froot.into()).ok() }
    }
    pub unsafe fn OnTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionPrepare)(windows_core::Interface::as_raw(self), pinfo, guidtx, fvoteyes.into()).ok() }
    }
    pub unsafe fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionAbort)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok() }
    }
    pub unsafe fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTransactionCommit)(windows_core::Interface::as_raw(self), pinfo, guidtx).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComTransactionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTransactionStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnTransactionPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID, windows_core::BOOL) -> windows_core::HRESULT,
    pub OnTransactionAbort: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnTransactionCommit: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComTransactionEvents_Impl: windows_core::IUnknownImpl {
    fn OnTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: windows_core::BOOL) -> windows_core::Result<()>;
    fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComTransactionEvents_Vtbl {
    pub const fn new<Identity: IComTransactionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTransactionStart<Identity: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransactionEvents_Impl::OnTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot)).into()
            }
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransactionEvents_Impl::OnTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
            }
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransactionEvents_Impl::OnTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
            }
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComTransactionEvents_Impl::OnTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Identity, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTransactionEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComTransactionEvents {}
windows_core::imp::define_interface!(IComUserEvent, IComUserEvent_Vtbl, 0x683130a4_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IComUserEvent, windows_core::IUnknown);
impl IComUserEvent {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUserEvent)(windows_core::Interface::as_raw(self), pinfo, core::mem::transmute(pvarevent)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComUserEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OnUserEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMSVCSEVENTINFO, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OnUserEvent: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComUserEvent_Impl: windows_core::IUnknownImpl {
    fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComUserEvent_Vtbl {
    pub const fn new<Identity: IComUserEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUserEvent<Identity: IComUserEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComUserEvent_Impl::OnUserEvent(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pvarevent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUserEvent: OnUserEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComUserEvent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IComUserEvent {}
windows_core::imp::define_interface!(IContextProperties, IContextProperties_Vtbl, 0xd396da85_bf8f_11d1_bbae_00c04fc2fa5f);
windows_core::imp::interface_hierarchy!(IContextProperties, windows_core::IUnknown);
impl IContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), plcount as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute(pproperty)).ok() }
    }
    pub unsafe fn EnumNames(&self) -> windows_core::Result<IEnumNames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, name: &windows_core::BSTR, property: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(property)).ok() }
    }
    pub unsafe fn RemoveProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
    pub RemoveProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IContextProperties_Impl: windows_core::IUnknownImpl {
    fn Count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EnumNames(&self) -> windows_core::Result<IEnumNames>;
    fn SetProperty(&self, name: &windows_core::BSTR, property: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RemoveProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IContextProperties_Vtbl {
    pub const fn new<Identity: IContextProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pproperty: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn EnumNames<Identity: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContextProperties_Impl::EnumNames(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, property: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextProperties_Impl::SetProperty(this, core::mem::transmute(&name), core::mem::transmute(&property)).into()
            }
        }
        unsafe extern "system" fn RemoveProperty<Identity: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextProperties_Impl::RemoveProperty(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumNames: EnumNames::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IContextProperties {}
windows_core::imp::define_interface!(IContextSecurityPerimeter, IContextSecurityPerimeter_Vtbl, 0xa7549a29_a7c4_42e1_8dc1_7e3d748dc24a);
windows_core::imp::interface_hierarchy!(IContextSecurityPerimeter, windows_core::IUnknown);
impl IContextSecurityPerimeter {
    pub unsafe fn GetPerimeterFlag(&self, pflag: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPerimeterFlag)(windows_core::Interface::as_raw(self), pflag as _).ok() }
    }
    pub unsafe fn SetPerimeterFlag(&self, fflag: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPerimeterFlag)(windows_core::Interface::as_raw(self), fflag.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextSecurityPerimeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPerimeterFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetPerimeterFlag: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IContextSecurityPerimeter_Impl: windows_core::IUnknownImpl {
    fn GetPerimeterFlag(&self, pflag: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetPerimeterFlag(&self, fflag: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IContextSecurityPerimeter_Vtbl {
    pub const fn new<Identity: IContextSecurityPerimeter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPerimeterFlag<Identity: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflag: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextSecurityPerimeter_Impl::GetPerimeterFlag(this, core::mem::transmute_copy(&pflag)).into()
            }
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fflag: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextSecurityPerimeter_Impl::SetPerimeterFlag(this, core::mem::transmute_copy(&fflag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Identity, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContextSecurityPerimeter {}
windows_core::imp::define_interface!(IContextState, IContextState_Vtbl, 0x3c05e54b_a42a_11d2_afc4_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IContextState, windows_core::IUnknown);
impl IContextState {
    pub unsafe fn SetDeactivateOnReturn(&self, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeactivateOnReturn)(windows_core::Interface::as_raw(self), bdeactivate).ok() }
    }
    pub unsafe fn GetDeactivateOnReturn(&self, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeactivateOnReturn)(windows_core::Interface::as_raw(self), pbdeactivate as _).ok() }
    }
    pub unsafe fn SetMyTransactionVote(&self, txvote: TransactionVote) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMyTransactionVote)(windows_core::Interface::as_raw(self), txvote).ok() }
    }
    pub unsafe fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMyTransactionVote)(windows_core::Interface::as_raw(self), ptxvote as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDeactivateOnReturn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetDeactivateOnReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMyTransactionVote: unsafe extern "system" fn(*mut core::ffi::c_void, TransactionVote) -> windows_core::HRESULT,
    pub GetMyTransactionVote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TransactionVote) -> windows_core::HRESULT,
}
pub trait IContextState_Impl: windows_core::IUnknownImpl {
    fn SetDeactivateOnReturn(&self, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetDeactivateOnReturn(&self, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetMyTransactionVote(&self, txvote: TransactionVote) -> windows_core::Result<()>;
    fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> windows_core::Result<()>;
}
impl IContextState_Vtbl {
    pub const fn new<Identity: IContextState_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextState_Impl::SetDeactivateOnReturn(this, core::mem::transmute_copy(&bdeactivate)).into()
            }
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextState_Impl::GetDeactivateOnReturn(this, core::mem::transmute_copy(&pbdeactivate)).into()
            }
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, txvote: TransactionVote) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextState_Impl::SetMyTransactionVote(this, core::mem::transmute_copy(&txvote)).into()
            }
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxvote: *mut TransactionVote) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextState_Impl::GetMyTransactionVote(this, core::mem::transmute_copy(&ptxvote)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextState as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContextState {}
windows_core::imp::define_interface!(ICreateWithLocalTransaction, ICreateWithLocalTransaction_Vtbl, 0x227ac7a8_8423_42ce_b7cf_03061ec9aaa3);
windows_core::imp::interface_hierarchy!(ICreateWithLocalTransaction, windows_core::IUnknown);
impl ICreateWithLocalTransaction {
    pub unsafe fn CreateInstanceWithSysTx<P0>(&self, ptransaction: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateInstanceWithSysTx)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), rclsid, riid, pobject as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithLocalTransaction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstanceWithSysTx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICreateWithLocalTransaction_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceWithSysTx(&self, ptransaction: windows_core::Ref<windows_core::IUnknown>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICreateWithLocalTransaction_Vtbl {
    pub const fn new<Identity: ICreateWithLocalTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: ICreateWithLocalTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateWithLocalTransaction_Impl::CreateInstanceWithSysTx(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICreateWithLocalTransaction {}
windows_core::imp::define_interface!(ICreateWithTipTransactionEx, ICreateWithTipTransactionEx_Vtbl, 0x455acf59_5345_11d2_99cf_00c04f797bc9);
windows_core::imp::interface_hierarchy!(ICreateWithTipTransactionEx, windows_core::IUnknown);
impl ICreateWithTipTransactionEx {
    pub unsafe fn CreateInstance<T>(&self, bstrtipurl: &windows_core::BSTR, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtipurl), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTipTransactionEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICreateWithTipTransactionEx_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, bstrtipurl: &windows_core::BSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICreateWithTipTransactionEx_Vtbl {
    pub const fn new<Identity: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtipurl: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateWithTipTransactionEx_Impl::CreateInstance(this, core::mem::transmute(&bstrtipurl), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICreateWithTipTransactionEx {}
windows_core::imp::define_interface!(ICreateWithTransactionEx, ICreateWithTransactionEx_Vtbl, 0x455acf57_5345_11d2_99cf_00c04f797bc9);
windows_core::imp::interface_hierarchy!(ICreateWithTransactionEx, windows_core::IUnknown);
impl ICreateWithTransactionEx {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateInstance<P0, T>(&self, ptransaction: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), ptransaction.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateWithTransactionEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateInstance: usize,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, ptransaction: windows_core::Ref<super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionEx_Vtbl {
    pub const fn new<Identity: ICreateWithTransactionEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ICreateWithTransactionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateWithTransactionEx_Impl::CreateInstance(this, core::mem::transmute_copy(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ICreateWithTransactionEx {}
windows_core::imp::define_interface!(ICrmCompensator, ICrmCompensator_Vtbl, 0xbbc01830_8d3b_11d1_82ec_00a0c91eede9);
windows_core::imp::interface_hierarchy!(ICrmCompensator, windows_core::IUnknown);
impl ICrmCompensator {
    pub unsafe fn SetLogControl<P0>(&self, plogcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICrmLogControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLogControl)(windows_core::Interface::as_raw(self), plogcontrol.param().abi()).ok() }
    }
    pub unsafe fn BeginPrepare(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPrepare)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrepareRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrepareRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndPrepare(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndPrepare)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginCommit(&self, frecovery: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginCommit)(windows_core::Interface::as_raw(self), frecovery.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginAbort(&self, frecovery: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginAbort)(windows_core::Interface::as_raw(self), frecovery.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AbortRecord(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AbortRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLogControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginPrepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrepareRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrepareRecord: usize,
    pub EndPrepare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub BeginCommit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitRecord: usize,
    pub EndCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginAbort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AbortRecord: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AbortRecord: usize,
    pub EndAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmCompensator_Impl: windows_core::IUnknownImpl {
    fn SetLogControl(&self, plogcontrol: windows_core::Ref<ICrmLogControl>) -> windows_core::Result<()>;
    fn BeginPrepare(&self) -> windows_core::Result<()>;
    fn PrepareRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL>;
    fn EndPrepare(&self) -> windows_core::Result<windows_core::BOOL>;
    fn BeginCommit(&self, frecovery: windows_core::BOOL) -> windows_core::Result<()>;
    fn CommitRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL>;
    fn EndCommit(&self) -> windows_core::Result<()>;
    fn BeginAbort(&self, frecovery: windows_core::BOOL) -> windows_core::Result<()>;
    fn AbortRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<windows_core::BOOL>;
    fn EndAbort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ICrmCompensator_Vtbl {
    pub const fn new<Identity: ICrmCompensator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLogControl<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::SetLogControl(this, core::mem::transmute_copy(&plogcontrol)).into()
            }
        }
        unsafe extern "system" fn BeginPrepare<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::BeginPrepare(this).into()
            }
        }
        unsafe extern "system" fn PrepareRecord<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensator_Impl::PrepareRecord(this, core::mem::transmute(&crmlogrec)) {
                    Ok(ok__) => {
                        pfforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndPrepare<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfoktoprepare: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensator_Impl::EndPrepare(this) {
                    Ok(ok__) => {
                        pfoktoprepare.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginCommit<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::BeginCommit(this, core::mem::transmute_copy(&frecovery)).into()
            }
        }
        unsafe extern "system" fn CommitRecord<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensator_Impl::CommitRecord(this, core::mem::transmute(&crmlogrec)) {
                    Ok(ok__) => {
                        pfforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndCommit<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::EndCommit(this).into()
            }
        }
        unsafe extern "system" fn BeginAbort<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::BeginAbort(this, core::mem::transmute_copy(&frecovery)).into()
            }
        }
        unsafe extern "system" fn AbortRecord<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensator_Impl::AbortRecord(this, core::mem::transmute(&crmlogrec)) {
                    Ok(ok__) => {
                        pfforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndAbort<Identity: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensator_Impl::EndAbort(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControl: SetLogControl::<Identity, OFFSET>,
            BeginPrepare: BeginPrepare::<Identity, OFFSET>,
            PrepareRecord: PrepareRecord::<Identity, OFFSET>,
            EndPrepare: EndPrepare::<Identity, OFFSET>,
            BeginCommit: BeginCommit::<Identity, OFFSET>,
            CommitRecord: CommitRecord::<Identity, OFFSET>,
            EndCommit: EndCommit::<Identity, OFFSET>,
            BeginAbort: BeginAbort::<Identity, OFFSET>,
            AbortRecord: AbortRecord::<Identity, OFFSET>,
            EndAbort: EndAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmCompensator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmCompensator {}
windows_core::imp::define_interface!(ICrmCompensatorVariants, ICrmCompensatorVariants_Vtbl, 0xf0baf8e4_7804_11d1_82e9_00a0c91eede9);
windows_core::imp::interface_hierarchy!(ICrmCompensatorVariants, windows_core::IUnknown);
impl ICrmCompensatorVariants {
    pub unsafe fn SetLogControlVariants<P0>(&self, plogcontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICrmLogControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLogControlVariants)(windows_core::Interface::as_raw(self), plogcontrol.param().abi()).ok() }
    }
    pub unsafe fn BeginPrepareVariants(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginPrepareVariants)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PrepareRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrepareRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndPrepareVariants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndPrepareVariants)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginCommitVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginCommitVariants)(windows_core::Interface::as_raw(self), brecovery).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CommitRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndCommitVariants(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndCommitVariants)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginAbortVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginAbortVariants)(windows_core::Interface::as_raw(self), brecovery).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AbortRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AbortRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndAbortVariants(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndAbortVariants)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmCompensatorVariants_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLogControlVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginPrepareVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PrepareRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PrepareRecordVariants: usize,
    pub EndPrepareVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub BeginCommitVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CommitRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CommitRecordVariants: usize,
    pub EndCommitVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginAbortVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AbortRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AbortRecordVariants: usize,
    pub EndAbortVariants: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmCompensatorVariants_Impl: windows_core::IUnknownImpl {
    fn SetLogControlVariants(&self, plogcontrol: windows_core::Ref<ICrmLogControl>) -> windows_core::Result<()>;
    fn BeginPrepareVariants(&self) -> windows_core::Result<()>;
    fn PrepareRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndPrepareVariants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BeginCommitVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CommitRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndCommitVariants(&self) -> windows_core::Result<()>;
    fn BeginAbortVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AbortRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndAbortVariants(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmCompensatorVariants_Vtbl {
    pub const fn new<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLogControlVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::SetLogControlVariants(this, core::mem::transmute_copy(&plogcontrol)).into()
            }
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::BeginPrepareVariants(this).into()
            }
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensatorVariants_Impl::PrepareRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                    Ok(ok__) => {
                        pbforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboktoprepare: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensatorVariants_Impl::EndPrepareVariants(this) {
                    Ok(ok__) => {
                        pboktoprepare.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::BeginCommitVariants(this, core::mem::transmute_copy(&brecovery)).into()
            }
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensatorVariants_Impl::CommitRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                    Ok(ok__) => {
                        pbforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndCommitVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::EndCommitVariants(this).into()
            }
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::BeginAbortVariants(this, core::mem::transmute_copy(&brecovery)).into()
            }
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmCompensatorVariants_Impl::AbortRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                    Ok(ok__) => {
                        pbforget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndAbortVariants<Identity: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmCompensatorVariants_Impl::EndAbortVariants(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControlVariants: SetLogControlVariants::<Identity, OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Identity, OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Identity, OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Identity, OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Identity, OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Identity, OFFSET>,
            EndCommitVariants: EndCommitVariants::<Identity, OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Identity, OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Identity, OFFSET>,
            EndAbortVariants: EndAbortVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmCompensatorVariants {}
windows_core::imp::define_interface!(ICrmFormatLogRecords, ICrmFormatLogRecords_Vtbl, 0x9c51d821_c98b_11d1_82fb_00a0c91eede9);
windows_core::imp::interface_hierarchy!(ICrmFormatLogRecords, windows_core::IUnknown);
impl ICrmFormatLogRecords {
    pub unsafe fn GetColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetColumnHeaders(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetColumn(&self, crmlogrec: CrmLogRecordRead) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), core::mem::transmute(crmlogrec), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetColumnVariants(&self, logrecord: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnVariants)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logrecord), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmFormatLogRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetColumnHeaders: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, CrmLogRecordRead, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetColumn: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetColumnVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetColumnVariants: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmFormatLogRecords_Impl: windows_core::IUnknownImpl {
    fn GetColumnCount(&self) -> windows_core::Result<i32>;
    fn GetColumnHeaders(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetColumn(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<super::Variant::VARIANT>;
    fn GetColumnVariants(&self, logrecord: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmFormatLogRecords_Vtbl {
    pub const fn new<Identity: ICrmFormatLogRecords_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColumnCount<Identity: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcolumncount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmFormatLogRecords_Impl::GetColumnCount(this) {
                    Ok(ok__) => {
                        plcolumncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaders: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmFormatLogRecords_Impl::GetColumnHeaders(this) {
                    Ok(ok__) => {
                        pheaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmFormatLogRecords_Impl::GetColumn(this, core::mem::transmute(&crmlogrec)) {
                    Ok(ok__) => {
                        pformattedlogrecord.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnVariants<Identity: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecord: super::Variant::VARIANT, pformattedlogrecord: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmFormatLogRecords_Impl::GetColumnVariants(this, core::mem::transmute(&logrecord)) {
                    Ok(ok__) => {
                        pformattedlogrecord.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmFormatLogRecords {}
windows_core::imp::define_interface!(ICrmLogControl, ICrmLogControl_Vtbl, 0xa0e174b3_d26e_11d2_8f84_00805fc7bcd9);
windows_core::imp::interface_hierarchy!(ICrmLogControl, windows_core::IUnknown);
impl ICrmLogControl {
    pub unsafe fn TransactionUOW(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionUOW)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RegisterCompensator<P0, P1>(&self, lpcwstrprogidcompensator: P0, lpcwstrdescription: P1, lcrmregflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterCompensator)(windows_core::Interface::as_raw(self), lpcwstrprogidcompensator.param().abi(), lpcwstrdescription.param().abi(), lcrmregflags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn WriteLogRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteLogRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute(plogrecord)).ok() }
    }
    pub unsafe fn ForceLog(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForceLog)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ForgetLogRecord(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForgetLogRecord)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ForceTransactionToAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForceTransactionToAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteLogRecord(&self, rgblob: &[super::Com::BLOB]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteLogRecord)(windows_core::Interface::as_raw(self), core::mem::transmute(rgblob.as_ptr()), rgblob.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmLogControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransactionUOW: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterCompensator: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub WriteLogRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    WriteLogRecordVariants: usize,
    pub ForceLog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForgetLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForceTransactionToAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::BLOB, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteLogRecord: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmLogControl_Impl: windows_core::IUnknownImpl {
    fn TransactionUOW(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterCompensator(&self, lpcwstrprogidcompensator: &windows_core::PCWSTR, lpcwstrdescription: &windows_core::PCWSTR, lcrmregflags: i32) -> windows_core::Result<()>;
    fn WriteLogRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn ForceLog(&self) -> windows_core::Result<()>;
    fn ForgetLogRecord(&self) -> windows_core::Result<()>;
    fn ForceTransactionToAbort(&self) -> windows_core::Result<()>;
    fn WriteLogRecord(&self, rgblob: *const super::Com::BLOB, cblob: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmLogControl_Vtbl {
    pub const fn new<Identity: ICrmLogControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransactionUOW<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmLogControl_Impl::TransactionUOW(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterCompensator<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcwstrprogidcompensator: windows_core::PCWSTR, lpcwstrdescription: windows_core::PCWSTR, lcrmregflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::RegisterCompensator(this, core::mem::transmute(&lpcwstrprogidcompensator), core::mem::transmute(&lpcwstrdescription), core::mem::transmute_copy(&lcrmregflags)).into()
            }
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::WriteLogRecordVariants(this, core::mem::transmute_copy(&plogrecord)).into()
            }
        }
        unsafe extern "system" fn ForceLog<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::ForceLog(this).into()
            }
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::ForgetLogRecord(this).into()
            }
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::ForceTransactionToAbort(this).into()
            }
        }
        unsafe extern "system" fn WriteLogRecord<Identity: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmLogControl_Impl::WriteLogRecord(this, core::mem::transmute_copy(&rgblob), core::mem::transmute_copy(&cblob)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionUOW: TransactionUOW::<Identity, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, OFFSET>,
            ForceLog: ForceLog::<Identity, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmLogControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmLogControl {}
windows_core::imp::define_interface!(ICrmMonitor, ICrmMonitor_Vtbl, 0x70c8e443_c7ed_11d1_82fb_00a0c91eede9);
windows_core::imp::interface_hierarchy!(ICrmMonitor, windows_core::IUnknown);
impl ICrmMonitor {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClerks(&self) -> windows_core::Result<ICrmMonitorClerks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClerks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn HoldClerk(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoldClerk)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClerks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClerks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub HoldClerk: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    HoldClerk: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitor_Impl: windows_core::IUnknownImpl {
    fn GetClerks(&self) -> windows_core::Result<ICrmMonitorClerks>;
    fn HoldClerk(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitor_Vtbl {
    pub const fn new<Identity: ICrmMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClerks<Identity: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclerks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitor_Impl::GetClerks(this) {
                    Ok(ok__) => {
                        pclerks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HoldClerk<Identity: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitor_Impl::HoldClerk(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClerks: GetClerks::<Identity, OFFSET>,
            HoldClerk: HoldClerk::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmMonitor {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICrmMonitorClerks, ICrmMonitorClerks_Vtbl, 0x70c8e442_c7ed_11d1_82fb_00a0c91eede9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICrmMonitorClerks {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICrmMonitorClerks, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerks {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ProgIdCompensator(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgIdCompensator)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Description(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TransactionUOW(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionUOW)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ActivityId(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivityId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorClerks_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ProgIdCompensator: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ProgIdCompensator: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Description: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TransactionUOW: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TransactionUOW: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ActivityId: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorClerks_Impl: super::Com::IDispatch_Impl {
    fn Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ProgIdCompensator(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
    fn Description(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
    fn TransactionUOW(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
    fn ActivityId(&self, index: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitorClerks_Vtbl {
    pub const fn new<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Item<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::ProgIdCompensator(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::Description(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionUOW<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::TransactionUOW(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivityId<Identity: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorClerks_Impl::ActivityId(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            TransactionUOW: TransactionUOW::<Identity, OFFSET>,
            ActivityId: ActivityId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmMonitorClerks {}
windows_core::imp::define_interface!(ICrmMonitorLogRecords, ICrmMonitorLogRecords_Vtbl, 0x70c8e441_c7ed_11d1_82fb_00a0c91eede9);
windows_core::imp::interface_hierarchy!(ICrmMonitorLogRecords, windows_core::IUnknown);
impl ICrmMonitorLogRecords {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransactionState(&self) -> windows_core::Result<CrmTransactionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StructuredRecords(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StructuredRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLogRecord)(windows_core::Interface::as_raw(self), dwindex, pcrmlogrec as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetLogRecordVariants(&self, indexnumber: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogRecordVariants)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(indexnumber), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrmMonitorLogRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TransactionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrmTransactionState) -> windows_core::HRESULT,
    pub StructuredRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetLogRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CrmLogRecordRead) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetLogRecord: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetLogRecordVariants: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetLogRecordVariants: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorLogRecords_Impl: windows_core::IUnknownImpl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn TransactionState(&self) -> windows_core::Result<CrmTransactionState>;
    fn StructuredRecords(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::Result<()>;
    fn GetLogRecordVariants(&self, indexnumber: &super::Variant::VARIANT) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitorLogRecords_Vtbl {
    pub const fn new<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorLogRecords_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransactionState<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut CrmTransactionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorLogRecords_Impl::TransactionState(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StructuredRecords<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorLogRecords_Impl::StructuredRecords(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLogRecord<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICrmMonitorLogRecords_Impl::GetLogRecord(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcrmlogrec)).into()
            }
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexnumber: super::Variant::VARIANT, plogrecord: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrmMonitorLogRecords_Impl::GetLogRecordVariants(this, core::mem::transmute(&indexnumber)) {
                    Ok(ok__) => {
                        plogrecord.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            TransactionState: TransactionState::<Identity, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICrmMonitorLogRecords {}
windows_core::imp::define_interface!(IDispenserDriver, IDispenserDriver_Vtbl, 0x208b3651_2b48_11cf_be10_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IDispenserDriver, windows_core::IUnknown);
impl IDispenserDriver {
    pub unsafe fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateResource)(windows_core::Interface::as_raw(self), restypid, presid as _, psecsfreebeforedestroy as _).ok() }
    }
    pub unsafe fn RateResource(&self, restypid: usize, resid: usize, frequirestransactionenlistment: bool, prating: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RateResource)(windows_core::Interface::as_raw(self), restypid, resid, frequirestransactionenlistment.into(), prating as _).ok() }
    }
    pub unsafe fn EnlistResource(&self, resid: usize, transid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnlistResource)(windows_core::Interface::as_raw(self), resid, transid).ok() }
    }
    pub unsafe fn ResetResource(&self, resid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetResource)(windows_core::Interface::as_raw(self), resid).ok() }
    }
    pub unsafe fn DestroyResource(&self, resid: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyResource)(windows_core::Interface::as_raw(self), resid).ok() }
    }
    pub unsafe fn DestroyResourceS(&self, resid: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyResourceS)(windows_core::Interface::as_raw(self), resid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserDriver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut i32) -> windows_core::HRESULT,
    pub RateResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub EnlistResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ResetResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DestroyResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DestroyResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
pub trait IDispenserDriver_Impl: windows_core::IUnknownImpl {
    fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::Result<()>;
    fn RateResource(&self, restypid: usize, resid: usize, frequirestransactionenlistment: windows_core::BOOL, prating: *mut u32) -> windows_core::Result<()>;
    fn EnlistResource(&self, resid: usize, transid: usize) -> windows_core::Result<()>;
    fn ResetResource(&self, resid: usize) -> windows_core::Result<()>;
    fn DestroyResource(&self, resid: usize) -> windows_core::Result<()>;
    fn DestroyResourceS(&self, resid: *mut u16) -> windows_core::Result<()>;
}
impl IDispenserDriver_Vtbl {
    pub const fn new<Identity: IDispenserDriver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateResource<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::CreateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&presid), core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
            }
        }
        unsafe extern "system" fn RateResource<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: windows_core::BOOL, prating: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::RateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&frequirestransactionenlistment), core::mem::transmute_copy(&prating)).into()
            }
        }
        unsafe extern "system" fn EnlistResource<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize, transid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::EnlistResource(this, core::mem::transmute_copy(&resid), core::mem::transmute_copy(&transid)).into()
            }
        }
        unsafe extern "system" fn ResetResource<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::ResetResource(this, core::mem::transmute_copy(&resid)).into()
            }
        }
        unsafe extern "system" fn DestroyResource<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::DestroyResource(this, core::mem::transmute_copy(&resid)).into()
            }
        }
        unsafe extern "system" fn DestroyResourceS<Identity: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserDriver_Impl::DestroyResourceS(this, core::mem::transmute_copy(&resid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateResource: CreateResource::<Identity, OFFSET>,
            RateResource: RateResource::<Identity, OFFSET>,
            EnlistResource: EnlistResource::<Identity, OFFSET>,
            ResetResource: ResetResource::<Identity, OFFSET>,
            DestroyResource: DestroyResource::<Identity, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispenserDriver as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDispenserDriver {}
windows_core::imp::define_interface!(IDispenserManager, IDispenserManager_Vtbl, 0x5cb31e10_2b5f_11cf_be10_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IDispenserManager, windows_core::IUnknown);
impl IDispenserManager {
    pub unsafe fn RegisterDispenser<P0, P1>(&self, __midl__idispensermanager0000: P0, szdispensername: P1) -> windows_core::Result<IHolder>
    where
        P0: windows_core::Param<IDispenserDriver>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterDispenser)(windows_core::Interface::as_raw(self), __midl__idispensermanager0000.param().abi(), szdispensername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), __midl__idispensermanager0002 as _, __midl__idispensermanager0003 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispenserManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterDispenser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut usize) -> windows_core::HRESULT,
}
pub trait IDispenserManager_Impl: windows_core::IUnknownImpl {
    fn RegisterDispenser(&self, __midl__idispensermanager0000: windows_core::Ref<IDispenserDriver>, szdispensername: &windows_core::PCWSTR) -> windows_core::Result<IHolder>;
    fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::Result<()>;
}
impl IDispenserManager_Vtbl {
    pub const fn new<Identity: IDispenserManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterDispenser<Identity: IDispenserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0000: *mut core::ffi::c_void, szdispensername: windows_core::PCWSTR, __midl__idispensermanager0001: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispenserManager_Impl::RegisterDispenser(this, core::mem::transmute_copy(&__midl__idispensermanager0000), core::mem::transmute(&szdispensername)) {
                    Ok(ok__) => {
                        __midl__idispensermanager0001.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContext<Identity: IDispenserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispenserManager_Impl::GetContext(this, core::mem::transmute_copy(&__midl__idispensermanager0002), core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispenserManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDispenserManager {}
windows_core::imp::define_interface!(IEnumNames, IEnumNames_Vtbl, 0x51372af2_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IEnumNames, windows_core::IUnknown);
impl IEnumNames {
    pub unsafe fn Next(&self, celt: u32, rgname: *mut windows_core::BSTR, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgname), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumNames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumNames_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgname: *mut windows_core::BSTR, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNames>;
}
impl IEnumNames_Vtbl {
    pub const fn new<Identity: IEnumNames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgname: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgname), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumNames_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumNames_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumNames {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEventServerTrace, IEventServerTrace_Vtbl, 0x9a9f12b8_80af_47ab_a579_35ea57725370);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEventServerTrace {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEventServerTrace, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTrace {
    pub unsafe fn StartTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartTraceGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidevent), core::mem::transmute_copy(bstrguidfilter), lpidfilter).ok() }
    }
    pub unsafe fn StopTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopTraceGuid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrguidevent), core::mem::transmute_copy(bstrguidfilter), lpidfilter).ok() }
    }
    pub unsafe fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumTraceGuid)(windows_core::Interface::as_raw(self), plcntguids as _, core::mem::transmute(pbstrguidlist)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventServerTrace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub StartTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StopTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumTraceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventServerTrace_Impl: super::Com::IDispatch_Impl {
    fn StartTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()>;
    fn StopTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()>;
    fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventServerTrace_Vtbl {
    pub const fn new<Identity: IEventServerTrace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartTraceGuid<Identity: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: *mut core::ffi::c_void, bstrguidfilter: *mut core::ffi::c_void, lpidfilter: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventServerTrace_Impl::StartTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
            }
        }
        unsafe extern "system" fn StopTraceGuid<Identity: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: *mut core::ffi::c_void, bstrguidfilter: *mut core::ffi::c_void, lpidfilter: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventServerTrace_Impl::StopTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
            }
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventServerTrace_Impl::EnumTraceGuid(this, core::mem::transmute_copy(&plcntguids), core::mem::transmute_copy(&pbstrguidlist)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartTraceGuid: StartTraceGuid::<Identity, OFFSET>,
            StopTraceGuid: StopTraceGuid::<Identity, OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventServerTrace as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventServerTrace {}
windows_core::imp::define_interface!(IGetAppTrackerData, IGetAppTrackerData_Vtbl, 0x507c3ac8_3e12_4cb0_9366_653d3e050638);
windows_core::imp::interface_hierarchy!(IGetAppTrackerData, windows_core::IUnknown);
impl IGetAppTrackerData {
    pub unsafe fn GetApplicationProcesses(&self, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetApplicationProcesses)(windows_core::Interface::as_raw(self), partitionid, applicationid, flags, numapplicationprocesses as _, applicationprocesses as _).ok() }
    }
    pub unsafe fn GetApplicationProcessDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: Option<*mut ApplicationProcessSummary>, statistics: Option<*mut ApplicationProcessStatistics>, recycleinfo: Option<*mut ApplicationProcessRecycleInfo>, anycomponentshangmonitored: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetApplicationProcessDetails)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, flags, summary.unwrap_or(core::mem::zeroed()) as _, statistics.unwrap_or(core::mem::zeroed()) as _, recycleinfo.unwrap_or(core::mem::zeroed()) as _, anycomponentshangmonitored.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetApplicationsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetApplicationsInProcess)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, partitionid, flags, numapplicationsinprocess as _, applications as _).ok() }
    }
    pub unsafe fn GetComponentsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentsInProcess)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, partitionid, applicationid, flags, numcomponentsinprocess as _, components as _).ok() }
    }
    pub unsafe fn GetComponentDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: Option<*mut ComponentSummary>, statistics: Option<*mut ComponentStatistics>, hangmonitorinfo: Option<*mut ComponentHangMonitorInfo>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetComponentDetails)(windows_core::Interface::as_raw(self), applicationinstanceid, processid, clsid, flags, summary.unwrap_or(core::mem::zeroed()) as _, statistics.unwrap_or(core::mem::zeroed()) as _, hangmonitorinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetTrackerDataAsCollectionObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrackerDataAsCollectionObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSuggestedPollingInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuggestedPollingInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetAppTrackerData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetApplicationProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *mut u32, *mut *mut ApplicationProcessSummary) -> windows_core::HRESULT,
    pub GetApplicationProcessDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *mut ApplicationProcessSummary, *mut ApplicationProcessStatistics, *mut ApplicationProcessRecycleInfo, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetApplicationsInProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *mut u32, *mut *mut ApplicationSummary) -> windows_core::HRESULT,
    pub GetComponentsInProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *const windows_core::GUID, u32, *mut u32, *mut *mut ComponentSummary) -> windows_core::HRESULT,
    pub GetComponentDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *mut ComponentSummary, *mut ComponentStatistics, *mut ComponentHangMonitorInfo) -> windows_core::HRESULT,
    pub GetTrackerDataAsCollectionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSuggestedPollingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IGetAppTrackerData_Impl: windows_core::IUnknownImpl {
    fn GetApplicationProcesses(&self, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::Result<()>;
    fn GetApplicationProcessDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetApplicationsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::Result<()>;
    fn GetComponentsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::Result<()>;
    fn GetComponentDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> windows_core::Result<()>;
    fn GetTrackerDataAsCollectionObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSuggestedPollingInterval(&self) -> windows_core::Result<u32>;
}
impl IGetAppTrackerData_Vtbl {
    pub const fn new<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetApplicationProcesses<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetAppTrackerData_Impl::GetApplicationProcesses(this, core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationprocesses), core::mem::transmute_copy(&applicationprocesses)).into()
            }
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetAppTrackerData_Impl::GetApplicationProcessDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&recycleinfo), core::mem::transmute_copy(&anycomponentshangmonitored)).into()
            }
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetAppTrackerData_Impl::GetApplicationsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationsinprocess), core::mem::transmute_copy(&applications)).into()
            }
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetAppTrackerData_Impl::GetComponentsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numcomponentsinprocess), core::mem::transmute_copy(&components)).into()
            }
        }
        unsafe extern "system" fn GetComponentDetails<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetAppTrackerData_Impl::GetComponentDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&hangmonitorinfo)).into()
            }
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, toplevelcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetAppTrackerData_Impl::GetTrackerDataAsCollectionObject(this) {
                    Ok(ok__) => {
                        toplevelcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollingintervalinseconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetAppTrackerData_Impl::GetSuggestedPollingInterval(this) {
                    Ok(ok__) => {
                        pollingintervalinseconds.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Identity, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetAppTrackerData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetAppTrackerData {}
windows_core::imp::define_interface!(IGetContextProperties, IGetContextProperties_Vtbl, 0x51372af4_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IGetContextProperties, windows_core::IUnknown);
impl IGetContextProperties {
    pub unsafe fn Count(&self, plcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), plcount as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute(pproperty)).ok() }
    }
    pub unsafe fn EnumNames(&self) -> windows_core::Result<IEnumNames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetContextProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    pub EnumNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetContextProperties_Impl: windows_core::IUnknownImpl {
    fn Count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn EnumNames(&self) -> windows_core::Result<IEnumNames>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGetContextProperties_Vtbl {
    pub const fn new<Identity: IGetContextProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pproperty: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn EnumNames<Identity: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetContextProperties_Impl::EnumNames(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumNames: EnumNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetContextProperties as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGetContextProperties {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IGetSecurityCallContext, IGetSecurityCallContext_Vtbl, 0xcafc823f_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IGetSecurityCallContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IGetSecurityCallContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContext {
    pub unsafe fn GetSecurityCallContext(&self) -> windows_core::Result<ISecurityCallContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityCallContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGetSecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetSecurityCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetSecurityCallContext_Impl: super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(&self) -> windows_core::Result<ISecurityCallContext>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGetSecurityCallContext_Vtbl {
    pub const fn new<Identity: IGetSecurityCallContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecurityCallContext<Identity: IGetSecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGetSecurityCallContext_Impl::GetSecurityCallContext(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetSecurityCallContext: GetSecurityCallContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IGetSecurityCallContext {}
windows_core::imp::define_interface!(IHolder, IHolder_Vtbl, 0xbf6a1850_2b45_11cf_be10_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IHolder, windows_core::IUnknown);
impl IHolder {
    pub unsafe fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllocResource)(windows_core::Interface::as_raw(self), __midl__iholder0000, __midl__iholder0001 as _).ok() }
    }
    pub unsafe fn FreeResource(&self, __midl__iholder0002: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeResource)(windows_core::Interface::as_raw(self), __midl__iholder0002).ok() }
    }
    pub unsafe fn TrackResource(&self, __midl__iholder0003: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TrackResource)(windows_core::Interface::as_raw(self), __midl__iholder0003).ok() }
    }
    pub unsafe fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TrackResourceS)(windows_core::Interface::as_raw(self), __midl__iholder0004 as _).ok() }
    }
    pub unsafe fn UntrackResource(&self, __midl__iholder0005: usize, __midl__iholder0006: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UntrackResource)(windows_core::Interface::as_raw(self), __midl__iholder0005, __midl__iholder0006.into()).ok() }
    }
    pub unsafe fn UntrackResourceS(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UntrackResourceS)(windows_core::Interface::as_raw(self), __midl__iholder0007 as _, __midl__iholder0008.into()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestDestroyResource)(windows_core::Interface::as_raw(self), __midl__iholder0009).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub FreeResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub TrackResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub TrackResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub UntrackResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL) -> windows_core::HRESULT,
    pub UntrackResourceS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, windows_core::BOOL) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestDestroyResource: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait IHolder_Impl: windows_core::IUnknownImpl {
    fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::Result<()>;
    fn FreeResource(&self, __midl__iholder0002: usize) -> windows_core::Result<()>;
    fn TrackResource(&self, __midl__iholder0003: usize) -> windows_core::Result<()>;
    fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> windows_core::Result<()>;
    fn UntrackResource(&self, __midl__iholder0005: usize, __midl__iholder0006: windows_core::BOOL) -> windows_core::Result<()>;
    fn UntrackResourceS(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: windows_core::BOOL) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> windows_core::Result<()>;
}
impl IHolder_Vtbl {
    pub const fn new<Identity: IHolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllocResource<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::AllocResource(this, core::mem::transmute_copy(&__midl__iholder0000), core::mem::transmute_copy(&__midl__iholder0001)).into()
            }
        }
        unsafe extern "system" fn FreeResource<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0002: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::FreeResource(this, core::mem::transmute_copy(&__midl__iholder0002)).into()
            }
        }
        unsafe extern "system" fn TrackResource<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0003: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::TrackResource(this, core::mem::transmute_copy(&__midl__iholder0003)).into()
            }
        }
        unsafe extern "system" fn TrackResourceS<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0004: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::TrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0004)).into()
            }
        }
        unsafe extern "system" fn UntrackResource<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::UntrackResource(this, core::mem::transmute_copy(&__midl__iholder0005), core::mem::transmute_copy(&__midl__iholder0006)).into()
            }
        }
        unsafe extern "system" fn UntrackResourceS<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::UntrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0007), core::mem::transmute_copy(&__midl__iholder0008)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0009: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolder_Impl::RequestDestroyResource(this, core::mem::transmute_copy(&__midl__iholder0009)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocResource: AllocResource::<Identity, OFFSET>,
            FreeResource: FreeResource::<Identity, OFFSET>,
            TrackResource: TrackResource::<Identity, OFFSET>,
            TrackResourceS: TrackResourceS::<Identity, OFFSET>,
            UntrackResource: UntrackResource::<Identity, OFFSET>,
            UntrackResourceS: UntrackResourceS::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHolder {}
windows_core::imp::define_interface!(ILBEvents, ILBEvents_Vtbl, 0x683130b4_2e50_11d2_98a5_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(ILBEvents, windows_core::IUnknown);
impl ILBEvents {
    pub unsafe fn TargetUp(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TargetUp)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute_copy(bstrclsideng)).ok() }
    }
    pub unsafe fn TargetDown(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TargetDown)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrservername), core::mem::transmute_copy(bstrclsideng)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn EngineDefined(&self, bstrpropname: &windows_core::BSTR, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EngineDefined)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), core::mem::transmute(varpropvalue), core::mem::transmute_copy(bstrclsideng)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILBEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub EngineDefined: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    EngineDefined: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILBEvents_Impl: windows_core::IUnknownImpl {
    fn TargetUp(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetDown(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EngineDefined(&self, bstrpropname: &windows_core::BSTR, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILBEvents_Vtbl {
    pub const fn new<Identity: ILBEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TargetUp<Identity: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bstrclsideng: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILBEvents_Impl::TargetUp(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
            }
        }
        unsafe extern "system" fn TargetDown<Identity: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: *mut core::ffi::c_void, bstrclsideng: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILBEvents_Impl::TargetDown(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
            }
        }
        unsafe extern "system" fn EngineDefined<Identity: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILBEvents_Impl::EngineDefined(this, core::mem::transmute(&bstrpropname), core::mem::transmute_copy(&varpropvalue), core::mem::transmute(&bstrclsideng)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetUp: TargetUp::<Identity, OFFSET>,
            TargetDown: TargetDown::<Identity, OFFSET>,
            EngineDefined: EngineDefined::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILBEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ILBEvents {}
windows_core::imp::define_interface!(IMTSActivity, IMTSActivity_Vtbl, 0x51372af0_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IMTSActivity, windows_core::IUnknown);
impl IMTSActivity {
    pub unsafe fn SynchronousCall<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMTSCall>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCall)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok() }
    }
    pub unsafe fn AsyncCall<P0>(&self, pcall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMTSCall>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncCall)(windows_core::Interface::as_raw(self), pcall.param().abi()).ok() }
    }
    pub unsafe fn Reserved1(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BindToCurrentThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindToCurrentThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnbindFromThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnbindFromThread)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsyncCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub BindToCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMTSActivity_Impl: windows_core::IUnknownImpl {
    fn SynchronousCall(&self, pcall: windows_core::Ref<IMTSCall>) -> windows_core::Result<()>;
    fn AsyncCall(&self, pcall: windows_core::Ref<IMTSCall>) -> windows_core::Result<()>;
    fn Reserved1(&self);
    fn BindToCurrentThread(&self) -> windows_core::Result<()>;
    fn UnbindFromThread(&self) -> windows_core::Result<()>;
}
impl IMTSActivity_Vtbl {
    pub const fn new<Identity: IMTSActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCall<Identity: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSActivity_Impl::SynchronousCall(this, core::mem::transmute_copy(&pcall)).into()
            }
        }
        unsafe extern "system" fn AsyncCall<Identity: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSActivity_Impl::AsyncCall(this, core::mem::transmute_copy(&pcall)).into()
            }
        }
        unsafe extern "system" fn Reserved1<Identity: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSActivity_Impl::Reserved1(this)
            }
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSActivity_Impl::BindToCurrentThread(this).into()
            }
        }
        unsafe extern "system" fn UnbindFromThread<Identity: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSActivity_Impl::UnbindFromThread(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, OFFSET>,
            AsyncCall: AsyncCall::<Identity, OFFSET>,
            Reserved1: Reserved1::<Identity, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSActivity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMTSActivity {}
windows_core::imp::define_interface!(IMTSCall, IMTSCall_Vtbl, 0x51372aef_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IMTSCall, windows_core::IUnknown);
impl IMTSCall {
    pub unsafe fn OnCall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMTSCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMTSCall_Impl: windows_core::IUnknownImpl {
    fn OnCall(&self) -> windows_core::Result<()>;
}
impl IMTSCall_Vtbl {
    pub const fn new<Identity: IMTSCall_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCall<Identity: IMTSCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMTSCall_Impl::OnCall(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSCall as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMTSCall {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMTSLocator, IMTSLocator_Vtbl, 0xd19b8bfd_7f88_11d0_b16e_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMTSLocator {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMTSLocator, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocator {
    pub unsafe fn GetEventDispatcher(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventDispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMTSLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetEventDispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMTSLocator_Impl: super::Com::IDispatch_Impl {
    fn GetEventDispatcher(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMTSLocator_Vtbl {
    pub const fn new<Identity: IMTSLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventDispatcher<Identity: IMTSLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMTSLocator_Impl::GetEventDispatcher(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMTSLocator {}
windows_core::imp::define_interface!(IManagedActivationEvents, IManagedActivationEvents_Vtbl, 0xa5f325af_572f_46da_b8ab_827c3d95d99e);
windows_core::imp::interface_hierarchy!(IManagedActivationEvents, windows_core::IUnknown);
impl IManagedActivationEvents {
    pub unsafe fn CreateManagedStub<P0>(&self, pinfo: P0, fdist: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IManagedObjectInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateManagedStub)(windows_core::Interface::as_raw(self), pinfo.param().abi(), fdist.into()).ok() }
    }
    pub unsafe fn DestroyManagedStub<P0>(&self, pinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IManagedObjectInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).DestroyManagedStub)(windows_core::Interface::as_raw(self), pinfo.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedActivationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateManagedStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub DestroyManagedStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IManagedActivationEvents_Impl: windows_core::IUnknownImpl {
    fn CreateManagedStub(&self, pinfo: windows_core::Ref<IManagedObjectInfo>, fdist: windows_core::BOOL) -> windows_core::Result<()>;
    fn DestroyManagedStub(&self, pinfo: windows_core::Ref<IManagedObjectInfo>) -> windows_core::Result<()>;
}
impl IManagedActivationEvents_Vtbl {
    pub const fn new<Identity: IManagedActivationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateManagedStub<Identity: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, fdist: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedActivationEvents_Impl::CreateManagedStub(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&fdist)).into()
            }
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedActivationEvents_Impl::DestroyManagedStub(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Identity, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedActivationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManagedActivationEvents {}
windows_core::imp::define_interface!(IManagedObjectInfo, IManagedObjectInfo_Vtbl, 0x1427c51a_4584_49d8_90a0_c50d8086cbe9);
windows_core::imp::interface_hierarchy!(IManagedObjectInfo, windows_core::IUnknown);
impl IManagedObjectInfo {
    pub unsafe fn GetIUnknown(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIUnknown)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIObjectControl(&self) -> windows_core::Result<IObjectControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIObjectControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetInPool<P1>(&self, binpool: bool, ppooledobj: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IManagedPooledObj>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInPool)(windows_core::Interface::as_raw(self), binpool.into(), ppooledobj.param().abi()).ok() }
    }
    pub unsafe fn SetWrapperStrength(&self, bstrong: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWrapperStrength)(windows_core::Interface::as_raw(self), bstrong.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIObjectControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInPool: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWrapperStrength: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IManagedObjectInfo_Impl: windows_core::IUnknownImpl {
    fn GetIUnknown(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetIObjectControl(&self) -> windows_core::Result<IObjectControl>;
    fn SetInPool(&self, binpool: windows_core::BOOL, ppooledobj: windows_core::Ref<IManagedPooledObj>) -> windows_core::Result<()>;
    fn SetWrapperStrength(&self, bstrong: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IManagedObjectInfo_Vtbl {
    pub const fn new<Identity: IManagedObjectInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIUnknown<Identity: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManagedObjectInfo_Impl::GetIUnknown(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIObjectControl<Identity: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManagedObjectInfo_Impl::GetIObjectControl(this) {
                    Ok(ok__) => {
                        pctrl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInPool<Identity: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binpool: windows_core::BOOL, ppooledobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedObjectInfo_Impl::SetInPool(this, core::mem::transmute_copy(&binpool), core::mem::transmute_copy(&ppooledobj)).into()
            }
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrong: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedObjectInfo_Impl::SetWrapperStrength(this, core::mem::transmute_copy(&bstrong)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIUnknown: GetIUnknown::<Identity, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, OFFSET>,
            SetInPool: SetInPool::<Identity, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedObjectInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManagedObjectInfo {}
windows_core::imp::define_interface!(IManagedPoolAction, IManagedPoolAction_Vtbl, 0xda91b74e_5388_4783_949d_c1cd5fb00506);
windows_core::imp::interface_hierarchy!(IManagedPoolAction, windows_core::IUnknown);
impl IManagedPoolAction {
    pub unsafe fn LastRelease(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LastRelease)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPoolAction_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LastRelease: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IManagedPoolAction_Impl: windows_core::IUnknownImpl {
    fn LastRelease(&self) -> windows_core::Result<()>;
}
impl IManagedPoolAction_Vtbl {
    pub const fn new<Identity: IManagedPoolAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastRelease<Identity: IManagedPoolAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedPoolAction_Impl::LastRelease(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LastRelease: LastRelease::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedPoolAction as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManagedPoolAction {}
windows_core::imp::define_interface!(IManagedPooledObj, IManagedPooledObj_Vtbl, 0xc5da4bea_1b42_4437_8926_b6a38860a770);
windows_core::imp::interface_hierarchy!(IManagedPooledObj, windows_core::IUnknown);
impl IManagedPooledObj {
    pub unsafe fn SetHeld(&self, m_bheld: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHeld)(windows_core::Interface::as_raw(self), m_bheld.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManagedPooledObj_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHeld: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IManagedPooledObj_Impl: windows_core::IUnknownImpl {
    fn SetHeld(&self, m_bheld: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IManagedPooledObj_Vtbl {
    pub const fn new<Identity: IManagedPooledObj_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHeld<Identity: IManagedPooledObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, m_bheld: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManagedPooledObj_Impl::SetHeld(this, core::mem::transmute_copy(&m_bheld)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetHeld: SetHeld::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedPooledObj as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IManagedPooledObj {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMessageMover, IMessageMover_Vtbl, 0x588a085a_b795_11d1_8054_00c04fc340ee);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMessageMover {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMessageMover, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMessageMover {
    pub unsafe fn SourcePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourcePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSourcePath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourcePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn DestPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DestPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDestPath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)).ok() }
    }
    pub unsafe fn CommitBatchSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitBatchSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCommitBatchSize(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCommitBatchSize)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn MoveMessages(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveMessages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMessageMover_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SourcePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSourcePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DestPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDestPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCommitBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MoveMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMessageMover_Impl: super::Com::IDispatch_Impl {
    fn SourcePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSourcePath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DestPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDestPath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CommitBatchSize(&self) -> windows_core::Result<i32>;
    fn SetCommitBatchSize(&self, newval: i32) -> windows_core::Result<()>;
    fn MoveMessages(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMessageMover_Vtbl {
    pub const fn new<Identity: IMessageMover_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SourcePath<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessageMover_Impl::SourcePath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSourcePath<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageMover_Impl::SetSourcePath(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn DestPath<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessageMover_Impl::DestPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDestPath<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageMover_Impl::SetDestPath(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn CommitBatchSize<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessageMover_Impl::CommitBatchSize(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageMover_Impl::SetCommitBatchSize(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MoveMessages<Identity: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmessagesmoved: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMessageMover_Impl::MoveMessages(this) {
                    Ok(ok__) => {
                        plmessagesmoved.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SourcePath: SourcePath::<Identity, OFFSET>,
            SetSourcePath: SetSourcePath::<Identity, OFFSET>,
            DestPath: DestPath::<Identity, OFFSET>,
            SetDestPath: SetDestPath::<Identity, OFFSET>,
            CommitBatchSize: CommitBatchSize::<Identity, OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Identity, OFFSET>,
            MoveMessages: MoveMessages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageMover as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMessageMover {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsEventInfo, IMtsEventInfo_Vtbl, 0xd56c3dc1_8482_11d0_b170_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsEventInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsEventInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfo {
    pub unsafe fn Names(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Names)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EventID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Value(&self, skey: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(skey), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEventInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Names: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Value: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEventInfo_Impl: super::Com::IDispatch_Impl {
    fn Names(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EventID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Value(&self, skey: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsEventInfo_Vtbl {
    pub const fn new<Identity: IMtsEventInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Names<Identity: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEventInfo_Impl::Names(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEventInfo_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        sdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EventID<Identity: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sguideventid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEventInfo_Impl::EventID(this) {
                    Ok(ok__) => {
                        sguideventid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEventInfo_Impl::Count(this) {
                    Ok(ok__) => {
                        lcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Value<Identity: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, skey: *mut core::ffi::c_void, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEventInfo_Impl::get_Value(this, core::mem::transmute(&skey)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Names: Names::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            EventID: EventID::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsEventInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMtsEventInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsEvents, IMtsEvents_Vtbl, 0xbacedf4d_74ab_11d0_b162_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsEvents {
    pub unsafe fn PackageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PackageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PackageGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PackageGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PostEvent(&self, vevent: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PostEvent)(windows_core::Interface::as_raw(self), core::mem::transmute(vevent)).ok() }
    }
    pub unsafe fn FireEvents(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FireEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProcessID(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PackageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PostEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PostEvent: usize,
    pub FireEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetProcessID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEvents_Impl: super::Com::IDispatch_Impl {
    fn PackageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PackageGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PostEvent(&self, vevent: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn FireEvents(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProcessID(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsEvents_Vtbl {
    pub const fn new<Identity: IMtsEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PackageName<Identity: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEvents_Impl::PackageName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PackageGuid<Identity: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEvents_Impl::PackageGuid(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PostEvent<Identity: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vevent: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMtsEvents_Impl::PostEvent(this, core::mem::transmute_copy(&vevent)).into()
            }
        }
        unsafe extern "system" fn FireEvents<Identity: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEvents_Impl::FireEvents(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProcessID<Identity: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsEvents_Impl::GetProcessID(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PackageName: PackageName::<Identity, OFFSET>,
            PackageGuid: PackageGuid::<Identity, OFFSET>,
            PostEvent: PostEvent::<Identity, OFFSET>,
            FireEvents: FireEvents::<Identity, OFFSET>,
            GetProcessID: GetProcessID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMtsEvents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMtsGrp, IMtsGrp_Vtbl, 0x4b2e958c_0393_11d1_b1ab_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMtsGrp {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMtsGrp, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrp {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, lindex: i32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMtsGrp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsGrp_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, lindex: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsGrp_Vtbl {
    pub const fn new<Identity: IMtsGrp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsGrp_Impl::Count(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMtsGrp_Impl::Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppunkdispatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMtsGrp_Impl::Refresh(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsGrp as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMtsGrp {}
windows_core::imp::define_interface!(IObjPool, IObjPool_Vtbl, 0x7d8805a0_2ea7_11d1_b1cc_00aa00ba3258);
windows_core::imp::interface_hierarchy!(IObjPool, windows_core::IUnknown);
impl IObjPool {
    pub unsafe fn Reserved1(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved2(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved3(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved3)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved4(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved4)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PutEndTx<P0>(&self, pobj: P0)
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutEndTx)(windows_core::Interface::as_raw(self), pobj.param().abi()) }
    }
    pub unsafe fn Reserved5(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved5)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved6(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved6)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjPool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PutEndTx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IObjPool_Impl: windows_core::IUnknownImpl {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn PutEndTx(&self, pobj: windows_core::Ref<windows_core::IUnknown>);
    fn Reserved5(&self);
    fn Reserved6(&self);
}
impl IObjPool_Vtbl {
    pub const fn new<Identity: IObjPool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reserved1<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved1(this)
            }
        }
        unsafe extern "system" fn Reserved2<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved2(this)
            }
        }
        unsafe extern "system" fn Reserved3<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved3(this)
            }
        }
        unsafe extern "system" fn Reserved4<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved4(this)
            }
        }
        unsafe extern "system" fn PutEndTx<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobj: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::PutEndTx(this, core::mem::transmute_copy(&pobj))
            }
        }
        unsafe extern "system" fn Reserved5<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved5(this)
            }
        }
        unsafe extern "system" fn Reserved6<Identity: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjPool_Impl::Reserved6(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            Reserved3: Reserved3::<Identity, OFFSET>,
            Reserved4: Reserved4::<Identity, OFFSET>,
            PutEndTx: PutEndTx::<Identity, OFFSET>,
            Reserved5: Reserved5::<Identity, OFFSET>,
            Reserved6: Reserved6::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjPool as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjPool {}
windows_core::imp::define_interface!(IObjectConstruct, IObjectConstruct_Vtbl, 0x41c4f8b3_7439_11d2_98cb_00c04f8ee1c4);
windows_core::imp::interface_hierarchy!(IObjectConstruct, windows_core::IUnknown);
impl IObjectConstruct {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct<P0>(&self, pctorobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Construct)(windows_core::Interface::as_raw(self), pctorobj.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstruct_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: windows_core::IUnknownImpl {
    fn Construct(&self, pctorobj: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstruct_Vtbl {
    pub const fn new<Identity: IObjectConstruct_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Construct<Identity: IObjectConstruct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctorobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectConstruct_Impl::Construct(this, core::mem::transmute_copy(&pctorobj)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectConstruct as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IObjectConstruct {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IObjectConstructString, IObjectConstructString_Vtbl, 0x41c4f8b2_7439_11d2_98cb_00c04f8ee1c4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IObjectConstructString {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IObjectConstructString, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructString {
    pub unsafe fn ConstructString(&self, pval: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConstructString)(windows_core::Interface::as_raw(self), core::mem::transmute(pval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IObjectConstructString_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConstructString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectConstructString_Impl: super::Com::IDispatch_Impl {
    fn ConstructString(&self, pval: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectConstructString_Vtbl {
    pub const fn new<Identity: IObjectConstructString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConstructString<Identity: IObjectConstructString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectConstructString_Impl::ConstructString(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ConstructString: ConstructString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectConstructString as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IObjectConstructString {}
windows_core::imp::define_interface!(IObjectContext, IObjectContext_Vtbl, 0x51372ae0_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IObjectContext, windows_core::IUnknown);
impl IObjectContext {
    pub unsafe fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), rclsid, riid, ppv as _).ok() }
    }
    pub unsafe fn SetComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsInTransaction(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSecurityEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR, pfisinrole: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrole), pfisinrole as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IObjectContext_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetComplete(&self) -> windows_core::Result<()>;
    fn SetAbort(&self) -> windows_core::Result<()>;
    fn EnableCommit(&self) -> windows_core::Result<()>;
    fn DisableCommit(&self) -> windows_core::Result<()>;
    fn IsInTransaction(&self) -> windows_core::BOOL;
    fn IsSecurityEnabled(&self) -> windows_core::BOOL;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR, pfisinrole: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IObjectContext_Vtbl {
    pub const fn new<Identity: IObjectContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn SetComplete<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::SetComplete(this).into()
            }
        }
        unsafe extern "system" fn SetAbort<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::SetAbort(this).into()
            }
        }
        unsafe extern "system" fn EnableCommit<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::EnableCommit(this).into()
            }
        }
        unsafe extern "system" fn DisableCommit<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::DisableCommit(this).into()
            }
        }
        unsafe extern "system" fn IsInTransaction<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::IsInTransaction(this)
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::IsSecurityEnabled(this)
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: *mut core::ffi::c_void, pfisinrole: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole), core::mem::transmute_copy(&pfisinrole)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            SetComplete: SetComplete::<Identity, OFFSET>,
            SetAbort: SetAbort::<Identity, OFFSET>,
            EnableCommit: EnableCommit::<Identity, OFFSET>,
            DisableCommit: DisableCommit::<Identity, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectContext {}
windows_core::imp::define_interface!(IObjectContextActivity, IObjectContextActivity_Vtbl, 0x51372afc_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IObjectContextActivity, windows_core::IUnknown);
impl IObjectContextActivity {
    pub unsafe fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IObjectContextActivity_Impl: windows_core::IUnknownImpl {
    fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IObjectContextActivity_Vtbl {
    pub const fn new<Identity: IObjectContextActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivityId<Identity: IObjectContextActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextActivity_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivityId: GetActivityId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextActivity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectContextActivity {}
windows_core::imp::define_interface!(IObjectContextInfo, IObjectContextInfo_Vtbl, 0x75b52ddb_e8ed_11d1_93ad_00aa00ba3258);
windows_core::imp::interface_hierarchy!(IObjectContextInfo, windows_core::IUnknown);
impl IObjectContextInfo {
    pub unsafe fn IsInTransaction(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransaction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTransactionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
    pub unsafe fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetActivityId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
    pub unsafe fn GetContextId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContextId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTransactionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetContextId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IObjectContextInfo_Impl: windows_core::IUnknownImpl {
    fn IsInTransaction(&self) -> windows_core::BOOL;
    fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTransactionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetContextId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IObjectContextInfo_Vtbl {
    pub const fn new<Identity: IObjectContextInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsInTransaction<Identity: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo_Impl::IsInTransaction(this)
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrans: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IObjectContextInfo_Impl::GetTransaction(this) {
                    Ok(ok__) => {
                        pptrans.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo_Impl::GetTransactionId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn GetContextId<Identity: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo_Impl::GetContextId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, OFFSET>,
            GetActivityId: GetActivityId::<Identity, OFFSET>,
            GetContextId: GetContextId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectContextInfo {}
windows_core::imp::define_interface!(IObjectContextInfo2, IObjectContextInfo2_Vtbl, 0x594be71a_4bc4_438b_9197_cfd176248b09);
impl core::ops::Deref for IObjectContextInfo2 {
    type Target = IObjectContextInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectContextInfo2, windows_core::IUnknown, IObjectContextInfo);
impl IObjectContextInfo2 {
    pub unsafe fn GetPartitionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPartitionId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
    pub unsafe fn GetApplicationId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetApplicationId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
    pub unsafe fn GetApplicationInstanceId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetApplicationInstanceId)(windows_core::Interface::as_raw(self), pguid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextInfo2_Vtbl {
    pub base__: IObjectContextInfo_Vtbl,
    pub GetPartitionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetApplicationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetApplicationInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IObjectContextInfo2_Impl: IObjectContextInfo_Impl {
    fn GetPartitionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetApplicationId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetApplicationInstanceId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IObjectContextInfo2_Vtbl {
    pub const fn new<Identity: IObjectContextInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPartitionId<Identity: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo2_Impl::GetPartitionId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo2_Impl::GetApplicationId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextInfo2_Impl::GetApplicationInstanceId(this, core::mem::transmute_copy(&pguid)).into()
            }
        }
        Self {
            base__: IObjectContextInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as windows_core::Interface>::IID || iid == &<IObjectContextInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectContextInfo2 {}
windows_core::imp::define_interface!(IObjectContextTip, IObjectContextTip_Vtbl, 0x92fd41ca_bad9_11d2_9a2d_00c04f797bc9);
windows_core::imp::interface_hierarchy!(IObjectContextTip, windows_core::IUnknown);
impl IObjectContextTip {
    pub unsafe fn GetTipUrl(&self, ptipurl: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTipUrl)(windows_core::Interface::as_raw(self), core::mem::transmute(ptipurl)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectContextTip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTipUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IObjectContextTip_Impl: windows_core::IUnknownImpl {
    fn GetTipUrl(&self, ptipurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IObjectContextTip_Vtbl {
    pub const fn new<Identity: IObjectContextTip_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTipUrl<Identity: IObjectContextTip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptipurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectContextTip_Impl::GetTipUrl(this, core::mem::transmute_copy(&ptipurl)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTipUrl: GetTipUrl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextTip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectContextTip {}
windows_core::imp::define_interface!(IObjectControl, IObjectControl_Vtbl, 0x51372aec_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IObjectControl, windows_core::IUnknown);
impl IObjectControl {
    pub unsafe fn Activate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Deactivate(&self) {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CanBePooled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).CanBePooled)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CanBePooled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IObjectControl_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<()>;
    fn Deactivate(&self);
    fn CanBePooled(&self) -> windows_core::BOOL;
}
impl IObjectControl_Vtbl {
    pub const fn new<Identity: IObjectControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectControl_Impl::Activate(this).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectControl_Impl::Deactivate(this)
            }
        }
        unsafe extern "system" fn CanBePooled<Identity: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectControl_Impl::CanBePooled(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CanBePooled: CanBePooled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectControl {}
windows_core::imp::define_interface!(IPlaybackControl, IPlaybackControl_Vtbl, 0x51372afd_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IPlaybackControl, windows_core::IUnknown);
impl IPlaybackControl {
    pub unsafe fn FinalClientRetry(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinalClientRetry)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FinalServerRetry(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FinalServerRetry)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FinalClientRetry: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalServerRetry: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPlaybackControl_Impl: windows_core::IUnknownImpl {
    fn FinalClientRetry(&self) -> windows_core::Result<()>;
    fn FinalServerRetry(&self) -> windows_core::Result<()>;
}
impl IPlaybackControl_Vtbl {
    pub const fn new<Identity: IPlaybackControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinalClientRetry<Identity: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlaybackControl_Impl::FinalClientRetry(this).into()
            }
        }
        unsafe extern "system" fn FinalServerRetry<Identity: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlaybackControl_Impl::FinalServerRetry(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Identity, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlaybackControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPlaybackControl {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPoolManager, IPoolManager_Vtbl, 0x0a469861_5a91_43a0_99b6_d5e179bb0631);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPoolManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPoolManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPoolManager {
    pub unsafe fn ShutdownPool(&self, clsidorprogid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShutdownPool)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clsidorprogid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPoolManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ShutdownPool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPoolManager_Impl: super::Com::IDispatch_Impl {
    fn ShutdownPool(&self, clsidorprogid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPoolManager_Vtbl {
    pub const fn new<Identity: IPoolManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShutdownPool<Identity: IPoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidorprogid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPoolManager_Impl::ShutdownPool(this, core::mem::transmute(&clsidorprogid)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ShutdownPool: ShutdownPool::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPoolManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPoolManager {}
windows_core::imp::define_interface!(IProcessInitializer, IProcessInitializer_Vtbl, 0x1113f52d_dc7f_4943_aed6_88d04027e32a);
windows_core::imp::interface_hierarchy!(IProcessInitializer, windows_core::IUnknown);
impl IProcessInitializer {
    pub unsafe fn Startup<P0>(&self, punkprocesscontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Startup)(windows_core::Interface::as_raw(self), punkprocesscontrol.param().abi()).ok() }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Startup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProcessInitializer_Impl: windows_core::IUnknownImpl {
    fn Startup(&self, punkprocesscontrol: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl IProcessInitializer_Vtbl {
    pub const fn new<Identity: IProcessInitializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Startup<Identity: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkprocesscontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessInitializer_Impl::Startup(this, core::mem::transmute_copy(&punkprocesscontrol)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessInitializer_Impl::Shutdown(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Startup: Startup::<Identity, OFFSET>, Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessInitializer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessInitializer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityCallContext, ISecurityCallContext_Vtbl, 0xcafc823e_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityCallContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityCallContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContext {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrole), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn IsUserInRole(&self, puser: *const super::Variant::VARIANT, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUserInRole)(windows_core::Interface::as_raw(self), core::mem::transmute(puser), core::mem::transmute_copy(bstrrole), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub IsUserInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Variant::VARIANT, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    IsUserInRole: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallContext_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUserInRole(&self, puser: *const super::Variant::VARIANT, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityCallContext_Vtbl {
    pub const fn new<Identity: ISecurityCallContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: *mut core::ffi::c_void, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                    Ok(ok__) => {
                        pfinrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::IsSecurityEnabled(this) {
                    Ok(ok__) => {
                        pfisenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUserInRole<Identity: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *const super::Variant::VARIANT, bstrrole: *mut core::ffi::c_void, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallContext_Impl::IsUserInRole(this, core::mem::transmute_copy(&puser), core::mem::transmute(&bstrrole)) {
                    Ok(ok__) => {
                        pfinrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsUserInRole: IsUserInRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityCallContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityCallersColl, ISecurityCallersColl_Vtbl, 0xcafc823d_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityCallersColl {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityCallersColl, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersColl {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Item(&self, lindex: i32) -> windows_core::Result<ISecurityIdentityColl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityCallersColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallersColl_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<ISecurityIdentityColl>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityCallersColl_Vtbl {
    pub const fn new<Identity: ISecurityCallersColl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallersColl_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallersColl_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityCallersColl_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityCallersColl as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISecurityCallersColl {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISecurityIdentityColl, ISecurityIdentityColl_Vtbl, 0xcafc823c_b441_11d1_b82b_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISecurityIdentityColl {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISecurityIdentityColl, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityColl {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityIdentityColl_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityIdentityColl_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityIdentityColl_Vtbl {
    pub const fn new<Identity: ISecurityIdentityColl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityIdentityColl_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityIdentityColl_Impl::get_Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISecurityIdentityColl_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISecurityIdentityColl {}
windows_core::imp::define_interface!(ISecurityProperty, ISecurityProperty_Vtbl, 0x51372aea_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(ISecurityProperty, windows_core::IUnknown);
impl ISecurityProperty {
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDirectCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDirectCreatorSID)(windows_core::Interface::as_raw(self), psid as _).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetOriginalCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOriginalCreatorSID)(windows_core::Interface::as_raw(self), psid as _).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetDirectCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDirectCallerSID)(windows_core::Interface::as_raw(self), psid as _).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn GetOriginalCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOriginalCallerSID)(windows_core::Interface::as_raw(self), psid as _).ok() }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn ReleaseSID(&self, psid: super::super::Security::PSID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseSID)(windows_core::Interface::as_raw(self), psid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Security")]
    pub GetDirectCreatorSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDirectCreatorSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetOriginalCreatorSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetOriginalCreatorSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetDirectCallerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetDirectCallerSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub GetOriginalCallerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    GetOriginalCallerSID: usize,
    #[cfg(feature = "Win32_Security")]
    pub ReleaseSID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Security::PSID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    ReleaseSID: usize,
}
#[cfg(feature = "Win32_Security")]
pub trait ISecurityProperty_Impl: windows_core::IUnknownImpl {
    fn GetDirectCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetOriginalCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetDirectCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetOriginalCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn ReleaseSID(&self, psid: super::super::Security::PSID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security")]
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: ISecurityProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityProperty_Impl::GetDirectCreatorSID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityProperty_Impl::GetOriginalCreatorSID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityProperty_Impl::GetDirectCallerSID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityProperty_Impl::GetOriginalCallerSID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        unsafe extern "system" fn ReleaseSID<Identity: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: super::super::Security::PSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISecurityProperty_Impl::ReleaseSID(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for ISecurityProperty {}
windows_core::imp::define_interface!(ISelectCOMLBServer, ISelectCOMLBServer_Vtbl, 0xdcf443f4_3f8a_4872_b9f0_369a796d12d6);
windows_core::imp::interface_hierarchy!(ISelectCOMLBServer, windows_core::IUnknown);
impl ISelectCOMLBServer {
    pub unsafe fn Init(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetLBServer<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetLBServer)(windows_core::Interface::as_raw(self), punk.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectCOMLBServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLBServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISelectCOMLBServer_Impl: windows_core::IUnknownImpl {
    fn Init(&self) -> windows_core::Result<()>;
    fn GetLBServer(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ISelectCOMLBServer_Vtbl {
    pub const fn new<Identity: ISelectCOMLBServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectCOMLBServer_Impl::Init(this).into()
            }
        }
        unsafe extern "system" fn GetLBServer<Identity: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectCOMLBServer_Impl::GetLBServer(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, OFFSET>, GetLBServer: GetLBServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISelectCOMLBServer {}
windows_core::imp::define_interface!(ISendMethodEvents, ISendMethodEvents_Vtbl, 0x2732fd59_b2b4_4d44_878c_8b8f09626008);
windows_core::imp::interface_hierarchy!(ISendMethodEvents, windows_core::IUnknown);
impl ISendMethodEvents {
    pub unsafe fn SendMethodCall(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMethodCall)(windows_core::Interface::as_raw(self), pidentity, riid, dwmeth).ok() }
    }
    pub unsafe fn SendMethodReturn(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMethodReturn)(windows_core::Interface::as_raw(self), pidentity, riid, dwmeth, hrcall, hrserver).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISendMethodEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendMethodCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SendMethodReturn: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const windows_core::GUID, u32, windows_core::HRESULT, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ISendMethodEvents_Impl: windows_core::IUnknownImpl {
    fn SendMethodCall(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::Result<()>;
    fn SendMethodReturn(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ISendMethodEvents_Vtbl {
    pub const fn new<Identity: ISendMethodEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendMethodCall<Identity: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISendMethodEvents_Impl::SendMethodCall(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth)).into()
            }
        }
        unsafe extern "system" fn SendMethodReturn<Identity: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISendMethodEvents_Impl::SendMethodReturn(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth), core::mem::transmute_copy(&hrcall), core::mem::transmute_copy(&hrserver)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMethodCall: SendMethodCall::<Identity, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISendMethodEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISendMethodEvents {}
windows_core::imp::define_interface!(IServiceActivity, IServiceActivity_Vtbl, 0x67532e0c_9e2f_4450_a354_035633944e17);
windows_core::imp::interface_hierarchy!(IServiceActivity, windows_core::IUnknown);
impl IServiceActivity {
    pub unsafe fn SynchronousCall<P0>(&self, piservicecall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IServiceCall>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCall)(windows_core::Interface::as_raw(self), piservicecall.param().abi()).ok() }
    }
    pub unsafe fn AsynchronousCall<P0>(&self, piservicecall: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IServiceCall>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsynchronousCall)(windows_core::Interface::as_raw(self), piservicecall.param().abi()).ok() }
    }
    pub unsafe fn BindToCurrentThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BindToCurrentThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnbindFromThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnbindFromThread)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AsynchronousCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindFromThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IServiceActivity_Impl: windows_core::IUnknownImpl {
    fn SynchronousCall(&self, piservicecall: windows_core::Ref<IServiceCall>) -> windows_core::Result<()>;
    fn AsynchronousCall(&self, piservicecall: windows_core::Ref<IServiceCall>) -> windows_core::Result<()>;
    fn BindToCurrentThread(&self) -> windows_core::Result<()>;
    fn UnbindFromThread(&self) -> windows_core::Result<()>;
}
impl IServiceActivity_Vtbl {
    pub const fn new<Identity: IServiceActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCall<Identity: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceActivity_Impl::SynchronousCall(this, core::mem::transmute_copy(&piservicecall)).into()
            }
        }
        unsafe extern "system" fn AsynchronousCall<Identity: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceActivity_Impl::AsynchronousCall(this, core::mem::transmute_copy(&piservicecall)).into()
            }
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceActivity_Impl::BindToCurrentThread(this).into()
            }
        }
        unsafe extern "system" fn UnbindFromThread<Identity: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceActivity_Impl::UnbindFromThread(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceActivity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceActivity {}
windows_core::imp::define_interface!(IServiceCall, IServiceCall_Vtbl, 0xbd3e2e12_42dd_40f4_a09a_95a50c58304b);
windows_core::imp::interface_hierarchy!(IServiceCall, windows_core::IUnknown);
impl IServiceCall {
    pub unsafe fn OnCall(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceCall_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IServiceCall_Impl: windows_core::IUnknownImpl {
    fn OnCall(&self) -> windows_core::Result<()>;
}
impl IServiceCall_Vtbl {
    pub const fn new<Identity: IServiceCall_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCall<Identity: IServiceCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceCall_Impl::OnCall(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceCall as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceCall {}
windows_core::imp::define_interface!(IServiceComTIIntrinsicsConfig, IServiceComTIIntrinsicsConfig_Vtbl, 0x09e6831e_04e1_4ed4_9d0f_e8b168bafeaf);
windows_core::imp::interface_hierarchy!(IServiceComTIIntrinsicsConfig, windows_core::IUnknown);
impl IServiceComTIIntrinsicsConfig {
    pub unsafe fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ComTIIntrinsicsConfig)(windows_core::Interface::as_raw(self), comtiintrinsicsconfig).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceComTIIntrinsicsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ComTIIntrinsicsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_COMTIIntrinsicsConfig) -> windows_core::HRESULT,
}
pub trait IServiceComTIIntrinsicsConfig_Impl: windows_core::IUnknownImpl {
    fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::Result<()>;
}
impl IServiceComTIIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceComTIIntrinsicsConfig_Impl::ComTIIntrinsicsConfig(this, core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceComTIIntrinsicsConfig {}
windows_core::imp::define_interface!(IServiceIISIntrinsicsConfig, IServiceIISIntrinsicsConfig_Vtbl, 0x1a0cf920_d452_46f4_bc36_48118d54ea52);
windows_core::imp::interface_hierarchy!(IServiceIISIntrinsicsConfig, windows_core::IUnknown);
impl IServiceIISIntrinsicsConfig {
    pub unsafe fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IISIntrinsicsConfig)(windows_core::Interface::as_raw(self), iisintrinsicsconfig).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceIISIntrinsicsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IISIntrinsicsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_IISIntrinsicsConfig) -> windows_core::HRESULT,
}
pub trait IServiceIISIntrinsicsConfig_Impl: windows_core::IUnknownImpl {
    fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::Result<()>;
}
impl IServiceIISIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceIISIntrinsicsConfig_Impl::IISIntrinsicsConfig(this, core::mem::transmute_copy(&iisintrinsicsconfig)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceIISIntrinsicsConfig {}
windows_core::imp::define_interface!(IServiceInheritanceConfig, IServiceInheritanceConfig_Vtbl, 0x92186771_d3b4_4d77_a8ea_ee842d586f35);
windows_core::imp::interface_hierarchy!(IServiceInheritanceConfig, windows_core::IUnknown);
impl IServiceInheritanceConfig {
    pub unsafe fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ContainingContextTreatment)(windows_core::Interface::as_raw(self), inheritanceconfig).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceInheritanceConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContainingContextTreatment: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_InheritanceConfig) -> windows_core::HRESULT,
}
pub trait IServiceInheritanceConfig_Impl: windows_core::IUnknownImpl {
    fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::Result<()>;
}
impl IServiceInheritanceConfig_Vtbl {
    pub const fn new<Identity: IServiceInheritanceConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContainingContextTreatment<Identity: IServiceInheritanceConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceInheritanceConfig_Impl::ContainingContextTreatment(this, core::mem::transmute_copy(&inheritanceconfig)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ContainingContextTreatment: ContainingContextTreatment::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceInheritanceConfig {}
windows_core::imp::define_interface!(IServicePartitionConfig, IServicePartitionConfig_Vtbl, 0x80182d03_5ea4_4831_ae97_55beffc2e590);
windows_core::imp::interface_hierarchy!(IServicePartitionConfig, windows_core::IUnknown);
impl IServicePartitionConfig {
    pub unsafe fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PartitionConfig)(windows_core::Interface::as_raw(self), partitionconfig).ok() }
    }
    pub unsafe fn PartitionID(&self, guidpartitionid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PartitionID)(windows_core::Interface::as_raw(self), guidpartitionid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePartitionConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PartitionConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_PartitionConfig) -> windows_core::HRESULT,
    pub PartitionID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IServicePartitionConfig_Impl: windows_core::IUnknownImpl {
    fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> windows_core::Result<()>;
    fn PartitionID(&self, guidpartitionid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IServicePartitionConfig_Vtbl {
    pub const fn new<Identity: IServicePartitionConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PartitionConfig<Identity: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePartitionConfig_Impl::PartitionConfig(this, core::mem::transmute_copy(&partitionconfig)).into()
            }
        }
        unsafe extern "system" fn PartitionID<Identity: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpartitionid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePartitionConfig_Impl::PartitionID(this, core::mem::transmute_copy(&guidpartitionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PartitionConfig: PartitionConfig::<Identity, OFFSET>,
            PartitionID: PartitionID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePartitionConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServicePartitionConfig {}
windows_core::imp::define_interface!(IServicePool, IServicePool_Vtbl, 0xb302df81_ea45_451e_99a2_09f9fd1b1e13);
windows_core::imp::interface_hierarchy!(IServicePool, windows_core::IUnknown);
impl IServicePool {
    pub unsafe fn Initialize<P0>(&self, ppoolconfig: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), ppoolconfig.param().abi()).ok() }
    }
    pub unsafe fn GetObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), riid, ppv as _).ok() }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IServicePool_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, ppoolconfig: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl IServicePool_Vtbl {
    pub const fn new<Identity: IServicePool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePool_Impl::Initialize(this, core::mem::transmute_copy(&ppoolconfig)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePool_Impl::GetObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePool_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePool as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServicePool {}
windows_core::imp::define_interface!(IServicePoolConfig, IServicePoolConfig_Vtbl, 0xa9690656_5bca_470c_8451_250c1f43a33e);
windows_core::imp::interface_hierarchy!(IServicePoolConfig, windows_core::IUnknown);
impl IServicePoolConfig {
    pub unsafe fn SetMaxPoolSize(&self, dwmaxpool: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxPoolSize)(windows_core::Interface::as_raw(self), dwmaxpool).ok() }
    }
    pub unsafe fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MaxPoolSize)(windows_core::Interface::as_raw(self), pdwmaxpool as _).ok() }
    }
    pub unsafe fn SetMinPoolSize(&self, dwminpool: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinPoolSize)(windows_core::Interface::as_raw(self), dwminpool).ok() }
    }
    pub unsafe fn MinPoolSize(&self, pdwminpool: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MinPoolSize)(windows_core::Interface::as_raw(self), pdwminpool as _).ok() }
    }
    pub unsafe fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreationTimeout)(windows_core::Interface::as_raw(self), dwcreationtimeout).ok() }
    }
    pub unsafe fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreationTimeout)(windows_core::Interface::as_raw(self), pdwcreationtimeout as _).ok() }
    }
    pub unsafe fn SetTransactionAffinity(&self, ftxaffinity: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransactionAffinity)(windows_core::Interface::as_raw(self), ftxaffinity.into()).ok() }
    }
    pub unsafe fn TransactionAffinity(&self, pftxaffinity: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransactionAffinity)(windows_core::Interface::as_raw(self), pftxaffinity as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassFactory<P0>(&self, pfactory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IClassFactory>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClassFactory)(windows_core::Interface::as_raw(self), pfactory.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassFactory(&self) -> windows_core::Result<super::Com::IClassFactory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServicePoolConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMaxPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MinPoolSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCreationTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreationTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTransactionAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub TransactionAffinity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassFactory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassFactory: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServicePoolConfig_Impl: windows_core::IUnknownImpl {
    fn SetMaxPoolSize(&self, dwmaxpool: u32) -> windows_core::Result<()>;
    fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> windows_core::Result<()>;
    fn SetMinPoolSize(&self, dwminpool: u32) -> windows_core::Result<()>;
    fn MinPoolSize(&self, pdwminpool: *mut u32) -> windows_core::Result<()>;
    fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> windows_core::Result<()>;
    fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> windows_core::Result<()>;
    fn SetTransactionAffinity(&self, ftxaffinity: windows_core::BOOL) -> windows_core::Result<()>;
    fn TransactionAffinity(&self, pftxaffinity: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetClassFactory(&self, pfactory: windows_core::Ref<super::Com::IClassFactory>) -> windows_core::Result<()>;
    fn ClassFactory(&self) -> windows_core::Result<super::Com::IClassFactory>;
}
#[cfg(feature = "Win32_System_Com")]
impl IServicePoolConfig_Vtbl {
    pub const fn new<Identity: IServicePoolConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaxPoolSize<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxpool: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::SetMaxPoolSize(this, core::mem::transmute_copy(&dwmaxpool)).into()
            }
        }
        unsafe extern "system" fn MaxPoolSize<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxpool: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::MaxPoolSize(this, core::mem::transmute_copy(&pdwmaxpool)).into()
            }
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminpool: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::SetMinPoolSize(this, core::mem::transmute_copy(&dwminpool)).into()
            }
        }
        unsafe extern "system" fn MinPoolSize<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminpool: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::MinPoolSize(this, core::mem::transmute_copy(&pdwminpool)).into()
            }
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreationtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::SetCreationTimeout(this, core::mem::transmute_copy(&dwcreationtimeout)).into()
            }
        }
        unsafe extern "system" fn CreationTimeout<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcreationtimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::CreationTimeout(this, core::mem::transmute_copy(&pdwcreationtimeout)).into()
            }
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftxaffinity: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::SetTransactionAffinity(this, core::mem::transmute_copy(&ftxaffinity)).into()
            }
        }
        unsafe extern "system" fn TransactionAffinity<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftxaffinity: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::TransactionAffinity(this, core::mem::transmute_copy(&pftxaffinity)).into()
            }
        }
        unsafe extern "system" fn SetClassFactory<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServicePoolConfig_Impl::SetClassFactory(this, core::mem::transmute_copy(&pfactory)).into()
            }
        }
        unsafe extern "system" fn ClassFactory<Identity: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IServicePoolConfig_Impl::ClassFactory(this) {
                    Ok(ok__) => {
                        pfactory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMaxPoolSize: SetMaxPoolSize::<Identity, OFFSET>,
            MaxPoolSize: MaxPoolSize::<Identity, OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Identity, OFFSET>,
            MinPoolSize: MinPoolSize::<Identity, OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Identity, OFFSET>,
            CreationTimeout: CreationTimeout::<Identity, OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Identity, OFFSET>,
            TransactionAffinity: TransactionAffinity::<Identity, OFFSET>,
            SetClassFactory: SetClassFactory::<Identity, OFFSET>,
            ClassFactory: ClassFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePoolConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IServicePoolConfig {}
windows_core::imp::define_interface!(IServiceSxsConfig, IServiceSxsConfig_Vtbl, 0xc7cd7379_f3f2_4634_811b_703281d73e08);
windows_core::imp::interface_hierarchy!(IServiceSxsConfig, windows_core::IUnknown);
impl IServiceSxsConfig {
    pub unsafe fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SxsConfig)(windows_core::Interface::as_raw(self), scsconfig).ok() }
    }
    pub unsafe fn SxsName<P0>(&self, szsxsname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SxsName)(windows_core::Interface::as_raw(self), szsxsname.param().abi()).ok() }
    }
    pub unsafe fn SxsDirectory<P0>(&self, szsxsdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SxsDirectory)(windows_core::Interface::as_raw(self), szsxsdirectory.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSxsConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SxsConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_SxsConfig) -> windows_core::HRESULT,
    pub SxsName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SxsDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IServiceSxsConfig_Impl: windows_core::IUnknownImpl {
    fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> windows_core::Result<()>;
    fn SxsName(&self, szsxsname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SxsDirectory(&self, szsxsdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IServiceSxsConfig_Vtbl {
    pub const fn new<Identity: IServiceSxsConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SxsConfig<Identity: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scsconfig: CSC_SxsConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceSxsConfig_Impl::SxsConfig(this, core::mem::transmute_copy(&scsconfig)).into()
            }
        }
        unsafe extern "system" fn SxsName<Identity: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceSxsConfig_Impl::SxsName(this, core::mem::transmute(&szsxsname)).into()
            }
        }
        unsafe extern "system" fn SxsDirectory<Identity: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceSxsConfig_Impl::SxsDirectory(this, core::mem::transmute(&szsxsdirectory)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SxsConfig: SxsConfig::<Identity, OFFSET>,
            SxsName: SxsName::<Identity, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSxsConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceSxsConfig {}
windows_core::imp::define_interface!(IServiceSynchronizationConfig, IServiceSynchronizationConfig_Vtbl, 0xfd880e81_6dce_4c58_af83_a208846c0030);
windows_core::imp::interface_hierarchy!(IServiceSynchronizationConfig, windows_core::IUnknown);
impl IServiceSynchronizationConfig {
    pub unsafe fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureSynchronization)(windows_core::Interface::as_raw(self), synchconfig).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSynchronizationConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureSynchronization: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_SynchronizationConfig) -> windows_core::HRESULT,
}
pub trait IServiceSynchronizationConfig_Impl: windows_core::IUnknownImpl {
    fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> windows_core::Result<()>;
}
impl IServiceSynchronizationConfig_Vtbl {
    pub const fn new<Identity: IServiceSynchronizationConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureSynchronization<Identity: IServiceSynchronizationConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceSynchronizationConfig_Impl::ConfigureSynchronization(this, core::mem::transmute_copy(&synchconfig)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceSynchronizationConfig {}
windows_core::imp::define_interface!(IServiceSysTxnConfig, IServiceSysTxnConfig_Vtbl, 0x33caf1a1_fcb8_472b_b45e_967448ded6d8);
impl core::ops::Deref for IServiceSysTxnConfig {
    type Target = IServiceTransactionConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceSysTxnConfig, windows_core::IUnknown, IServiceTransactionConfigBase, IServiceTransactionConfig);
impl IServiceSysTxnConfig {
    pub unsafe fn ConfigureBYOTSysTxn<P0>(&self, ptxproxy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITransactionProxy>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureBYOTSysTxn)(windows_core::Interface::as_raw(self), ptxproxy.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceSysTxnConfig_Vtbl {
    pub base__: IServiceTransactionConfig_Vtbl,
    pub ConfigureBYOTSysTxn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceSysTxnConfig_Impl: IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(&self, ptxproxy: windows_core::Ref<ITransactionProxy>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceSysTxnConfig_Vtbl {
    pub const fn new<Identity: IServiceSysTxnConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: IServiceSysTxnConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxproxy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceSysTxnConfig_Impl::ConfigureBYOTSysTxn(this, core::mem::transmute_copy(&ptxproxy)).into()
            }
        }
        Self { base__: IServiceTransactionConfig_Vtbl::new::<Identity, OFFSET>(), ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as windows_core::Interface>::IID || iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID || iid == &<IServiceTransactionConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for IServiceSysTxnConfig {}
windows_core::imp::define_interface!(IServiceThreadPoolConfig, IServiceThreadPoolConfig_Vtbl, 0x186d89bc_f277_4bcc_80d5_4df7b836ef4a);
windows_core::imp::interface_hierarchy!(IServiceThreadPoolConfig, windows_core::IUnknown);
impl IServiceThreadPoolConfig {
    pub unsafe fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectThreadPool)(windows_core::Interface::as_raw(self), threadpool).ok() }
    }
    pub unsafe fn SetBindingInfo(&self, binding: CSC_Binding) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBindingInfo)(windows_core::Interface::as_raw(self), binding).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceThreadPoolConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SelectThreadPool: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_ThreadPool) -> windows_core::HRESULT,
    pub SetBindingInfo: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_Binding) -> windows_core::HRESULT,
}
pub trait IServiceThreadPoolConfig_Impl: windows_core::IUnknownImpl {
    fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> windows_core::Result<()>;
    fn SetBindingInfo(&self, binding: CSC_Binding) -> windows_core::Result<()>;
}
impl IServiceThreadPoolConfig_Vtbl {
    pub const fn new<Identity: IServiceThreadPoolConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SelectThreadPool<Identity: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadpool: CSC_ThreadPool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceThreadPoolConfig_Impl::SelectThreadPool(this, core::mem::transmute_copy(&threadpool)).into()
            }
        }
        unsafe extern "system" fn SetBindingInfo<Identity: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binding: CSC_Binding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceThreadPoolConfig_Impl::SetBindingInfo(this, core::mem::transmute_copy(&binding)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Identity, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceThreadPoolConfig {}
windows_core::imp::define_interface!(IServiceTrackerConfig, IServiceTrackerConfig_Vtbl, 0x6c3a3e1d_0ba6_4036_b76f_d0404db816c9);
windows_core::imp::interface_hierarchy!(IServiceTrackerConfig, windows_core::IUnknown);
impl IServiceTrackerConfig {
    pub unsafe fn TrackerConfig<P1, P2>(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: P1, sztrackerctxname: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).TrackerConfig)(windows_core::Interface::as_raw(self), trackerconfig, sztrackerappname.param().abi(), sztrackerctxname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTrackerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TrackerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_TrackerConfig, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IServiceTrackerConfig_Impl: windows_core::IUnknownImpl {
    fn TrackerConfig(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: &windows_core::PCWSTR, sztrackerctxname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IServiceTrackerConfig_Vtbl {
    pub const fn new<Identity: IServiceTrackerConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TrackerConfig<Identity: IServiceTrackerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: windows_core::PCWSTR, sztrackerctxname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTrackerConfig_Impl::TrackerConfig(this, core::mem::transmute_copy(&trackerconfig), core::mem::transmute(&sztrackerappname), core::mem::transmute(&sztrackerctxname)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TrackerConfig: TrackerConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceTrackerConfig {}
windows_core::imp::define_interface!(IServiceTransactionConfig, IServiceTransactionConfig_Vtbl, 0x59f4c2a3_d3d7_4a31_b6e4_6ab3177c50b9);
impl core::ops::Deref for IServiceTransactionConfig {
    type Target = IServiceTransactionConfigBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceTransactionConfig, windows_core::IUnknown, IServiceTransactionConfigBase);
impl IServiceTransactionConfig {
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn ConfigureBYOT<P0>(&self, pitxbyot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransaction>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConfigureBYOT)(windows_core::Interface::as_raw(self), pitxbyot.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfig_Vtbl {
    pub base__: IServiceTransactionConfigBase_Vtbl,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub ConfigureBYOT: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    ConfigureBYOT: usize,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceTransactionConfig_Impl: IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(&self, pitxbyot: windows_core::Ref<super::DistributedTransactionCoordinator::ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceTransactionConfig_Vtbl {
    pub const fn new<Identity: IServiceTransactionConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureBYOT<Identity: IServiceTransactionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitxbyot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfig_Impl::ConfigureBYOT(this, core::mem::transmute_copy(&pitxbyot)).into()
            }
        }
        Self { base__: IServiceTransactionConfigBase_Vtbl::new::<Identity, OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as windows_core::Interface>::IID || iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for IServiceTransactionConfig {}
windows_core::imp::define_interface!(IServiceTransactionConfigBase, IServiceTransactionConfigBase_Vtbl, 0x772b3fbe_6ffd_42fb_b5f8_8f9b260f3810);
windows_core::imp::interface_hierarchy!(IServiceTransactionConfigBase, windows_core::IUnknown);
impl IServiceTransactionConfigBase {
    pub unsafe fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureTransaction)(windows_core::Interface::as_raw(self), transactionconfig).ok() }
    }
    pub unsafe fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsolationLevel)(windows_core::Interface::as_raw(self), option).ok() }
    }
    pub unsafe fn TransactionTimeout(&self, ultimeoutsec: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransactionTimeout)(windows_core::Interface::as_raw(self), ultimeoutsec).ok() }
    }
    pub unsafe fn BringYourOwnTransaction<P0>(&self, sztipurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BringYourOwnTransaction)(windows_core::Interface::as_raw(self), sztipurl.param().abi()).ok() }
    }
    pub unsafe fn NewTransactionDescription<P0>(&self, sztxdesc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).NewTransactionDescription)(windows_core::Interface::as_raw(self), sztxdesc.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceTransactionConfigBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConfigureTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, CSC_TransactionConfig) -> windows_core::HRESULT,
    pub IsolationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, COMAdminTxIsolationLevelOptions) -> windows_core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BringYourOwnTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub NewTransactionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IServiceTransactionConfigBase_Impl: windows_core::IUnknownImpl {
    fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> windows_core::Result<()>;
    fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> windows_core::Result<()>;
    fn TransactionTimeout(&self, ultimeoutsec: u32) -> windows_core::Result<()>;
    fn BringYourOwnTransaction(&self, sztipurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn NewTransactionDescription(&self, sztxdesc: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IServiceTransactionConfigBase_Vtbl {
    pub const fn new<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConfigureTransaction<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfigBase_Impl::ConfigureTransaction(this, core::mem::transmute_copy(&transactionconfig)).into()
            }
        }
        unsafe extern "system" fn IsolationLevel<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfigBase_Impl::IsolationLevel(this, core::mem::transmute_copy(&option)).into()
            }
        }
        unsafe extern "system" fn TransactionTimeout<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultimeoutsec: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfigBase_Impl::TransactionTimeout(this, core::mem::transmute_copy(&ultimeoutsec)).into()
            }
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztipurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfigBase_Impl::BringYourOwnTransaction(this, core::mem::transmute(&sztipurl)).into()
            }
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztxdesc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceTransactionConfigBase_Impl::NewTransactionDescription(this, core::mem::transmute(&sztxdesc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Identity, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceTransactionConfigBase {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedProperty, ISharedProperty_Vtbl, 0x2a005c01_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedProperty {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedProperty, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedProperty {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, val: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedProperty_Impl: super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, val: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedProperty_Vtbl {
    pub const fn new<Identity: ISharedProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISharedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISharedProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISharedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISharedProperty_Impl::SetValue(this, core::mem::transmute(&val)).into()
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Value: Value::<Identity, OFFSET>, SetValue: SetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISharedProperty {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedPropertyGroup, ISharedPropertyGroup_Vtbl, 0x2a005c07_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedPropertyGroup {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedPropertyGroup, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroup {
    pub unsafe fn CreatePropertyByPosition(&self, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreatePropertyByPosition)(windows_core::Interface::as_raw(self), index, fexists as _, core::mem::transmute(ppprop)).ok() }
    }
    pub unsafe fn get_PropertyByPosition(&self, index: i32) -> windows_core::Result<ISharedProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyByPosition)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateProperty(&self, name: &windows_core::BSTR, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), fexists as _, core::mem::transmute(ppprop)).ok() }
    }
    pub unsafe fn get_Property(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreatePropertyByPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PropertyByPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroup_Impl: super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(&self, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: windows_core::OutRef<ISharedProperty>) -> windows_core::Result<()>;
    fn get_PropertyByPosition(&self, index: i32) -> windows_core::Result<ISharedProperty>;
    fn CreateProperty(&self, name: &windows_core::BSTR, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: windows_core::OutRef<ISharedProperty>) -> windows_core::Result<()>;
    fn get_Property(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedProperty>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedPropertyGroup_Vtbl {
    pub const fn new<Identity: ISharedPropertyGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISharedPropertyGroup_Impl::CreatePropertyByPosition(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
            }
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISharedPropertyGroup_Impl::get_PropertyByPosition(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISharedPropertyGroup_Impl::CreateProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
            }
        }
        unsafe extern "system" fn get_Property<Identity: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISharedPropertyGroup_Impl::get_Property(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        ppproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyByPosition: CreatePropertyByPosition::<Identity, OFFSET>,
            get_PropertyByPosition: get_PropertyByPosition::<Identity, OFFSET>,
            CreateProperty: CreateProperty::<Identity, OFFSET>,
            get_Property: get_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISharedPropertyGroup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISharedPropertyGroupManager, ISharedPropertyGroupManager_Vtbl, 0x2a005c0d_a5de_11cf_9e66_00aa00a3f464);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISharedPropertyGroupManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISharedPropertyGroupManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManager {
    pub unsafe fn CreatePropertyGroup(&self, name: &windows_core::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut Option<ISharedPropertyGroup>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreatePropertyGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), dwisomode as _, dwrelmode as _, fexists as _, core::mem::transmute(ppgroup)).ok() }
    }
    pub unsafe fn get_Group(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedPropertyGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Group)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPropertyGroupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreatePropertyGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut i32, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroupManager_Impl: super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(&self, name: &windows_core::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: windows_core::OutRef<ISharedPropertyGroup>) -> windows_core::Result<()>;
    fn get_Group(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedPropertyGroup>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedPropertyGroupManager_Vtbl {
    pub const fn new<Identity: ISharedPropertyGroupManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertyGroup<Identity: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISharedPropertyGroupManager_Impl::CreatePropertyGroup(this, core::mem::transmute(&name), core::mem::transmute_copy(&dwisomode), core::mem::transmute_copy(&dwrelmode), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppgroup)).into()
            }
        }
        unsafe extern "system" fn get_Group<Identity: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISharedPropertyGroupManager_Impl::get_Group(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISharedPropertyGroupManager_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyGroup: CreatePropertyGroup::<Identity, OFFSET>,
            get_Group: get_Group::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISharedPropertyGroupManager {}
windows_core::imp::define_interface!(ISystemAppEventData, ISystemAppEventData_Vtbl, 0xd6d48a3c_d5c5_49e7_8c74_99e4889ed52f);
windows_core::imp::interface_hierarchy!(ISystemAppEventData, windows_core::IUnknown);
impl ISystemAppEventData {
    pub unsafe fn Startup(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Startup)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnDataChanged(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &windows_core::BSTR, dwreason: u32, u64tracehandle: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDataChanged)(windows_core::Interface::as_raw(self), dwpid, dwmask, dwnumbersinks, core::mem::transmute_copy(bstrdwmethodmask), dwreason, u64tracehandle).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemAppEventData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Startup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, u32, u64) -> windows_core::HRESULT,
}
pub trait ISystemAppEventData_Impl: windows_core::IUnknownImpl {
    fn Startup(&self) -> windows_core::Result<()>;
    fn OnDataChanged(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &windows_core::BSTR, dwreason: u32, u64tracehandle: u64) -> windows_core::Result<()>;
}
impl ISystemAppEventData_Vtbl {
    pub const fn new<Identity: ISystemAppEventData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Startup<Identity: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemAppEventData_Impl::Startup(this).into()
            }
        }
        unsafe extern "system" fn OnDataChanged<Identity: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: *mut core::ffi::c_void, dwreason: u32, u64tracehandle: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemAppEventData_Impl::OnDataChanged(this, core::mem::transmute_copy(&dwpid), core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwnumbersinks), core::mem::transmute(&bstrdwmethodmask), core::mem::transmute_copy(&dwreason), core::mem::transmute_copy(&u64tracehandle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemAppEventData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISystemAppEventData {}
windows_core::imp::define_interface!(IThreadPoolKnobs, IThreadPoolKnobs_Vtbl, 0x51372af7_cae7_11cf_be81_00aa00a2fa25);
windows_core::imp::interface_hierarchy!(IThreadPoolKnobs, windows_core::IUnknown);
impl IThreadPoolKnobs {
    pub unsafe fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMaxThreads)(windows_core::Interface::as_raw(self), plcmaxthreads as _).ok() }
    }
    pub unsafe fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentThreads)(windows_core::Interface::as_raw(self), plccurrentthreads as _).ok() }
    }
    pub unsafe fn SetMaxThreads(&self, lcmaxthreads: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxThreads)(windows_core::Interface::as_raw(self), lcmaxthreads).ok() }
    }
    pub unsafe fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeleteDelay)(windows_core::Interface::as_raw(self), pmsecdeletedelay as _).ok() }
    }
    pub unsafe fn SetDeleteDelay(&self, msecdeletedelay: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeleteDelay)(windows_core::Interface::as_raw(self), msecdeletedelay).ok() }
    }
    pub unsafe fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMaxQueuedRequests)(windows_core::Interface::as_raw(self), plcmaxqueuedrequests as _).ok() }
    }
    pub unsafe fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentQueuedRequests)(windows_core::Interface::as_raw(self), plccurrentqueuedrequests as _).ok() }
    }
    pub unsafe fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxQueuedRequests)(windows_core::Interface::as_raw(self), lcmaxqueuedrequests).ok() }
    }
    pub unsafe fn SetMinThreads(&self, lcminthreads: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinThreads)(windows_core::Interface::as_raw(self), lcminthreads).ok() }
    }
    pub unsafe fn SetQueueDepth(&self, lcqueuedepth: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQueueDepth)(windows_core::Interface::as_raw(self), lcqueuedepth).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolKnobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxThreads: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetDeleteDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeleteDelay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetMaxQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxQueuedRequests: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetMinThreads: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetQueueDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IThreadPoolKnobs_Impl: windows_core::IUnknownImpl {
    fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, lcmaxthreads: i32) -> windows_core::Result<()>;
    fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> windows_core::Result<()>;
    fn SetDeleteDelay(&self, msecdeletedelay: i32) -> windows_core::Result<()>;
    fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> windows_core::Result<()>;
    fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> windows_core::Result<()>;
    fn SetMinThreads(&self, lcminthreads: i32) -> windows_core::Result<()>;
    fn SetQueueDepth(&self, lcqueuedepth: i32) -> windows_core::Result<()>;
}
impl IThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaxThreads<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxthreads: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::GetMaxThreads(this, core::mem::transmute_copy(&plcmaxthreads)).into()
            }
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentthreads: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::GetCurrentThreads(this, core::mem::transmute_copy(&plccurrentthreads)).into()
            }
        }
        unsafe extern "system" fn SetMaxThreads<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxthreads: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::SetMaxThreads(this, core::mem::transmute_copy(&lcmaxthreads)).into()
            }
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsecdeletedelay: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::GetDeleteDelay(this, core::mem::transmute_copy(&pmsecdeletedelay)).into()
            }
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msecdeletedelay: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::SetDeleteDelay(this, core::mem::transmute_copy(&msecdeletedelay)).into()
            }
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::GetMaxQueuedRequests(this, core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
            }
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::GetCurrentQueuedRequests(this, core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
            }
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxqueuedrequests: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::SetMaxQueuedRequests(this, core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
            }
        }
        unsafe extern "system" fn SetMinThreads<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcminthreads: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::SetMinThreads(this, core::mem::transmute_copy(&lcminthreads)).into()
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcqueuedepth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&lcqueuedepth)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Identity, OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Identity, OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Identity, OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Identity, OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IThreadPoolKnobs {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITransactionContext, ITransactionContext_Vtbl, 0x7999fc21_d3c6_11cf_acab_00a024a55aef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITransactionContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITransactionContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContext {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateInstance(&self, pszprogid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pszprogid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateInstance: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITransactionContext_Impl: super::Com::IDispatch_Impl {
    fn CreateInstance(&self, pszprogid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITransactionContext_Vtbl {
    pub const fn new<Identity: ITransactionContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprogid: *mut core::ffi::c_void, pobject: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionContext_Impl::CreateInstance(this, core::mem::transmute(&pszprogid)) {
                    Ok(ok__) => {
                        pobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Commit<Identity: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionContext_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionContext_Impl::Abort(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITransactionContext {}
windows_core::imp::define_interface!(ITransactionContextEx, ITransactionContextEx_Vtbl, 0x7999fc22_d3c6_11cf_acab_00a024a55aef);
windows_core::imp::interface_hierarchy!(ITransactionContextEx, windows_core::IUnknown);
impl ITransactionContextEx {
    pub unsafe fn CreateInstance<T>(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionContextEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionContextEx_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl ITransactionContextEx_Vtbl {
    pub const fn new<Identity: ITransactionContextEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionContextEx_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionContextEx_Impl::Commit(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionContextEx_Impl::Abort(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionContextEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionContextEx {}
windows_core::imp::define_interface!(ITransactionProperty, ITransactionProperty_Vtbl, 0x788ea814_87b1_11d1_bba6_00c04fc2fa5f);
windows_core::imp::interface_hierarchy!(ITransactionProperty, windows_core::IUnknown);
impl ITransactionProperty {
    pub unsafe fn Reserved1(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved2(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved2)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved3(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved3)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved4(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved4)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved5(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved5)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved6(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved6)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved7(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved7)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved8(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved8)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved9(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved9)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTransactionResourcePool(&self) -> windows_core::Result<ITransactionResourcePool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransactionResourcePool)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Reserved10(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved10)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved11(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved11)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved12(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved12)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved13(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved13)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved14(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved14)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved15(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved15)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved16(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved16)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reserved17(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reserved17)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved2: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved3: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved4: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved5: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved6: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved7: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved8: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved9: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetTransactionResourcePool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reserved10: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved11: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved12: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved13: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved14: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved15: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved16: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Reserved17: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait ITransactionProperty_Impl: windows_core::IUnknownImpl {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn Reserved5(&self);
    fn Reserved6(&self);
    fn Reserved7(&self);
    fn Reserved8(&self);
    fn Reserved9(&self);
    fn GetTransactionResourcePool(&self) -> windows_core::Result<ITransactionResourcePool>;
    fn Reserved10(&self);
    fn Reserved11(&self);
    fn Reserved12(&self);
    fn Reserved13(&self);
    fn Reserved14(&self);
    fn Reserved15(&self);
    fn Reserved16(&self);
    fn Reserved17(&self);
}
impl ITransactionProperty_Vtbl {
    pub const fn new<Identity: ITransactionProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reserved1<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved1(this)
            }
        }
        unsafe extern "system" fn Reserved2<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved2(this)
            }
        }
        unsafe extern "system" fn Reserved3<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved3(this)
            }
        }
        unsafe extern "system" fn Reserved4<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved4(this)
            }
        }
        unsafe extern "system" fn Reserved5<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved5(this)
            }
        }
        unsafe extern "system" fn Reserved6<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved6(this)
            }
        }
        unsafe extern "system" fn Reserved7<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved7(this)
            }
        }
        unsafe extern "system" fn Reserved8<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved8(this)
            }
        }
        unsafe extern "system" fn Reserved9<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved9(this)
            }
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxpool: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionProperty_Impl::GetTransactionResourcePool(this) {
                    Ok(ok__) => {
                        pptxpool.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reserved10<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved10(this)
            }
        }
        unsafe extern "system" fn Reserved11<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved11(this)
            }
        }
        unsafe extern "system" fn Reserved12<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved12(this)
            }
        }
        unsafe extern "system" fn Reserved13<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved13(this)
            }
        }
        unsafe extern "system" fn Reserved14<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved14(this)
            }
        }
        unsafe extern "system" fn Reserved15<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved15(this)
            }
        }
        unsafe extern "system" fn Reserved16<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved16(this)
            }
        }
        unsafe extern "system" fn Reserved17<Identity: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProperty_Impl::Reserved17(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            Reserved3: Reserved3::<Identity, OFFSET>,
            Reserved4: Reserved4::<Identity, OFFSET>,
            Reserved5: Reserved5::<Identity, OFFSET>,
            Reserved6: Reserved6::<Identity, OFFSET>,
            Reserved7: Reserved7::<Identity, OFFSET>,
            Reserved8: Reserved8::<Identity, OFFSET>,
            Reserved9: Reserved9::<Identity, OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Identity, OFFSET>,
            Reserved10: Reserved10::<Identity, OFFSET>,
            Reserved11: Reserved11::<Identity, OFFSET>,
            Reserved12: Reserved12::<Identity, OFFSET>,
            Reserved13: Reserved13::<Identity, OFFSET>,
            Reserved14: Reserved14::<Identity, OFFSET>,
            Reserved15: Reserved15::<Identity, OFFSET>,
            Reserved16: Reserved16::<Identity, OFFSET>,
            Reserved17: Reserved17::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionProperty as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionProperty {}
windows_core::imp::define_interface!(ITransactionProxy, ITransactionProxy_Vtbl, 0x02558374_df2e_4dae_bd6b_1d5c994f9bdc);
windows_core::imp::interface_hierarchy!(ITransactionProxy, windows_core::IUnknown);
impl ITransactionProxy {
    pub unsafe fn Commit(&self, guid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute(guid)).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn Promote(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Promote)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn CreateVoter<P0>(&self, ptxasync: P0) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>
    where
        P0: windows_core::Param<super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVoter)(windows_core::Interface::as_raw(self), ptxasync.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIsolationLevel)(windows_core::Interface::as_raw(self), __midl__itransactionproxy0000 as _).ok() }
    }
    pub unsafe fn GetIdentifier(&self, pbstridentifier: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), pbstridentifier as _).ok() }
    }
    pub unsafe fn IsReusable(&self, pfisreusable: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsReusable)(windows_core::Interface::as_raw(self), pfisreusable as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub Promote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    Promote: usize,
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub CreateVoter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_DistributedTransactionCoordinator"))]
    CreateVoter: usize,
    pub GetIsolationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsReusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionProxy_Impl: windows_core::IUnknownImpl {
    fn Commit(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn Promote(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(&self, ptxasync: windows_core::Ref<super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> windows_core::Result<()>;
    fn GetIdentifier(&self, pbstridentifier: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn IsReusable(&self, pfisreusable: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionProxy_Vtbl {
    pub const fn new<Identity: ITransactionProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Commit<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProxy_Impl::Commit(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProxy_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn Promote<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionProxy_Impl::Promote(this) {
                    Ok(ok__) => {
                        ptransaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVoter<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxasync: *mut core::ffi::c_void, ppballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionProxy_Impl::CreateVoter(this, core::mem::transmute_copy(&ptxasync)) {
                    Ok(ok__) => {
                        ppballot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProxy_Impl::GetIsolationLevel(this, core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
            }
        }
        unsafe extern "system" fn GetIdentifier<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstridentifier: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProxy_Impl::GetIdentifier(this, core::mem::transmute_copy(&pbstridentifier)).into()
            }
        }
        unsafe extern "system" fn IsReusable<Identity: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisreusable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionProxy_Impl::IsReusable(this, core::mem::transmute_copy(&pfisreusable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Promote: Promote::<Identity, OFFSET>,
            CreateVoter: CreateVoter::<Identity, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, OFFSET>,
            IsReusable: IsReusable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionProxy as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ITransactionProxy {}
windows_core::imp::define_interface!(ITransactionResourcePool, ITransactionResourcePool_Vtbl, 0xc5feb7c1_346a_11d1_b1cc_00aa00ba3258);
windows_core::imp::interface_hierarchy!(ITransactionResourcePool, windows_core::IUnknown);
impl ITransactionResourcePool {
    pub unsafe fn PutResource<P0, P1>(&self, ppool: P0, punk: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObjPool>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutResource)(windows_core::Interface::as_raw(self), ppool.param().abi(), punk.param().abi()).ok() }
    }
    pub unsafe fn GetResource<P0>(&self, ppool: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IObjPool>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), ppool.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionResourcePool_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITransactionResourcePool_Impl: windows_core::IUnknownImpl {
    fn PutResource(&self, ppool: windows_core::Ref<IObjPool>, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetResource(&self, ppool: windows_core::Ref<IObjPool>) -> windows_core::Result<windows_core::IUnknown>;
}
impl ITransactionResourcePool_Vtbl {
    pub const fn new<Identity: ITransactionResourcePool_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PutResource<Identity: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionResourcePool_Impl::PutResource(this, core::mem::transmute_copy(&ppool), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn GetResource<Identity: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransactionResourcePool_Impl::GetResource(this, core::mem::transmute_copy(&ppool)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutResource: PutResource::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResourcePool as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionResourcePool {}
windows_core::imp::define_interface!(ITransactionStatus, ITransactionStatus_Vtbl, 0x61f589e8_3724_4898_a0a4_664ae9e1d1b4);
windows_core::imp::interface_hierarchy!(ITransactionStatus, windows_core::IUnknown);
impl ITransactionStatus {
    pub unsafe fn SetTransactionStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransactionStatus)(windows_core::Interface::as_raw(self), hrstatus).ok() }
    }
    pub unsafe fn GetTransactionStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTransactionStatus)(windows_core::Interface::as_raw(self), phrstatus as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransactionStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTransactionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetTransactionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITransactionStatus_Impl: windows_core::IUnknownImpl {
    fn SetTransactionStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetTransactionStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITransactionStatus_Vtbl {
    pub const fn new<Identity: ITransactionStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTransactionStatus<Identity: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionStatus_Impl::SetTransactionStatus(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransactionStatus_Impl::GetTransactionStatus(this, core::mem::transmute_copy(&phrstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Identity, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransactionStatus {}
windows_core::imp::define_interface!(ITxProxyHolder, ITxProxyHolder_Vtbl, 0x13d86f31_0139_41af_bcad_c7d50435fe9f);
windows_core::imp::interface_hierarchy!(ITxProxyHolder, windows_core::IUnknown);
impl ITxProxyHolder {
    pub unsafe fn GetIdentifier(&self, pguidltx: *mut windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), pguidltx as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITxProxyHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID),
}
pub trait ITxProxyHolder_Impl: windows_core::IUnknownImpl {
    fn GetIdentifier(&self, pguidltx: *mut windows_core::GUID);
}
impl ITxProxyHolder_Vtbl {
    pub const fn new<Identity: ITxProxyHolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentifier<Identity: ITxProxyHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidltx: *mut windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITxProxyHolder_Impl::GetIdentifier(this, core::mem::transmute_copy(&pguidltx))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITxProxyHolder as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITxProxyHolder {}
pub const LBEvents: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
pub const LockMethod: LockModes = LockModes(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LockModes(pub i32);
pub const LockSetGet: LockModes = LockModes(0i32);
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub const MessageMover: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: windows_core::GUID = windows_core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ObjectContext, ObjectContext_Vtbl, 0x74c08646_cedb_11cf_8b49_00aa00b8a790);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ObjectContext {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ObjectContext, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ObjectContext {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateInstance(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprogid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnableCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableCommit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableCommit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInTransaction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSecurityEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCallerInRole)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrrole), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Security(&self) -> windows_core::Result<SecurityProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ContextInfo(&self) -> windows_core::Result<ContextInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContextInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ObjectContext_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateInstance: usize,
    pub SetComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCommit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsInTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsSecurityEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsCallerInRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContextInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ObjectContext_Impl: super::Com::IDispatch_Impl {
    fn CreateInstance(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetComplete(&self) -> windows_core::Result<()>;
    fn SetAbort(&self) -> windows_core::Result<()>;
    fn EnableCommit(&self) -> windows_core::Result<()>;
    fn DisableCommit(&self) -> windows_core::Result<()>;
    fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Security(&self) -> windows_core::Result<SecurityProperty>;
    fn ContextInfo(&self) -> windows_core::Result<ContextInfo>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ObjectContext_Vtbl {
    pub const fn new<Identity: ObjectContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprogid: *mut core::ffi::c_void, pobject: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::CreateInstance(this, core::mem::transmute(&bstrprogid)) {
                    Ok(ok__) => {
                        pobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetComplete<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectContext_Impl::SetComplete(this).into()
            }
        }
        unsafe extern "system" fn SetAbort<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectContext_Impl::SetAbort(this).into()
            }
        }
        unsafe extern "system" fn EnableCommit<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectContext_Impl::EnableCommit(this).into()
            }
        }
        unsafe extern "system" fn DisableCommit<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectContext_Impl::DisableCommit(this).into()
            }
        }
        unsafe extern "system" fn IsInTransaction<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::IsInTransaction(this) {
                    Ok(ok__) => {
                        pbisintx.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::IsSecurityEnabled(this) {
                    Ok(ok__) => {
                        pbisenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: *mut core::ffi::c_void, pbinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                    Ok(ok__) => {
                        pbinrole.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pitem: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::Security(this) {
                    Ok(ok__) => {
                        ppsecurityproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContextInfo<Identity: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontextinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ObjectContext_Impl::ContextInfo(this) {
                    Ok(ok__) => {
                        ppcontextinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            SetComplete: SetComplete::<Identity, OFFSET>,
            SetAbort: SetAbort::<Identity, OFFSET>,
            EnableCommit: EnableCommit::<Identity, OFFSET>,
            DisableCommit: DisableCommit::<Identity, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            ContextInfo: ContextInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ObjectContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ObjectContext {}
windows_core::imp::define_interface!(ObjectControl, ObjectControl_Vtbl, 0x7dc41850_0c31_11d0_8b79_00aa00b8a790);
windows_core::imp::interface_hierarchy!(ObjectControl, windows_core::IUnknown);
impl ObjectControl {
    pub unsafe fn Activate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CanBePooled(&self, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CanBePooled)(windows_core::Interface::as_raw(self), pbpoolable as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ObjectControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanBePooled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait ObjectControl_Impl: windows_core::IUnknownImpl {
    fn Activate(&self) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CanBePooled(&self, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl ObjectControl_Vtbl {
    pub const fn new<Identity: ObjectControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectControl_Impl::Activate(this).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectControl_Impl::Deactivate(this).into()
            }
        }
        unsafe extern "system" fn CanBePooled<Identity: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ObjectControl_Impl::CanBePooled(this, core::mem::transmute_copy(&pbpoolable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CanBePooled: CanBePooled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ObjectControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ObjectControl {}
pub const PoolMgr: windows_core::GUID = windows_core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
pub const Process: ReleaseModes = ReleaseModes(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: windows_core::GUID,
    pub ProcessStartTime: i64,
    pub dwRecycleLifetimeLimit: u32,
    pub dwRecycleMemoryLimit: u32,
    pub dwRecycleExpirationTimeout: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReleaseModes(pub i32);
pub const SecurityCallContext: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(SecurityProperty, SecurityProperty_Vtbl, 0xe74a7215_014d_11d1_a63c_00a0c911b4e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for SecurityProperty {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(SecurityProperty, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl SecurityProperty {
    pub unsafe fn GetDirectCallerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirectCallerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDirectCreatorName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirectCreatorName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetOriginalCallerName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginalCallerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetOriginalCreatorName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginalCreatorName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SecurityProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDirectCallerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDirectCreatorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOriginalCallerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOriginalCreatorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SecurityProperty_Impl: super::Com::IDispatch_Impl {
    fn GetDirectCallerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDirectCreatorName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetOriginalCallerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetOriginalCreatorName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SecurityProperty_Vtbl {
    pub const fn new<Identity: SecurityProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDirectCallerName<Identity: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SecurityProperty_Impl::GetDirectCallerName(this) {
                    Ok(ok__) => {
                        bstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SecurityProperty_Impl::GetDirectCreatorName(this) {
                    Ok(ok__) => {
                        bstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SecurityProperty_Impl::GetOriginalCallerName(this) {
                    Ok(ok__) => {
                        bstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match SecurityProperty_Impl::GetOriginalCreatorName(this) {
                    Ok(ok__) => {
                        bstrusername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCallerName: GetDirectCallerName::<Identity, OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Identity, OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Identity, OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<SecurityProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for SecurityProperty {}
pub const ServicePool: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: windows_core::GUID = windows_core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: windows_core::GUID = windows_core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
pub const Standard: ReleaseModes = ReleaseModes(0i32);
pub const TRACKER_INIT_EVENT: windows_core::PCWSTR = windows_core::w!("Global\\COM+ Tracker Init Event");
pub const TRACKER_STARTSTOP_EVENT: windows_core::PCWSTR = windows_core::w!("Global\\COM+ Tracker Push Event");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACKING_COLL_TYPE(pub i32);
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(1i32);
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(2i32);
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = TRACKING_COLL_TYPE(0i32);
pub const TrackerServer: windows_core::GUID = windows_core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: windows_core::GUID = windows_core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: windows_core::GUID = windows_core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TransactionVote(pub i32);
pub const TxAbort: TransactionVote = TransactionVote(1i32);
pub const TxCommit: TransactionVote = TransactionVote(0i32);
pub const TxState_Aborted: CrmTransactionState = CrmTransactionState(2i32);
pub const TxState_Active: CrmTransactionState = CrmTransactionState(0i32);
pub const TxState_Committed: CrmTransactionState = CrmTransactionState(1i32);
pub const TxState_Indoubt: CrmTransactionState = CrmTransactionState(3i32);
pub const comQCErrApplicationNotQueued: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599296u32);
pub const comQCErrNoQueueableInterfaces: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599297u32);
pub const comQCErrQueueTransactMismatch: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599299u32);
pub const comQCErrQueuingServiceNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599298u32);
pub const comqcErrBadMarshaledObject: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599382u32);
pub const comqcErrInvalidMessage: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599376u32);
pub const comqcErrMarshaledObjSameTxn: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599304u32);
pub const comqcErrMsgNotAuthenticated: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599380u32);
pub const comqcErrMsmqConnectorUsed: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599381u32);
pub const comqcErrMsmqServiceUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599379u32);
pub const comqcErrMsmqSidUnavailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599377u32);
pub const comqcErrOutParam: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599301u32);
pub const comqcErrPSLoad: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599303u32);
pub const comqcErrRecorderMarshalled: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599300u32);
pub const comqcErrRecorderNotTrusted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599302u32);
pub const comqcErrWrongMsgExtension: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2148599378u32);
pub const mtsErrCtxAborted: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803138u32);
pub const mtsErrCtxAborting: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803139u32);
pub const mtsErrCtxNoContext: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803140u32);
pub const mtsErrCtxNoSecurity: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803149u32);
pub const mtsErrCtxNotRegistered: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803141u32);
pub const mtsErrCtxOldReference: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803143u32);
pub const mtsErrCtxRoleNotFound: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803148u32);
pub const mtsErrCtxSynchTimeout: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803142u32);
pub const mtsErrCtxTMNotAvailable: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803151u32);
pub const mtsErrCtxWrongThread: AutoSvcs_Error_Constants = AutoSvcs_Error_Constants(2147803150u32);
