#[inline]
pub unsafe fn HcsAddResourceToOperation<P2>(operation: HCS_OPERATION, r#type: HCS_RESOURCE_TYPE, uri: P2, handle: super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsAddResourceToOperation(operation : HCS_OPERATION, r#type : HCS_RESOURCE_TYPE, uri : windows_core::PCWSTR, handle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { HcsAddResourceToOperation(operation, r#type, uri.param().abi(), handle).ok() }
}
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<P0, P1>(layerpath: P0, layerdata: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsAttachLayerStorageFilter(layerpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsAttachLayerStorageFilter(layerpath.param().abi(), layerdata.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsCancelOperation(operation: HCS_OPERATION) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsCancelOperation(operation : HCS_OPERATION) -> windows_core::HRESULT);
    unsafe { HcsCancelOperation(operation).ok() }
}
#[inline]
pub unsafe fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM) {
    windows_core::link!("computecore.dll" "system" fn HcsCloseComputeSystem(computesystem : HCS_SYSTEM));
    unsafe { HcsCloseComputeSystem(computesystem) }
}
#[inline]
pub unsafe fn HcsCloseOperation(operation: HCS_OPERATION) {
    windows_core::link!("computecore.dll" "system" fn HcsCloseOperation(operation : HCS_OPERATION));
    unsafe { HcsCloseOperation(operation) }
}
#[inline]
pub unsafe fn HcsCloseProcess(process: HCS_PROCESS) {
    windows_core::link!("computecore.dll" "system" fn HcsCloseProcess(process : HCS_PROCESS));
    unsafe { HcsCloseProcess(process) }
}
#[inline]
pub unsafe fn HcsCrashComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCrashComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsCrashComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn HcsCreateComputeSystem<P0, P1>(id: P0, configuration: P1, operation: HCS_OPERATION, securitydescriptor: Option<*const super::super::Security::SECURITY_DESCRIPTOR>) -> windows_core::Result<HCS_SYSTEM>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCreateComputeSystem(id : windows_core::PCWSTR, configuration : windows_core::PCWSTR, operation : HCS_OPERATION, securitydescriptor : *const super::super::Security:: SECURITY_DESCRIPTOR, computesystem : *mut HCS_SYSTEM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsCreateComputeSystem(id.param().abi(), configuration.param().abi(), operation, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<P0, P1, P2>(idnamespace: P0, id: P1, configuration: P2, operation: HCS_OPERATION, options: Option<*const HCS_CREATE_OPTIONS>) -> windows_core::Result<HCS_SYSTEM>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCreateComputeSystemInNamespace(idnamespace : windows_core::PCWSTR, id : windows_core::PCWSTR, configuration : windows_core::PCWSTR, operation : HCS_OPERATION, options : *const HCS_CREATE_OPTIONS, computesystem : *mut HCS_SYSTEM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsCreateComputeSystemInNamespace(idnamespace.param().abi(), id.param().abi(), configuration.param().abi(), operation, options.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<P0>(gueststatefilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCreateEmptyGuestStateFile(gueststatefilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsCreateEmptyGuestStateFile(gueststatefilepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<P0>(runtimestatefilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsCreateOperation(context: Option<*const core::ffi::c_void>, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION {
    windows_core::link!("computecore.dll" "system" fn HcsCreateOperation(context : *const core::ffi::c_void, callback : HCS_OPERATION_COMPLETION) -> HCS_OPERATION);
    unsafe { HcsCreateOperation(context.unwrap_or(core::mem::zeroed()) as _, callback) }
}
#[inline]
pub unsafe fn HcsCreateOperationWithNotifications(eventtypes: HCS_OPERATION_OPTIONS, context: Option<*const core::ffi::c_void>, callback: HCS_EVENT_CALLBACK) -> HCS_OPERATION {
    windows_core::link!("computecore.dll" "system" fn HcsCreateOperationWithNotifications(eventtypes : HCS_OPERATION_OPTIONS, context : *const core::ffi::c_void, callback : HCS_EVENT_CALLBACK) -> HCS_OPERATION);
    unsafe { HcsCreateOperationWithNotifications(eventtypes, context.unwrap_or(core::mem::zeroed()) as _, callback) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn HcsCreateProcess<P1>(computesystem: HCS_SYSTEM, processparameters: P1, operation: HCS_OPERATION, securitydescriptor: Option<*const super::super::Security::SECURITY_DESCRIPTOR>) -> windows_core::Result<HCS_PROCESS>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsCreateProcess(computesystem : HCS_SYSTEM, processparameters : windows_core::PCWSTR, operation : HCS_OPERATION, securitydescriptor : *const super::super::Security:: SECURITY_DESCRIPTOR, process : *mut HCS_PROCESS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsCreateProcess(computesystem, processparameters.param().abi(), operation, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsDestroyLayer<P0>(layerpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsDestroyLayer(layerpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsDestroyLayer(layerpath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<P0>(layerpath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsDetachLayerStorageFilter(layerpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsDetachLayerStorageFilter(layerpath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<P0>(query: P0, operation: HCS_OPERATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsEnumerateComputeSystems(query : windows_core::PCWSTR, operation : HCS_OPERATION) -> windows_core::HRESULT);
    unsafe { HcsEnumerateComputeSystems(query.param().abi(), operation).ok() }
}
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<P0, P1>(idnamespace: P0, query: P1, operation: HCS_OPERATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsEnumerateComputeSystemsInNamespace(idnamespace : windows_core::PCWSTR, query : windows_core::PCWSTR, operation : HCS_OPERATION) -> windows_core::HRESULT);
    unsafe { HcsEnumerateComputeSystemsInNamespace(idnamespace.param().abi(), query.param().abi(), operation).ok() }
}
#[inline]
pub unsafe fn HcsExportLayer<P0, P1, P2, P3>(layerpath: P0, exportfolderpath: P1, layerdata: P2, options: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsExportLayer(layerpath : windows_core::PCWSTR, exportfolderpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsExportLayer(layerpath.param().abi(), exportfolderpath.param().abi(), layerdata.param().abi(), options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, exportfolderpath: P2, layerdata: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsExportLegacyWritableLayer(writablelayermountpath : windows_core::PCWSTR, writablelayerfolderpath : windows_core::PCWSTR, exportfolderpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsExportLegacyWritableLayer(writablelayermountpath.param().abi(), writablelayerfolderpath.param().abi(), exportfolderpath.param().abi(), layerdata.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("computestorage.dll" "system" fn HcsFormatWritableLayerVhd(vhdhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { HcsFormatWritableLayerVhd(vhdhandle).ok() }
}
#[inline]
pub unsafe fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM {
    windows_core::link!("computecore.dll" "system" fn HcsGetComputeSystemFromOperation(operation : HCS_OPERATION) -> HCS_SYSTEM);
    unsafe { HcsGetComputeSystemFromOperation(operation) }
}
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGetComputeSystemProperties(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, propertyquery : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsGetComputeSystemProperties(computesystem, operation, propertyquery.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("computestorage.dll" "system" fn HcsGetLayerVhdMountPath(vhdhandle : super::super::Foundation:: HANDLE, mountpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsGetLayerVhdMountPath(vhdhandle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut core::ffi::c_void {
    windows_core::link!("computecore.dll" "system" fn HcsGetOperationContext(operation : HCS_OPERATION) -> *mut core::ffi::c_void);
    unsafe { HcsGetOperationContext(operation) }
}
#[inline]
pub unsafe fn HcsGetOperationId(operation: HCS_OPERATION) -> u64 {
    windows_core::link!("computecore.dll" "system" fn HcsGetOperationId(operation : HCS_OPERATION) -> u64);
    unsafe { HcsGetOperationId(operation) }
}
#[inline]
pub unsafe fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsGetOperationResult(operation : HCS_OPERATION, resultdocument : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsGetOperationResult(operation, resultdocument.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: Option<*mut HCS_PROCESS_INFORMATION>, resultdocument: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsGetOperationResultAndProcessInfo(operation : HCS_OPERATION, processinformation : *mut HCS_PROCESS_INFORMATION, resultdocument : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsGetOperationResultAndProcessInfo(operation, processinformation.unwrap_or(core::mem::zeroed()) as _, resultdocument.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE {
    windows_core::link!("computecore.dll" "system" fn HcsGetOperationType(operation : HCS_OPERATION) -> HCS_OPERATION_TYPE);
    unsafe { HcsGetOperationType(operation) }
}
#[inline]
pub unsafe fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS {
    windows_core::link!("computecore.dll" "system" fn HcsGetProcessFromOperation(operation : HCS_OPERATION) -> HCS_PROCESS);
    unsafe { HcsGetProcessFromOperation(operation) }
}
#[inline]
pub unsafe fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsGetProcessInfo(process : HCS_PROCESS, operation : HCS_OPERATION) -> windows_core::HRESULT);
    unsafe { HcsGetProcessInfo(process, operation).ok() }
}
#[inline]
pub unsafe fn HcsGetProcessProperties<P2>(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGetProcessProperties(process : HCS_PROCESS, operation : HCS_OPERATION, propertyquery : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsGetProcessProperties(process, operation, propertyquery.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<P0>(runtimefilename: P0, processorfeaturesstring: Option<*mut windows_core::PCWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename : windows_core::PCWSTR, processorfeaturesstring : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsGetProcessorCompatibilityFromSavedState(runtimefilename.param().abi(), processorfeaturesstring.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsGetServiceProperties<P0>(propertyquery: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGetServiceProperties(propertyquery : windows_core::PCWSTR, result : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsGetServiceProperties(propertyquery.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsGrantVmAccess<P0, P1>(vmid: P0, filepath: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGrantVmAccess(vmid : windows_core::PCWSTR, filepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsGrantVmAccess(vmid.param().abi(), filepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<P0>(filepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsGrantVmGroupAccess(filepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsGrantVmGroupAccess(filepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsImportLayer<P0, P1, P2>(layerpath: P0, sourcefolderpath: P1, layerdata: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsImportLayer(layerpath : windows_core::PCWSTR, sourcefolderpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsImportLayer(layerpath.param().abi(), sourcefolderpath.param().abi(), layerdata.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, layerdata: P2, options: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsInitializeLegacyWritableLayer(writablelayermountpath : windows_core::PCWSTR, writablelayerfolderpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsInitializeLegacyWritableLayer(writablelayermountpath.param().abi(), writablelayerfolderpath.param().abi(), layerdata.param().abi(), options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsInitializeWritableLayer<P0, P1, P2>(writablelayerpath: P0, layerdata: P1, options: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsInitializeWritableLayer(writablelayerpath : windows_core::PCWSTR, layerdata : windows_core::PCWSTR, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsInitializeWritableLayer(writablelayerpath.param().abi(), layerdata.param().abi(), options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsModifyComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: P2, identity: Option<super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsModifyComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, configuration : windows_core::PCWSTR, identity : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { HcsModifyComputeSystem(computesystem, operation, configuration.param().abi(), identity.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsModifyProcess<P2>(process: HCS_PROCESS, operation: HCS_OPERATION, settings: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsModifyProcess(process : HCS_PROCESS, operation : HCS_OPERATION, settings : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsModifyProcess(process, operation, settings.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsModifyServiceSettings<P0>(settings: P0, result: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsModifyServiceSettings(settings : windows_core::PCWSTR, result : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsModifyServiceSettings(settings.param().abi(), result.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsOpenComputeSystem<P0>(id: P0, requestedaccess: u32) -> windows_core::Result<HCS_SYSTEM>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsOpenComputeSystem(id : windows_core::PCWSTR, requestedaccess : u32, computesystem : *mut HCS_SYSTEM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsOpenComputeSystem(id.param().abi(), requestedaccess, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<P0, P1>(idnamespace: P0, id: P1, requestedaccess: u32) -> windows_core::Result<HCS_SYSTEM>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsOpenComputeSystemInNamespace(idnamespace : windows_core::PCWSTR, id : windows_core::PCWSTR, requestedaccess : u32, computesystem : *mut HCS_SYSTEM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsOpenComputeSystemInNamespace(idnamespace.param().abi(), id.param().abi(), requestedaccess, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32) -> windows_core::Result<HCS_PROCESS> {
    windows_core::link!("computecore.dll" "system" fn HcsOpenProcess(computesystem : HCS_SYSTEM, processid : u32, requestedaccess : u32, process : *mut HCS_PROCESS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcsOpenProcess(computesystem, processid, requestedaccess, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcsPauseComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsPauseComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsPauseComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsResumeComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsResumeComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsResumeComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsRevokeVmAccess<P0, P1>(vmid: P0, filepath: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsRevokeVmAccess(vmid : windows_core::PCWSTR, filepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsRevokeVmAccess(vmid.param().abi(), filepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<P0>(filepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsRevokeVmGroupAccess(filepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsRevokeVmGroupAccess(filepath.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsSaveComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsSaveComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsSaveComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: Option<*const core::ffi::c_void>, callback: HCS_EVENT_CALLBACK) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsSetComputeSystemCallback(computesystem : HCS_SYSTEM, callbackoptions : HCS_EVENT_OPTIONS, context : *const core::ffi::c_void, callback : HCS_EVENT_CALLBACK) -> windows_core::HRESULT);
    unsafe { HcsSetComputeSystemCallback(computesystem, callbackoptions, context.unwrap_or(core::mem::zeroed()) as _, callback).ok() }
}
#[inline]
pub unsafe fn HcsSetOperationCallback(operation: HCS_OPERATION, context: Option<*const core::ffi::c_void>, callback: HCS_OPERATION_COMPLETION) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsSetOperationCallback(operation : HCS_OPERATION, context : *const core::ffi::c_void, callback : HCS_OPERATION_COMPLETION) -> windows_core::HRESULT);
    unsafe { HcsSetOperationCallback(operation, context.unwrap_or(core::mem::zeroed()) as _, callback).ok() }
}
#[inline]
pub unsafe fn HcsSetOperationContext(operation: HCS_OPERATION, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsSetOperationContext(operation : HCS_OPERATION, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcsSetOperationContext(operation, context.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsSetProcessCallback(process : HCS_PROCESS, callbackoptions : HCS_EVENT_OPTIONS, context : *const core::ffi::c_void, callback : HCS_EVENT_CALLBACK) -> windows_core::HRESULT);
    unsafe { HcsSetProcessCallback(process, callbackoptions, context, callback).ok() }
}
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<P0, P2>(layerpath: P0, vhdhandle: super::super::Foundation::HANDLE, options: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsSetupBaseOSLayer(layerpath : windows_core::PCWSTR, vhdhandle : super::super::Foundation:: HANDLE, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsSetupBaseOSLayer(layerpath.param().abi(), vhdhandle, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<P0, P1, P2>(layerpath: P0, volumepath: P1, options: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computestorage.dll" "system" fn HcsSetupBaseOSVolume(layerpath : windows_core::PCWSTR, volumepath : windows_core::PCWSTR, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsSetupBaseOSVolume(layerpath.param().abi(), volumepath.param().abi(), options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsShutDownComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsShutDownComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsShutDownComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsSignalProcess<P2>(process: HCS_PROCESS, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsSignalProcess(process : HCS_PROCESS, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsSignalProcess(process, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsStartComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsStartComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsStartComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsSubmitWerReport<P0>(settings: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsSubmitWerReport(settings : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsSubmitWerReport(settings.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsTerminateComputeSystem<P2>(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsTerminateComputeSystem(computesystem : HCS_SYSTEM, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsTerminateComputeSystem(computesystem, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsTerminateProcess<P2>(process: HCS_PROCESS, operation: HCS_OPERATION, options: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computecore.dll" "system" fn HcsTerminateProcess(process : HCS_PROCESS, operation : HCS_OPERATION, options : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HcsTerminateProcess(process, operation, options.param().abi()).ok() }
}
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsWaitForComputeSystemExit(computesystem : HCS_SYSTEM, timeoutms : u32, result : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsWaitForComputeSystemExit(computesystem, timeoutms, result.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsWaitForOperationResult(operation : HCS_OPERATION, timeoutms : u32, resultdocument : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsWaitForOperationResult(operation, timeoutms, resultdocument.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: Option<*mut HCS_PROCESS_INFORMATION>, resultdocument: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsWaitForOperationResultAndProcessInfo(operation : HCS_OPERATION, timeoutms : u32, processinformation : *mut HCS_PROCESS_INFORMATION, resultdocument : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsWaitForOperationResultAndProcessInfo(operation, timeoutms, processinformation.unwrap_or(core::mem::zeroed()) as _, resultdocument.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computecore.dll" "system" fn HcsWaitForProcessExit(computesystem : HCS_PROCESS, timeoutms : u32, result : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcsWaitForProcessExit(computesystem, timeoutms, result.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_CREATE_OPTIONS(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug)]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: super::super::Foundation::HANDLE,
    pub SecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut core::ffi::c_void,
    pub Callback: HCS_EVENT_CALLBACK,
}
#[cfg(feature = "Win32_Security")]
impl Default for HCS_CREATE_OPTIONS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: windows_core::PCWSTR,
    pub Operation: HCS_OPERATION,
}
pub type HCS_EVENT_CALLBACK = Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_EVENT_OPTIONS(pub i32);
impl HCS_EVENT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HCS_EVENT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HCS_EVENT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_EVENT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_NOTIFICATIONS(pub i32);
pub type HCS_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: u32, context: *const core::ffi::c_void, notificationstatus: windows_core::HRESULT, notificationdata: windows_core::PCWSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_NOTIFICATION_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_OPERATION(pub *mut core::ffi::c_void);
impl HCS_OPERATION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCS_OPERATION {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("computecore.dll" "system" fn HcsCloseOperation(operation : *mut core::ffi::c_void));
            unsafe {
                HcsCloseOperation(self.0);
            }
        }
    }
}
impl Default for HCS_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HCS_OPERATION_COMPLETION = Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_OPERATION_OPTIONS(pub i32);
impl HCS_OPERATION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HCS_OPERATION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HCS_OPERATION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HCS_OPERATION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HCS_OPERATION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HCS_OPERATION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_OPERATION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_PROCESS(pub *mut core::ffi::c_void);
impl HCS_PROCESS {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCS_PROCESS {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("computecore.dll" "system" fn HcsCloseProcess(process : *mut core::ffi::c_void));
            unsafe {
                HcsCloseProcess(self.0);
            }
        }
    }
}
impl Default for HCS_PROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: super::super::Foundation::HANDLE,
    pub StdOutput: super::super::Foundation::HANDLE,
    pub StdError: super::super::Foundation::HANDLE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCS_RESOURCE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_SYSTEM(pub *mut core::ffi::c_void);
impl HCS_SYSTEM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCS_SYSTEM {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("computecore.dll" "system" fn HcsCloseComputeSystem(computesystem : *mut core::ffi::c_void));
            unsafe {
                HcsCloseComputeSystem(self.0);
            }
        }
    }
}
impl Default for HCS_SYSTEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = HCS_CREATE_OPTIONS(65536i32);
pub const HcsEventGroupOperationInfo: HCS_EVENT_TYPE = HCS_EVENT_TYPE(-1073741823i32);
pub const HcsEventGroupVmLifecycle: HCS_EVENT_TYPE = HCS_EVENT_TYPE(-2147483646i32);
pub const HcsEventInvalid: HCS_EVENT_TYPE = HCS_EVENT_TYPE(0i32);
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = HCS_EVENT_TYPE(16777216i32);
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(1i32);
pub const HcsEventOptionEnableVmLifecycle: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(2i32);
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(0i32);
pub const HcsEventProcessExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(65536i32);
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = HCS_EVENT_TYPE(33554432i32);
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(2i32);
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = HCS_EVENT_TYPE(3i32);
pub const HcsEventSystemExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(1i32);
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = HCS_EVENT_TYPE(6i32);
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = HCS_EVENT_TYPE(4i32);
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(5i32);
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(-2147483648i32);
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(0i32);
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(-268435456i32);
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(0i32);
pub const HcsNotificationOperationProgressUpdate: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(256i32);
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(65536i32);
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16777216i32);
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(13i32);
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(6i32);
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(2i32);
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(1i32);
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(11i32);
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(14i32);
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(12i32);
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(15i32);
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16i32);
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(4i32);
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(9i32);
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(5i32);
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(8i32);
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(7i32);
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(3i32);
pub const HcsOperationOptionNone: HCS_OPERATION_OPTIONS = HCS_OPERATION_OPTIONS(0i32);
pub const HcsOperationOptionProgressUpdate: HCS_OPERATION_OPTIONS = HCS_OPERATION_OPTIONS(1i32);
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(15i32);
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(1i32);
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(10i32);
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(0i32);
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(12i32);
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(13i32);
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(9i32);
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(8i32);
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(14i32);
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(-1i32);
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(4i32);
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(5i32);
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(6i32);
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(3i32);
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(11i32);
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(2i32);
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(7i32);
pub const HcsResourceTypeFile: HCS_RESOURCE_TYPE = HCS_RESOURCE_TYPE(1i32);
pub const HcsResourceTypeJob: HCS_RESOURCE_TYPE = HCS_RESOURCE_TYPE(2i32);
pub const HcsResourceTypeNone: HCS_RESOURCE_TYPE = HCS_RESOURCE_TYPE(0i32);
