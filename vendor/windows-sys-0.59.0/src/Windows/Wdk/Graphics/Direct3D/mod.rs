windows_targets::link!("gdi32.dll" "system" fn D3DKMTAcquireKeyedMutex(param0 : *mut D3DKMT_ACQUIREKEYEDMUTEX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTAcquireKeyedMutex2(param0 : *mut D3DKMT_ACQUIREKEYEDMUTEX2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTAdjustFullscreenGamma(param0 : *const D3DKMT_ADJUSTFULLSCREENGAMMA) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCancelPresents(param0 : *const D3DKMT_CANCEL_PRESENTS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTChangeSurfacePointer(param0 : *const D3DKMT_CHANGESURFACEPOINTER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTChangeVideoMemoryReservation(param0 : *const D3DKMT_CHANGEVIDEOMEMORYRESERVATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckExclusiveOwnership() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckMonitorPowerState(param0 : *const D3DKMT_CHECKMONITORPOWERSTATE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport2(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport3(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckOcclusion(param0 : *const D3DKMT_CHECKOCCLUSION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckSharedResourceAccess(param0 : *const D3DKMT_CHECKSHAREDRESOURCEACCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCheckVidPnExclusiveOwnership(param0 : *const D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCloseAdapter(param0 : *const D3DKMT_CLOSEADAPTER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTConfigureSharedResource(param0 : *const D3DKMT_CONFIGURESHAREDRESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateAllocation(param0 : *mut D3DKMT_CREATEALLOCATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateAllocation2(param0 : *mut D3DKMT_CREATEALLOCATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateContext(param0 : *mut D3DKMT_CREATECONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateContextVirtual(param0 : *const D3DKMT_CREATECONTEXTVIRTUAL) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateDCFromMemory(param0 : *mut D3DKMT_CREATEDCFROMMEMORY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateDevice(param0 : *mut D3DKMT_CREATEDEVICE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateHwContext(param0 : *mut D3DKMT_CREATEHWCONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateHwQueue(param0 : *mut D3DKMT_CREATEHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateKeyedMutex(param0 : *mut D3DKMT_CREATEKEYEDMUTEX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateKeyedMutex2(param0 : *mut D3DKMT_CREATEKEYEDMUTEX2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateOutputDupl(param0 : *const D3DKMT_CREATE_OUTPUTDUPL) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateOverlay(param0 : *mut D3DKMT_CREATEOVERLAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreatePagingQueue(param0 : *mut D3DKMT_CREATEPAGINGQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateProtectedSession(param0 : *mut D3DKMT_CREATEPROTECTEDSESSION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateSynchronizationObject(param0 : *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTCreateSynchronizationObject2(param0 : *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyAllocation(param0 : *const D3DKMT_DESTROYALLOCATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyAllocation2(param0 : *const D3DKMT_DESTROYALLOCATION2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyContext(param0 : *const D3DKMT_DESTROYCONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyDCFromMemory(param0 : *const D3DKMT_DESTROYDCFROMMEMORY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyDevice(param0 : *const D3DKMT_DESTROYDEVICE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyHwContext(param0 : *const D3DKMT_DESTROYHWCONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyHwQueue(param0 : *const D3DKMT_DESTROYHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyKeyedMutex(param0 : *const D3DKMT_DESTROYKEYEDMUTEX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyOutputDupl(param0 : *const D3DKMT_DESTROY_OUTPUTDUPL) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyOverlay(param0 : *const D3DKMT_DESTROYOVERLAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyPagingQueue(param0 : *mut D3DDDI_DESTROYPAGINGQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroyProtectedSession(param0 : *mut D3DKMT_DESTROYPROTECTEDSESSION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTDestroySynchronizationObject(param0 : *const D3DKMT_DESTROYSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTEnumAdapters(param0 : *mut D3DKMT_ENUMADAPTERS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTEnumAdapters2(param0 : *mut D3DKMT_ENUMADAPTERS2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("api-ms-win-dx-d3dkmt-l1-1-6.dll" "system" fn D3DKMTEnumAdapters3(param0 : *mut D3DKMT_ENUMADAPTERS3) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTEscape(param0 : *const D3DKMT_ESCAPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTEvict(param0 : *mut D3DKMT_EVICT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTFlipOverlay(param0 : *const D3DKMT_FLIPOVERLAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTFlushHeapTransitions(param0 : *const D3DKMT_FLUSHHEAPTRANSITIONS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTFreeGpuVirtualAddress(param0 : *const D3DKMT_FREEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetAllocationPriority(param0 : *const D3DKMT_GETALLOCATIONPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetContextInProcessSchedulingPriority(param0 : *mut D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetContextSchedulingPriority(param0 : *mut D3DKMT_GETCONTEXTSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetDWMVerticalBlankEvent(param0 : *const D3DKMT_GETVERTICALBLANKEVENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetDeviceState(param0 : *mut D3DKMT_GETDEVICESTATE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetDisplayModeList(param0 : *mut D3DKMT_GETDISPLAYMODELIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetMultiPlaneOverlayCaps(param0 : *mut D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetMultisampleMethodList(param0 : *mut D3DKMT_GETMULTISAMPLEMETHODLIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetOverlayState(param0 : *mut D3DKMT_GETOVERLAYSTATE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetPostCompositionCaps(param0 : *mut D3DKMT_GET_POST_COMPOSITION_CAPS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetPresentHistory(param0 : *mut D3DKMT_GETPRESENTHISTORY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetPresentQueueEvent(hadapter : u32, param1 : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetProcessDeviceRemovalSupport(param0 : *mut D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetProcessSchedulingPriorityClass(param0 : super::super::super::Win32::Foundation:: HANDLE, param1 : *mut D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetResourcePresentPrivateDriverData(param0 : *mut D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetRuntimeData(param0 : *mut D3DKMT_GETRUNTIMEDATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetScanLine(param0 : *mut D3DKMT_GETSCANLINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetSharedPrimaryHandle(param0 : *mut D3DKMT_GETSHAREDPRIMARYHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTGetSharedResourceAdapterLuid(param0 : *mut D3DKMT_GETSHAREDRESOURCEADAPTERLUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTInvalidateActiveVidPn(param0 : *const D3DKMT_INVALIDATEACTIVEVIDPN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTInvalidateCache(param0 : *const D3DKMT_INVALIDATECACHE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTLock(param0 : *mut D3DKMT_LOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTLock2(param0 : *mut D3DKMT_LOCK2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTMakeResident(param0 : *mut D3DDDI_MAKERESIDENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTMapGpuVirtualAddress(param0 : *mut D3DDDI_MAPGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTMarkDeviceAsError(param0 : *const D3DKMT_MARKDEVICEASERROR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOfferAllocations(param0 : *const D3DKMT_OFFERALLOCATIONS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromDeviceName(param0 : *mut D3DKMT_OPENADAPTERFROMDEVICENAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromGdiDisplayName(param0 : *mut D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromHdc(param0 : *mut D3DKMT_OPENADAPTERFROMHDC) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromLuid(param0 : *mut D3DKMT_OPENADAPTERFROMLUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutex(param0 : *mut D3DKMT_OPENKEYEDMUTEX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutex2(param0 : *mut D3DKMT_OPENKEYEDMUTEX2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutexFromNtHandle(param0 : *mut D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenNtHandleFromName(param0 : *mut D3DKMT_OPENNTHANDLEFROMNAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenProtectedSessionFromNtHandle(param0 : *mut D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenResource(param0 : *mut D3DKMT_OPENRESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenResource2(param0 : *mut D3DKMT_OPENRESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenResourceFromNtHandle(param0 : *mut D3DKMT_OPENRESOURCEFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectFromNtHandle(param0 : *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectFromNtHandle2(param0 : *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectNtHandleFromName(param0 : *mut D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOpenSynchronizationObject(param0 : *mut D3DKMT_OPENSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetFrameInfo(param0 : *mut D3DKMT_OUTPUTDUPL_GET_FRAMEINFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetMetaData(param0 : *mut D3DKMT_OUTPUTDUPL_METADATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetPointerShapeData(param0 : *mut D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOutputDuplPresent(param0 : *const D3DKMT_OUTPUTDUPLPRESENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTOutputDuplPresentToHwQueue(param0 : *const D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTOutputDuplReleaseFrame(param0 : *mut D3DKMT_OUTPUTDUPL_RELEASE_FRAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPollDisplayChildren(param0 : *const D3DKMT_POLLDISPLAYCHILDREN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPresent(param0 : *mut D3DKMT_PRESENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay2(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay3(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY3) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTPresentRedirected(param0 : *const D3DKMT_PRESENT_REDIRECTED) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryAdapterInfo(param0 : *mut D3DKMT_QUERYADAPTERINFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryAllocationResidency(param0 : *const D3DKMT_QUERYALLOCATIONRESIDENCY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryClockCalibration(param0 : *mut D3DKMT_QUERYCLOCKCALIBRATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryFSEBlock(param0 : *mut D3DKMT_QUERYFSEBLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryProcessOfferInfo(param0 : *mut D3DKMT_QUERYPROCESSOFFERINFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryProtectedSessionInfoFromNtHandle(param0 : *mut D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryProtectedSessionStatus(param0 : *mut D3DKMT_QUERYPROTECTEDSESSIONSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryRemoteVidPnSourceFromGdiDisplayName(param0 : *mut D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryResourceInfo(param0 : *mut D3DKMT_QUERYRESOURCEINFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryResourceInfoFromNtHandle(param0 : *mut D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryStatistics(param0 : *const D3DKMT_QUERYSTATISTICS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryVidPnExclusiveOwnership(param0 : *mut D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTQueryVideoMemoryInfo(param0 : *mut D3DKMT_QUERYVIDEOMEMORYINFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReclaimAllocations(param0 : *mut D3DKMT_RECLAIMALLOCATIONS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReclaimAllocations2(param0 : *mut D3DKMT_RECLAIMALLOCATIONS2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTRegisterTrimNotification(param0 : *mut D3DKMT_REGISTERTRIMNOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTRegisterVailProcess(param0 : *const windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReleaseKeyedMutex(param0 : *mut D3DKMT_RELEASEKEYEDMUTEX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReleaseKeyedMutex2(param0 : *mut D3DKMT_RELEASEKEYEDMUTEX2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReleaseProcessVidPnSourceOwners(param0 : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTRender(param0 : *mut D3DKMT_RENDER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTReserveGpuVirtualAddress(param0 : *mut D3DDDI_RESERVEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetAllocationPriority(param0 : *const D3DKMT_SETALLOCATIONPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetContextInProcessSchedulingPriority(param0 : *const D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetContextSchedulingPriority(param0 : *const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetDisplayMode(param0 : *mut D3DKMT_SETDISPLAYMODE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetDisplayPrivateDriverFormat(param0 : *const D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetFSEBlock(param0 : *const D3DKMT_SETFSEBLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetGammaRamp(param0 : *const D3DKMT_SETGAMMARAMP) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetHwProtectionTeardownRecovery(param0 : *const D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetMonitorColorSpaceTransform(param0 : *const D3DKMT_SET_COLORSPACE_TRANSFORM) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetProcessSchedulingPriorityClass(param0 : super::super::super::Win32::Foundation:: HANDLE, param1 : D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetQueuedLimit(param0 : *const D3DKMT_SETQUEUEDLIMIT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetStablePowerState(param0 : *const D3DKMT_SETSTABLEPOWERSTATE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetSyncRefreshCountWaitTarget(param0 : *const D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceHwProtection(param0 : *const D3DKMT_SETVIDPNSOURCEHWPROTECTION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner1(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER1) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner2(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER2) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("gdi32.dll" "system" fn D3DKMTShareObjects(cobjects : u32, hobjects : *const u32, pobjectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, dwdesiredaccess : u32, phsharednthandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSharedPrimaryLockNotification(param0 : *const D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSharedPrimaryUnLockNotification(param0 : *const D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObject(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObject2(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromCpu(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromGpu(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromGpu2(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSubmitCommand(param0 : *const D3DKMT_SUBMITCOMMAND) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSubmitCommandToHwQueue(param0 : *const D3DKMT_SUBMITCOMMANDTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTSubmitPresentBltToHwQueue(param0 : *const D3DKMT_SUBMITPRESENTBLTTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTSubmitPresentToHwQueue(param0 : *mut D3DKMT_SUBMITPRESENTTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSubmitSignalSyncObjectsToHwQueue(param0 : *const D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTSubmitWaitForSyncObjectsToHwQueue(param0 : *const D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTTrimProcessCommitment(param0 : *mut D3DKMT_TRIMPROCESSCOMMITMENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUnlock(param0 : *const D3DKMT_UNLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUnlock2(param0 : *const D3DKMT_UNLOCK2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUnregisterTrimNotification(param0 : *mut D3DKMT_UNREGISTERTRIMNOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUpdateAllocationProperty(param0 : *mut D3DDDI_UPDATEALLOCPROPERTY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUpdateGpuVirtualAddress(param0 : *const D3DKMT_UPDATEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTUpdateOverlay(param0 : *const D3DKMT_UPDATEOVERLAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForIdle(param0 : *const D3DKMT_WAITFORIDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObject(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObject2(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObjectFromCpu(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObjectFromGpu(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForVerticalBlankEvent(param0 : *const D3DKMT_WAITFORVERTICALBLANKEVENT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("gdi32.dll" "system" fn D3DKMTWaitForVerticalBlankEvent2(param0 : *const D3DKMT_WAITFORVERTICALBLANKEVENT2) -> super::super::super::Win32::Foundation:: NTSTATUS);
pub const D3DCLEAR_COMPUTERECTS: i32 = 8i32;
pub const D3DDDIFMT_A1: D3DDDIFORMAT = 118u32;
pub const D3DDDIFMT_A16B16G16R16: D3DDDIFORMAT = 36u32;
pub const D3DDDIFMT_A16B16G16R16F: D3DDDIFORMAT = 113u32;
pub const D3DDDIFMT_A1R5G5B5: D3DDDIFORMAT = 25u32;
pub const D3DDDIFMT_A2B10G10R10: D3DDDIFORMAT = 31u32;
pub const D3DDDIFMT_A2B10G10R10_XR_BIAS: D3DDDIFORMAT = 119u32;
pub const D3DDDIFMT_A2R10G10B10: D3DDDIFORMAT = 35u32;
pub const D3DDDIFMT_A2W10V10U10: D3DDDIFORMAT = 67u32;
pub const D3DDDIFMT_A32B32G32R32F: D3DDDIFORMAT = 116u32;
pub const D3DDDIFMT_A4L4: D3DDDIFORMAT = 52u32;
pub const D3DDDIFMT_A4R4G4B4: D3DDDIFORMAT = 26u32;
pub const D3DDDIFMT_A8: D3DDDIFORMAT = 28u32;
pub const D3DDDIFMT_A8B8G8R8: D3DDDIFORMAT = 32u32;
pub const D3DDDIFMT_A8L8: D3DDDIFORMAT = 51u32;
pub const D3DDDIFMT_A8P8: D3DDDIFORMAT = 40u32;
pub const D3DDDIFMT_A8R3G3B2: D3DDDIFORMAT = 29u32;
pub const D3DDDIFMT_A8R8G8B8: D3DDDIFORMAT = 21u32;
pub const D3DDDIFMT_BINARYBUFFER: D3DDDIFORMAT = 199u32;
pub const D3DDDIFMT_BITSTREAMDATA: D3DDDIFORMAT = 156u32;
pub const D3DDDIFMT_CxV8U8: D3DDDIFORMAT = 117u32;
pub const D3DDDIFMT_D15S1: D3DDDIFORMAT = 73u32;
pub const D3DDDIFMT_D16: D3DDDIFORMAT = 80u32;
pub const D3DDDIFMT_D16_LOCKABLE: D3DDDIFORMAT = 70u32;
pub const D3DDDIFMT_D24FS8: D3DDDIFORMAT = 83u32;
pub const D3DDDIFMT_D24S8: D3DDDIFORMAT = 75u32;
pub const D3DDDIFMT_D24X4S4: D3DDDIFORMAT = 79u32;
pub const D3DDDIFMT_D24X8: D3DDDIFORMAT = 77u32;
pub const D3DDDIFMT_D32: D3DDDIFORMAT = 71u32;
pub const D3DDDIFMT_D32F_LOCKABLE: D3DDDIFORMAT = 82u32;
pub const D3DDDIFMT_D32_LOCKABLE: D3DDDIFORMAT = 84u32;
pub const D3DDDIFMT_DEBLOCKINGDATA: D3DDDIFORMAT = 153u32;
pub const D3DDDIFMT_DXT1: D3DDDIFORMAT = 827611204u32;
pub const D3DDDIFMT_DXT2: D3DDDIFORMAT = 844388420u32;
pub const D3DDDIFMT_DXT3: D3DDDIFORMAT = 861165636u32;
pub const D3DDDIFMT_DXT4: D3DDDIFORMAT = 877942852u32;
pub const D3DDDIFMT_DXT5: D3DDDIFORMAT = 894720068u32;
pub const D3DDDIFMT_DXVACOMPBUFFER_BASE: D3DDDIFORMAT = 150u32;
pub const D3DDDIFMT_DXVACOMPBUFFER_MAX: D3DDDIFORMAT = 181u32;
pub const D3DDDIFMT_DXVA_RESERVED10: D3DDDIFORMAT = 160u32;
pub const D3DDDIFMT_DXVA_RESERVED11: D3DDDIFORMAT = 161u32;
pub const D3DDDIFMT_DXVA_RESERVED12: D3DDDIFORMAT = 162u32;
pub const D3DDDIFMT_DXVA_RESERVED13: D3DDDIFORMAT = 163u32;
pub const D3DDDIFMT_DXVA_RESERVED14: D3DDDIFORMAT = 164u32;
pub const D3DDDIFMT_DXVA_RESERVED15: D3DDDIFORMAT = 165u32;
pub const D3DDDIFMT_DXVA_RESERVED16: D3DDDIFORMAT = 166u32;
pub const D3DDDIFMT_DXVA_RESERVED17: D3DDDIFORMAT = 167u32;
pub const D3DDDIFMT_DXVA_RESERVED18: D3DDDIFORMAT = 168u32;
pub const D3DDDIFMT_DXVA_RESERVED19: D3DDDIFORMAT = 169u32;
pub const D3DDDIFMT_DXVA_RESERVED20: D3DDDIFORMAT = 170u32;
pub const D3DDDIFMT_DXVA_RESERVED21: D3DDDIFORMAT = 171u32;
pub const D3DDDIFMT_DXVA_RESERVED22: D3DDDIFORMAT = 172u32;
pub const D3DDDIFMT_DXVA_RESERVED23: D3DDDIFORMAT = 173u32;
pub const D3DDDIFMT_DXVA_RESERVED24: D3DDDIFORMAT = 174u32;
pub const D3DDDIFMT_DXVA_RESERVED25: D3DDDIFORMAT = 175u32;
pub const D3DDDIFMT_DXVA_RESERVED26: D3DDDIFORMAT = 176u32;
pub const D3DDDIFMT_DXVA_RESERVED27: D3DDDIFORMAT = 177u32;
pub const D3DDDIFMT_DXVA_RESERVED28: D3DDDIFORMAT = 178u32;
pub const D3DDDIFMT_DXVA_RESERVED29: D3DDDIFORMAT = 179u32;
pub const D3DDDIFMT_DXVA_RESERVED30: D3DDDIFORMAT = 180u32;
pub const D3DDDIFMT_DXVA_RESERVED31: D3DDDIFORMAT = 181u32;
pub const D3DDDIFMT_DXVA_RESERVED9: D3DDDIFORMAT = 159u32;
pub const D3DDDIFMT_FILMGRAINBUFFER: D3DDDIFORMAT = 158u32;
pub const D3DDDIFMT_G16R16: D3DDDIFORMAT = 34u32;
pub const D3DDDIFMT_G16R16F: D3DDDIFORMAT = 112u32;
pub const D3DDDIFMT_G32R32F: D3DDDIFORMAT = 115u32;
pub const D3DDDIFMT_G8R8: D3DDDIFORMAT = 91u32;
pub const D3DDDIFMT_G8R8_G8B8: D3DDDIFORMAT = 1111970375u32;
pub const D3DDDIFMT_INDEX16: D3DDDIFORMAT = 101u32;
pub const D3DDDIFMT_INDEX32: D3DDDIFORMAT = 102u32;
pub const D3DDDIFMT_INVERSEQUANTIZATIONDATA: D3DDDIFORMAT = 154u32;
pub const D3DDDIFMT_L16: D3DDDIFORMAT = 81u32;
pub const D3DDDIFMT_L6V5U5: D3DDDIFORMAT = 61u32;
pub const D3DDDIFMT_L8: D3DDDIFORMAT = 50u32;
pub const D3DDDIFMT_MACROBLOCKDATA: D3DDDIFORMAT = 151u32;
pub const D3DDDIFMT_MOTIONVECTORBUFFER: D3DDDIFORMAT = 157u32;
pub const D3DDDIFMT_MULTI2_ARGB8: D3DDDIFORMAT = 827606349u32;
pub const D3DDDIFMT_P8: D3DDDIFORMAT = 41u32;
pub const D3DDDIFMT_PICTUREPARAMSDATA: D3DDDIFORMAT = 150u32;
pub const D3DDDIFMT_Q16W16V16U16: D3DDDIFORMAT = 110u32;
pub const D3DDDIFMT_Q8W8V8U8: D3DDDIFORMAT = 63u32;
pub const D3DDDIFMT_R16F: D3DDDIFORMAT = 111u32;
pub const D3DDDIFMT_R32F: D3DDDIFORMAT = 114u32;
pub const D3DDDIFMT_R3G3B2: D3DDDIFORMAT = 27u32;
pub const D3DDDIFMT_R5G6B5: D3DDDIFORMAT = 23u32;
pub const D3DDDIFMT_R8: D3DDDIFORMAT = 92u32;
pub const D3DDDIFMT_R8G8B8: D3DDDIFORMAT = 20u32;
pub const D3DDDIFMT_R8G8_B8G8: D3DDDIFORMAT = 1195525970u32;
pub const D3DDDIFMT_RESIDUALDIFFERENCEDATA: D3DDDIFORMAT = 152u32;
pub const D3DDDIFMT_S1D15: D3DDDIFORMAT = 72u32;
pub const D3DDDIFMT_S8D24: D3DDDIFORMAT = 74u32;
pub const D3DDDIFMT_S8_LOCKABLE: D3DDDIFORMAT = 85u32;
pub const D3DDDIFMT_SLICECONTROLDATA: D3DDDIFORMAT = 155u32;
pub const D3DDDIFMT_UNKNOWN: D3DDDIFORMAT = 0u32;
pub const D3DDDIFMT_UYVY: D3DDDIFORMAT = 1498831189u32;
pub const D3DDDIFMT_V16U16: D3DDDIFORMAT = 64u32;
pub const D3DDDIFMT_V8U8: D3DDDIFORMAT = 60u32;
pub const D3DDDIFMT_VERTEXDATA: D3DDDIFORMAT = 100u32;
pub const D3DDDIFMT_W11V11U10: D3DDDIFORMAT = 65u32;
pub const D3DDDIFMT_X1R5G5B5: D3DDDIFORMAT = 24u32;
pub const D3DDDIFMT_X4R4G4B4: D3DDDIFORMAT = 30u32;
pub const D3DDDIFMT_X4S4D24: D3DDDIFORMAT = 78u32;
pub const D3DDDIFMT_X8B8G8R8: D3DDDIFORMAT = 33u32;
pub const D3DDDIFMT_X8D24: D3DDDIFORMAT = 76u32;
pub const D3DDDIFMT_X8L8V8U8: D3DDDIFORMAT = 62u32;
pub const D3DDDIFMT_X8R8G8B8: D3DDDIFORMAT = 22u32;
pub const D3DDDIFMT_YUY2: D3DDDIFORMAT = 844715353u32;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_NO_ACCESS: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 0i32;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_NO_COMMIT: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 2i32;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_ZERO: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 1i32;
pub const D3DDDIMULTISAMPLE_10_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 10i32;
pub const D3DDDIMULTISAMPLE_11_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 11i32;
pub const D3DDDIMULTISAMPLE_12_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 12i32;
pub const D3DDDIMULTISAMPLE_13_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 13i32;
pub const D3DDDIMULTISAMPLE_14_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 14i32;
pub const D3DDDIMULTISAMPLE_15_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 15i32;
pub const D3DDDIMULTISAMPLE_16_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 16i32;
pub const D3DDDIMULTISAMPLE_2_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 2i32;
pub const D3DDDIMULTISAMPLE_3_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 3i32;
pub const D3DDDIMULTISAMPLE_4_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 4i32;
pub const D3DDDIMULTISAMPLE_5_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 5i32;
pub const D3DDDIMULTISAMPLE_6_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 6i32;
pub const D3DDDIMULTISAMPLE_7_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 7i32;
pub const D3DDDIMULTISAMPLE_8_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 8i32;
pub const D3DDDIMULTISAMPLE_9_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 9i32;
pub const D3DDDIMULTISAMPLE_NONE: D3DDDIMULTISAMPLE_TYPE = 0i32;
pub const D3DDDIMULTISAMPLE_NONMASKABLE: D3DDDIMULTISAMPLE_TYPE = 1i32;
pub const D3DDDIPOOL_LOCALVIDMEM: D3DDDI_POOL = 3i32;
pub const D3DDDIPOOL_NONLOCALVIDMEM: D3DDDI_POOL = 4i32;
pub const D3DDDIPOOL_STAGINGMEM: D3DDDI_POOL = 5i32;
pub const D3DDDIPOOL_SYSTEMMEM: D3DDDI_POOL = 1i32;
pub const D3DDDIPOOL_VIDEOMEMORY: D3DDDI_POOL = 2i32;
pub const D3DDDI_ALLOCATIONPRIORITY_HIGH: u32 = 2684354560u32;
pub const D3DDDI_ALLOCATIONPRIORITY_LOW: u32 = 1342177280u32;
pub const D3DDDI_ALLOCATIONPRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const D3DDDI_ALLOCATIONPRIORITY_MINIMUM: u32 = 671088640u32;
pub const D3DDDI_ALLOCATIONPRIORITY_NORMAL: u32 = 2013265920u32;
pub const D3DDDI_COLOR_SPACE_CUSTOM: D3DDDI_COLOR_SPACE_TYPE = -1i32;
pub const D3DDDI_COLOR_SPACE_RESERVED: D3DDDI_COLOR_SPACE_TYPE = 4i32;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 1i32;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 12i32;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 17i32;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 0i32;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 14i32;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 3i32;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 2i32;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 21i32;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 20i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 11i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: D3DDDI_COLOR_SPACE_TYPE = 7i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 9i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: D3DDDI_COLOR_SPACE_TYPE = 5i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 19i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 13i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 16i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 10i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: D3DDDI_COLOR_SPACE_TYPE = 6i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 8i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 15i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 23i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 22i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 24i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 18i32;
pub const D3DDDI_CPU_NOTIFICATION: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 4i32;
pub const D3DDDI_DOORBELLSTATUS_CONNECTED: D3DDDI_DOORBELLSTATUS = 0i32;
pub const D3DDDI_DOORBELLSTATUS_CONNECTED_NOTIFY_KMD: D3DDDI_DOORBELLSTATUS = 1i32;
pub const D3DDDI_DOORBELLSTATUS_DISCONNECTED_ABORT: D3DDDI_DOORBELLSTATUS = 3i32;
pub const D3DDDI_DOORBELLSTATUS_DISCONNECTED_RETRY: D3DDDI_DOORBELLSTATUS = 2i32;
pub const D3DDDI_DOORBELL_PRIVATEDATA_MAX_BYTES_WDDM3_1: u32 = 16u32;
pub const D3DDDI_DRIVERESCAPETYPE_CPUEVENTUSAGE: D3DDDI_DRIVERESCAPETYPE = 2i32;
pub const D3DDDI_DRIVERESCAPETYPE_MAX: D3DDDI_DRIVERESCAPETYPE = 3i32;
pub const D3DDDI_DRIVERESCAPETYPE_TRANSLATEALLOCATIONHANDLE: D3DDDI_DRIVERESCAPETYPE = 0i32;
pub const D3DDDI_DRIVERESCAPETYPE_TRANSLATERESOURCEHANDLE: D3DDDI_DRIVERESCAPETYPE = 1i32;
pub const D3DDDI_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 3i32;
pub const D3DDDI_FLIPINTERVAL_FOUR: D3DDDI_FLIPINTERVAL_TYPE = 4i32;
pub const D3DDDI_FLIPINTERVAL_IMMEDIATE: D3DDDI_FLIPINTERVAL_TYPE = 0i32;
pub const D3DDDI_FLIPINTERVAL_IMMEDIATE_ALLOW_TEARING: D3DDDI_FLIPINTERVAL_TYPE = 5i32;
pub const D3DDDI_FLIPINTERVAL_ONE: D3DDDI_FLIPINTERVAL_TYPE = 1i32;
pub const D3DDDI_FLIPINTERVAL_THREE: D3DDDI_FLIPINTERVAL_TYPE = 3i32;
pub const D3DDDI_FLIPINTERVAL_TWO: D3DDDI_FLIPINTERVAL_TYPE = 2i32;
pub const D3DDDI_GAMMARAMP_DEFAULT: D3DDDI_GAMMARAMP_TYPE = 1i32;
pub const D3DDDI_GAMMARAMP_DXGI_1: D3DDDI_GAMMARAMP_TYPE = 3i32;
pub const D3DDDI_GAMMARAMP_MATRIX_3x4: D3DDDI_GAMMARAMP_TYPE = 4i32;
pub const D3DDDI_GAMMARAMP_MATRIX_V2: D3DDDI_GAMMARAMP_TYPE = 5i32;
pub const D3DDDI_GAMMARAMP_RGB256x3x16: D3DDDI_GAMMARAMP_TYPE = 2i32;
pub const D3DDDI_GAMMARAMP_UNINITIALIZED: D3DDDI_GAMMARAMP_TYPE = 0i32;
pub const D3DDDI_HDR_METADATA_TYPE_HDR10: D3DDDI_HDR_METADATA_TYPE = 1i32;
pub const D3DDDI_HDR_METADATA_TYPE_HDR10PLUS: D3DDDI_HDR_METADATA_TYPE = 2i32;
pub const D3DDDI_HDR_METADATA_TYPE_NONE: D3DDDI_HDR_METADATA_TYPE = 0i32;
pub const D3DDDI_MAX_BROADCAST_CONTEXT: u32 = 64u32;
pub const D3DDDI_MAX_MPO_PRESENT_DIRTY_RECTS: u32 = 4095u32;
pub const D3DDDI_MAX_OBJECT_SIGNALED: u32 = 32u32;
pub const D3DDDI_MAX_OBJECT_WAITED_ON: u32 = 32u32;
pub const D3DDDI_MAX_WRITTEN_PRIMARIES: u32 = 16u32;
pub const D3DDDI_MONITORED_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 5i32;
pub const D3DDDI_OFFER_PRIORITY_AUTO: D3DDDI_OFFER_PRIORITY = 4i32;
pub const D3DDDI_OFFER_PRIORITY_HIGH: D3DDDI_OFFER_PRIORITY = 3i32;
pub const D3DDDI_OFFER_PRIORITY_LOW: D3DDDI_OFFER_PRIORITY = 1i32;
pub const D3DDDI_OFFER_PRIORITY_NONE: D3DDDI_OFFER_PRIORITY = 0i32;
pub const D3DDDI_OFFER_PRIORITY_NORMAL: D3DDDI_OFFER_PRIORITY = 2i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 12i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 33i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 32i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P2020: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 31i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P709: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 0i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 30i32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_RESERVED: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 4i32;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_ABOVE_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = 1i32;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_BELOW_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = -1i32;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = 0i32;
pub const D3DDDI_PERIODIC_MONITORED_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 6i32;
pub const D3DDDI_QUERYREGISTRY_ADAPTERKEY: D3DDDI_QUERYREGISTRY_TYPE = 1i32;
pub const D3DDDI_QUERYREGISTRY_DRIVERIMAGEPATH: D3DDDI_QUERYREGISTRY_TYPE = 3i32;
pub const D3DDDI_QUERYREGISTRY_DRIVERSTOREPATH: D3DDDI_QUERYREGISTRY_TYPE = 2i32;
pub const D3DDDI_QUERYREGISTRY_MAX: D3DDDI_QUERYREGISTRY_TYPE = 4i32;
pub const D3DDDI_QUERYREGISTRY_SERVICEKEY: D3DDDI_QUERYREGISTRY_TYPE = 0i32;
pub const D3DDDI_QUERYREGISTRY_STATUS_BUFFER_OVERFLOW: D3DDDI_QUERYREGISTRY_STATUS = 1i32;
pub const D3DDDI_QUERYREGISTRY_STATUS_FAIL: D3DDDI_QUERYREGISTRY_STATUS = 2i32;
pub const D3DDDI_QUERYREGISTRY_STATUS_MAX: D3DDDI_QUERYREGISTRY_STATUS = 3i32;
pub const D3DDDI_QUERYREGISTRY_STATUS_SUCCESS: D3DDDI_QUERYREGISTRY_STATUS = 0i32;
pub const D3DDDI_RECLAIM_RESULT_DISCARDED: D3DDDI_RECLAIM_RESULT = 1i32;
pub const D3DDDI_RECLAIM_RESULT_NOT_COMMITTED: D3DDDI_RECLAIM_RESULT = 2i32;
pub const D3DDDI_RECLAIM_RESULT_OK: D3DDDI_RECLAIM_RESULT = 0i32;
pub const D3DDDI_ROTATION_180: D3DDDI_ROTATION = 3i32;
pub const D3DDDI_ROTATION_270: D3DDDI_ROTATION = 4i32;
pub const D3DDDI_ROTATION_90: D3DDDI_ROTATION = 2i32;
pub const D3DDDI_ROTATION_IDENTITY: D3DDDI_ROTATION = 1i32;
pub const D3DDDI_SCANLINEORDERING_INTERLACED: D3DDDI_SCANLINEORDERING = 2i32;
pub const D3DDDI_SCANLINEORDERING_PROGRESSIVE: D3DDDI_SCANLINEORDERING = 1i32;
pub const D3DDDI_SCANLINEORDERING_UNKNOWN: D3DDDI_SCANLINEORDERING = 0i32;
pub const D3DDDI_SEMAPHORE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 2i32;
pub const D3DDDI_SYNCHRONIZATION_MUTEX: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 1i32;
pub const D3DDDI_SYNCHRONIZATION_TYPE_LIMIT: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 7i32;
pub const D3DDDI_SYNC_OBJECT_SIGNAL: u32 = 2u32;
pub const D3DDDI_SYNC_OBJECT_WAIT: u32 = 1u32;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_COPY: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 2i32;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_MAP: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 0i32;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_MAP_PROTECT: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 3i32;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_UNMAP: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 1i32;
pub const D3DDDI_VSSLO_INTERLACED_LOWERFIELDFIRST: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 3i32;
pub const D3DDDI_VSSLO_INTERLACED_UPPERFIELDFIRST: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 2i32;
pub const D3DDDI_VSSLO_OTHER: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 255i32;
pub const D3DDDI_VSSLO_PROGRESSIVE: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 1i32;
pub const D3DDDI_VSSLO_UNINITIALIZED: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 0i32;
pub const D3DDEVCAPS_HWINDEXBUFFER: i32 = 67108864i32;
pub const D3DDEVCAPS_HWVERTEXBUFFER: i32 = 33554432i32;
pub const D3DDEVCAPS_SUBVOLUMELOCK: i32 = 134217728i32;
pub const D3DDEVINFOID_VCACHE: u32 = 4u32;
pub const D3DDP2OP_ADDDIRTYBOX: D3DHAL_DP2OPERATION = 67i32;
pub const D3DDP2OP_ADDDIRTYRECT: D3DHAL_DP2OPERATION = 66i32;
pub const D3DDP2OP_BLT: D3DHAL_DP2OPERATION = 81i32;
pub const D3DDP2OP_BUFFERBLT: D3DHAL_DP2OPERATION = 64i32;
pub const D3DDP2OP_CLEAR: D3DHAL_DP2OPERATION = 42i32;
pub const D3DDP2OP_CLIPPEDTRIANGLEFAN: D3DHAL_DP2OPERATION = 58i32;
pub const D3DDP2OP_COLORFILL: D3DHAL_DP2OPERATION = 82i32;
pub const D3DDP2OP_COMPOSERECTS: D3DHAL_DP2OPERATION = 98i32;
pub const D3DDP2OP_CREATELIGHT: D3DHAL_DP2OPERATION = 35i32;
pub const D3DDP2OP_CREATEPIXELSHADER: D3DHAL_DP2OPERATION = 54i32;
pub const D3DDP2OP_CREATEQUERY: D3DHAL_DP2OPERATION = 84i32;
pub const D3DDP2OP_CREATEVERTEXSHADER: D3DHAL_DP2OPERATION = 45i32;
pub const D3DDP2OP_CREATEVERTEXSHADERDECL: D3DHAL_DP2OPERATION = 71i32;
pub const D3DDP2OP_CREATEVERTEXSHADERFUNC: D3DHAL_DP2OPERATION = 74i32;
pub const D3DDP2OP_DELETEPIXELSHADER: D3DHAL_DP2OPERATION = 55i32;
pub const D3DDP2OP_DELETEQUERY: D3DHAL_DP2OPERATION = 90i32;
pub const D3DDP2OP_DELETEVERTEXSHADER: D3DHAL_DP2OPERATION = 46i32;
pub const D3DDP2OP_DELETEVERTEXSHADERDECL: D3DHAL_DP2OPERATION = 72i32;
pub const D3DDP2OP_DELETEVERTEXSHADERFUNC: D3DHAL_DP2OPERATION = 75i32;
pub const D3DDP2OP_DRAWINDEXEDPRIMITIVE: D3DHAL_DP2OPERATION = 53i32;
pub const D3DDP2OP_DRAWINDEXEDPRIMITIVE2: D3DHAL_DP2OPERATION = 60i32;
pub const D3DDP2OP_DRAWPRIMITIVE: D3DHAL_DP2OPERATION = 52i32;
pub const D3DDP2OP_DRAWPRIMITIVE2: D3DHAL_DP2OPERATION = 59i32;
pub const D3DDP2OP_DRAWRECTPATCH: D3DHAL_DP2OPERATION = 61i32;
pub const D3DDP2OP_DRAWTRIPATCH: D3DHAL_DP2OPERATION = 62i32;
pub const D3DDP2OP_GENERATEMIPSUBLEVELS: D3DHAL_DP2OPERATION = 89i32;
pub const D3DDP2OP_INDEXEDLINELIST: D3DHAL_DP2OPERATION = 2i32;
pub const D3DDP2OP_INDEXEDLINELIST2: D3DHAL_DP2OPERATION = 27i32;
pub const D3DDP2OP_INDEXEDLINESTRIP: D3DHAL_DP2OPERATION = 17i32;
pub const D3DDP2OP_INDEXEDTRIANGLEFAN: D3DHAL_DP2OPERATION = 22i32;
pub const D3DDP2OP_INDEXEDTRIANGLELIST: D3DHAL_DP2OPERATION = 3i32;
pub const D3DDP2OP_INDEXEDTRIANGLELIST2: D3DHAL_DP2OPERATION = 26i32;
pub const D3DDP2OP_INDEXEDTRIANGLESTRIP: D3DHAL_DP2OPERATION = 20i32;
pub const D3DDP2OP_ISSUEQUERY: D3DHAL_DP2OPERATION = 91i32;
pub const D3DDP2OP_LINELIST: D3DHAL_DP2OPERATION = 15i32;
pub const D3DDP2OP_LINELIST_IMM: D3DHAL_DP2OPERATION = 24i32;
pub const D3DDP2OP_LINESTRIP: D3DHAL_DP2OPERATION = 16i32;
pub const D3DDP2OP_MULTIPLYTRANSFORM: D3DHAL_DP2OPERATION = 65i32;
pub const D3DDP2OP_POINTS: D3DHAL_DP2OPERATION = 1i32;
pub const D3DDP2OP_RENDERSTATE: D3DHAL_DP2OPERATION = 8i32;
pub const D3DDP2OP_RESPONSECONTINUE: D3DHAL_DP2OPERATION = 87i32;
pub const D3DDP2OP_RESPONSEQUERY: D3DHAL_DP2OPERATION = 88i32;
pub const D3DDP2OP_SETCLIPPLANE: D3DHAL_DP2OPERATION = 44i32;
pub const D3DDP2OP_SETCONVOLUTIONKERNELMONO: D3DHAL_DP2OPERATION = 97i32;
pub const D3DDP2OP_SETDEPTHSTENCIL: D3DHAL_DP2OPERATION = 86i32;
pub const D3DDP2OP_SETINDICES: D3DHAL_DP2OPERATION = 51i32;
pub const D3DDP2OP_SETLIGHT: D3DHAL_DP2OPERATION = 34i32;
pub const D3DDP2OP_SETMATERIAL: D3DHAL_DP2OPERATION = 33i32;
pub const D3DDP2OP_SETPALETTE: D3DHAL_DP2OPERATION = 30i32;
pub const D3DDP2OP_SETPIXELSHADER: D3DHAL_DP2OPERATION = 56i32;
pub const D3DDP2OP_SETPIXELSHADERCONST: D3DHAL_DP2OPERATION = 57i32;
pub const D3DDP2OP_SETPIXELSHADERCONSTB: D3DHAL_DP2OPERATION = 94i32;
pub const D3DDP2OP_SETPIXELSHADERCONSTI: D3DHAL_DP2OPERATION = 93i32;
pub const D3DDP2OP_SETPRIORITY: D3DHAL_DP2OPERATION = 40i32;
pub const D3DDP2OP_SETRENDERTARGET: D3DHAL_DP2OPERATION = 41i32;
pub const D3DDP2OP_SETRENDERTARGET2: D3DHAL_DP2OPERATION = 85i32;
pub const D3DDP2OP_SETSCISSORRECT: D3DHAL_DP2OPERATION = 79i32;
pub const D3DDP2OP_SETSTREAMSOURCE: D3DHAL_DP2OPERATION = 49i32;
pub const D3DDP2OP_SETSTREAMSOURCE2: D3DHAL_DP2OPERATION = 80i32;
pub const D3DDP2OP_SETSTREAMSOURCEFREQ: D3DHAL_DP2OPERATION = 95i32;
pub const D3DDP2OP_SETSTREAMSOURCEUM: D3DHAL_DP2OPERATION = 50i32;
pub const D3DDP2OP_SETTEXLOD: D3DHAL_DP2OPERATION = 43i32;
pub const D3DDP2OP_SETTRANSFORM: D3DHAL_DP2OPERATION = 36i32;
pub const D3DDP2OP_SETVERTEXSHADER: D3DHAL_DP2OPERATION = 47i32;
pub const D3DDP2OP_SETVERTEXSHADERCONST: D3DHAL_DP2OPERATION = 48i32;
pub const D3DDP2OP_SETVERTEXSHADERCONSTB: D3DHAL_DP2OPERATION = 83i32;
pub const D3DDP2OP_SETVERTEXSHADERCONSTI: D3DHAL_DP2OPERATION = 77i32;
pub const D3DDP2OP_SETVERTEXSHADERDECL: D3DHAL_DP2OPERATION = 73i32;
pub const D3DDP2OP_SETVERTEXSHADERFUNC: D3DHAL_DP2OPERATION = 76i32;
pub const D3DDP2OP_STATESET: D3DHAL_DP2OPERATION = 39i32;
pub const D3DDP2OP_SURFACEBLT: D3DHAL_DP2OPERATION = 96i32;
pub const D3DDP2OP_TEXBLT: D3DHAL_DP2OPERATION = 38i32;
pub const D3DDP2OP_TEXTURESTAGESTATE: D3DHAL_DP2OPERATION = 25i32;
pub const D3DDP2OP_TRIANGLEFAN: D3DHAL_DP2OPERATION = 21i32;
pub const D3DDP2OP_TRIANGLEFAN_IMM: D3DHAL_DP2OPERATION = 23i32;
pub const D3DDP2OP_TRIANGLELIST: D3DHAL_DP2OPERATION = 18i32;
pub const D3DDP2OP_TRIANGLESTRIP: D3DHAL_DP2OPERATION = 19i32;
pub const D3DDP2OP_UPDATEPALETTE: D3DHAL_DP2OPERATION = 31i32;
pub const D3DDP2OP_VIEWPORTINFO: D3DHAL_DP2OPERATION = 28i32;
pub const D3DDP2OP_VOLUMEBLT: D3DHAL_DP2OPERATION = 63i32;
pub const D3DDP2OP_WINFO: D3DHAL_DP2OPERATION = 29i32;
pub const D3DDP2OP_ZRANGE: D3DHAL_DP2OPERATION = 32i32;
pub const D3DFVF_FOG: i32 = 8192i32;
pub const D3DGDI2_MAGIC: u32 = 4294967295u32;
pub const D3DGDI2_TYPE_DEFERRED_AGP_AWARE: u32 = 24u32;
pub const D3DGDI2_TYPE_DEFER_AGP_FREES: u32 = 32u32;
pub const D3DGDI2_TYPE_DXVERSION: u32 = 4u32;
pub const D3DGDI2_TYPE_FREE_DEFERRED_AGP: u32 = 25u32;
pub const D3DGDI2_TYPE_GETADAPTERGROUP: u32 = 19u32;
pub const D3DGDI2_TYPE_GETD3DCAPS8: u32 = 1u32;
pub const D3DGDI2_TYPE_GETD3DCAPS9: u32 = 16u32;
pub const D3DGDI2_TYPE_GETD3DQUERY: u32 = 34u32;
pub const D3DGDI2_TYPE_GETD3DQUERYCOUNT: u32 = 33u32;
pub const D3DGDI2_TYPE_GETDDIVERSION: u32 = 35u32;
pub const D3DGDI2_TYPE_GETEXTENDEDMODE: u32 = 18u32;
pub const D3DGDI2_TYPE_GETEXTENDEDMODECOUNT: u32 = 17u32;
pub const D3DGDI2_TYPE_GETFORMAT: u32 = 3u32;
pub const D3DGDI2_TYPE_GETFORMATCOUNT: u32 = 2u32;
pub const D3DGDI2_TYPE_GETMULTISAMPLEQUALITYLEVELS: u32 = 22u32;
pub const D3DGPU_NULL: u32 = 0u32;
pub const D3DHAL2_CB32_CLEAR: i32 = 2i32;
pub const D3DHAL2_CB32_DRAWONEINDEXEDPRIMITIVE: i32 = 8i32;
pub const D3DHAL2_CB32_DRAWONEPRIMITIVE: i32 = 4i32;
pub const D3DHAL2_CB32_DRAWPRIMITIVES: i32 = 16i32;
pub const D3DHAL2_CB32_SETRENDERTARGET: i32 = 1i32;
pub const D3DHAL3_CB32_CLEAR2: i32 = 1i32;
pub const D3DHAL3_CB32_DRAWPRIMITIVES2: i32 = 8i32;
pub const D3DHAL3_CB32_RESERVED: i32 = 2i32;
pub const D3DHAL3_CB32_VALIDATETEXTURESTAGESTATE: i32 = 4i32;
pub const D3DHALDP2_EXECUTEBUFFER: i32 = 2i32;
pub const D3DHALDP2_REQCOMMANDBUFSIZE: i32 = 32i32;
pub const D3DHALDP2_REQVERTEXBUFSIZE: i32 = 16i32;
pub const D3DHALDP2_SWAPCOMMANDBUFFER: i32 = 8i32;
pub const D3DHALDP2_SWAPVERTEXBUFFER: i32 = 4i32;
pub const D3DHALDP2_USERMEMVERTICES: i32 = 1i32;
pub const D3DHALDP2_VIDMEMCOMMANDBUF: i32 = 128i32;
pub const D3DHALDP2_VIDMEMVERTEXBUF: i32 = 64i32;
pub const D3DHALSTATE_GET_LIGHT: i32 = 2i32;
pub const D3DHALSTATE_GET_RENDER: i32 = 4i32;
pub const D3DHALSTATE_GET_TRANSFORM: i32 = 1i32;
pub const D3DHAL_COL_WEIGHTS: u32 = 2u32;
pub const D3DHAL_CONTEXT_BAD: i64 = 512i64;
pub const D3DHAL_EXECUTE_ABORT: i32 = 528i32;
pub const D3DHAL_EXECUTE_NORMAL: i32 = 0i32;
pub const D3DHAL_EXECUTE_OVERRIDE: i32 = 1i32;
pub const D3DHAL_EXECUTE_UNHANDLED: i32 = 529i32;
pub const D3DHAL_MAX_RSTATES: u32 = 256u32;
pub const D3DHAL_MAX_RSTATES_DX6: u32 = 256u32;
pub const D3DHAL_MAX_RSTATES_DX7: u32 = 256u32;
pub const D3DHAL_MAX_RSTATES_DX8: u32 = 256u32;
pub const D3DHAL_MAX_RSTATES_DX9: u32 = 256u32;
pub const D3DHAL_MAX_TEXTURESTATES: u32 = 13u32;
pub const D3DHAL_NUMCLIPVERTICES: u32 = 20u32;
pub const D3DHAL_OUTOFCONTEXTS: i64 = 513i64;
pub const D3DHAL_ROW_WEIGHTS: u32 = 1u32;
pub const D3DHAL_SAMPLER_MAXSAMP: u32 = 16u32;
pub const D3DHAL_SAMPLER_MAXVERTEXSAMP: u32 = 4u32;
pub const D3DHAL_SCENE_CAPTURE_END: i32 = 1i32;
pub const D3DHAL_SCENE_CAPTURE_START: i32 = 0i32;
pub const D3DHAL_SETLIGHT_DATA: u32 = 2u32;
pub const D3DHAL_SETLIGHT_DISABLE: u32 = 1u32;
pub const D3DHAL_SETLIGHT_ENABLE: u32 = 0u32;
pub const D3DHAL_STATESETBEGIN: u32 = 0u32;
pub const D3DHAL_STATESETCAPTURE: u32 = 4u32;
pub const D3DHAL_STATESETCREATE: u32 = 5u32;
pub const D3DHAL_STATESETDELETE: u32 = 2u32;
pub const D3DHAL_STATESETEND: u32 = 1u32;
pub const D3DHAL_STATESETEXECUTE: u32 = 3u32;
pub const D3DHAL_TEXTURESTATEBUF_SIZE: u32 = 14u32;
pub const D3DHAL_TSS_MAXSTAGES: u32 = 8u32;
pub const D3DHAL_TSS_RENDERSTATEBASE: u32 = 256u32;
pub const D3DHAL_TSS_STATESPERSTAGE: u32 = 64u32;
pub const D3DINFINITEINSTRUCTIONS: u32 = 4294967295u32;
pub const D3DKMDT_BITS_PER_COMPONENT_06: u32 = 1u32;
pub const D3DKMDT_BITS_PER_COMPONENT_08: u32 = 2u32;
pub const D3DKMDT_BITS_PER_COMPONENT_10: u32 = 4u32;
pub const D3DKMDT_BITS_PER_COMPONENT_12: u32 = 8u32;
pub const D3DKMDT_BITS_PER_COMPONENT_14: u32 = 16u32;
pub const D3DKMDT_BITS_PER_COMPONENT_16: u32 = 32u32;
pub const D3DKMDT_CB_INTENSITY: D3DKMDT_COLOR_BASIS = 1i32;
pub const D3DKMDT_CB_SCRGB: D3DKMDT_COLOR_BASIS = 3i32;
pub const D3DKMDT_CB_SRGB: D3DKMDT_COLOR_BASIS = 2i32;
pub const D3DKMDT_CB_UNINITIALIZED: D3DKMDT_COLOR_BASIS = 0i32;
pub const D3DKMDT_CB_YCBCR: D3DKMDT_COLOR_BASIS = 4i32;
pub const D3DKMDT_CB_YPBPR: D3DKMDT_COLOR_BASIS = 5i32;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_BYPASS: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 2i32;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_ENABLE: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 1i32;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_NO_CHANGE: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 0i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 200i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 100i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_NONE: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 0i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_SHADER_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 500i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_THREAD_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 400i32;
pub const D3DKMDT_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = 300i32;
pub const D3DKMDT_EPT_NOPIVOT: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 5i32;
pub const D3DKMDT_EPT_ROTATION: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 4i32;
pub const D3DKMDT_EPT_SCALING: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 3i32;
pub const D3DKMDT_EPT_UNINITIALIZED: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 0i32;
pub const D3DKMDT_EPT_VIDPNSOURCE: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 1i32;
pub const D3DKMDT_EPT_VIDPNTARGET: D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = 2i32;
pub const D3DKMDT_GDISURFACE_EXISTINGSYSMEM: D3DKMDT_GDISURFACETYPE = 5i32;
pub const D3DKMDT_GDISURFACE_INVALID: D3DKMDT_GDISURFACETYPE = 0i32;
pub const D3DKMDT_GDISURFACE_LOOKUPTABLE: D3DKMDT_GDISURFACETYPE = 4i32;
pub const D3DKMDT_GDISURFACE_STAGING: D3DKMDT_GDISURFACETYPE = 3i32;
pub const D3DKMDT_GDISURFACE_STAGING_CPUVISIBLE: D3DKMDT_GDISURFACETYPE = 2i32;
pub const D3DKMDT_GDISURFACE_TEXTURE: D3DKMDT_GDISURFACETYPE = 1i32;
pub const D3DKMDT_GDISURFACE_TEXTURE_CPUVISIBLE: D3DKMDT_GDISURFACETYPE = 6i32;
pub const D3DKMDT_GDISURFACE_TEXTURE_CPUVISIBLE_CROSSADAPTER: D3DKMDT_GDISURFACETYPE = 8i32;
pub const D3DKMDT_GDISURFACE_TEXTURE_CROSSADAPTER: D3DKMDT_GDISURFACETYPE = 7i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 100i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_NONE: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 0i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 400i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 200i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_SHADER_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 500i32;
pub const D3DKMDT_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = 300i32;
pub const D3DKMDT_GTF_COMPLIANT: D3DKMDT_GTFCOMPLIANCE = 1i32;
pub const D3DKMDT_GTF_NOTCOMPLIANT: D3DKMDT_GTFCOMPLIANCE = 2i32;
pub const D3DKMDT_GTF_UNINITIALIZED: D3DKMDT_GTFCOMPLIANCE = 0i32;
pub const D3DKMDT_MACROVISION_OEMCOPYPROTECTION_SIZE: u32 = 256u32;
pub const D3DKMDT_MAX_OVERLAYS_BITCOUNT: u32 = 2u32;
pub const D3DKMDT_MAX_VIDPN_SOURCES_BITCOUNT: u32 = 4u32;
pub const D3DKMDT_MCC_ENFORCE: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 2i32;
pub const D3DKMDT_MCC_IGNORE: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 1i32;
pub const D3DKMDT_MCC_UNINITIALIZED: D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = 0i32;
pub const D3DKMDT_MCO_DEFAULTMONITORPROFILE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 1i32;
pub const D3DKMDT_MCO_DRIVER: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 5i32;
pub const D3DKMDT_MCO_MONITORDESCRIPTOR: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 2i32;
pub const D3DKMDT_MCO_MONITORDESCRIPTOR_REGISTRYOVERRIDE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 3i32;
pub const D3DKMDT_MCO_SPECIFICCAP_REGISTRYOVERRIDE: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 4i32;
pub const D3DKMDT_MCO_UNINITIALIZED: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = 0i32;
pub const D3DKMDT_MDT_OTHER: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 255i32;
pub const D3DKMDT_MDT_UNINITIALIZED: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 0i32;
pub const D3DKMDT_MDT_VESA_EDID_V1_BASEBLOCK: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 1i32;
pub const D3DKMDT_MDT_VESA_EDID_V1_BLOCKMAP: D3DKMDT_MONITOR_DESCRIPTOR_TYPE = 2i32;
pub const D3DKMDT_MFRC_ACTIVESIZE: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 1i32;
pub const D3DKMDT_MFRC_MAXPIXELRATE: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 2i32;
pub const D3DKMDT_MFRC_UNINITIALIZED: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = 0i32;
pub const D3DKMDT_MOA_INTERRUPTIBLE: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 3i32;
pub const D3DKMDT_MOA_NONE: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 1i32;
pub const D3DKMDT_MOA_POLLED: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 2i32;
pub const D3DKMDT_MOA_UNINITIALIZED: D3DKMDT_MONITOR_ORIENTATION_AWARENESS = 0i32;
pub const D3DKMDT_MO_0DEG: D3DKMDT_MONITOR_ORIENTATION = 1i32;
pub const D3DKMDT_MO_180DEG: D3DKMDT_MONITOR_ORIENTATION = 3i32;
pub const D3DKMDT_MO_270DEG: D3DKMDT_MONITOR_ORIENTATION = 4i32;
pub const D3DKMDT_MO_90DEG: D3DKMDT_MONITOR_ORIENTATION = 2i32;
pub const D3DKMDT_MO_UNINITIALIZED: D3DKMDT_MONITOR_ORIENTATION = 0i32;
pub const D3DKMDT_MPR_ALLCAPS: D3DKMDT_MODE_PRUNING_REASON = 1i32;
pub const D3DKMDT_MPR_CLONE_PATH_PRUNED: D3DKMDT_MODE_PRUNING_REASON = 9i32;
pub const D3DKMDT_MPR_DEFAULT_PROFILE_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 6i32;
pub const D3DKMDT_MPR_DESCRIPTOR_MONITOR_FREQUENCY_RANGE: D3DKMDT_MODE_PRUNING_REASON = 3i32;
pub const D3DKMDT_MPR_DESCRIPTOR_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 2i32;
pub const D3DKMDT_MPR_DESCRIPTOR_OVERRIDE_MONITOR_FREQUENCY_RANGE: D3DKMDT_MODE_PRUNING_REASON = 5i32;
pub const D3DKMDT_MPR_DESCRIPTOR_OVERRIDE_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 4i32;
pub const D3DKMDT_MPR_DRIVER_RECOMMENDED_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 7i32;
pub const D3DKMDT_MPR_MAXVALID: D3DKMDT_MODE_PRUNING_REASON = 10i32;
pub const D3DKMDT_MPR_MONITOR_FREQUENCY_RANGE_OVERRIDE: D3DKMDT_MODE_PRUNING_REASON = 8i32;
pub const D3DKMDT_MPR_UNINITIALIZED: D3DKMDT_MODE_PRUNING_REASON = 0i32;
pub const D3DKMDT_MP_NOTPREFERRED: D3DKMDT_MODE_PREFERENCE = 2i32;
pub const D3DKMDT_MP_PREFERRED: D3DKMDT_MODE_PREFERENCE = 1i32;
pub const D3DKMDT_MP_UNINITIALIZED: D3DKMDT_MODE_PREFERENCE = 0i32;
pub const D3DKMDT_MTT_DEFAULTMONITORPROFILE: D3DKMDT_MONITOR_TIMING_TYPE = 5i32;
pub const D3DKMDT_MTT_DETAILED: D3DKMDT_MONITOR_TIMING_TYPE = 4i32;
pub const D3DKMDT_MTT_DRIVER: D3DKMDT_MONITOR_TIMING_TYPE = 6i32;
pub const D3DKMDT_MTT_ESTABLISHED: D3DKMDT_MONITOR_TIMING_TYPE = 1i32;
pub const D3DKMDT_MTT_EXTRASTANDARD: D3DKMDT_MONITOR_TIMING_TYPE = 3i32;
pub const D3DKMDT_MTT_STANDARD: D3DKMDT_MONITOR_TIMING_TYPE = 2i32;
pub const D3DKMDT_MTT_UNINITIALIZED: D3DKMDT_MONITOR_TIMING_TYPE = 0i32;
pub const D3DKMDT_PVAM_DIRECT: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 1i32;
pub const D3DKMDT_PVAM_PRESETPALETTE: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 2i32;
pub const D3DKMDT_PVAM_SETTABLEPALETTE: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 3i32;
pub const D3DKMDT_PVAM_UNINITIALIZED: D3DKMDT_PIXEL_VALUE_ACCESS_MODE = 0i32;
pub const D3DKMDT_RMT_GRAPHICS: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 1i32;
pub const D3DKMDT_RMT_GRAPHICS_STEREO: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 3i32;
pub const D3DKMDT_RMT_GRAPHICS_STEREO_ADVANCED_SCAN: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 4i32;
pub const D3DKMDT_RMT_TEXT: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 2i32;
pub const D3DKMDT_RMT_UNINITIALIZED: D3DKMDT_VIDPN_SOURCE_MODE_TYPE = 0i32;
pub const D3DKMDT_STANDARDALLOCATION_GDISURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 4i32;
pub const D3DKMDT_STANDARDALLOCATION_SHADOWSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 2i32;
pub const D3DKMDT_STANDARDALLOCATION_SHAREDPRIMARYSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 1i32;
pub const D3DKMDT_STANDARDALLOCATION_STAGINGSURFACE: D3DKMDT_STANDARDALLOCATION_TYPE = 3i32;
pub const D3DKMDT_STANDARDALLOCATION_VGPU: D3DKMDT_STANDARDALLOCATION_TYPE = 5i32;
pub const D3DKMDT_TRF_UNINITIALIZED: D3DKMDT_TEXT_RENDERING_FORMAT = 0i32;
pub const D3DKMDT_VOT_BNC: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3i32;
pub const D3DKMDT_VOT_COMPONENT_VIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3i32;
pub const D3DKMDT_VOT_COMPOSITE_VIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 2i32;
pub const D3DKMDT_VOT_DISPLAYPORT_EMBEDDED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 11i32;
pub const D3DKMDT_VOT_DISPLAYPORT_EXTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 10i32;
pub const D3DKMDT_VOT_DVI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 4i32;
pub const D3DKMDT_VOT_D_JPN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 8i32;
pub const D3DKMDT_VOT_HD15: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 0i32;
pub const D3DKMDT_VOT_HDMI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 5i32;
pub const D3DKMDT_VOT_INDIRECT_WIRED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 16i32;
pub const D3DKMDT_VOT_INTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -2147483648i32;
pub const D3DKMDT_VOT_LVDS: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 6i32;
pub const D3DKMDT_VOT_MIRACAST: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 15i32;
pub const D3DKMDT_VOT_OTHER: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -1i32;
pub const D3DKMDT_VOT_RCA_3COMPONENT: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 3i32;
pub const D3DKMDT_VOT_RF: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 2i32;
pub const D3DKMDT_VOT_SDI: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 9i32;
pub const D3DKMDT_VOT_SDTVDONGLE: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 14i32;
pub const D3DKMDT_VOT_SVIDEO: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1i32;
pub const D3DKMDT_VOT_SVIDEO_4PIN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1i32;
pub const D3DKMDT_VOT_SVIDEO_7PIN: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 1i32;
pub const D3DKMDT_VOT_UDI_EMBEDDED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 13i32;
pub const D3DKMDT_VOT_UDI_EXTERNAL: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = 12i32;
pub const D3DKMDT_VOT_UNINITIALIZED: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = -2i32;
pub const D3DKMDT_VPPC_GRAPHICS: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 1i32;
pub const D3DKMDT_VPPC_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 255i32;
pub const D3DKMDT_VPPC_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 0i32;
pub const D3DKMDT_VPPC_VIDEO: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = 2i32;
pub const D3DKMDT_VPPI_DENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 10i32;
pub const D3DKMDT_VPPI_NONARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 9i32;
pub const D3DKMDT_VPPI_OCTONARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 8i32;
pub const D3DKMDT_VPPI_PRIMARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 1i32;
pub const D3DKMDT_VPPI_QUATERNARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 4i32;
pub const D3DKMDT_VPPI_QUINARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 5i32;
pub const D3DKMDT_VPPI_SECONDARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 2i32;
pub const D3DKMDT_VPPI_SENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 6i32;
pub const D3DKMDT_VPPI_SEPTENARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 7i32;
pub const D3DKMDT_VPPI_TERTIARY: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 3i32;
pub const D3DKMDT_VPPI_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = 0i32;
pub const D3DKMDT_VPPMT_MACROVISION_APSTRIGGER: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 2i32;
pub const D3DKMDT_VPPMT_MACROVISION_FULLSUPPORT: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 3i32;
pub const D3DKMDT_VPPMT_NOPROTECTION: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 1i32;
pub const D3DKMDT_VPPMT_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = 0i32;
pub const D3DKMDT_VPPR_IDENTITY: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 1i32;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 9i32;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 13i32;
pub const D3DKMDT_VPPR_IDENTITY_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 5i32;
pub const D3DKMDT_VPPR_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 255i32;
pub const D3DKMDT_VPPR_ROTATE180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 3i32;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 11i32;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 15i32;
pub const D3DKMDT_VPPR_ROTATE180_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 7i32;
pub const D3DKMDT_VPPR_ROTATE270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 4i32;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 12i32;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 16i32;
pub const D3DKMDT_VPPR_ROTATE270_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 8i32;
pub const D3DKMDT_VPPR_ROTATE90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 2i32;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET180: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 10i32;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET270: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 14i32;
pub const D3DKMDT_VPPR_ROTATE90_OFFSET90: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 6i32;
pub const D3DKMDT_VPPR_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 0i32;
pub const D3DKMDT_VPPR_UNPINNED: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = 254i32;
pub const D3DKMDT_VPPS_ASPECTRATIOCENTEREDMAX: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 4i32;
pub const D3DKMDT_VPPS_CENTERED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 2i32;
pub const D3DKMDT_VPPS_CUSTOM: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 5i32;
pub const D3DKMDT_VPPS_IDENTITY: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 1i32;
pub const D3DKMDT_VPPS_NOTSPECIFIED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 255i32;
pub const D3DKMDT_VPPS_RESERVED1: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 253i32;
pub const D3DKMDT_VPPS_STRETCHED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 3i32;
pub const D3DKMDT_VPPS_UNINITIALIZED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 0i32;
pub const D3DKMDT_VPPS_UNPINNED: D3DKMDT_VIDPN_PRESENT_PATH_SCALING = 254i32;
pub const D3DKMDT_VSS_APPLE: D3DKMDT_VIDEO_SIGNAL_STANDARD = 5i32;
pub const D3DKMDT_VSS_EIA_861: D3DKMDT_VIDEO_SIGNAL_STANDARD = 25i32;
pub const D3DKMDT_VSS_EIA_861A: D3DKMDT_VIDEO_SIGNAL_STANDARD = 26i32;
pub const D3DKMDT_VSS_EIA_861B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 27i32;
pub const D3DKMDT_VSS_IBM: D3DKMDT_VIDEO_SIGNAL_STANDARD = 4i32;
pub const D3DKMDT_VSS_NTSC_443: D3DKMDT_VIDEO_SIGNAL_STANDARD = 8i32;
pub const D3DKMDT_VSS_NTSC_J: D3DKMDT_VIDEO_SIGNAL_STANDARD = 7i32;
pub const D3DKMDT_VSS_NTSC_M: D3DKMDT_VIDEO_SIGNAL_STANDARD = 6i32;
pub const D3DKMDT_VSS_OTHER: D3DKMDT_VIDEO_SIGNAL_STANDARD = 255i32;
pub const D3DKMDT_VSS_PAL_B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 9i32;
pub const D3DKMDT_VSS_PAL_B1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 10i32;
pub const D3DKMDT_VSS_PAL_D: D3DKMDT_VIDEO_SIGNAL_STANDARD = 14i32;
pub const D3DKMDT_VSS_PAL_G: D3DKMDT_VIDEO_SIGNAL_STANDARD = 11i32;
pub const D3DKMDT_VSS_PAL_H: D3DKMDT_VIDEO_SIGNAL_STANDARD = 12i32;
pub const D3DKMDT_VSS_PAL_I: D3DKMDT_VIDEO_SIGNAL_STANDARD = 13i32;
pub const D3DKMDT_VSS_PAL_K: D3DKMDT_VIDEO_SIGNAL_STANDARD = 28i32;
pub const D3DKMDT_VSS_PAL_K1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 29i32;
pub const D3DKMDT_VSS_PAL_L: D3DKMDT_VIDEO_SIGNAL_STANDARD = 30i32;
pub const D3DKMDT_VSS_PAL_M: D3DKMDT_VIDEO_SIGNAL_STANDARD = 31i32;
pub const D3DKMDT_VSS_PAL_N: D3DKMDT_VIDEO_SIGNAL_STANDARD = 15i32;
pub const D3DKMDT_VSS_PAL_NC: D3DKMDT_VIDEO_SIGNAL_STANDARD = 16i32;
pub const D3DKMDT_VSS_SECAM_B: D3DKMDT_VIDEO_SIGNAL_STANDARD = 17i32;
pub const D3DKMDT_VSS_SECAM_D: D3DKMDT_VIDEO_SIGNAL_STANDARD = 18i32;
pub const D3DKMDT_VSS_SECAM_G: D3DKMDT_VIDEO_SIGNAL_STANDARD = 19i32;
pub const D3DKMDT_VSS_SECAM_H: D3DKMDT_VIDEO_SIGNAL_STANDARD = 20i32;
pub const D3DKMDT_VSS_SECAM_K: D3DKMDT_VIDEO_SIGNAL_STANDARD = 21i32;
pub const D3DKMDT_VSS_SECAM_K1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 22i32;
pub const D3DKMDT_VSS_SECAM_L: D3DKMDT_VIDEO_SIGNAL_STANDARD = 23i32;
pub const D3DKMDT_VSS_SECAM_L1: D3DKMDT_VIDEO_SIGNAL_STANDARD = 24i32;
pub const D3DKMDT_VSS_UNINITIALIZED: D3DKMDT_VIDEO_SIGNAL_STANDARD = 0i32;
pub const D3DKMDT_VSS_VESA_CVT: D3DKMDT_VIDEO_SIGNAL_STANDARD = 3i32;
pub const D3DKMDT_VSS_VESA_DMT: D3DKMDT_VIDEO_SIGNAL_STANDARD = 1i32;
pub const D3DKMDT_VSS_VESA_GTF: D3DKMDT_VIDEO_SIGNAL_STANDARD = 2i32;
pub const D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE_EXTRA_CCD_DATABASE_INFO: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = 0i32;
pub const D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE_MODES_PRUNED: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = 15i32;
pub const D3DKMT_ADAPTER_VERIFIER_OPTION_VIDMM_FLAGS: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = 1000i32;
pub const D3DKMT_ADAPTER_VERIFIER_OPTION_VIDMM_TRIM_INTERVAL: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = 1001i32;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_NOTRESIDENT: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 3i32;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_RESIDENTINGPUMEMORY: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 1i32;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_RESIDENTINSHAREDMEMORY: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 2i32;
pub const D3DKMT_AUXILIARYPRESENTINFO_TYPE_FLIPMANAGER: D3DKMT_AUXILIARYPRESENTINFO_TYPE = 0i32;
pub const D3DKMT_AllocationPriorityClassHigh: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 3i32;
pub const D3DKMT_AllocationPriorityClassLow: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 1i32;
pub const D3DKMT_AllocationPriorityClassMaximum: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 4i32;
pub const D3DKMT_AllocationPriorityClassMinimum: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 0i32;
pub const D3DKMT_AllocationPriorityClassNormal: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 2i32;
pub const D3DKMT_BRIGHTNESS_INFO_BEGIN_MANUAL_MODE: D3DKMT_BRIGHTNESS_INFO_TYPE = 8i32;
pub const D3DKMT_BRIGHTNESS_INFO_END_MANUAL_MODE: D3DKMT_BRIGHTNESS_INFO_TYPE = 9i32;
pub const D3DKMT_BRIGHTNESS_INFO_GET: D3DKMT_BRIGHTNESS_INFO_TYPE = 2i32;
pub const D3DKMT_BRIGHTNESS_INFO_GET_CAPS: D3DKMT_BRIGHTNESS_INFO_TYPE = 4i32;
pub const D3DKMT_BRIGHTNESS_INFO_GET_NIT_RANGES: D3DKMT_BRIGHTNESS_INFO_TYPE = 11i32;
pub const D3DKMT_BRIGHTNESS_INFO_GET_POSSIBLE_LEVELS: D3DKMT_BRIGHTNESS_INFO_TYPE = 1i32;
pub const D3DKMT_BRIGHTNESS_INFO_GET_REDUCTION: D3DKMT_BRIGHTNESS_INFO_TYPE = 7i32;
pub const D3DKMT_BRIGHTNESS_INFO_SET: D3DKMT_BRIGHTNESS_INFO_TYPE = 3i32;
pub const D3DKMT_BRIGHTNESS_INFO_SET_OPTIMIZATION: D3DKMT_BRIGHTNESS_INFO_TYPE = 6i32;
pub const D3DKMT_BRIGHTNESS_INFO_SET_STATE: D3DKMT_BRIGHTNESS_INFO_TYPE = 5i32;
pub const D3DKMT_BRIGHTNESS_INFO_TOGGLE_LOGGING: D3DKMT_BRIGHTNESS_INFO_TYPE = 10i32;
pub const D3DKMT_CANCEL_PRESENTS_OPERATION_CANCEL_FROM: D3DKMT_CANCEL_PRESENTS_OPERATION = 0i32;
pub const D3DKMT_CANCEL_PRESENTS_OPERATION_REPROGRAM_INTERRUPT: D3DKMT_CANCEL_PRESENTS_OPERATION = 1i32;
pub const D3DKMT_CLIENTHINT_11ON12: D3DKMT_CLIENTHINT = 14i32;
pub const D3DKMT_CLIENTHINT_9ON12: D3DKMT_CLIENTHINT = 13i32;
pub const D3DKMT_CLIENTHINT_CDD: D3DKMT_CLIENTHINT = 2i32;
pub const D3DKMT_CLIENTHINT_CLON12: D3DKMT_CLIENTHINT = 17i32;
pub const D3DKMT_CLIENTHINT_CUDA: D3DKMT_CLIENTHINT = 5i32;
pub const D3DKMT_CLIENTHINT_DML_PYTORCH: D3DKMT_CLIENTHINT = 20i32;
pub const D3DKMT_CLIENTHINT_DML_TENSORFLOW: D3DKMT_CLIENTHINT = 18i32;
pub const D3DKMT_CLIENTHINT_DX10: D3DKMT_CLIENTHINT = 10i32;
pub const D3DKMT_CLIENTHINT_DX11: D3DKMT_CLIENTHINT = 11i32;
pub const D3DKMT_CLIENTHINT_DX12: D3DKMT_CLIENTHINT = 12i32;
pub const D3DKMT_CLIENTHINT_DX7: D3DKMT_CLIENTHINT = 7i32;
pub const D3DKMT_CLIENTHINT_DX8: D3DKMT_CLIENTHINT = 8i32;
pub const D3DKMT_CLIENTHINT_DX9: D3DKMT_CLIENTHINT = 9i32;
pub const D3DKMT_CLIENTHINT_GLON12: D3DKMT_CLIENTHINT = 16i32;
pub const D3DKMT_CLIENTHINT_MAX: D3DKMT_CLIENTHINT = 21i32;
pub const D3DKMT_CLIENTHINT_MFT_ENCODE: D3DKMT_CLIENTHINT = 15i32;
pub const D3DKMT_CLIENTHINT_ONEAPI_LEVEL0: D3DKMT_CLIENTHINT = 19i32;
pub const D3DKMT_CLIENTHINT_OPENCL: D3DKMT_CLIENTHINT = 3i32;
pub const D3DKMT_CLIENTHINT_OPENGL: D3DKMT_CLIENTHINT = 1i32;
pub const D3DKMT_CLIENTHINT_RESERVED: D3DKMT_CLIENTHINT = 6i32;
pub const D3DKMT_CLIENTHINT_UNKNOWN: D3DKMT_CLIENTHINT = 0i32;
pub const D3DKMT_CLIENTHINT_VULKAN: D3DKMT_CLIENTHINT = 4i32;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_COPY: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 1i32;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_NONE: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 0i32;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_SCANOUT: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 3i32;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_TEXTURE: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 2i32;
pub const D3DKMT_CROSS_ADAPTER_RESOURCE_HEIGHT_ALIGNMENT: u32 = 4u32;
pub const D3DKMT_CROSS_ADAPTER_RESOURCE_PITCH_ALIGNMENT: u32 = 128u32;
pub const D3DKMT_ClientPagingBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 1i32;
pub const D3DKMT_ClientRenderBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 0i32;
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_DOWNWARD: D3DKMT_DEFRAG_ESCAPE_OPERATION = 2i32;
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_PASS: D3DKMT_DEFRAG_ESCAPE_OPERATION = 3i32;
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_UPWARD: D3DKMT_DEFRAG_ESCAPE_OPERATION = 1i32;
pub const D3DKMT_DEFRAG_ESCAPE_GET_FRAGMENTATION_STATS: D3DKMT_DEFRAG_ESCAPE_OPERATION = 0i32;
pub const D3DKMT_DEFRAG_ESCAPE_VERIFY_TRANSFER: D3DKMT_DEFRAG_ESCAPE_OPERATION = 4i32;
pub const D3DKMT_DEVICEESCAPE_RESTOREGAMMA: D3DKMT_DEVICEESCAPE_TYPE = 1i32;
pub const D3DKMT_DEVICEESCAPE_VIDPNFROMALLOCATION: D3DKMT_DEVICEESCAPE_TYPE = 0i32;
pub const D3DKMT_DEVICEEXECUTION_ACTIVE: D3DKMT_DEVICEEXECUTION_STATE = 1i32;
pub const D3DKMT_DEVICEEXECUTION_ERROR_DMAFAULT: D3DKMT_DEVICEEXECUTION_STATE = 6i32;
pub const D3DKMT_DEVICEEXECUTION_ERROR_DMAPAGEFAULT: D3DKMT_DEVICEEXECUTION_STATE = 7i32;
pub const D3DKMT_DEVICEEXECUTION_ERROR_OUTOFMEMORY: D3DKMT_DEVICEEXECUTION_STATE = 5i32;
pub const D3DKMT_DEVICEEXECUTION_HUNG: D3DKMT_DEVICEEXECUTION_STATE = 3i32;
pub const D3DKMT_DEVICEEXECUTION_RESET: D3DKMT_DEVICEEXECUTION_STATE = 2i32;
pub const D3DKMT_DEVICEEXECUTION_STOPPED: D3DKMT_DEVICEEXECUTION_STATE = 4i32;
pub const D3DKMT_DEVICESTATE_EXECUTION: D3DKMT_DEVICESTATE_TYPE = 1i32;
pub const D3DKMT_DEVICESTATE_PAGE_FAULT: D3DKMT_DEVICESTATE_TYPE = 5i32;
pub const D3DKMT_DEVICESTATE_PRESENT: D3DKMT_DEVICESTATE_TYPE = 2i32;
pub const D3DKMT_DEVICESTATE_PRESENT_DWM: D3DKMT_DEVICESTATE_TYPE = 4i32;
pub const D3DKMT_DEVICESTATE_PRESENT_QUEUE: D3DKMT_DEVICESTATE_TYPE = 6i32;
pub const D3DKMT_DEVICESTATE_RESET: D3DKMT_DEVICESTATE_TYPE = 3i32;
pub const D3DKMT_DEVICE_ERROR_REASON_DRIVER_ERROR: D3DKMT_DEVICE_ERROR_REASON = -2147483642i32;
pub const D3DKMT_DEVICE_ERROR_REASON_GENERIC: D3DKMT_DEVICE_ERROR_REASON = -2147483648i32;
pub const D3DKMT_DMMESCAPETYPE_ACTIVEVIDPN_COFUNCPATHMODALITY_INFO: D3DKMT_DMMESCAPETYPE = 11i32;
pub const D3DKMT_DMMESCAPETYPE_ACTIVEVIDPN_SOURCEMODESET_INFO: D3DKMT_DMMESCAPETYPE = 10i32;
pub const D3DKMT_DMMESCAPETYPE_GET_ACTIVEVIDPN_INFO: D3DKMT_DMMESCAPETYPE = 4i32;
pub const D3DKMT_DMMESCAPETYPE_GET_LASTCLIENTCOMMITTEDVIDPN_INFO: D3DKMT_DMMESCAPETYPE = 12i32;
pub const D3DKMT_DMMESCAPETYPE_GET_MONITORS_INFO: D3DKMT_DMMESCAPETYPE = 5i32;
pub const D3DKMT_DMMESCAPETYPE_GET_SUMMARY_INFO: D3DKMT_DMMESCAPETYPE = 1i32;
pub const D3DKMT_DMMESCAPETYPE_GET_VERSION_INFO: D3DKMT_DMMESCAPETYPE = 13i32;
pub const D3DKMT_DMMESCAPETYPE_GET_VIDEO_PRESENT_SOURCES_INFO: D3DKMT_DMMESCAPETYPE = 2i32;
pub const D3DKMT_DMMESCAPETYPE_GET_VIDEO_PRESENT_TARGETS_INFO: D3DKMT_DMMESCAPETYPE = 3i32;
pub const D3DKMT_DMMESCAPETYPE_RECENTLY_COMMITTED_VIDPNS_INFO: D3DKMT_DMMESCAPETYPE = 6i32;
pub const D3DKMT_DMMESCAPETYPE_RECENTLY_RECOMMENDED_VIDPNS_INFO: D3DKMT_DMMESCAPETYPE = 8i32;
pub const D3DKMT_DMMESCAPETYPE_RECENT_MODECHANGE_REQUESTS_INFO: D3DKMT_DMMESCAPETYPE = 7i32;
pub const D3DKMT_DMMESCAPETYPE_RECENT_MONITOR_PRESENCE_EVENTS_INFO: D3DKMT_DMMESCAPETYPE = 9i32;
pub const D3DKMT_DMMESCAPETYPE_UNINITIALIZED: D3DKMT_DMMESCAPETYPE = 0i32;
pub const D3DKMT_DMMESCAPETYPE_VIDPN_MGR_DIAGNOSTICS: D3DKMT_DMMESCAPETYPE = 14i32;
pub const D3DKMT_DeferredCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 1i32;
pub const D3DKMT_DeviceCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 6i32;
pub const D3DKMT_DmaPacketTypeMax: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 4i32;
pub const D3DKMT_ESCAPE_ACTIVATE_SPECIFIC_DIAG: D3DKMT_ESCAPETYPE = 14i32;
pub const D3DKMT_ESCAPE_ADAPTER_VERIFIER_OPTION: D3DKMT_ESCAPETYPE = 29i32;
pub const D3DKMT_ESCAPE_BDD_FALLBACK: D3DKMT_ESCAPETYPE = 13i32;
pub const D3DKMT_ESCAPE_BDD_PNP: D3DKMT_ESCAPETYPE = 12i32;
pub const D3DKMT_ESCAPE_BRIGHTNESS: D3DKMT_ESCAPETYPE = 17i32;
pub const D3DKMT_ESCAPE_CCD_DATABASE: D3DKMT_ESCAPETYPE = 38i32;
pub const D3DKMT_ESCAPE_DEBUG_SNAPSHOT: D3DKMT_ESCAPETYPE = 6i32;
pub const D3DKMT_ESCAPE_DEVICE: D3DKMT_ESCAPETYPE = 4i32;
pub const D3DKMT_ESCAPE_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 9i32;
pub const D3DKMT_ESCAPE_DMM: D3DKMT_ESCAPETYPE = 5i32;
pub const D3DKMT_ESCAPE_DOD_SET_DIRTYRECT_MODE: D3DKMT_ESCAPETYPE = 31i32;
pub const D3DKMT_ESCAPE_DRIVERPRIVATE: D3DKMT_ESCAPETYPE = 0i32;
pub const D3DKMT_ESCAPE_DRT_TEST: D3DKMT_ESCAPETYPE = 8i32;
pub const D3DKMT_ESCAPE_EDID_CACHE: D3DKMT_ESCAPETYPE = 18i32;
pub const D3DKMT_ESCAPE_FORCE_BDDFALLBACK_HEADLESS: D3DKMT_ESCAPETYPE = 24i32;
pub const D3DKMT_ESCAPE_GET_DISPLAY_CONFIGURATIONS: D3DKMT_ESCAPETYPE = 36i32;
pub const D3DKMT_ESCAPE_GET_EXTERNAL_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 34i32;
pub const D3DKMT_ESCAPE_HISTORY_BUFFER_STATUS: D3DKMT_ESCAPETYPE = 21i32;
pub const D3DKMT_ESCAPE_IDD_REQUEST: D3DKMT_ESCAPETYPE = 30i32;
pub const D3DKMT_ESCAPE_LOG_CODEPOINT_PACKET: D3DKMT_ESCAPETYPE = 32i32;
pub const D3DKMT_ESCAPE_LOG_USERMODE_DAIG_PACKET: D3DKMT_ESCAPETYPE = 33i32;
pub const D3DKMT_ESCAPE_MIRACAST_ADAPTER_DIAG_INFO: D3DKMT_ESCAPETYPE = 23i32;
pub const D3DKMT_ESCAPE_MIRACAST_DISPLAY_REQUEST: D3DKMT_ESCAPETYPE = 20i32;
pub const D3DKMT_ESCAPE_MODES_PRUNED_OUT: D3DKMT_ESCAPETYPE = 15i32;
pub const D3DKMT_ESCAPE_OUTPUTDUPL_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 11i32;
pub const D3DKMT_ESCAPE_OUTPUTDUPL_SNAPSHOT: D3DKMT_ESCAPETYPE = 10i32;
pub const D3DKMT_ESCAPE_PFN_CONTROL_DEFAULT: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 0i32;
pub const D3DKMT_ESCAPE_PFN_CONTROL_FORCE_CPU: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 1i32;
pub const D3DKMT_ESCAPE_PFN_CONTROL_FORCE_GPU: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 2i32;
pub const D3DKMT_ESCAPE_PROCESS_VERIFIER_OPTION: D3DKMT_ESCAPETYPE = 28i32;
pub const D3DKMT_ESCAPE_QUERY_DMA_REMAPPING_STATUS: D3DKMT_ESCAPETYPE = 39i32;
pub const D3DKMT_ESCAPE_QUERY_IOMMU_STATUS: D3DKMT_ESCAPETYPE = 37i32;
pub const D3DKMT_ESCAPE_REQUEST_MACHINE_CRASH: D3DKMT_ESCAPETYPE = 25i32;
pub const D3DKMT_ESCAPE_SOFTGPU_ENABLE_DISABLE_HMD: D3DKMT_ESCAPETYPE = 27i32;
pub const D3DKMT_ESCAPE_TDRDBGCTRL: D3DKMT_ESCAPETYPE = 2i32;
pub const D3DKMT_ESCAPE_VIDMM: D3DKMT_ESCAPETYPE = 1i32;
pub const D3DKMT_ESCAPE_VIDSCH: D3DKMT_ESCAPETYPE = 3i32;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_BASE_DESKTOP_DURATION: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 0i32;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_PROCESS_BOOST_ELIGIBLE: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 2i32;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_VSYNC_MULTIPLIER: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 1i32;
pub const D3DKMT_ESCAPE_WHQL_INFO: D3DKMT_ESCAPETYPE = 16i32;
pub const D3DKMT_ESCAPE_WIN32K_BDD_FALLBACK: D3DKMT_ESCAPETYPE = 1029i32;
pub const D3DKMT_ESCAPE_WIN32K_COLOR_PROFILE_INFO: D3DKMT_ESCAPETYPE = 1036i32;
pub const D3DKMT_ESCAPE_WIN32K_DDA_TEST_CTL: D3DKMT_ESCAPETYPE = 1030i32;
pub const D3DKMT_ESCAPE_WIN32K_DISPBROKER_TEST: D3DKMT_ESCAPETYPE = 1035i32;
pub const D3DKMT_ESCAPE_WIN32K_DPI_INFO: D3DKMT_ESCAPETYPE = 1026i32;
pub const D3DKMT_ESCAPE_WIN32K_HIP_DEVICE_INFO: D3DKMT_ESCAPETYPE = 1024i32;
pub const D3DKMT_ESCAPE_WIN32K_PRESENTER_VIEW_INFO: D3DKMT_ESCAPETYPE = 1027i32;
pub const D3DKMT_ESCAPE_WIN32K_QUERY_CD_ROTATION_BLOCK: D3DKMT_ESCAPETYPE = 1025i32;
pub const D3DKMT_ESCAPE_WIN32K_SET_DIMMED_STATE: D3DKMT_ESCAPETYPE = 1037i32;
pub const D3DKMT_ESCAPE_WIN32K_SPECIALIZED_DISPLAY_TEST: D3DKMT_ESCAPETYPE = 1038i32;
pub const D3DKMT_ESCAPE_WIN32K_START: D3DKMT_ESCAPETYPE = 1024i32;
pub const D3DKMT_ESCAPE_WIN32K_SYSTEM_DPI: D3DKMT_ESCAPETYPE = 1028i32;
pub const D3DKMT_ESCAPE_WIN32K_USER_DETECTED_BLACK_SCREEN: D3DKMT_ESCAPETYPE = 1031i32;
pub const D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE_FLIP_COMPLETE: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = 1i32;
pub const D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE_FLIP_SUBMITTED: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = 0i32;
pub const D3DKMT_GDI_STYLE_HANDLE_DECORATION: u32 = 2u32;
pub const D3DKMT_GETPRESENTHISTORY_MAXTOKENS: u32 = 2048u32;
pub const D3DKMT_GET_PTE_MAX: u32 = 64u32;
pub const D3DKMT_GET_QUEUEDLIMIT_PRESENT: D3DKMT_QUEUEDLIMIT_TYPE = 2i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_HIGH_PERFORMANCE: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 1i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_MINIMUM_POWER: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 2i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_NOT_FOUND: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 4i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_UNINITIALIZED: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 0i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_UNSPECIFIED: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 3i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_USER_SPECIFIED_GPU: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 5i32;
pub const D3DKMT_GPU_PREFERENCE_TYPE_DX_DATABASE: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 1i32;
pub const D3DKMT_GPU_PREFERENCE_TYPE_IHV_DLIST: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 0i32;
pub const D3DKMT_GPU_PREFERENCE_TYPE_USER_PREFERENCE: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 2i32;
pub const D3DKMT_MAX_BUNDLE_OBJECTS_PER_HANDLE: u32 = 16u32;
pub const D3DKMT_MAX_DMM_ESCAPE_DATASIZE: i32 = 102400i32;
pub const D3DKMT_MAX_MULTIPLANE_OVERLAY_ALLOCATIONS_PER_PLANE: u32 = 256u32;
pub const D3DKMT_MAX_MULTIPLANE_OVERLAY_PLANES: u32 = 8u32;
pub const D3DKMT_MAX_OBJECTS_PER_HANDLE: u32 = 3u32;
pub const D3DKMT_MAX_PRESENT_HISTORY_RECTS: u32 = 16u32;
pub const D3DKMT_MAX_PRESENT_HISTORY_SCATTERBLTS: u32 = 12u32;
pub const D3DKMT_MAX_SEGMENT_COUNT: u32 = 32u32;
pub const D3DKMT_MAX_WAITFORVERTICALBLANK_OBJECTS: u32 = 8u32;
pub const D3DKMT_MEMORY_SEGMENT_GROUP_LOCAL: D3DKMT_MEMORY_SEGMENT_GROUP = 0i32;
pub const D3DKMT_MEMORY_SEGMENT_GROUP_NON_LOCAL: D3DKMT_MEMORY_SEGMENT_GROUP = 1i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_CANCELLED: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483637i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_ERROR: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483645i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_NOT_FOUND: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483642i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_NOT_STARTED: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483641i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_GPU_RESOURCE_IN_USE: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483646i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INSUFFICIENT_BANDWIDTH: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483639i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INSUFFICIENT_MEMORY: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483638i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INVALID_PARAMETER: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483640i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_PENDING: D3DKMT_MIRACAST_DEVICE_STATUS = 2i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_REMOTE_SESSION: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483643i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_SUCCESS: D3DKMT_MIRACAST_DEVICE_STATUS = 0i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_SUCCESS_NO_MONITOR: D3DKMT_MIRACAST_DEVICE_STATUS = 1i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_UNKOWN_ERROR: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483647i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_UNKOWN_PAIRING: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483644i32;
pub const D3DKMT_MIRACAST_DRIVER_IHV: D3DKMT_MIRACAST_DRIVER_TYPE = 1i32;
pub const D3DKMT_MIRACAST_DRIVER_MS: D3DKMT_MIRACAST_DRIVER_TYPE = 2i32;
pub const D3DKMT_MIRACAST_DRIVER_NOT_SUPPORTED: D3DKMT_MIRACAST_DRIVER_TYPE = 0i32;
pub const D3DKMT_MULIIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 0i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_BLEND_ALPHABLEND: D3DKMT_MULTIPLANE_OVERLAY_BLEND = 1i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_BLEND_OPAQUE: D3DKMT_MULTIPLANE_OVERLAY_BLEND = 0i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_HORIZONTAL_FLIP: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 2i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_STATIC_CHECK: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 4i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_VERTICAL_FLIP: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 1i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_HORIZONTAL: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 1i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_VERTICAL: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 2i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 2i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 1i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4i32;
pub const D3DKMT_MaxAllocationPriorityClass: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 5i32;
pub const D3DKMT_MmIoFlipCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 3i32;
pub const D3DKMT_OFFER_PRIORITY_AUTO: D3DKMT_OFFER_PRIORITY = 4i32;
pub const D3DKMT_OFFER_PRIORITY_HIGH: D3DKMT_OFFER_PRIORITY = 3i32;
pub const D3DKMT_OFFER_PRIORITY_LOW: D3DKMT_OFFER_PRIORITY = 1i32;
pub const D3DKMT_OFFER_PRIORITY_NORMAL: D3DKMT_OFFER_PRIORITY = 2i32;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 2i32;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 4i32;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 1i32;
pub const D3DKMT_OUTPUTDUPL_METADATATYPE_DIRTY_RECTS: D3DKMT_OUTPUTDUPL_METADATATYPE = 0i32;
pub const D3DKMT_OUTPUTDUPL_METADATATYPE_MOVE_RECTS: D3DKMT_OUTPUTDUPL_METADATATYPE = 1i32;
pub const D3DKMT_PM_FLIPMANAGER: D3DKMT_PRESENT_MODEL = 9i32;
pub const D3DKMT_PM_REDIRECTED_BLT: D3DKMT_PRESENT_MODEL = 3i32;
pub const D3DKMT_PM_REDIRECTED_COMPOSITION: D3DKMT_PRESENT_MODEL = 7i32;
pub const D3DKMT_PM_REDIRECTED_FLIP: D3DKMT_PRESENT_MODEL = 2i32;
pub const D3DKMT_PM_REDIRECTED_GDI: D3DKMT_PRESENT_MODEL = 1i32;
pub const D3DKMT_PM_REDIRECTED_GDI_SYSMEM: D3DKMT_PRESENT_MODEL = 6i32;
pub const D3DKMT_PM_REDIRECTED_VISTABLT: D3DKMT_PRESENT_MODEL = 4i32;
pub const D3DKMT_PM_SCREENCAPTUREFENCE: D3DKMT_PRESENT_MODEL = 5i32;
pub const D3DKMT_PM_SURFACECOMPLETE: D3DKMT_PRESENT_MODEL = 8i32;
pub const D3DKMT_PM_UNINITIALIZED: D3DKMT_PRESENT_MODEL = 0i32;
pub const D3DKMT_PNP_KEY_HARDWARE: D3DKMT_PNP_KEY_TYPE = 1i32;
pub const D3DKMT_PNP_KEY_SOFTWARE: D3DKMT_PNP_KEY_TYPE = 2i32;
pub const D3DKMT_PROCESS_VERIFIER_OPTION_VIDMM_FLAGS: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = 1000i32;
pub const D3DKMT_PROCESS_VERIFIER_OPTION_VIDMM_RESTRICT_BUDGET: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = 1001i32;
pub const D3DKMT_PROTECTED_SESSION_STATUS_INVALID: D3DKMT_PROTECTED_SESSION_STATUS = 1i32;
pub const D3DKMT_PROTECTED_SESSION_STATUS_OK: D3DKMT_PROTECTED_SESSION_STATUS = 0i32;
pub const D3DKMT_PreemptionAttempt: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 0i32;
pub const D3DKMT_PreemptionAttemptMissAlreadyPreempting: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 13i32;
pub const D3DKMT_PreemptionAttemptMissAlreadyRunning: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 15i32;
pub const D3DKMT_PreemptionAttemptMissFenceCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 7i32;
pub const D3DKMT_PreemptionAttemptMissGlobalBlock: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 14i32;
pub const D3DKMT_PreemptionAttemptMissLessPriority: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 10i32;
pub const D3DKMT_PreemptionAttemptMissNextFence: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 4i32;
pub const D3DKMT_PreemptionAttemptMissNoCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 2i32;
pub const D3DKMT_PreemptionAttemptMissNotEnabled: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 3i32;
pub const D3DKMT_PreemptionAttemptMissNotMakingProgress: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 9i32;
pub const D3DKMT_PreemptionAttemptMissPagingCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 5i32;
pub const D3DKMT_PreemptionAttemptMissRemainingPreemptionQuantum: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 12i32;
pub const D3DKMT_PreemptionAttemptMissRemainingQuantum: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 11i32;
pub const D3DKMT_PreemptionAttemptMissRenderPendingFlip: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 8i32;
pub const D3DKMT_PreemptionAttemptMissSplittedCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 6i32;
pub const D3DKMT_PreemptionAttemptStatisticsMax: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 16i32;
pub const D3DKMT_PreemptionAttemptSuccess: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 1i32;
pub const D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT_MAX: u32 = 16u32;
pub const D3DKMT_QUERYSTATISTICS_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 0i32;
pub const D3DKMT_QUERYSTATISTICS_ADAPTER2: D3DKMT_QUERYSTATISTICS_TYPE = 11i32;
pub const D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS_MAX: u32 = 5u32;
pub const D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_MAX: u32 = 4u32;
pub const D3DKMT_QUERYSTATISTICS_NODE: D3DKMT_QUERYSTATISTICS_TYPE = 5i32;
pub const D3DKMT_QUERYSTATISTICS_NODE2: D3DKMT_QUERYSTATISTICS_TYPE = 18i32;
pub const D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 10i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS: D3DKMT_QUERYSTATISTICS_TYPE = 1i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 2i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER2: D3DKMT_QUERYSTATISTICS_TYPE = 13i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_BUCKET_COUNT: u32 = 9u32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_NODE: D3DKMT_QUERYSTATISTICS_TYPE = 6i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_NODE2: D3DKMT_QUERYSTATISTICS_TYPE = 19i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT: D3DKMT_QUERYSTATISTICS_TYPE = 4i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT2: D3DKMT_QUERYSTATISTICS_TYPE = 14i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP: D3DKMT_QUERYSTATISTICS_TYPE = 9i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP2: D3DKMT_QUERYSTATISTICS_TYPE = 15i32;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE: D3DKMT_QUERYSTATISTICS_TYPE = 8i32;
pub const D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_MAX: u32 = 8u32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT: D3DKMT_QUERYSTATISTICS_TYPE = 3i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT2: D3DKMT_QUERYSTATISTICS_TYPE = 12i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_GROUP_USAGE: D3DKMT_QUERYSTATISTICS_TYPE = 17i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_PREFERENCE_MAX: u32 = 5u32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_APERTURE: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 0i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_MEMORY: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 1i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_SYSMEM: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 2i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_USAGE: D3DKMT_QUERYSTATISTICS_TYPE = 16i32;
pub const D3DKMT_QUERYSTATISTICS_VIDPNSOURCE: D3DKMT_QUERYSTATISTICS_TYPE = 7i32;
pub const D3DKMT_QueuePacketTypeMax: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 8i32;
pub const D3DKMT_RenderCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 0i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_ABOVE_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 3i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_BELOW_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 1i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH: D3DKMT_SCHEDULINGPRIORITYCLASS = 4i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_IDLE: D3DKMT_SCHEDULINGPRIORITYCLASS = 0i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 2i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_REALTIME: D3DKMT_SCHEDULINGPRIORITYCLASS = 5i32;
pub const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY_ABSOLUTE: u32 = 1073741824u32;
pub const D3DKMT_SET_QUEUEDLIMIT_PRESENT: D3DKMT_QUEUEDLIMIT_TYPE = 1i32;
pub const D3DKMT_STANDARDALLOCATIONTYPE_EXISTINGHEAP: D3DKMT_STANDARDALLOCATIONTYPE = 1i32;
pub const D3DKMT_STANDARDALLOCATIONTYPE_INTERNALBACKINGSTORE: D3DKMT_STANDARDALLOCATIONTYPE = 2i32;
pub const D3DKMT_STANDARDALLOCATIONTYPE_MAX: D3DKMT_STANDARDALLOCATIONTYPE = 3i32;
pub const D3DKMT_SUBKEY_DX9: windows_sys::core::PCWSTR = windows_sys::core::w!("DX9");
pub const D3DKMT_SUBKEY_OPENGL: windows_sys::core::PCWSTR = windows_sys::core::w!("OpenGL");
pub const D3DKMT_SignalCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 5i32;
pub const D3DKMT_SoftwareCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 7i32;
pub const D3DKMT_SystemCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 2i32;
pub const D3DKMT_SystemPagingBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 2i32;
pub const D3DKMT_SystemPreemptionBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 3i32;
pub const D3DKMT_TDRDBGCTRLTYPE_DISABLEBREAK: D3DKMT_TDRDBGCTRLTYPE = 1i32;
pub const D3DKMT_TDRDBGCTRLTYPE_ENABLEBREAK: D3DKMT_TDRDBGCTRLTYPE = 2i32;
pub const D3DKMT_TDRDBGCTRLTYPE_ENGINETDR: D3DKMT_TDRDBGCTRLTYPE = 8i32;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCEDODTDR: D3DKMT_TDRDBGCTRLTYPE = 6i32;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCEDODVSYNCTDR: D3DKMT_TDRDBGCTRLTYPE = 7i32;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCETDR: D3DKMT_TDRDBGCTRLTYPE = 0i32;
pub const D3DKMT_TDRDBGCTRLTYPE_GPUTDR: D3DKMT_TDRDBGCTRLTYPE = 5i32;
pub const D3DKMT_TDRDBGCTRLTYPE_UNCONDITIONAL: D3DKMT_TDRDBGCTRLTYPE = 3i32;
pub const D3DKMT_TDRDBGCTRLTYPE_VSYNCTDR: D3DKMT_TDRDBGCTRLTYPE = 4i32;
pub const D3DKMT_VAD_ESCAPE_GETNUMVADS: D3DKMT_VAD_ESCAPE_COMMAND = 0i32;
pub const D3DKMT_VAD_ESCAPE_GETVAD: D3DKMT_VAD_ESCAPE_COMMAND = 1i32;
pub const D3DKMT_VAD_ESCAPE_GETVADRANGE: D3DKMT_VAD_ESCAPE_COMMAND = 2i32;
pub const D3DKMT_VAD_ESCAPE_GET_GPUMMU_CAPS: D3DKMT_VAD_ESCAPE_COMMAND = 4i32;
pub const D3DKMT_VAD_ESCAPE_GET_PTE: D3DKMT_VAD_ESCAPE_COMMAND = 3i32;
pub const D3DKMT_VAD_ESCAPE_GET_SEGMENT_CAPS: D3DKMT_VAD_ESCAPE_COMMAND = 5i32;
pub const D3DKMT_VERIFIER_OPTION_QUERY: D3DKMT_VERIFIER_OPTION_MODE = 0i32;
pub const D3DKMT_VERIFIER_OPTION_SET: D3DKMT_VERIFIER_OPTION_MODE = 1i32;
pub const D3DKMT_VIDMMESCAPETYPE_APERTURE_CORRUPTION_CHECK: D3DKMT_VIDMMESCAPETYPE = 3i32;
pub const D3DKMT_VIDMMESCAPETYPE_DEFRAG: D3DKMT_VIDMMESCAPETYPE = 15i32;
pub const D3DKMT_VIDMMESCAPETYPE_DELAYEXECUTION: D3DKMT_VIDMMESCAPETYPE = 16i32;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT: D3DKMT_VIDMMESCAPETYPE = 5i32;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT_BY_CRITERIA: D3DKMT_VIDMMESCAPETYPE = 13i32;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT_BY_NT_HANDLE: D3DKMT_VIDMMESCAPETYPE = 6i32;
pub const D3DKMT_VIDMMESCAPETYPE_GET_BUDGET: D3DKMT_VIDMMESCAPETYPE = 11i32;
pub const D3DKMT_VIDMMESCAPETYPE_GET_VAD_INFO: D3DKMT_VIDMMESCAPETYPE = 7i32;
pub const D3DKMT_VIDMMESCAPETYPE_RESUME_PROCESS: D3DKMT_VIDMMESCAPETYPE = 10i32;
pub const D3DKMT_VIDMMESCAPETYPE_RUN_COHERENCY_TEST: D3DKMT_VIDMMESCAPETYPE = 1i32;
pub const D3DKMT_VIDMMESCAPETYPE_RUN_UNMAP_TO_DUMMY_PAGE_TEST: D3DKMT_VIDMMESCAPETYPE = 2i32;
pub const D3DKMT_VIDMMESCAPETYPE_SETFAULT: D3DKMT_VIDMMESCAPETYPE = 0i32;
pub const D3DKMT_VIDMMESCAPETYPE_SET_BUDGET: D3DKMT_VIDMMESCAPETYPE = 8i32;
pub const D3DKMT_VIDMMESCAPETYPE_SET_EVICTION_CONFIG: D3DKMT_VIDMMESCAPETYPE = 18i32;
pub const D3DKMT_VIDMMESCAPETYPE_SET_TRIM_INTERVALS: D3DKMT_VIDMMESCAPETYPE = 12i32;
pub const D3DKMT_VIDMMESCAPETYPE_SUSPEND_CPU_ACCESS_TEST: D3DKMT_VIDMMESCAPETYPE = 4i32;
pub const D3DKMT_VIDMMESCAPETYPE_SUSPEND_PROCESS: D3DKMT_VIDMMESCAPETYPE = 9i32;
pub const D3DKMT_VIDMMESCAPETYPE_VALIDATE_INTEGRITY: D3DKMT_VIDMMESCAPETYPE = 17i32;
pub const D3DKMT_VIDMMESCAPETYPE_WAKE: D3DKMT_VIDMMESCAPETYPE = 14i32;
pub const D3DKMT_VIDPNSOURCEOWNER_EMULATED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 4i32;
pub const D3DKMT_VIDPNSOURCEOWNER_EXCLUSIVE: D3DKMT_VIDPNSOURCEOWNER_TYPE = 2i32;
pub const D3DKMT_VIDPNSOURCEOWNER_EXCLUSIVEGDI: D3DKMT_VIDPNSOURCEOWNER_TYPE = 3i32;
pub const D3DKMT_VIDPNSOURCEOWNER_SHARED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 1i32;
pub const D3DKMT_VIDPNSOURCEOWNER_UNOWNED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 0i32;
pub const D3DKMT_VIDSCHESCAPETYPE_CONFIGURE_TDR_LIMIT: D3DKMT_VIDSCHESCAPETYPE = 5i32;
pub const D3DKMT_VIDSCHESCAPETYPE_ENABLECONTEXTDELAY: D3DKMT_VIDSCHESCAPETYPE = 4i32;
pub const D3DKMT_VIDSCHESCAPETYPE_PFN_CONTROL: D3DKMT_VIDSCHESCAPETYPE = 7i32;
pub const D3DKMT_VIDSCHESCAPETYPE_PREEMPTIONCONTROL: D3DKMT_VIDSCHESCAPETYPE = 0i32;
pub const D3DKMT_VIDSCHESCAPETYPE_SUSPENDRESUME: D3DKMT_VIDSCHESCAPETYPE = 3i32;
pub const D3DKMT_VIDSCHESCAPETYPE_SUSPENDSCHEDULER: D3DKMT_VIDSCHESCAPETYPE = 1i32;
pub const D3DKMT_VIDSCHESCAPETYPE_TDRCONTROL: D3DKMT_VIDSCHESCAPETYPE = 2i32;
pub const D3DKMT_VIDSCHESCAPETYPE_VGPU_RESET: D3DKMT_VIDSCHESCAPETYPE = 6i32;
pub const D3DKMT_VIDSCHESCAPETYPE_VIRTUAL_REFRESH_RATE: D3DKMT_VIDSCHESCAPETYPE = 8i32;
pub const D3DKMT_WaitCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 4i32;
pub const D3DNTCLEAR_COMPUTERECTS: i32 = 8i32;
pub const D3DNTDP2OP_ADDDIRTYBOX: D3DNTHAL_DP2OPERATION = 67i32;
pub const D3DNTDP2OP_ADDDIRTYRECT: D3DNTHAL_DP2OPERATION = 66i32;
pub const D3DNTDP2OP_BLT: D3DNTHAL_DP2OPERATION = 81i32;
pub const D3DNTDP2OP_BUFFERBLT: D3DNTHAL_DP2OPERATION = 64i32;
pub const D3DNTDP2OP_CLEAR: D3DNTHAL_DP2OPERATION = 42i32;
pub const D3DNTDP2OP_CLIPPEDTRIANGLEFAN: D3DNTHAL_DP2OPERATION = 58i32;
pub const D3DNTDP2OP_COLORFILL: D3DNTHAL_DP2OPERATION = 82i32;
pub const D3DNTDP2OP_COMPOSERECTS: D3DNTHAL_DP2OPERATION = 98i32;
pub const D3DNTDP2OP_CREATELIGHT: D3DNTHAL_DP2OPERATION = 35i32;
pub const D3DNTDP2OP_CREATEPIXELSHADER: D3DNTHAL_DP2OPERATION = 54i32;
pub const D3DNTDP2OP_CREATEQUERY: D3DNTHAL_DP2OPERATION = 84i32;
pub const D3DNTDP2OP_CREATEVERTEXSHADER: D3DNTHAL_DP2OPERATION = 45i32;
pub const D3DNTDP2OP_CREATEVERTEXSHADERDECL: D3DNTHAL_DP2OPERATION = 71i32;
pub const D3DNTDP2OP_CREATEVERTEXSHADERFUNC: D3DNTHAL_DP2OPERATION = 74i32;
pub const D3DNTDP2OP_DELETEPIXELSHADER: D3DNTHAL_DP2OPERATION = 55i32;
pub const D3DNTDP2OP_DELETEQUERY: D3DNTHAL_DP2OPERATION = 90i32;
pub const D3DNTDP2OP_DELETEVERTEXSHADER: D3DNTHAL_DP2OPERATION = 46i32;
pub const D3DNTDP2OP_DELETEVERTEXSHADERDECL: D3DNTHAL_DP2OPERATION = 72i32;
pub const D3DNTDP2OP_DELETEVERTEXSHADERFUNC: D3DNTHAL_DP2OPERATION = 75i32;
pub const D3DNTDP2OP_DRAWINDEXEDPRIMITIVE: D3DNTHAL_DP2OPERATION = 53i32;
pub const D3DNTDP2OP_DRAWINDEXEDPRIMITIVE2: D3DNTHAL_DP2OPERATION = 60i32;
pub const D3DNTDP2OP_DRAWPRIMITIVE: D3DNTHAL_DP2OPERATION = 52i32;
pub const D3DNTDP2OP_DRAWPRIMITIVE2: D3DNTHAL_DP2OPERATION = 59i32;
pub const D3DNTDP2OP_DRAWRECTPATCH: D3DNTHAL_DP2OPERATION = 61i32;
pub const D3DNTDP2OP_DRAWTRIPATCH: D3DNTHAL_DP2OPERATION = 62i32;
pub const D3DNTDP2OP_GENERATEMIPSUBLEVELS: D3DNTHAL_DP2OPERATION = 89i32;
pub const D3DNTDP2OP_INDEXEDLINELIST: D3DNTHAL_DP2OPERATION = 2i32;
pub const D3DNTDP2OP_INDEXEDLINELIST2: D3DNTHAL_DP2OPERATION = 27i32;
pub const D3DNTDP2OP_INDEXEDLINESTRIP: D3DNTHAL_DP2OPERATION = 17i32;
pub const D3DNTDP2OP_INDEXEDTRIANGLEFAN: D3DNTHAL_DP2OPERATION = 22i32;
pub const D3DNTDP2OP_INDEXEDTRIANGLELIST: D3DNTHAL_DP2OPERATION = 3i32;
pub const D3DNTDP2OP_INDEXEDTRIANGLELIST2: D3DNTHAL_DP2OPERATION = 26i32;
pub const D3DNTDP2OP_INDEXEDTRIANGLESTRIP: D3DNTHAL_DP2OPERATION = 20i32;
pub const D3DNTDP2OP_ISSUEQUERY: D3DNTHAL_DP2OPERATION = 91i32;
pub const D3DNTDP2OP_LINELIST: D3DNTHAL_DP2OPERATION = 15i32;
pub const D3DNTDP2OP_LINELIST_IMM: D3DNTHAL_DP2OPERATION = 24i32;
pub const D3DNTDP2OP_LINESTRIP: D3DNTHAL_DP2OPERATION = 16i32;
pub const D3DNTDP2OP_MULTIPLYTRANSFORM: D3DNTHAL_DP2OPERATION = 65i32;
pub const D3DNTDP2OP_POINTS: D3DNTHAL_DP2OPERATION = 1i32;
pub const D3DNTDP2OP_RENDERSTATE: D3DNTHAL_DP2OPERATION = 8i32;
pub const D3DNTDP2OP_RESPONSECONTINUE: D3DNTHAL_DP2OPERATION = 87i32;
pub const D3DNTDP2OP_RESPONSEQUERY: D3DNTHAL_DP2OPERATION = 88i32;
pub const D3DNTDP2OP_SETCLIPPLANE: D3DNTHAL_DP2OPERATION = 44i32;
pub const D3DNTDP2OP_SETCONVOLUTIONKERNELMONO: D3DNTHAL_DP2OPERATION = 97i32;
pub const D3DNTDP2OP_SETDEPTHSTENCIL: D3DNTHAL_DP2OPERATION = 86i32;
pub const D3DNTDP2OP_SETINDICES: D3DNTHAL_DP2OPERATION = 51i32;
pub const D3DNTDP2OP_SETLIGHT: D3DNTHAL_DP2OPERATION = 34i32;
pub const D3DNTDP2OP_SETMATERIAL: D3DNTHAL_DP2OPERATION = 33i32;
pub const D3DNTDP2OP_SETPALETTE: D3DNTHAL_DP2OPERATION = 30i32;
pub const D3DNTDP2OP_SETPIXELSHADER: D3DNTHAL_DP2OPERATION = 56i32;
pub const D3DNTDP2OP_SETPIXELSHADERCONST: D3DNTHAL_DP2OPERATION = 57i32;
pub const D3DNTDP2OP_SETPIXELSHADERCONSTB: D3DNTHAL_DP2OPERATION = 94i32;
pub const D3DNTDP2OP_SETPIXELSHADERCONSTI: D3DNTHAL_DP2OPERATION = 93i32;
pub const D3DNTDP2OP_SETPRIORITY: D3DNTHAL_DP2OPERATION = 40i32;
pub const D3DNTDP2OP_SETRENDERTARGET: D3DNTHAL_DP2OPERATION = 41i32;
pub const D3DNTDP2OP_SETRENDERTARGET2: D3DNTHAL_DP2OPERATION = 85i32;
pub const D3DNTDP2OP_SETSCISSORRECT: D3DNTHAL_DP2OPERATION = 79i32;
pub const D3DNTDP2OP_SETSTREAMSOURCE: D3DNTHAL_DP2OPERATION = 49i32;
pub const D3DNTDP2OP_SETSTREAMSOURCE2: D3DNTHAL_DP2OPERATION = 80i32;
pub const D3DNTDP2OP_SETSTREAMSOURCEFREQ: D3DNTHAL_DP2OPERATION = 95i32;
pub const D3DNTDP2OP_SETSTREAMSOURCEUM: D3DNTHAL_DP2OPERATION = 50i32;
pub const D3DNTDP2OP_SETTEXLOD: D3DNTHAL_DP2OPERATION = 43i32;
pub const D3DNTDP2OP_SETTRANSFORM: D3DNTHAL_DP2OPERATION = 36i32;
pub const D3DNTDP2OP_SETVERTEXSHADER: D3DNTHAL_DP2OPERATION = 47i32;
pub const D3DNTDP2OP_SETVERTEXSHADERCONST: D3DNTHAL_DP2OPERATION = 48i32;
pub const D3DNTDP2OP_SETVERTEXSHADERCONSTB: D3DNTHAL_DP2OPERATION = 83i32;
pub const D3DNTDP2OP_SETVERTEXSHADERCONSTI: D3DNTHAL_DP2OPERATION = 77i32;
pub const D3DNTDP2OP_SETVERTEXSHADERDECL: D3DNTHAL_DP2OPERATION = 73i32;
pub const D3DNTDP2OP_SETVERTEXSHADERFUNC: D3DNTHAL_DP2OPERATION = 76i32;
pub const D3DNTDP2OP_STATESET: D3DNTHAL_DP2OPERATION = 39i32;
pub const D3DNTDP2OP_SURFACEBLT: D3DNTHAL_DP2OPERATION = 96i32;
pub const D3DNTDP2OP_TEXBLT: D3DNTHAL_DP2OPERATION = 38i32;
pub const D3DNTDP2OP_TEXTURESTAGESTATE: D3DNTHAL_DP2OPERATION = 25i32;
pub const D3DNTDP2OP_TRIANGLEFAN: D3DNTHAL_DP2OPERATION = 21i32;
pub const D3DNTDP2OP_TRIANGLEFAN_IMM: D3DNTHAL_DP2OPERATION = 23i32;
pub const D3DNTDP2OP_TRIANGLELIST: D3DNTHAL_DP2OPERATION = 18i32;
pub const D3DNTDP2OP_TRIANGLESTRIP: D3DNTHAL_DP2OPERATION = 19i32;
pub const D3DNTDP2OP_UPDATEPALETTE: D3DNTHAL_DP2OPERATION = 31i32;
pub const D3DNTDP2OP_VIEWPORTINFO: D3DNTHAL_DP2OPERATION = 28i32;
pub const D3DNTDP2OP_VOLUMEBLT: D3DNTHAL_DP2OPERATION = 63i32;
pub const D3DNTDP2OP_WINFO: D3DNTHAL_DP2OPERATION = 29i32;
pub const D3DNTDP2OP_ZRANGE: D3DNTHAL_DP2OPERATION = 32i32;
pub const D3DNTHAL2_CB32_SETRENDERTARGET: i32 = 1i32;
pub const D3DNTHAL3_CB32_CLEAR2: i32 = 1i32;
pub const D3DNTHAL3_CB32_DRAWPRIMITIVES2: i32 = 8i32;
pub const D3DNTHAL3_CB32_RESERVED: i32 = 2i32;
pub const D3DNTHAL3_CB32_VALIDATETEXTURESTAGESTATE: i32 = 4i32;
pub const D3DNTHALDP2_EXECUTEBUFFER: i32 = 2i32;
pub const D3DNTHALDP2_REQCOMMANDBUFSIZE: i32 = 32i32;
pub const D3DNTHALDP2_REQVERTEXBUFSIZE: i32 = 16i32;
pub const D3DNTHALDP2_SWAPCOMMANDBUFFER: i32 = 8i32;
pub const D3DNTHALDP2_SWAPVERTEXBUFFER: i32 = 4i32;
pub const D3DNTHALDP2_USERMEMVERTICES: i32 = 1i32;
pub const D3DNTHALDP2_VIDMEMCOMMANDBUF: i32 = 128i32;
pub const D3DNTHALDP2_VIDMEMVERTEXBUF: i32 = 64i32;
pub const D3DNTHAL_COL_WEIGHTS: u32 = 2u32;
pub const D3DNTHAL_CONTEXT_BAD: i64 = 512i64;
pub const D3DNTHAL_NUMCLIPVERTICES: u32 = 20u32;
pub const D3DNTHAL_OUTOFCONTEXTS: i64 = 513i64;
pub const D3DNTHAL_ROW_WEIGHTS: u32 = 1u32;
pub const D3DNTHAL_SCENE_CAPTURE_END: i32 = 1i32;
pub const D3DNTHAL_SCENE_CAPTURE_START: i32 = 0i32;
pub const D3DNTHAL_STATESETCREATE: u32 = 5u32;
pub const D3DNTHAL_TSS_MAXSTAGES: u32 = 8u32;
pub const D3DNTHAL_TSS_RENDERSTATEBASE: u32 = 256u32;
pub const D3DNTHAL_TSS_STATESPERSTAGE: u32 = 64u32;
pub const D3DPMISCCAPS_FOGINFVF: i32 = 8192i32;
pub const D3DPMISCCAPS_LINEPATTERNREP: i32 = 4i32;
pub const D3DPRASTERCAPS_PAT: i32 = 8i32;
pub const D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE: i32 = 8388608i32;
pub const D3DPS_COLOROUT_MAX_V2_0: u32 = 4u32;
pub const D3DPS_COLOROUT_MAX_V2_1: u32 = 4u32;
pub const D3DPS_COLOROUT_MAX_V3_0: u32 = 4u32;
pub const D3DPS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
pub const D3DPS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
pub const D3DPS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
pub const D3DPS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
pub const D3DPS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
pub const D3DPS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
pub const D3DPS_CONSTREG_MAX_DX8: u32 = 8u32;
pub const D3DPS_CONSTREG_MAX_SW_DX9: u32 = 8192u32;
pub const D3DPS_CONSTREG_MAX_V1_1: u32 = 8u32;
pub const D3DPS_CONSTREG_MAX_V1_2: u32 = 8u32;
pub const D3DPS_CONSTREG_MAX_V1_3: u32 = 8u32;
pub const D3DPS_CONSTREG_MAX_V1_4: u32 = 8u32;
pub const D3DPS_CONSTREG_MAX_V2_0: u32 = 32u32;
pub const D3DPS_CONSTREG_MAX_V2_1: u32 = 32u32;
pub const D3DPS_CONSTREG_MAX_V3_0: u32 = 224u32;
pub const D3DPS_INPUTREG_MAX_DX8: u32 = 8u32;
pub const D3DPS_INPUTREG_MAX_SW_DX9: u32 = 14u32;
pub const D3DPS_INPUTREG_MAX_V1_1: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V1_2: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V1_3: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V1_4: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V2_0: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V2_1: u32 = 2u32;
pub const D3DPS_INPUTREG_MAX_V3_0: u32 = 10u32;
pub const D3DPS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
pub const D3DPS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
pub const D3DPS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
pub const D3DPS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
pub const D3DPS_MAXLOOPSTEP_V2_1: u32 = 128u32;
pub const D3DPS_MAXLOOPSTEP_V3_0: u32 = 128u32;
pub const D3DPS_PREDICATE_MAX_V2_1: u32 = 1u32;
pub const D3DPS_PREDICATE_MAX_V3_0: u32 = 1u32;
pub const D3DPS_TEMPREG_MAX_DX8: u32 = 8u32;
pub const D3DPS_TEMPREG_MAX_V1_1: u32 = 2u32;
pub const D3DPS_TEMPREG_MAX_V1_2: u32 = 2u32;
pub const D3DPS_TEMPREG_MAX_V1_3: u32 = 2u32;
pub const D3DPS_TEMPREG_MAX_V1_4: u32 = 6u32;
pub const D3DPS_TEMPREG_MAX_V2_0: u32 = 12u32;
pub const D3DPS_TEMPREG_MAX_V2_1: u32 = 32u32;
pub const D3DPS_TEMPREG_MAX_V3_0: u32 = 32u32;
pub const D3DPS_TEXTUREREG_MAX_DX8: u32 = 8u32;
pub const D3DPS_TEXTUREREG_MAX_V1_1: u32 = 4u32;
pub const D3DPS_TEXTUREREG_MAX_V1_2: u32 = 4u32;
pub const D3DPS_TEXTUREREG_MAX_V1_3: u32 = 4u32;
pub const D3DPS_TEXTUREREG_MAX_V1_4: u32 = 6u32;
pub const D3DPS_TEXTUREREG_MAX_V2_0: u32 = 8u32;
pub const D3DPS_TEXTUREREG_MAX_V2_1: u32 = 8u32;
pub const D3DPS_TEXTUREREG_MAX_V3_0: u32 = 0u32;
pub const D3DRENDERSTATE_EVICTMANAGEDTEXTURES: u32 = 61u32;
pub const D3DRENDERSTATE_SCENECAPTURE: u32 = 62u32;
pub const D3DRS_DELETERTPATCH: u32 = 169u32;
pub const D3DRS_MAXPIXELSHADERINST: u32 = 197u32;
pub const D3DRS_MAXVERTEXSHADERINST: u32 = 196u32;
pub const D3DTEXF_FLATCUBIC: u32 = 4u32;
pub const D3DTEXF_GAUSSIANCUBIC: u32 = 5u32;
pub const D3DTRANSFORMSTATE_WORLD1_DX7: u32 = 4u32;
pub const D3DTRANSFORMSTATE_WORLD2_DX7: u32 = 5u32;
pub const D3DTRANSFORMSTATE_WORLD3_DX7: u32 = 6u32;
pub const D3DTRANSFORMSTATE_WORLD_DX7: u32 = 1u32;
pub const D3DTSS_TEXTUREMAP: u32 = 0u32;
pub const D3DVSDE_BLENDINDICES: u32 = 2u32;
pub const D3DVSDE_BLENDWEIGHT: u32 = 1u32;
pub const D3DVSDE_DIFFUSE: u32 = 5u32;
pub const D3DVSDE_NORMAL: u32 = 3u32;
pub const D3DVSDE_NORMAL2: u32 = 16u32;
pub const D3DVSDE_POSITION: u32 = 0u32;
pub const D3DVSDE_POSITION2: u32 = 15u32;
pub const D3DVSDE_PSIZE: u32 = 4u32;
pub const D3DVSDE_SPECULAR: u32 = 6u32;
pub const D3DVSDE_TEXCOORD0: u32 = 7u32;
pub const D3DVSDE_TEXCOORD1: u32 = 8u32;
pub const D3DVSDE_TEXCOORD2: u32 = 9u32;
pub const D3DVSDE_TEXCOORD3: u32 = 10u32;
pub const D3DVSDE_TEXCOORD4: u32 = 11u32;
pub const D3DVSDE_TEXCOORD5: u32 = 12u32;
pub const D3DVSDE_TEXCOORD6: u32 = 13u32;
pub const D3DVSDE_TEXCOORD7: u32 = 14u32;
pub const D3DVSDT_D3DCOLOR: u32 = 4u32;
pub const D3DVSDT_FLOAT1: u32 = 0u32;
pub const D3DVSDT_FLOAT2: u32 = 1u32;
pub const D3DVSDT_FLOAT3: u32 = 2u32;
pub const D3DVSDT_FLOAT4: u32 = 3u32;
pub const D3DVSDT_SHORT2: u32 = 6u32;
pub const D3DVSDT_SHORT4: u32 = 7u32;
pub const D3DVSDT_UBYTE4: u32 = 5u32;
pub const D3DVSD_CONSTADDRESSSHIFT: u32 = 0u32;
pub const D3DVSD_CONSTCOUNTSHIFT: u32 = 25u32;
pub const D3DVSD_CONSTRSSHIFT: u32 = 16u32;
pub const D3DVSD_DATALOADTYPESHIFT: u32 = 28u32;
pub const D3DVSD_DATATYPESHIFT: u32 = 16u32;
pub const D3DVSD_EXTCOUNTSHIFT: u32 = 24u32;
pub const D3DVSD_EXTINFOSHIFT: u32 = 0u32;
pub const D3DVSD_SKIPCOUNTSHIFT: u32 = 16u32;
pub const D3DVSD_STREAMNUMBERSHIFT: u32 = 0u32;
pub const D3DVSD_STREAMTESSSHIFT: u32 = 28u32;
pub const D3DVSD_TOKENTYPESHIFT: u32 = 29u32;
pub const D3DVSD_TOKEN_CONSTMEM: D3DVSD_TOKENTYPE = 4i32;
pub const D3DVSD_TOKEN_END: D3DVSD_TOKENTYPE = 7i32;
pub const D3DVSD_TOKEN_EXT: D3DVSD_TOKENTYPE = 5i32;
pub const D3DVSD_TOKEN_NOP: D3DVSD_TOKENTYPE = 0i32;
pub const D3DVSD_TOKEN_STREAM: D3DVSD_TOKENTYPE = 1i32;
pub const D3DVSD_TOKEN_STREAMDATA: D3DVSD_TOKENTYPE = 2i32;
pub const D3DVSD_TOKEN_TESSELLATOR: D3DVSD_TOKENTYPE = 3i32;
pub const D3DVSD_VERTEXREGINSHIFT: u32 = 20u32;
pub const D3DVSD_VERTEXREGSHIFT: u32 = 0u32;
pub const D3DVS_ADDRREG_MAX_V1_1: u32 = 1u32;
pub const D3DVS_ADDRREG_MAX_V2_0: u32 = 1u32;
pub const D3DVS_ADDRREG_MAX_V2_1: u32 = 1u32;
pub const D3DVS_ADDRREG_MAX_V3_0: u32 = 1u32;
pub const D3DVS_ATTROUTREG_MAX_V1_1: u32 = 2u32;
pub const D3DVS_ATTROUTREG_MAX_V2_0: u32 = 2u32;
pub const D3DVS_ATTROUTREG_MAX_V2_1: u32 = 2u32;
pub const D3DVS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
pub const D3DVS_CONSTBOOLREG_MAX_V2_0: u32 = 16u32;
pub const D3DVS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
pub const D3DVS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
pub const D3DVS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
pub const D3DVS_CONSTINTREG_MAX_V2_0: u32 = 16u32;
pub const D3DVS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
pub const D3DVS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
pub const D3DVS_CONSTREG_MAX_V1_1: u32 = 96u32;
pub const D3DVS_CONSTREG_MAX_V2_0: u32 = 8192u32;
pub const D3DVS_CONSTREG_MAX_V2_1: u32 = 8192u32;
pub const D3DVS_CONSTREG_MAX_V3_0: u32 = 8192u32;
pub const D3DVS_INPUTREG_MAX_V1_1: u32 = 16u32;
pub const D3DVS_INPUTREG_MAX_V2_0: u32 = 16u32;
pub const D3DVS_INPUTREG_MAX_V2_1: u32 = 16u32;
pub const D3DVS_INPUTREG_MAX_V3_0: u32 = 16u32;
pub const D3DVS_LABEL_MAX_V3_0: u32 = 2048u32;
pub const D3DVS_MAXINSTRUCTIONCOUNT_V1_1: u32 = 128u32;
pub const D3DVS_MAXLOOPINITVALUE_V2_0: u32 = 255u32;
pub const D3DVS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
pub const D3DVS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
pub const D3DVS_MAXLOOPITERATIONCOUNT_V2_0: u32 = 255u32;
pub const D3DVS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
pub const D3DVS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
pub const D3DVS_MAXLOOPSTEP_V2_0: u32 = 128u32;
pub const D3DVS_MAXLOOPSTEP_V2_1: u32 = 128u32;
pub const D3DVS_MAXLOOPSTEP_V3_0: u32 = 128u32;
pub const D3DVS_OUTPUTREG_MAX_SW_DX9: u32 = 16u32;
pub const D3DVS_OUTPUTREG_MAX_V3_0: u32 = 12u32;
pub const D3DVS_PREDICATE_MAX_V2_1: u32 = 1u32;
pub const D3DVS_PREDICATE_MAX_V3_0: u32 = 1u32;
pub const D3DVS_TCRDOUTREG_MAX_V1_1: u32 = 8u32;
pub const D3DVS_TCRDOUTREG_MAX_V2_0: u32 = 8u32;
pub const D3DVS_TCRDOUTREG_MAX_V2_1: u32 = 8u32;
pub const D3DVS_TEMPREG_MAX_V1_1: u32 = 12u32;
pub const D3DVS_TEMPREG_MAX_V2_0: u32 = 12u32;
pub const D3DVS_TEMPREG_MAX_V2_1: u32 = 32u32;
pub const D3DVS_TEMPREG_MAX_V3_0: u32 = 32u32;
pub const D3DVTXPCAPS_NO_VSDT_UBYTE4: i32 = 128i32;
pub const D3D_UMD_INTERFACE_VERSION: u32 = 65536u32;
pub const D3D_UMD_INTERFACE_VERSION_VISTA: u32 = 12u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM1_3: u32 = 16386u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0: u32 = 20482u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1: u32 = 20480u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1_3: u32 = 20481u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M2_2: u32 = 20482u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1: u32 = 24579u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_1: u32 = 24576u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_2: u32 = 24577u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_3: u32 = 24578u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_4: u32 = 24579u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2: u32 = 28673u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_1: u32 = 28672u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_2: u32 = 28673u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3: u32 = 32769u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_1: u32 = 32768u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_2: u32 = 32769u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4: u32 = 36865u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_1: u32 = 36864u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_2: u32 = 36865u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5: u32 = 40962u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_1: u32 = 40960u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_2: u32 = 40961u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_3: u32 = 40962u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6: u32 = 45059u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_1: u32 = 45056u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_2: u32 = 45057u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_3: u32 = 45058u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_4: u32 = 45059u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7: u32 = 49153u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7_1: u32 = 49152u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7_2: u32 = 49153u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_8: u32 = 53248u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_8_1: u32 = 53248u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_9: u32 = 57344u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_9_1: u32 = 57344u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_0: u32 = 61440u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_0_1: u32 = 61440u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_1: u32 = 65536u32;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_1_1: u32 = 65536u32;
pub const D3D_UMD_INTERFACE_VERSION_WIN7: u32 = 8195u32;
pub const D3D_UMD_INTERFACE_VERSION_WIN8: u32 = 12292u32;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_CP: u32 = 12290u32;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_M3: u32 = 12289u32;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_RC: u32 = 12291u32;
pub const DDBLT_EXTENDED_PRESENTATION_STRETCHFACTOR: i32 = 16i32;
pub const DIDDT1_AspectRatio_15x9: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 3i32;
pub const DIDDT1_AspectRatio_16x10: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 5i32;
pub const DIDDT1_AspectRatio_16x9: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 4i32;
pub const DIDDT1_AspectRatio_1x1: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 0i32;
pub const DIDDT1_AspectRatio_4x3: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 2i32;
pub const DIDDT1_AspectRatio_5x4: DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = 1i32;
pub const DIDDT1_Dependent: DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 2i32;
pub const DIDDT1_Interlaced: DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = 1i32;
pub const DIDDT1_Monoscopic: DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 0i32;
pub const DIDDT1_Progressive: DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = 0i32;
pub const DIDDT1_Stereo: DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = 1i32;
pub const DIDDT1_Sync_Negative: DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = 1i32;
pub const DIDDT1_Sync_Positive: DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = 0i32;
pub const DISPLAYID_DETAILED_TIMING_TYPE_I_SIZE: u32 = 20u32;
pub const DP2BLT_LINEAR: i32 = 2i32;
pub const DP2BLT_POINT: i32 = 1i32;
pub const DX9_DDI_VERSION: u32 = 4u32;
pub const DXGKDDI_INTERFACE_VERSION: u32 = 65540u32;
pub const DXGKDDI_INTERFACE_VERSION_VISTA: u32 = 4178u32;
pub const DXGKDDI_INTERFACE_VERSION_VISTA_SP1: u32 = 4179u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3: u32 = 16386u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3_PATH_INDEPENDENT_ROTATION: u32 = 16387u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_0: u32 = 20515u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1: u32 = 24579u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1_5: u32 = 24592u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1_6: u32 = 24593u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_2: u32 = 28682u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_3: u32 = 32769u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_4: u32 = 36870u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_5: u32 = 40971u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_6: u32 = 45060u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_7: u32 = 49156u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_8: u32 = 53249u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_9: u32 = 57347u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM3_0: u32 = 61443u32;
pub const DXGKDDI_INTERFACE_VERSION_WDDM3_1: u32 = 65540u32;
pub const DXGKDDI_INTERFACE_VERSION_WIN7: u32 = 8197u32;
pub const DXGKDDI_INTERFACE_VERSION_WIN8: u32 = 12302u32;
pub const DXGKMDT_COPP_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 1i32;
pub const DXGKMDT_I2C_DEVICE_TRANSMITS_DATA_LENGTH: u32 = 1u32;
pub const DXGKMDT_I2C_NO_FLAGS: u32 = 0u32;
pub const DXGKMDT_INDIRECT_DISPLAY_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 3i32;
pub const DXGKMDT_OPM_128_BIT_RANDOM_NUMBER_SIZE: u32 = 16u32;
pub const DXGKMDT_OPM_ACP_LEVEL_ONE: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 1i32;
pub const DXGKMDT_OPM_ACP_LEVEL_THREE: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 3i32;
pub const DXGKMDT_OPM_ACP_LEVEL_TWO: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 2i32;
pub const DXGKMDT_OPM_ACP_OFF: DXGKMDT_OPM_ACP_PROTECTION_LEVEL = 0i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 1i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_14_BY_9_TOP: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 2i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 3i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_16_BY_9_TOP: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 4i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_BOX_GT_16_BY_9_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 5i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_16_BY_9_ANAMORPHIC: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 7i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 0i32;
pub const DXGKMDT_OPM_ASPECT_RATIO_EN300294_FULL_FORMAT_4_BY_3_PROTECTED_CENTER: DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = 6i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 262144i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 327680i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_INSIDE_OF_CHIPSET: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 65536i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = -2147483648i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 131072i32;
pub const DXGKMDT_OPM_BUS_IMPLEMENTATION_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 196608i32;
pub const DXGKMDT_OPM_BUS_TYPE_AGP: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 4i32;
pub const DXGKMDT_OPM_BUS_TYPE_OTHER: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 0i32;
pub const DXGKMDT_OPM_BUS_TYPE_PCI: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 1i32;
pub const DXGKMDT_OPM_BUS_TYPE_PCIEXPRESS: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 3i32;
pub const DXGKMDT_OPM_BUS_TYPE_PCIX: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = 2i32;
pub const DXGKMDT_OPM_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 0i32;
pub const DXGKMDT_OPM_CGMSA_COPY_FREELY: DXGKMDT_OPM_CGMSA = 1i32;
pub const DXGKMDT_OPM_CGMSA_COPY_NEVER: DXGKMDT_OPM_CGMSA = 4i32;
pub const DXGKMDT_OPM_CGMSA_COPY_NO_MORE: DXGKMDT_OPM_CGMSA = 2i32;
pub const DXGKMDT_OPM_CGMSA_COPY_ONE_GENERATION: DXGKMDT_OPM_CGMSA = 3i32;
pub const DXGKMDT_OPM_CGMSA_OFF: DXGKMDT_OPM_CGMSA = 0i32;
pub const DXGKMDT_OPM_CONFIGURE_SETTING_DATA_SIZE: u32 = 4056u32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_COMPONENT_VIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 3i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_COMPOSITE_VIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 2i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DISPLAYPORT_EMBEDDED: DXGKMDT_OPM_CONNECTOR_TYPE = 11i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DISPLAYPORT_EXTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = 10i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_DVI: DXGKMDT_OPM_CONNECTOR_TYPE = 4i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_D_JPN: DXGKMDT_OPM_CONNECTOR_TYPE = 8i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_HD15: DXGKMDT_OPM_CONNECTOR_TYPE = 0i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_HDMI: DXGKMDT_OPM_CONNECTOR_TYPE = 5i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_LVDS: DXGKMDT_OPM_CONNECTOR_TYPE = 6i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_MIRACAST: DXGKMDT_OPM_CONNECTOR_TYPE = 15i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_OTHER: DXGKMDT_OPM_CONNECTOR_TYPE = -1i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_RESERVED: DXGKMDT_OPM_CONNECTOR_TYPE = 14i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_SDI: DXGKMDT_OPM_CONNECTOR_TYPE = 9i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_SVIDEO: DXGKMDT_OPM_CONNECTOR_TYPE = 1i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A: DXGKMDT_OPM_CONNECTOR_TYPE = 16i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B: DXGKMDT_OPM_CONNECTOR_TYPE = 17i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_UDI_EMBEDDED: DXGKMDT_OPM_CONNECTOR_TYPE = 13i32;
pub const DXGKMDT_OPM_CONNECTOR_TYPE_UDI_EXTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = 12i32;
pub const DXGKMDT_OPM_COPP_COMPATIBLE_BUS_TYPE_INTEGRATED: DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = -2147483648i32;
pub const DXGKMDT_OPM_COPP_COMPATIBLE_CONNECTOR_TYPE_INTERNAL: DXGKMDT_OPM_CONNECTOR_TYPE = -2147483648i32;
pub const DXGKMDT_OPM_DPCP_OFF: DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = 0i32;
pub const DXGKMDT_OPM_DPCP_ON: DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = 1i32;
pub const DXGKMDT_OPM_DVI_CHARACTERISTIC_1_0: DXGKDT_OPM_DVI_CHARACTERISTICS = 1i32;
pub const DXGKMDT_OPM_DVI_CHARACTERISTIC_1_1_OR_ABOVE: DXGKDT_OPM_DVI_CHARACTERISTICS = 2i32;
pub const DXGKMDT_OPM_ENCRYPTED_PARAMETERS_SIZE: u32 = 256u32;
pub const DXGKMDT_OPM_GET_ACP_AND_CGMSA_SIGNALING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6629a591_3b79_4cf3_924a_11e8e7811671);
pub const DXGKMDT_OPM_GET_ACTUAL_OUTPUT_FORMAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7bf1ba3_ad13_4f8e_af98_0dcb3ca204cc);
pub const DXGKMDT_OPM_GET_ACTUAL_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1957210a_7766_452a_b99a_d27aed54f03a);
pub const DXGKMDT_OPM_GET_ADAPTER_BUS_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6f4d673_6174_4184_8e35_f6db5200bcba);
pub const DXGKMDT_OPM_GET_CODEC_INFO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f374491_8f5f_4445_9dba_95588f6b58b4);
pub const DXGKMDT_OPM_GET_CONNECTED_HDCP_DEVICE_INFORMATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db59d74_a992_492e_a0bd_c23fda564e00);
pub const DXGKMDT_OPM_GET_CONNECTOR_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
pub const DXGKMDT_OPM_GET_CURRENT_HDCP_SRM_VERSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x99c5ceff_5f1d_4879_81c1_c52443c9482b);
pub const DXGKMDT_OPM_GET_DVI_CHARACTERISTICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa470b3bb_5dd7_4172_839c_3d3776e0ebf5);
pub const DXGKMDT_OPM_GET_INFORMATION_PARAMETERS_SIZE: u32 = 4056u32;
pub const DXGKMDT_OPM_GET_OUTPUT_HARDWARE_PROTECTION_SUPPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3b129589_2af8_4ef0_96a2_704a845a218e);
pub const DXGKMDT_OPM_GET_OUTPUT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x72cb6df3_244f_40ce_b09e_20506af6302f);
pub const DXGKMDT_OPM_GET_SUPPORTED_PROTECTION_TYPES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38f2a801_9a6c_48bb_9107_b6696e6f1797);
pub const DXGKMDT_OPM_GET_VIRTUAL_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2075857_3eda_4d5d_88db_748f8c1a0549);
pub const DXGKMDT_OPM_HDCP_FLAG_NONE: DXGKMDT_OPM_HDCP_FLAG = 0i32;
pub const DXGKMDT_OPM_HDCP_FLAG_REPEATER: DXGKMDT_OPM_HDCP_FLAG = 1i32;
pub const DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR_SIZE: u32 = 5u32;
pub const DXGKMDT_OPM_HDCP_OFF: DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = 0i32;
pub const DXGKMDT_OPM_HDCP_ON: DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = 1i32;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_INTERLEAVED_EVEN_FIRST: DXGKMDT_OPM_INTERLEAVE_FORMAT = 3i32;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_INTERLEAVED_ODD_FIRST: DXGKMDT_OPM_INTERLEAVE_FORMAT = 4i32;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_OTHER: DXGKMDT_OPM_INTERLEAVE_FORMAT = 0i32;
pub const DXGKMDT_OPM_INTERLEAVE_FORMAT_PROGRESSIVE: DXGKMDT_OPM_INTERLEAVE_FORMAT = 2i32;
pub const DXGKMDT_OPM_OMAC_SIZE: u32 = 16u32;
pub const DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION_NOT_SUPPORTED: DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = 0i32;
pub const DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION_SUPPORTED: DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = 1i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 16384i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 2048i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 4096i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_ARIBTRB15_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 8192i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 128i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 32i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEA_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 64i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_1125I: DXGKMDT_OPM_PROTECTION_STANDARD = 1024i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_525P: DXGKMDT_OPM_PROTECTION_STANDARD = 256i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_CEA805A_TYPEB_750P: DXGKMDT_OPM_PROTECTION_STANDARD = 512i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_EIA608B_525: DXGKMDT_OPM_PROTECTION_STANDARD = 8i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_EN300294_625I: DXGKMDT_OPM_PROTECTION_STANDARD = 16i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC61880_2_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 2i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC61880_525I: DXGKMDT_OPM_PROTECTION_STANDARD = 1i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_IEC62375_625P: DXGKMDT_OPM_PROTECTION_STANDARD = 4i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_NONE: DXGKMDT_OPM_PROTECTION_STANDARD = 0i32;
pub const DXGKMDT_OPM_PROTECTION_STANDARD_OTHER: DXGKMDT_OPM_PROTECTION_STANDARD = -2147483648i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_ACP: DXGKMDT_OPM_PROTECTION_TYPE = 2i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_CGMSA: DXGKMDT_OPM_PROTECTION_TYPE = 4i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_COPP_COMPATIBLE_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 1i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_DPCP: DXGKMDT_OPM_PROTECTION_TYPE = 16i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 8i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_MASK: DXGKMDT_OPM_PROTECTION_TYPE = -2147483585i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_NONE: DXGKMDT_OPM_PROTECTION_TYPE = 0i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_OTHER: DXGKMDT_OPM_PROTECTION_TYPE = -2147483648i32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_SIZE: u32 = 4u32;
pub const DXGKMDT_OPM_PROTECTION_TYPE_TYPE_ENFORCEMENT_HDCP: DXGKMDT_OPM_PROTECTION_TYPE = 32i32;
pub const DXGKMDT_OPM_REDISTRIBUTION_CONTROL_REQUIRED: DXGKMDT_OPM_CGMSA = 8i32;
pub const DXGKMDT_OPM_REQUESTED_INFORMATION_SIZE: u32 = 4076u32;
pub const DXGKMDT_OPM_SET_ACP_AND_CGMSA_SIGNALING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a631a5_d684_4c60_8e4d_d3bb0f0be3ee);
pub const DXGKMDT_OPM_SET_HDCP_SRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8b5ef5d1_c30d_44ff_84a5_ea71dce78f13);
pub const DXGKMDT_OPM_SET_PROTECTION_LEVEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bb9327c_4eb5_4727_9f00_b42b0919c0da);
pub const DXGKMDT_OPM_SET_PROTECTION_LEVEL_ACCORDING_TO_CSS_DVD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39ce333e_4cc0_44ae_bfcc_da50b5f82e72);
pub const DXGKMDT_OPM_STATUS_LINK_LOST: DXGKMDT_OPM_STATUS = 1i32;
pub const DXGKMDT_OPM_STATUS_NORMAL: DXGKMDT_OPM_STATUS = 0i32;
pub const DXGKMDT_OPM_STATUS_RENEGOTIATION_REQUIRED: DXGKMDT_OPM_STATUS = 2i32;
pub const DXGKMDT_OPM_STATUS_REVOKED_HDCP_DEVICE_ATTACHED: DXGKMDT_OPM_STATUS = 8i32;
pub const DXGKMDT_OPM_STATUS_TAMPERING_DETECTED: DXGKMDT_OPM_STATUS = 4i32;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_OFF: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 0i32;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_NO_TYPE_RESTRICTION: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 1i32;
pub const DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_ON_WITH_TYPE1_RESTRICTION: DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = 2i32;
pub const DXGKMDT_OPM_VOS_COPP_SEMANTICS: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 0i32;
pub const DXGKMDT_OPM_VOS_OPM_INDIRECT_DISPLAY: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 2i32;
pub const DXGKMDT_OPM_VOS_OPM_SEMANTICS: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = 1i32;
pub const DXGKMDT_UAB_CERTIFICATE: DXGKMDT_CERTIFICATE_TYPE = 2i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_FRAME0: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 1i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_FRAME1: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 2i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_NONE: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 0i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_CHECKERBOARD: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 7i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_COLUMN_INTERLEAVED: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 6i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_MONO: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 0i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_MONO_OFFSET: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 4i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_ROW_INTERLEAVED: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 5i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_SEPARATE: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 3i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY_BILINEAR: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = 1i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY_HIGH: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = 2i32;
pub const DXGKMT_POWER_SHARED_TYPE_AUDIO: DXGKMT_POWER_SHARED_TYPE = 0i32;
pub const DXGKVGPU_ESCAPE_TYPE_GET_VGPU_TYPE: DXGKVGPU_ESCAPE_TYPE = 4i32;
pub const DXGKVGPU_ESCAPE_TYPE_INITIALIZE: DXGKVGPU_ESCAPE_TYPE = 2i32;
pub const DXGKVGPU_ESCAPE_TYPE_PAUSE: DXGKVGPU_ESCAPE_TYPE = 6i32;
pub const DXGKVGPU_ESCAPE_TYPE_POWERTRANSITIONCOMPLETE: DXGKVGPU_ESCAPE_TYPE = 5i32;
pub const DXGKVGPU_ESCAPE_TYPE_READ_PCI_CONFIG: DXGKVGPU_ESCAPE_TYPE = 0i32;
pub const DXGKVGPU_ESCAPE_TYPE_RELEASE: DXGKVGPU_ESCAPE_TYPE = 3i32;
pub const DXGKVGPU_ESCAPE_TYPE_RESUME: DXGKVGPU_ESCAPE_TYPE = 7i32;
pub const DXGKVGPU_ESCAPE_TYPE_WRITE_PCI_CONFIG: DXGKVGPU_ESCAPE_TYPE = 1i32;
pub const DXGK_BRIGHTNESS_MAXIMUM_NIT_RANGE_COUNT: u32 = 16u32;
pub const DXGK_DDT_DISPLAYID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 2u8;
pub const DXGK_DDT_EDID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 1u8;
pub const DXGK_DDT_INVALID: DXGK_DISPLAY_DESCRIPTOR_TYPE = 0u8;
pub const DXGK_DIAG_PROCESS_NAME_LENGTH: u32 = 16u32;
pub const DXGK_DT_INVALID: DXGK_DISPLAY_TECHNOLOGY = 0u8;
pub const DXGK_DT_LCD: DXGK_DISPLAY_TECHNOLOGY = 2u8;
pub const DXGK_DT_MAX: DXGK_DISPLAY_TECHNOLOGY = 5u8;
pub const DXGK_DT_OLED: DXGK_DISPLAY_TECHNOLOGY = 3u8;
pub const DXGK_DT_OTHER: DXGK_DISPLAY_TECHNOLOGY = 1u8;
pub const DXGK_DT_PROJECTOR: DXGK_DISPLAY_TECHNOLOGY = 4u8;
pub const DXGK_DU_ACCESSORY: DXGK_DISPLAY_USAGE = 5u8;
pub const DXGK_DU_AR: DXGK_DISPLAY_USAGE = 2u8;
pub const DXGK_DU_GENERIC: DXGK_DISPLAY_USAGE = 1u8;
pub const DXGK_DU_INVALID: DXGK_DISPLAY_USAGE = 0u8;
pub const DXGK_DU_MAX: DXGK_DISPLAY_USAGE = 6u8;
pub const DXGK_DU_MEDICAL_IMAGING: DXGK_DISPLAY_USAGE = 4u8;
pub const DXGK_DU_VR: DXGK_DISPLAY_USAGE = 3u8;
pub const DXGK_ENGINE_TYPE_3D: DXGK_ENGINE_TYPE = 1i32;
pub const DXGK_ENGINE_TYPE_COPY: DXGK_ENGINE_TYPE = 6i32;
pub const DXGK_ENGINE_TYPE_CRYPTO: DXGK_ENGINE_TYPE = 8i32;
pub const DXGK_ENGINE_TYPE_MAX: DXGK_ENGINE_TYPE = 9i32;
pub const DXGK_ENGINE_TYPE_OTHER: DXGK_ENGINE_TYPE = 0i32;
pub const DXGK_ENGINE_TYPE_OVERLAY: DXGK_ENGINE_TYPE = 7i32;
pub const DXGK_ENGINE_TYPE_SCENE_ASSEMBLY: DXGK_ENGINE_TYPE = 5i32;
pub const DXGK_ENGINE_TYPE_VIDEO_DECODE: DXGK_ENGINE_TYPE = 2i32;
pub const DXGK_ENGINE_TYPE_VIDEO_ENCODE: DXGK_ENGINE_TYPE = 3i32;
pub const DXGK_ENGINE_TYPE_VIDEO_PROCESSING: DXGK_ENGINE_TYPE = 4i32;
pub const DXGK_GENERAL_ERROR_INVALID_INSTRUCTION: DXGK_GENERAL_ERROR_CODE = 1i32;
pub const DXGK_GENERAL_ERROR_PAGE_FAULT: DXGK_GENERAL_ERROR_CODE = 0i32;
pub const DXGK_GRAPHICSPOWER_VERSION: u32 = 4098u32;
pub const DXGK_GRAPHICSPOWER_VERSION_1_0: u32 = 4096u32;
pub const DXGK_GRAPHICSPOWER_VERSION_1_1: u32 = 4097u32;
pub const DXGK_GRAPHICSPOWER_VERSION_1_2: u32 = 4098u32;
pub const DXGK_MAX_GPUVERSION_NAME_LENGTH: u32 = 32u32;
pub const DXGK_MAX_METADATA_NAME_LENGTH: u32 = 32u32;
pub const DXGK_MAX_PAGE_TABLE_LEVEL_COUNT: u32 = 6u32;
pub const DXGK_MIN_PAGE_TABLE_LEVEL_COUNT: u32 = 2u32;
pub const DXGK_MIRACAST_CHUNK_TYPE_COLOR_CONVERT_COMPLETE: DXGK_MIRACAST_CHUNK_TYPE = 1i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_COMPLETE: DXGK_MIRACAST_CHUNK_TYPE = 2i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_DRIVER_DEFINED_1: DXGK_MIRACAST_CHUNK_TYPE = -2147483648i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_DRIVER_DEFINED_2: DXGK_MIRACAST_CHUNK_TYPE = -2147483647i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_FRAME_DROPPED: DXGK_MIRACAST_CHUNK_TYPE = 4i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_FRAME_START: DXGK_MIRACAST_CHUNK_TYPE = 3i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_UNKNOWN: DXGK_MIRACAST_CHUNK_TYPE = 0i32;
pub const DXGK_PAGE_FAULT_ADAPTER_RESET_REQUIRED: DXGK_PAGE_FAULT_FLAGS = 4i32;
pub const DXGK_PAGE_FAULT_ENGINE_RESET_REQUIRED: DXGK_PAGE_FAULT_FLAGS = 8i32;
pub const DXGK_PAGE_FAULT_FATAL_HARDWARE_ERROR: DXGK_PAGE_FAULT_FLAGS = 16i32;
pub const DXGK_PAGE_FAULT_FENCE_INVALID: DXGK_PAGE_FAULT_FLAGS = 2i32;
pub const DXGK_PAGE_FAULT_HW_CONTEXT_VALID: DXGK_PAGE_FAULT_FLAGS = 64i32;
pub const DXGK_PAGE_FAULT_IOMMU: DXGK_PAGE_FAULT_FLAGS = 32i32;
pub const DXGK_PAGE_FAULT_PROCESS_HANDLE_VALID: DXGK_PAGE_FAULT_FLAGS = 128i32;
pub const DXGK_PAGE_FAULT_WRITE: DXGK_PAGE_FAULT_FLAGS = 1i32;
pub const DXGK_PTE_PAGE_TABLE_PAGE_4KB: DXGK_PTE_PAGE_SIZE = 0i32;
pub const DXGK_PTE_PAGE_TABLE_PAGE_64KB: DXGK_PTE_PAGE_SIZE = 1i32;
pub const DXGK_RENDER_PIPELINE_STAGE_GEOMETRY_SHADER: DXGK_RENDER_PIPELINE_STAGE = 3i32;
pub const DXGK_RENDER_PIPELINE_STAGE_INPUT_ASSEMBLER: DXGK_RENDER_PIPELINE_STAGE = 1i32;
pub const DXGK_RENDER_PIPELINE_STAGE_OUTPUT_MERGER: DXGK_RENDER_PIPELINE_STAGE = 7i32;
pub const DXGK_RENDER_PIPELINE_STAGE_PIXEL_SHADER: DXGK_RENDER_PIPELINE_STAGE = 6i32;
pub const DXGK_RENDER_PIPELINE_STAGE_RASTERIZER: DXGK_RENDER_PIPELINE_STAGE = 5i32;
pub const DXGK_RENDER_PIPELINE_STAGE_STREAM_OUTPUT: DXGK_RENDER_PIPELINE_STAGE = 4i32;
pub const DXGK_RENDER_PIPELINE_STAGE_UNKNOWN: DXGK_RENDER_PIPELINE_STAGE = 0i32;
pub const DXGK_RENDER_PIPELINE_STAGE_VERTEX_SHADER: DXGK_RENDER_PIPELINE_STAGE = 2i32;
pub const DxgkBacklightOptimizationDesktop: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 1i32;
pub const DxgkBacklightOptimizationDimmed: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 3i32;
pub const DxgkBacklightOptimizationDisable: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 0i32;
pub const DxgkBacklightOptimizationDynamic: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 2i32;
pub const DxgkBacklightOptimizationEDR: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = 4i32;
pub const FLIPEX_TIMEOUT_USER: u32 = 2000u32;
pub const GUID_DEVINTERFACE_GRAPHICSPOWER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea5c6870_e93c_4588_bef1_fec42fc9429a);
pub const HpdAwarenessAlwaysConnected: DXGK_CHILD_DEVICE_HPD_AWARENESS = 1i32;
pub const HpdAwarenessInterruptible: DXGK_CHILD_DEVICE_HPD_AWARENESS = 4i32;
pub const HpdAwarenessNone: DXGK_CHILD_DEVICE_HPD_AWARENESS = 2i32;
pub const HpdAwarenessPolled: DXGK_CHILD_DEVICE_HPD_AWARENESS = 3i32;
pub const HpdAwarenessUninitialized: DXGK_CHILD_DEVICE_HPD_AWARENESS = 0i32;
pub const IOCTL_GPUP_DRIVER_ESCAPE: u32 = 2253920u32;
pub const IOCTL_INTERNAL_GRAPHICSPOWER_REGISTER: u32 = 2304007u32;
pub const KMTQAITYPE_ADAPTERADDRESS: KMTQUERYADAPTERINFOTYPE = 6i32;
pub const KMTQAITYPE_ADAPTERADDRESS_RENDER: KMTQUERYADAPTERINFOTYPE = 53i32;
pub const KMTQAITYPE_ADAPTERGUID: KMTQUERYADAPTERINFOTYPE = 4i32;
pub const KMTQAITYPE_ADAPTERGUID_RENDER: KMTQUERYADAPTERINFOTYPE = 52i32;
pub const KMTQAITYPE_ADAPTERPERFDATA: KMTQUERYADAPTERINFOTYPE = 62i32;
pub const KMTQAITYPE_ADAPTERPERFDATA_CAPS: KMTQUERYADAPTERINFOTYPE = 63i32;
pub const KMTQAITYPE_ADAPTERREGISTRYINFO: KMTQUERYADAPTERINFOTYPE = 8i32;
pub const KMTQAITYPE_ADAPTERREGISTRYINFO_RENDER: KMTQUERYADAPTERINFOTYPE = 54i32;
pub const KMTQAITYPE_ADAPTERTYPE: KMTQUERYADAPTERINFOTYPE = 15i32;
pub const KMTQAITYPE_ADAPTERTYPE_RENDER: KMTQUERYADAPTERINFOTYPE = 57i32;
pub const KMTQAITYPE_BLOCKLIST_KERNEL: KMTQUERYADAPTERINFOTYPE = 50i32;
pub const KMTQAITYPE_BLOCKLIST_RUNTIME: KMTQUERYADAPTERINFOTYPE = 51i32;
pub const KMTQAITYPE_CHECKDRIVERUPDATESTATUS: KMTQUERYADAPTERINFOTYPE = 11i32;
pub const KMTQAITYPE_CHECKDRIVERUPDATESTATUS_RENDER: KMTQUERYADAPTERINFOTYPE = 55i32;
pub const KMTQAITYPE_CPDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 26i32;
pub const KMTQAITYPE_CROSSADAPTERRESOURCE_SUPPORT: KMTQUERYADAPTERINFOTYPE = 76i32;
pub const KMTQAITYPE_CURRENTDISPLAYMODE: KMTQUERYADAPTERINFOTYPE = 9i32;
pub const KMTQAITYPE_DIRECTFLIP_SUPPORT: KMTQUERYADAPTERINFOTYPE = 19i32;
pub const KMTQAITYPE_DISPLAY_CAPS: KMTQUERYADAPTERINFOTYPE = 74i32;
pub const KMTQAITYPE_DISPLAY_UMDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 71i32;
pub const KMTQAITYPE_DLIST_DRIVER_NAME: KMTQUERYADAPTERINFOTYPE = 21i32;
pub const KMTQAITYPE_DRIVERCAPS_EXT: KMTQUERYADAPTERINFOTYPE = 32i32;
pub const KMTQAITYPE_DRIVERVERSION: KMTQUERYADAPTERINFOTYPE = 13i32;
pub const KMTQAITYPE_DRIVERVERSION_RENDER: KMTQUERYADAPTERINFOTYPE = 56i32;
pub const KMTQAITYPE_DRIVER_DESCRIPTION: KMTQUERYADAPTERINFOTYPE = 65i32;
pub const KMTQAITYPE_DRIVER_DESCRIPTION_RENDER: KMTQUERYADAPTERINFOTYPE = 66i32;
pub const KMTQAITYPE_FLIPQUEUEINFO: KMTQUERYADAPTERINFOTYPE = 5i32;
pub const KMTQAITYPE_GETSEGMENTGROUPSIZE: KMTQUERYADAPTERINFOTYPE = 42i32;
pub const KMTQAITYPE_GETSEGMENTSIZE: KMTQUERYADAPTERINFOTYPE = 3i32;
pub const KMTQAITYPE_GET_DEVICE_VIDPN_OWNERSHIP_INFO: KMTQUERYADAPTERINFOTYPE = 47i32;
pub const KMTQAITYPE_HWDRM_SUPPORT: KMTQUERYADAPTERINFOTYPE = 44i32;
pub const KMTQAITYPE_HYBRID_DLIST_DLL_SUPPORT: KMTQUERYADAPTERINFOTYPE = 73i32;
pub const KMTQAITYPE_INDEPENDENTFLIP_SECONDARY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 39i32;
pub const KMTQAITYPE_INDEPENDENTFLIP_SUPPORT: KMTQUERYADAPTERINFOTYPE = 28i32;
pub const KMTQAITYPE_KMD_DRIVER_VERSION: KMTQUERYADAPTERINFOTYPE = 49i32;
pub const KMTQAITYPE_MIRACASTCOMPANIONDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 29i32;
pub const KMTQAITYPE_MODELIST: KMTQUERYADAPTERINFOTYPE = 10i32;
pub const KMTQAITYPE_MPO3DDI_SUPPORT: KMTQUERYADAPTERINFOTYPE = 43i32;
pub const KMTQAITYPE_MPOKERNELCAPS_SUPPORT: KMTQUERYADAPTERINFOTYPE = 45i32;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_HUD_SUPPORT: KMTQUERYADAPTERINFOTYPE = 23i32;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_SECONDARY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 38i32;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_STRETCH_SUPPORT: KMTQUERYADAPTERINFOTYPE = 46i32;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 20i32;
pub const KMTQAITYPE_NODEMETADATA: KMTQUERYADAPTERINFOTYPE = 25i32;
pub const KMTQAITYPE_NODEPERFDATA: KMTQUERYADAPTERINFOTYPE = 61i32;
pub const KMTQAITYPE_OUTPUTDUPLCONTEXTSCOUNT: KMTQUERYADAPTERINFOTYPE = 16i32;
pub const KMTQAITYPE_PANELFITTER_SUPPORT: KMTQUERYADAPTERINFOTYPE = 40i32;
pub const KMTQAITYPE_PARAVIRTUALIZATION_RENDER: KMTQUERYADAPTERINFOTYPE = 68i32;
pub const KMTQAITYPE_PHYSICALADAPTERCOUNT: KMTQUERYADAPTERINFOTYPE = 30i32;
pub const KMTQAITYPE_PHYSICALADAPTERDEVICEIDS: KMTQUERYADAPTERINFOTYPE = 31i32;
pub const KMTQAITYPE_PHYSICALADAPTERPNPKEY: KMTQUERYADAPTERINFOTYPE = 41i32;
pub const KMTQAITYPE_QUERYREGISTRY: KMTQUERYADAPTERINFOTYPE = 48i32;
pub const KMTQAITYPE_QUERY_ADAPTER_UNIQUE_GUID: KMTQUERYADAPTERINFOTYPE = 60i32;
pub const KMTQAITYPE_QUERY_GPUMMU_CAPS: KMTQUERYADAPTERINFOTYPE = 34i32;
pub const KMTQAITYPE_QUERY_HW_PROTECTION_TEARDOWN_COUNT: KMTQUERYADAPTERINFOTYPE = 36i32;
pub const KMTQAITYPE_QUERY_ISBADDRIVERFORHWPROTECTIONDISABLED: KMTQUERYADAPTERINFOTYPE = 37i32;
pub const KMTQAITYPE_QUERY_MIRACAST_DRIVER_TYPE: KMTQUERYADAPTERINFOTYPE = 33i32;
pub const KMTQAITYPE_QUERY_MULTIPLANEOVERLAY_DECODE_SUPPORT: KMTQUERYADAPTERINFOTYPE = 35i32;
pub const KMTQAITYPE_SCANOUT_CAPS: KMTQUERYADAPTERINFOTYPE = 67i32;
pub const KMTQAITYPE_SERVICENAME: KMTQUERYADAPTERINFOTYPE = 69i32;
pub const KMTQAITYPE_SETWORKINGSETINFO: KMTQUERYADAPTERINFOTYPE = 7i32;
pub const KMTQAITYPE_TRACKEDWORKLOAD_SUPPORT: KMTQUERYADAPTERINFOTYPE = 72i32;
pub const KMTQAITYPE_UMDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 1i32;
pub const KMTQAITYPE_UMDRIVERPRIVATE: KMTQUERYADAPTERINFOTYPE = 0i32;
pub const KMTQAITYPE_UMD_DRIVER_VERSION: KMTQUERYADAPTERINFOTYPE = 18i32;
pub const KMTQAITYPE_UMOPENGLINFO: KMTQUERYADAPTERINFOTYPE = 2i32;
pub const KMTQAITYPE_VGPUINTERFACEID: KMTQUERYADAPTERINFOTYPE = 79i32;
pub const KMTQAITYPE_VIRTUALADDRESSINFO: KMTQUERYADAPTERINFOTYPE = 12i32;
pub const KMTQAITYPE_WDDM_1_2_CAPS: KMTQUERYADAPTERINFOTYPE = 17i32;
pub const KMTQAITYPE_WDDM_1_2_CAPS_RENDER: KMTQUERYADAPTERINFOTYPE = 58i32;
pub const KMTQAITYPE_WDDM_1_3_CAPS: KMTQUERYADAPTERINFOTYPE = 22i32;
pub const KMTQAITYPE_WDDM_1_3_CAPS_RENDER: KMTQUERYADAPTERINFOTYPE = 59i32;
pub const KMTQAITYPE_WDDM_2_0_CAPS: KMTQUERYADAPTERINFOTYPE = 24i32;
pub const KMTQAITYPE_WDDM_2_7_CAPS: KMTQUERYADAPTERINFOTYPE = 70i32;
pub const KMTQAITYPE_WDDM_2_9_CAPS: KMTQUERYADAPTERINFOTYPE = 75i32;
pub const KMTQAITYPE_WDDM_3_0_CAPS: KMTQUERYADAPTERINFOTYPE = 77i32;
pub const KMTQAITYPE_WDDM_3_1_CAPS: KMTQUERYADAPTERINFOTYPE = 80i32;
pub const KMTQAITYPE_WSAUMDIMAGENAME: KMTQUERYADAPTERINFOTYPE = 78i32;
pub const KMTQAITYPE_XBOX: KMTQUERYADAPTERINFOTYPE = 27i32;
pub const KMTQUITYPE_GPUVERSION: KMTQUERYADAPTERINFOTYPE = 64i32;
pub const KMTUMDVERSION_DX10: KMTUMDVERSION = 1i32;
pub const KMTUMDVERSION_DX11: KMTUMDVERSION = 2i32;
pub const KMTUMDVERSION_DX12: KMTUMDVERSION = 3i32;
pub const KMTUMDVERSION_DX12_WSA32: KMTUMDVERSION = 4i32;
pub const KMTUMDVERSION_DX12_WSA64: KMTUMDVERSION = 5i32;
pub const KMTUMDVERSION_DX9: KMTUMDVERSION = 0i32;
pub const KMT_DISPLAY_UMDVERSION_1: KMT_DISPLAY_UMD_VERSION = 0i32;
pub const KMT_DRIVERVERSION_WDDM_1_0: D3DKMT_DRIVERVERSION = 1000i32;
pub const KMT_DRIVERVERSION_WDDM_1_1: D3DKMT_DRIVERVERSION = 1105i32;
pub const KMT_DRIVERVERSION_WDDM_1_1_PRERELEASE: D3DKMT_DRIVERVERSION = 1102i32;
pub const KMT_DRIVERVERSION_WDDM_1_2: D3DKMT_DRIVERVERSION = 1200i32;
pub const KMT_DRIVERVERSION_WDDM_1_3: D3DKMT_DRIVERVERSION = 1300i32;
pub const KMT_DRIVERVERSION_WDDM_2_0: D3DKMT_DRIVERVERSION = 2000i32;
pub const KMT_DRIVERVERSION_WDDM_2_1: D3DKMT_DRIVERVERSION = 2100i32;
pub const KMT_DRIVERVERSION_WDDM_2_2: D3DKMT_DRIVERVERSION = 2200i32;
pub const KMT_DRIVERVERSION_WDDM_2_3: D3DKMT_DRIVERVERSION = 2300i32;
pub const KMT_DRIVERVERSION_WDDM_2_4: D3DKMT_DRIVERVERSION = 2400i32;
pub const KMT_DRIVERVERSION_WDDM_2_5: D3DKMT_DRIVERVERSION = 2500i32;
pub const KMT_DRIVERVERSION_WDDM_2_6: D3DKMT_DRIVERVERSION = 2600i32;
pub const KMT_DRIVERVERSION_WDDM_2_7: D3DKMT_DRIVERVERSION = 2700i32;
pub const KMT_DRIVERVERSION_WDDM_2_8: D3DKMT_DRIVERVERSION = 2800i32;
pub const KMT_DRIVERVERSION_WDDM_2_9: D3DKMT_DRIVERVERSION = 2900i32;
pub const KMT_DRIVERVERSION_WDDM_3_0: D3DKMT_DRIVERVERSION = 3000i32;
pub const KMT_DRIVERVERSION_WDDM_3_1: D3DKMT_DRIVERVERSION = 3100i32;
pub const MAX_ENUM_ADAPTERS: u32 = 16u32;
pub const MiracastStartPending: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 1i32;
pub const MiracastStarted: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 2i32;
pub const MiracastStopPending: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 3i32;
pub const MiracastStopped: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 0i32;
pub const NUM_KMTUMDVERSIONS: KMTUMDVERSION = 6i32;
pub const NUM_KMT_DISPLAY_UMDVERSIONS: KMT_DISPLAY_UMD_VERSION = 1i32;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_ACTIVE: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 1i32;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_INACTIVE: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 0i32;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_PENDING_DESTROY: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 2i32;
pub const OUTPUTDUPL_CREATE_MAX_KEYEDMUTXES: u32 = 3u32;
pub const RTPATCHFLAG_HASINFO: i32 = 2i32;
pub const RTPATCHFLAG_HASSEGS: i32 = 1i32;
pub const SHARED_ALLOCATION_WRITE: u32 = 1u32;
pub const _NT_D3DDEVCAPS_HWINDEXBUFFER: i32 = 67108864i32;
pub const _NT_D3DDEVCAPS_HWVERTEXBUFFER: i32 = 33554432i32;
pub const _NT_D3DDEVCAPS_SUBVOLUMELOCK: i32 = 134217728i32;
pub const _NT_D3DFVF_FOG: i32 = 8192i32;
pub const _NT_D3DGDI2_MAGIC: u32 = 4294967295u32;
pub const _NT_D3DGDI2_TYPE_DEFERRED_AGP_AWARE: u32 = 24u32;
pub const _NT_D3DGDI2_TYPE_DEFER_AGP_FREES: u32 = 32u32;
pub const _NT_D3DGDI2_TYPE_DXVERSION: u32 = 4u32;
pub const _NT_D3DGDI2_TYPE_FREE_DEFERRED_AGP: u32 = 25u32;
pub const _NT_D3DGDI2_TYPE_GETADAPTERGROUP: u32 = 19u32;
pub const _NT_D3DGDI2_TYPE_GETD3DCAPS8: u32 = 1u32;
pub const _NT_D3DGDI2_TYPE_GETD3DCAPS9: u32 = 16u32;
pub const _NT_D3DGDI2_TYPE_GETD3DQUERY: u32 = 34u32;
pub const _NT_D3DGDI2_TYPE_GETD3DQUERYCOUNT: u32 = 33u32;
pub const _NT_D3DGDI2_TYPE_GETDDIVERSION: u32 = 35u32;
pub const _NT_D3DGDI2_TYPE_GETEXTENDEDMODE: u32 = 18u32;
pub const _NT_D3DGDI2_TYPE_GETEXTENDEDMODECOUNT: u32 = 17u32;
pub const _NT_D3DGDI2_TYPE_GETFORMAT: u32 = 3u32;
pub const _NT_D3DGDI2_TYPE_GETFORMATCOUNT: u32 = 2u32;
pub const _NT_D3DGDI2_TYPE_GETMULTISAMPLEQUALITYLEVELS: u32 = 22u32;
pub const _NT_D3DPMISCCAPS_FOGINFVF: i32 = 8192i32;
pub const _NT_D3DPS_COLOROUT_MAX_V2_0: u32 = 4u32;
pub const _NT_D3DPS_COLOROUT_MAX_V2_1: u32 = 4u32;
pub const _NT_D3DPS_COLOROUT_MAX_V3_0: u32 = 4u32;
pub const _NT_D3DPS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
pub const _NT_D3DPS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
pub const _NT_D3DPS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
pub const _NT_D3DPS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
pub const _NT_D3DPS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
pub const _NT_D3DPS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
pub const _NT_D3DPS_CONSTREG_MAX_DX8: u32 = 8u32;
pub const _NT_D3DPS_CONSTREG_MAX_SW_DX9: u32 = 8192u32;
pub const _NT_D3DPS_CONSTREG_MAX_V1_1: u32 = 8u32;
pub const _NT_D3DPS_CONSTREG_MAX_V1_2: u32 = 8u32;
pub const _NT_D3DPS_CONSTREG_MAX_V1_3: u32 = 8u32;
pub const _NT_D3DPS_CONSTREG_MAX_V1_4: u32 = 8u32;
pub const _NT_D3DPS_CONSTREG_MAX_V2_0: u32 = 32u32;
pub const _NT_D3DPS_CONSTREG_MAX_V2_1: u32 = 32u32;
pub const _NT_D3DPS_CONSTREG_MAX_V3_0: u32 = 224u32;
pub const _NT_D3DPS_INPUTREG_MAX_DX8: u32 = 8u32;
pub const _NT_D3DPS_INPUTREG_MAX_V1_1: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V1_2: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V1_3: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V1_4: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V2_0: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V2_1: u32 = 2u32;
pub const _NT_D3DPS_INPUTREG_MAX_V3_0: u32 = 12u32;
pub const _NT_D3DPS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
pub const _NT_D3DPS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
pub const _NT_D3DPS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
pub const _NT_D3DPS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
pub const _NT_D3DPS_MAXLOOPSTEP_V2_1: u32 = 128u32;
pub const _NT_D3DPS_MAXLOOPSTEP_V3_0: u32 = 128u32;
pub const _NT_D3DPS_PREDICATE_MAX_V2_1: u32 = 1u32;
pub const _NT_D3DPS_PREDICATE_MAX_V3_0: u32 = 1u32;
pub const _NT_D3DPS_TEMPREG_MAX_DX8: u32 = 8u32;
pub const _NT_D3DPS_TEMPREG_MAX_V1_1: u32 = 2u32;
pub const _NT_D3DPS_TEMPREG_MAX_V1_2: u32 = 2u32;
pub const _NT_D3DPS_TEMPREG_MAX_V1_3: u32 = 2u32;
pub const _NT_D3DPS_TEMPREG_MAX_V1_4: u32 = 6u32;
pub const _NT_D3DPS_TEMPREG_MAX_V2_0: u32 = 12u32;
pub const _NT_D3DPS_TEMPREG_MAX_V2_1: u32 = 32u32;
pub const _NT_D3DPS_TEMPREG_MAX_V3_0: u32 = 32u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_DX8: u32 = 8u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_1: u32 = 4u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_2: u32 = 4u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_3: u32 = 4u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_4: u32 = 6u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V2_0: u32 = 8u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V2_1: u32 = 8u32;
pub const _NT_D3DPS_TEXTUREREG_MAX_V3_0: u32 = 0u32;
pub const _NT_D3DRS_DELETERTPATCH: u32 = 169u32;
pub const _NT_D3DVS_ADDRREG_MAX_V1_1: u32 = 1u32;
pub const _NT_D3DVS_ADDRREG_MAX_V2_0: u32 = 1u32;
pub const _NT_D3DVS_ADDRREG_MAX_V2_1: u32 = 1u32;
pub const _NT_D3DVS_ADDRREG_MAX_V3_0: u32 = 1u32;
pub const _NT_D3DVS_ATTROUTREG_MAX_V1_1: u32 = 2u32;
pub const _NT_D3DVS_ATTROUTREG_MAX_V2_0: u32 = 2u32;
pub const _NT_D3DVS_ATTROUTREG_MAX_V2_1: u32 = 2u32;
pub const _NT_D3DVS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V2_0: u32 = 16u32;
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
pub const _NT_D3DVS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
pub const _NT_D3DVS_CONSTINTREG_MAX_V2_0: u32 = 16u32;
pub const _NT_D3DVS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
pub const _NT_D3DVS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
pub const _NT_D3DVS_CONSTREG_MAX_V1_1: u32 = 96u32;
pub const _NT_D3DVS_CONSTREG_MAX_V2_0: u32 = 8192u32;
pub const _NT_D3DVS_CONSTREG_MAX_V2_1: u32 = 8192u32;
pub const _NT_D3DVS_CONSTREG_MAX_V3_0: u32 = 8192u32;
pub const _NT_D3DVS_INPUTREG_MAX_V1_1: u32 = 16u32;
pub const _NT_D3DVS_INPUTREG_MAX_V2_0: u32 = 16u32;
pub const _NT_D3DVS_INPUTREG_MAX_V2_1: u32 = 16u32;
pub const _NT_D3DVS_INPUTREG_MAX_V3_0: u32 = 16u32;
pub const _NT_D3DVS_LABEL_MAX_V3_0: u32 = 2048u32;
pub const _NT_D3DVS_MAXINSTRUCTIONCOUNT_V1_1: u32 = 128u32;
pub const _NT_D3DVS_MAXLOOPINITVALUE_V2_0: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_0: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
pub const _NT_D3DVS_MAXLOOPSTEP_V2_0: u32 = 128u32;
pub const _NT_D3DVS_MAXLOOPSTEP_V2_1: u32 = 128u32;
pub const _NT_D3DVS_MAXLOOPSTEP_V3_0: u32 = 128u32;
pub const _NT_D3DVS_OUTPUTREG_MAX_SW_DX9: u32 = 16u32;
pub const _NT_D3DVS_OUTPUTREG_MAX_V3_0: u32 = 12u32;
pub const _NT_D3DVS_PREDICATE_MAX_V2_1: u32 = 1u32;
pub const _NT_D3DVS_PREDICATE_MAX_V3_0: u32 = 1u32;
pub const _NT_D3DVS_TCRDOUTREG_MAX_V1_1: u32 = 8u32;
pub const _NT_D3DVS_TCRDOUTREG_MAX_V2_0: u32 = 8u32;
pub const _NT_D3DVS_TCRDOUTREG_MAX_V2_1: u32 = 8u32;
pub const _NT_D3DVS_TEMPREG_MAX_V1_1: u32 = 12u32;
pub const _NT_D3DVS_TEMPREG_MAX_V2_0: u32 = 12u32;
pub const _NT_D3DVS_TEMPREG_MAX_V2_1: u32 = 32u32;
pub const _NT_D3DVS_TEMPREG_MAX_V3_0: u32 = 32u32;
pub const _NT_RTPATCHFLAG_HASINFO: i32 = 2i32;
pub const _NT_RTPATCHFLAG_HASSEGS: i32 = 1i32;
pub type D3DDDIFORMAT = u32;
pub type D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = i32;
pub type D3DDDIMULTISAMPLE_TYPE = i32;
pub type D3DDDI_COLOR_SPACE_TYPE = i32;
pub type D3DDDI_DOORBELLSTATUS = i32;
pub type D3DDDI_DRIVERESCAPETYPE = i32;
pub type D3DDDI_FLIPINTERVAL_TYPE = i32;
pub type D3DDDI_GAMMARAMP_TYPE = i32;
pub type D3DDDI_HDR_METADATA_TYPE = i32;
pub type D3DDDI_OFFER_PRIORITY = i32;
pub type D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = i32;
pub type D3DDDI_PAGINGQUEUE_PRIORITY = i32;
pub type D3DDDI_POOL = i32;
pub type D3DDDI_QUERYREGISTRY_STATUS = i32;
pub type D3DDDI_QUERYREGISTRY_TYPE = i32;
pub type D3DDDI_RECLAIM_RESULT = i32;
pub type D3DDDI_ROTATION = i32;
pub type D3DDDI_SCANLINEORDERING = i32;
pub type D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = i32;
pub type D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = i32;
pub type D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = i32;
pub type D3DHAL_DP2OPERATION = i32;
pub type D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = i32;
pub type D3DKMDT_COLOR_BASIS = i32;
pub type D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY = i32;
pub type D3DKMDT_ENUMCOFUNCMODALITY_PIVOT_TYPE = i32;
pub type D3DKMDT_GDISURFACETYPE = i32;
pub type D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY = i32;
pub type D3DKMDT_GTFCOMPLIANCE = i32;
pub type D3DKMDT_MODE_PREFERENCE = i32;
pub type D3DKMDT_MODE_PRUNING_REASON = i32;
pub type D3DKMDT_MONITOR_CAPABILITIES_ORIGIN = i32;
pub type D3DKMDT_MONITOR_CONNECTIVITY_CHECKS = i32;
pub type D3DKMDT_MONITOR_DESCRIPTOR_TYPE = i32;
pub type D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT = i32;
pub type D3DKMDT_MONITOR_ORIENTATION = i32;
pub type D3DKMDT_MONITOR_ORIENTATION_AWARENESS = i32;
pub type D3DKMDT_MONITOR_TIMING_TYPE = i32;
pub type D3DKMDT_PIXEL_VALUE_ACCESS_MODE = i32;
pub type D3DKMDT_STANDARDALLOCATION_TYPE = i32;
pub type D3DKMDT_TEXT_RENDERING_FORMAT = i32;
pub type D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY = i32;
pub type D3DKMDT_VIDEO_SIGNAL_STANDARD = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_CONTENT = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_ROTATION = i32;
pub type D3DKMDT_VIDPN_PRESENT_PATH_SCALING = i32;
pub type D3DKMDT_VIDPN_SOURCE_MODE_TYPE = i32;
pub type D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = i32;
pub type D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = i32;
pub type D3DKMT_ALLOCATIONRESIDENCYSTATUS = i32;
pub type D3DKMT_AUXILIARYPRESENTINFO_TYPE = i32;
pub type D3DKMT_BRIGHTNESS_INFO_TYPE = i32;
pub type D3DKMT_CANCEL_PRESENTS_OPERATION = i32;
pub type D3DKMT_CLIENTHINT = i32;
pub type D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = i32;
pub type D3DKMT_DEFRAG_ESCAPE_OPERATION = i32;
pub type D3DKMT_DEVICEESCAPE_TYPE = i32;
pub type D3DKMT_DEVICEEXECUTION_STATE = i32;
pub type D3DKMT_DEVICESTATE_TYPE = i32;
pub type D3DKMT_DEVICE_ERROR_REASON = i32;
pub type D3DKMT_DMMESCAPETYPE = i32;
pub type D3DKMT_DRIVERVERSION = i32;
pub type D3DKMT_ESCAPETYPE = i32;
pub type D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = i32;
pub type D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = i32;
pub type D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = i32;
pub type D3DKMT_GPU_PREFERENCE_QUERY_STATE = i32;
pub type D3DKMT_GPU_PREFERENCE_QUERY_TYPE = i32;
pub type D3DKMT_MEMORY_SEGMENT_GROUP = i32;
pub type D3DKMT_MIRACAST_DEVICE_STATUS = i32;
pub type D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = i32;
pub type D3DKMT_MIRACAST_DRIVER_TYPE = i32;
pub type D3DKMT_MULTIPLANE_OVERLAY_BLEND = i32;
pub type D3DKMT_MULTIPLANE_OVERLAY_FLAGS = i32;
pub type D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = i32;
pub type D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = i32;
pub type D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
pub type D3DKMT_OFFER_PRIORITY = i32;
pub type D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub type D3DKMT_OUTPUTDUPL_METADATATYPE = i32;
pub type D3DKMT_PNP_KEY_TYPE = i32;
pub type D3DKMT_PRESENT_MODEL = i32;
pub type D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = i32;
pub type D3DKMT_PROTECTED_SESSION_STATUS = i32;
pub type D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = i32;
pub type D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = i32;
pub type D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = i32;
pub type D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = i32;
pub type D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = i32;
pub type D3DKMT_QUERYSTATISTICS_TYPE = i32;
pub type D3DKMT_QUEUEDLIMIT_TYPE = i32;
pub type D3DKMT_SCHEDULINGPRIORITYCLASS = i32;
pub type D3DKMT_STANDARDALLOCATIONTYPE = i32;
pub type D3DKMT_TDRDBGCTRLTYPE = i32;
pub type D3DKMT_VAD_ESCAPE_COMMAND = i32;
pub type D3DKMT_VERIFIER_OPTION_MODE = i32;
pub type D3DKMT_VIDMMESCAPETYPE = i32;
pub type D3DKMT_VIDPNSOURCEOWNER_TYPE = i32;
pub type D3DKMT_VIDSCHESCAPETYPE = i32;
pub type D3DNTHAL_DP2OPERATION = i32;
pub type D3DVSD_TOKENTYPE = i32;
pub type DISPLAYID_DETAILED_TIMING_TYPE_I_ASPECT_RATIO = i32;
pub type DISPLAYID_DETAILED_TIMING_TYPE_I_SCANNING_MODE = i32;
pub type DISPLAYID_DETAILED_TIMING_TYPE_I_STEREO_MODE = i32;
pub type DISPLAYID_DETAILED_TIMING_TYPE_I_SYNC_POLARITY = i32;
pub type DXGKDT_OPM_DVI_CHARACTERISTICS = i32;
pub type DXGKMDT_CERTIFICATE_TYPE = i32;
pub type DXGKMDT_OPM_ACP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_BUS_TYPE_AND_IMPLEMENTATION = i32;
pub type DXGKMDT_OPM_CGMSA = i32;
pub type DXGKMDT_OPM_CONNECTOR_TYPE = i32;
pub type DXGKMDT_OPM_DPCP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_HDCP_FLAG = i32;
pub type DXGKMDT_OPM_HDCP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_IMAGE_ASPECT_RATIO_EN300294 = i32;
pub type DXGKMDT_OPM_INTERLEAVE_FORMAT = i32;
pub type DXGKMDT_OPM_OUTPUT_HARDWARE_PROTECTION = i32;
pub type DXGKMDT_OPM_PROTECTION_STANDARD = i32;
pub type DXGKMDT_OPM_PROTECTION_TYPE = i32;
pub type DXGKMDT_OPM_STATUS = i32;
pub type DXGKMDT_OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL = i32;
pub type DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS = i32;
pub type DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = i32;
pub type DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = i32;
pub type DXGKMT_POWER_SHARED_TYPE = i32;
pub type DXGKVGPU_ESCAPE_TYPE = i32;
pub type DXGK_BACKLIGHT_OPTIMIZATION_LEVEL = i32;
pub type DXGK_CHILD_DEVICE_HPD_AWARENESS = i32;
pub type DXGK_DISPLAY_DESCRIPTOR_TYPE = u8;
pub type DXGK_DISPLAY_TECHNOLOGY = u8;
pub type DXGK_DISPLAY_USAGE = u8;
pub type DXGK_ENGINE_TYPE = i32;
pub type DXGK_GENERAL_ERROR_CODE = i32;
pub type DXGK_MIRACAST_CHUNK_TYPE = i32;
pub type DXGK_PAGE_FAULT_FLAGS = i32;
pub type DXGK_PTE_PAGE_SIZE = i32;
pub type DXGK_RENDER_PIPELINE_STAGE = i32;
pub type KMTQUERYADAPTERINFOTYPE = i32;
pub type KMTUMDVERSION = i32;
pub type KMT_DISPLAY_UMD_VERSION = i32;
pub type OUTPUTDUPL_CONTEXT_DEBUG_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIARG_CREATERESOURCE {
    pub Format: D3DDDIFORMAT,
    pub Pool: D3DDDI_POOL,
    pub MultisampleType: D3DDDIMULTISAMPLE_TYPE,
    pub MultisampleQuality: u32,
    pub pSurfList: *const D3DDDI_SURFACEINFO,
    pub SurfCount: u32,
    pub MipLevels: u32,
    pub Fvf: u32,
    pub VidPnSourceId: u32,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub hResource: super::super::super::Win32::Foundation::HANDLE,
    pub Flags: D3DDDI_RESOURCEFLAGS,
    pub Rotation: D3DDDI_ROTATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIARG_CREATERESOURCE2 {
    pub Format: D3DDDIFORMAT,
    pub Pool: D3DDDI_POOL,
    pub MultisampleType: D3DDDIMULTISAMPLE_TYPE,
    pub MultisampleQuality: u32,
    pub pSurfList: *const D3DDDI_SURFACEINFO,
    pub SurfCount: u32,
    pub MipLevels: u32,
    pub Fvf: u32,
    pub VidPnSourceId: u32,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub hResource: super::super::super::Win32::Foundation::HANDLE,
    pub Flags: D3DDDI_RESOURCEFLAGS,
    pub Rotation: D3DDDI_ROTATION,
    pub Flags2: D3DDDI_RESOURCEFLAGS2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_DESTROYALLOCATION2FLAGS {
    pub Anonymous: D3DDDICB_DESTROYALLOCATION2FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_DESTROYALLOCATION2FLAGS_0 {
    pub Anonymous: D3DDDICB_DESTROYALLOCATION2FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_DESTROYALLOCATION2FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCK2FLAGS {
    pub Anonymous: D3DDDICB_LOCK2FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_LOCK2FLAGS_0 {
    pub Anonymous: D3DDDICB_LOCK2FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCK2FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCKFLAGS {
    pub Anonymous: D3DDDICB_LOCKFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_LOCKFLAGS_0 {
    pub Anonymous: D3DDDICB_LOCKFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCKFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_SIGNALFLAGS {
    pub Anonymous: D3DDDICB_SIGNALFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_SIGNALFLAGS_0 {
    pub Anonymous: D3DDDICB_SIGNALFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_SIGNALFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE {
    pub Anonymous: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0 {
    pub Anonymous: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0_0,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIRECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO {
    pub hAllocation: u32,
    pub pSystemMem: *const core::ffi::c_void,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub VidPnSourceId: u32,
    pub Flags: D3DDDI_ALLOCATIONINFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO_0 {
    pub Anonymous: D3DDDI_ALLOCATIONINFO_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO2 {
    pub hAllocation: u32,
    pub Anonymous1: D3DDDI_ALLOCATIONINFO2_0,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub VidPnSourceId: u32,
    pub Flags: D3DDDI_ALLOCATIONINFO2_2,
    pub GpuVirtualAddress: u64,
    pub Anonymous2: D3DDDI_ALLOCATIONINFO2_1,
    pub Reserved: [usize; 5],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_0 {
    pub hSection: super::super::super::Win32::Foundation::HANDLE,
    pub pSystemMem: *const core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_1 {
    pub Priority: u32,
    pub Unused: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_2 {
    pub Anonymous: D3DDDI_ALLOCATIONINFO2_2_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO2_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONLIST {
    pub hAllocation: u32,
    pub Anonymous: D3DDDI_ALLOCATIONLIST_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONLIST_0 {
    pub Anonymous: D3DDDI_ALLOCATIONLIST_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONLIST_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATECONTEXTFLAGS {
    pub Anonymous: D3DDDI_CREATECONTEXTFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATECONTEXTFLAGS_0 {
    pub Anonymous: D3DDDI_CREATECONTEXTFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATECONTEXTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWCONTEXTFLAGS {
    pub Anonymous: D3DDDI_CREATEHWCONTEXTFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATEHWCONTEXTFLAGS_0 {
    pub Anonymous: D3DDDI_CREATEHWCONTEXTFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWCONTEXTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWQUEUEFLAGS {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATEHWQUEUEFLAGS_0 {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWQUEUEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATENATIVEFENCEINFO {
    pub InitialFenceValue: u64,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub EngineAffinity: u32,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub hSyncObject: u32,
    pub NativeFenceMapping: D3DDDI_NATIVEFENCEMAPPING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DESTROYPAGINGQUEUE {
    pub hPagingQueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DRIVERESCAPE_CPUEVENTUSAGE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hSyncObject: u32,
    pub hKmdCpuEvent: u64,
    pub Usage: [u32; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DRIVERESCAPE_TRANSLATEALLOCATIONEHANDLE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hAllocation: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DRIVERESCAPE_TRANSLATERESOURCEHANDLE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hResource: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ESCAPEFLAGS {
    pub Anonymous: D3DDDI_ESCAPEFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ESCAPEFLAGS_0 {
    pub Anonymous: D3DDDI_ESCAPEFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ESCAPEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_EVICT_FLAGS {
    pub Anonymous: D3DDDI_EVICT_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_EVICT_FLAGS_0 {
    pub Anonymous: D3DDDI_EVICT_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_EVICT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GAMMA_RAMP_DXGI_1 {
    pub Scale: D3DDDI_DXGI_RGB,
    pub Offset: D3DDDI_DXGI_RGB,
    pub GammaCurve: [D3DDDI_DXGI_RGB; 1025],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GAMMA_RAMP_RGB256x3x16 {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA {
    pub hResource: u32,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_KERNELOVERLAYINFO {
    pub hAllocation: u32,
    pub DstRect: D3DDDIRECT,
    pub SrcRect: D3DDDIRECT,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAKERESIDENT {
    pub hPagingQueue: u32,
    pub NumAllocations: u32,
    pub AllocationList: *const u32,
    pub PriorityList: *const u32,
    pub Flags: D3DDDI_MAKERESIDENT_FLAGS,
    pub PagingFenceValue: u64,
    pub NumBytesToTrim: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAKERESIDENT_FLAGS {
    pub Anonymous: D3DDDI_MAKERESIDENT_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_MAKERESIDENT_FLAGS_0 {
    pub Anonymous: D3DDDI_MAKERESIDENT_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAKERESIDENT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAPGPUVIRTUALADDRESS {
    pub hPagingQueue: u32,
    pub BaseAddress: u64,
    pub MinimumAddress: u64,
    pub MaximumAddress: u64,
    pub hAllocation: u32,
    pub OffsetInPages: u64,
    pub SizeInPages: u64,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
    pub DriverProtection: u64,
    pub Reserved0: u32,
    pub Reserved1: u64,
    pub VirtualAddress: u64,
    pub PagingFenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MULTISAMPLINGMETHOD {
    pub NumSamples: u32,
    pub NumQualityLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_NATIVEFENCEMAPPING {
    pub CurrentValueCpuVa: *mut core::ffi::c_void,
    pub CurrentValueGpuVa: u64,
    pub MonitoredValueGpuVa: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OFFER_FLAGS {
    pub Anonymous: D3DDDI_OFFER_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_OFFER_FLAGS_0 {
    pub Anonymous: D3DDDI_OFFER_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OFFER_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OPENALLOCATIONINFO {
    pub hAllocation: u32,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OPENALLOCATIONINFO2 {
    pub hAllocation: u32,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub GpuVirtualAddress: u64,
    pub Reserved: [usize; 6],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_PATCHLOCATIONLIST {
    pub AllocationIndex: u32,
    pub Anonymous: D3DDDI_PATCHLOCATIONLIST_0,
    pub DriverId: u32,
    pub AllocationOffset: u32,
    pub PatchOffset: u32,
    pub SplitOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_PATCHLOCATIONLIST_0 {
    pub Anonymous: D3DDDI_PATCHLOCATIONLIST_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_PATCHLOCATIONLIST_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_QUERYREGISTRY_FLAGS {
    pub Anonymous: D3DDDI_QUERYREGISTRY_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_QUERYREGISTRY_FLAGS_0 {
    pub Anonymous: D3DDDI_QUERYREGISTRY_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_QUERYREGISTRY_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_QUERYREGISTRY_INFO {
    pub QueryType: D3DDDI_QUERYREGISTRY_TYPE,
    pub QueryFlags: D3DDDI_QUERYREGISTRY_FLAGS,
    pub ValueName: [u16; 260],
    pub ValueType: u32,
    pub PhysicalAdapterIndex: u32,
    pub OutputValueSize: u32,
    pub Status: D3DDDI_QUERYREGISTRY_STATUS,
    pub Anonymous: D3DDDI_QUERYREGISTRY_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_QUERYREGISTRY_INFO_0 {
    pub OutputDword: u32,
    pub OutputQword: u64,
    pub OutputString: [u16; 1],
    pub OutputBinary: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESERVEGPUVIRTUALADDRESS {
    pub Anonymous1: D3DDDI_RESERVEGPUVIRTUALADDRESS_0,
    pub BaseAddress: u64,
    pub MinimumAddress: u64,
    pub MaximumAddress: u64,
    pub Size: u64,
    pub Anonymous2: D3DDDI_RESERVEGPUVIRTUALADDRESS_1,
    pub Anonymous3: D3DDDI_RESERVEGPUVIRTUALADDRESS_2,
    pub VirtualAddress: u64,
    pub Anonymous4: D3DDDI_RESERVEGPUVIRTUALADDRESS_3,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_0 {
    pub hPagingQueue: u32,
    pub hAdapter: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_1 {
    pub ReservationType: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE,
    pub Reserved0: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_2 {
    pub DriverProtection: u64,
    pub Reserved1: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_3 {
    pub PagingFenceValue: u64,
    pub Reserved2: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS {
    pub Anonymous: D3DDDI_RESOURCEFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESOURCEFLAGS_0 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS2 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS2_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESOURCEFLAGS2_0 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS2_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS2_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SEGMENTPREFERENCE {
    pub Anonymous: D3DDDI_SEGMENTPREFERENCE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SEGMENTPREFERENCE_0 {
    pub Anonymous: D3DDDI_SEGMENTPREFERENCE_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SEGMENTPREFERENCE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SURFACEINFO {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO {
    pub Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECTINFO_0 {
    pub SynchronizationMutex: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_2,
    pub Semaphore: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_1,
    pub Reserved: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_0 {
    pub Reserved: [u32; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_1 {
    pub MaxCount: u32,
    pub InitialCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_2 {
    pub InitialState: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2 {
    pub Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0,
    pub SharedHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0 {
    pub SynchronizationMutex: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_6,
    pub Semaphore: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_5,
    pub Fence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_1,
    pub CPUNotification: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_0,
    pub MonitoredFence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_2,
    pub PeriodicMonitoredFence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_3,
    pub Reserved: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_4,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_0 {
    pub Event: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_1 {
    pub FenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_2 {
    pub InitialFenceValue: u64,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: u64,
    pub EngineAffinity: u32,
    pub Padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_3 {
    pub hAdapter: u32,
    pub VidPnTargetId: u32,
    pub Time: u64,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: u64,
    pub EngineAffinity: u32,
    pub Padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_4 {
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_5 {
    pub MaxCount: u32,
    pub InitialCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_6 {
    pub InitialState: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS {
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0 {
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_TRIMRESIDENCYSET_FLAGS {
    pub Anonymous: D3DDDI_TRIMRESIDENCYSET_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_TRIMRESIDENCYSET_FLAGS_0 {
    pub Anonymous: D3DDDI_TRIMRESIDENCYSET_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_TRIMRESIDENCYSET_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY {
    pub hPagingQueue: u32,
    pub hAllocation: u32,
    pub SupportedSegmentSet: u32,
    pub PreferredSegment: D3DDDI_SEGMENTPREFERENCE,
    pub Flags: D3DDDI_UPDATEALLOCPROPERTY_FLAGS,
    pub PagingFenceValue: u64,
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEALLOCPROPERTY_0 {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_0_0,
    pub PropertyMaskValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_FLAGS {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0 {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION {
    pub OperationType: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE,
    pub Anonymous: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0 {
    pub Map: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_2,
    pub MapProtect: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_1,
    pub Unmap: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_3,
    pub Copy: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_0 {
    pub SourceAddress: u64,
    pub SizeInBytes: u64,
    pub DestAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_1 {
    pub BaseAddress: u64,
    pub SizeInBytes: u64,
    pub hAllocation: u32,
    pub AllocationOffsetInBytes: u64,
    pub AllocationSizeInBytes: u64,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
    pub DriverProtection: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_2 {
    pub BaseAddress: u64,
    pub SizeInBytes: u64,
    pub hAllocation: u32,
    pub AllocationOffsetInBytes: u64,
    pub AllocationSizeInBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_3 {
    pub BaseAddress: u64,
    pub SizeInBytes: u64,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS {
    pub Anonymous: D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0 {
    pub Anonymous: D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DGPU_PHYSICAL_ADDRESS {
    pub SegmentId: u32,
    pub Padding: u32,
    pub SegmentOffset: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_CLIPPEDTRIANGLEFAN {
    pub FirstVertexOffset: u32,
    pub dwEdgeFlags: u32,
    pub PrimitiveCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_CONTEXTDESTROYALLDATA {
    pub dwPID: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_CONTEXTDESTROYDATA {
    pub dwhContext: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_D3DDX6EXTENDEDCAPS {
    pub dwSize: u32,
    pub dwMinTextureWidth: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMinStippleWidth: u32,
    pub dwMaxStippleWidth: u32,
    pub dwMinStippleHeight: u32,
    pub dwMaxStippleHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_D3DEXTENDEDCAPS {
    pub dwSize: u32,
    pub dwMinTextureWidth: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMinStippleWidth: u32,
    pub dwMaxStippleWidth: u32,
    pub dwMinStippleHeight: u32,
    pub dwMaxStippleHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
    pub dwMaxActiveLights: u32,
    pub dvMaxVertexW: f32,
    pub wMaxUserClipPlanes: u16,
    pub wMaxVertexBlendMatrices: u16,
    pub dwVertexProcessingCaps: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2ADDDIRTYRECT {
    pub dwSurface: u32,
    pub rDirtyArea: super::super::super::Win32::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2BLT {
    pub dwSource: u32,
    pub rSource: super::super::super::Win32::Foundation::RECTL,
    pub dwSourceMipLevel: u32,
    pub dwDest: u32,
    pub rDest: super::super::super::Win32::Foundation::RECTL,
    pub dwDestMipLevel: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CLEAR {
    pub dwFlags: u32,
    pub dwFillColor: u32,
    pub dvFillDepth: f32,
    pub dwFillStencil: u32,
    pub Rects: [super::super::super::Win32::Foundation::RECT; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2COLORFILL {
    pub dwSurface: u32,
    pub rRect: super::super::super::Win32::Foundation::RECTL,
    pub Color: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2COMMAND {
    pub bCommand: u8,
    pub bReserved: u8,
    pub Anonymous: D3DHAL_DP2COMMAND_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DHAL_DP2COMMAND_0 {
    pub wPrimitiveCount: u16,
    pub wStateCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CREATELIGHT {
    pub dwIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CREATEPIXELSHADER {
    pub dwHandle: u32,
    pub dwCodeSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CREATEVERTEXSHADER {
    pub dwHandle: u32,
    pub dwDeclSize: u32,
    pub dwCodeSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CREATEVERTEXSHADERDECL {
    pub dwHandle: u32,
    pub dwNumVertexElements: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2CREATEVERTEXSHADERFUNC {
    pub dwHandle: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2DELETEQUERY {
    pub dwQueryID: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2DRAWRECTPATCH {
    pub Handle: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2DRAWTRIPATCH {
    pub Handle: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2EXT {
    pub dwExtToken: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDLINELIST {
    pub wV1: u16,
    pub wV2: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDLINESTRIP {
    pub wV: [u16; 2],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDTRIANGLEFAN {
    pub wV: [u16; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDTRIANGLELIST {
    pub wV1: u16,
    pub wV2: u16,
    pub wV3: u16,
    pub wFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDTRIANGLELIST2 {
    pub wV1: u16,
    pub wV2: u16,
    pub wV3: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2INDEXEDTRIANGLESTRIP {
    pub wV: [u16; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2ISSUEQUERY {
    pub dwQueryID: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2LINELIST {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2LINESTRIP {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2PIXELSHADER {
    pub dwHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2POINTS {
    pub wCount: u16,
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2RESPONSE {
    pub bCommand: u8,
    pub bReserved: u8,
    pub wStateCount: u16,
    pub dwTotalSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2RESPONSEQUERY {
    pub dwQueryID: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETCLIPPLANE {
    pub dwIndex: u32,
    pub plane: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETCONVOLUTIONKERNELMONO {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETDEPTHSTENCIL {
    pub hZBuffer: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETINDICES {
    pub dwVBHandle: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETLIGHT {
    pub dwIndex: u32,
    pub dwDataType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETPALETTE {
    pub dwPaletteHandle: u32,
    pub dwPaletteFlags: u32,
    pub dwSurfaceHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETPIXELSHADERCONST {
    pub dwRegister: u32,
    pub dwCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETPRIORITY {
    pub dwDDSurface: u32,
    pub dwPriority: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETRENDERTARGET {
    pub hRenderTarget: u32,
    pub hZBuffer: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETRENDERTARGET2 {
    pub RTIndex: u32,
    pub hRenderTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETSTREAMSOURCE {
    pub dwStream: u32,
    pub dwVBHandle: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETSTREAMSOURCE2 {
    pub dwStream: u32,
    pub dwVBHandle: u32,
    pub dwOffset: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETSTREAMSOURCEFREQ {
    pub dwStream: u32,
    pub dwDivider: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETSTREAMSOURCEUM {
    pub dwStream: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETTEXLOD {
    pub dwDDSurface: u32,
    pub dwLOD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SETVERTEXSHADERCONST {
    pub dwRegister: u32,
    pub dwCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2STARTVERTEX {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2SURFACEBLT {
    pub dwSource: u32,
    pub rSource: super::super::super::Win32::Foundation::RECTL,
    pub dwSourceMipLevel: u32,
    pub dwDest: u32,
    pub rDest: super::super::super::Win32::Foundation::RECTL,
    pub dwDestMipLevel: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TEXBLT {
    pub dwDDDestSurface: u32,
    pub dwDDSrcSurface: u32,
    pub pDest: super::super::super::Win32::Foundation::POINT,
    pub rSrc: super::super::super::Win32::Foundation::RECTL,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TEXTURESTAGESTATE {
    pub wStage: u16,
    pub TSState: u16,
    pub dwValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TRIANGLEFAN {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TRIANGLEFAN_IMM {
    pub dwEdgeFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TRIANGLELIST {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2TRIANGLESTRIP {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2UPDATEPALETTE {
    pub dwPaletteHandle: u32,
    pub wStartIndex: u16,
    pub wNumEntries: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2VERTEXSHADER {
    pub dwHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2VIEWPORTINFO {
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2WINFO {
    pub dvWNear: f32,
    pub dvWFar: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DP2ZRANGE {
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DRAWPRIMCOUNTS {
    pub wNumStateChanges: u16,
    pub wPrimitiveType: u16,
    pub wVertexType: u16,
    pub wNumVertices: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_DRAWPRIMITIVESDATA {
    pub dwhContext: usize,
    pub dwFlags: u32,
    pub lpvData: *mut core::ffi::c_void,
    pub dwFVFControl: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_RENDERSTATEDATA {
    pub dwhContext: usize,
    pub dwOffset: u32,
    pub dwCount: u32,
    pub lpExeBuf: *mut core::ffi::c_void,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_SCENECAPTUREDATA {
    pub dwhContext: usize,
    pub dwFlag: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_TEXTURECREATEDATA {
    pub dwhContext: usize,
    pub lpDDS: *mut core::ffi::c_void,
    pub dwHandle: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_TEXTUREDESTROYDATA {
    pub dwhContext: usize,
    pub dwHandle: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_TEXTUREGETSURFDATA {
    pub dwhContext: usize,
    pub lpDDS: usize,
    pub dwHandle: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_TEXTURESWAPDATA {
    pub dwhContext: usize,
    pub dwHandle1: u32,
    pub dwHandle2: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHAL_VALIDATETEXTURESTAGESTATEDATA {
    pub dwhContext: usize,
    pub dwFlags: u32,
    pub dwReserved: usize,
    pub dwNumPasses: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_3x4_COLORSPACE_TRANSFORM {
    pub ColorMatrix3x4: [f32; 12],
    pub ScalarMultiplier: f32,
    pub LookupTable1D: [D3DDDI_DXGI_RGB; 4096],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2 {
    pub StageControlLookupTable1DDegamma: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DDegamma: [D3DDDI_DXGI_RGB; 4096],
    pub StageControlColorMatrix3x3: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub ColorMatrix3x3: [f32; 9],
    pub StageControlLookupTable1DRegamma: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DRegamma: [D3DDDI_DXGI_RGB; 4096],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES {
    pub FirstChannel: u32,
    pub SecondChannel: u32,
    pub ThirdChannel: u32,
    pub FourthChannel: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_DISPLAYMODE_FLAGS {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_FREQUENCY_RANGE {
    pub MinVSyncFreq: D3DDDI_RATIONAL,
    pub MaxVSyncFreq: D3DDDI_RATIONAL,
    pub MinHSyncFreq: D3DDDI_RATIONAL,
    pub MaxHSyncFreq: D3DDDI_RATIONAL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GAMMA_RAMP {
    pub Type: D3DDDI_GAMMARAMP_TYPE,
    pub DataSize: usize,
    pub Data: D3DKMDT_GAMMA_RAMP_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_GAMMA_RAMP_0 {
    pub pRgb256x3x16: *mut D3DDDI_GAMMA_RAMP_RGB256x3x16,
    pub pDxgi1: *mut D3DDDI_GAMMA_RAMP_DXGI_1,
    pub p3x4: *mut D3DKMDT_3x4_COLORSPACE_TRANSFORM,
    pub pMatrixV2: *mut D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2,
    pub pRaw: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GDISURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3DDDIFORMAT,
    pub Type: D3DKMDT_GDISURFACETYPE,
    pub Flags: D3DKMDT_GDISURFACEFLAGS,
    pub Pitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GDISURFACEFLAGS {
    pub Anonymous: D3DKMDT_GDISURFACEFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_GDISURFACEFLAGS_0 {
    pub Anonymous: D3DKMDT_GDISURFACEFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GDISURFACEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_GRAPHICS_RENDERING_FORMAT {
    pub PrimSurfSize: D3DKMDT_2DREGION,
    pub VisibleRegionSize: D3DKMDT_2DREGION,
    pub Stride: u32,
    pub PixelFormat: D3DDDIFORMAT,
    pub ColorBasis: D3DKMDT_COLOR_BASIS,
    pub PixelValueAccessMode: D3DKMDT_PIXEL_VALUE_ACCESS_MODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_DESCRIPTOR {
    pub Id: u32,
    pub Type: D3DKMDT_MONITOR_DESCRIPTOR_TYPE,
    pub DataSize: usize,
    pub pData: *mut core::ffi::c_void,
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_FREQUENCY_RANGE {
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
    pub RangeLimits: D3DKMDT_FREQUENCY_RANGE,
    pub ConstraintType: D3DKMDT_MONITOR_FREQUENCY_RANGE_CONSTRAINT,
    pub Constraint: D3DKMDT_MONITOR_FREQUENCY_RANGE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_MONITOR_FREQUENCY_RANGE_0 {
    pub ActiveSize: D3DKMDT_2DREGION,
    pub MaxPixelRate: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_MONITOR_SOURCE_MODE {
    pub Id: u32,
    pub VideoSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO,
    pub ColorBasis: D3DKMDT_COLOR_BASIS,
    pub ColorCoeffDynamicRanges: D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES,
    pub Origin: D3DKMDT_MONITOR_CAPABILITIES_ORIGIN,
    pub Preference: D3DKMDT_MODE_PREFERENCE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_PALETTEDATA {
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Unused: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_PREEMPTION_CAPS {
    pub GraphicsPreemptionGranularity: D3DKMDT_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: D3DKMDT_COMPUTE_PREEMPTION_GRANULARITY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_SHADOWSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3DDDIFORMAT,
    pub Pitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_SHAREDPRIMARYSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3DDDIFORMAT,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_STAGINGSURFACEDATA {
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDEO_PRESENT_SOURCE {
    pub Id: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDEO_PRESENT_TARGET {
    pub Id: u32,
    pub VideoOutputTechnology: D3DKMDT_VIDEO_OUTPUT_TECHNOLOGY,
    pub VideoOutputHpdAwareness: DXGK_CHILD_DEVICE_HPD_AWARENESS,
    pub MonitorOrientationAwareness: D3DKMDT_MONITOR_ORIENTATION_AWARENESS,
    pub SupportsSdtvModes: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDEO_SIGNAL_INFO {
    pub VideoStandard: D3DKMDT_VIDEO_SIGNAL_STANDARD,
    pub TotalSize: D3DKMDT_2DREGION,
    pub ActiveSize: D3DKMDT_2DREGION,
    pub VSyncFreq: D3DDDI_RATIONAL,
    pub HSyncFreq: D3DDDI_RATIONAL,
    pub PixelRate: usize,
    pub Anonymous: D3DKMDT_VIDEO_SIGNAL_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO_0_0,
    pub ScanLineOrdering: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_HW_CAPABILITY {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH {
    pub VidPnSourceId: u32,
    pub VidPnTargetId: u32,
    pub ImportanceOrdinal: D3DKMDT_VIDPN_PRESENT_PATH_IMPORTANCE,
    pub ContentTransformation: D3DKMDT_VIDPN_PRESENT_PATH_TRANSFORMATION,
    pub VisibleFromActiveTLOffset: D3DKMDT_2DREGION,
    pub VisibleFromActiveBROffset: D3DKMDT_2DREGION,
    pub VidPnTargetColorBasis: D3DKMDT_COLOR_BASIS,
    pub VidPnTargetColorCoeffDynamicRanges: D3DKMDT_COLOR_COEFF_DYNAMIC_RANGES,
    pub Content: D3DKMDT_VIDPN_PRESENT_PATH_CONTENT,
    pub CopyProtection: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION,
    pub GammaRamp: D3DKMDT_GAMMA_RAMP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION {
    pub CopyProtectionType: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_TYPE,
    pub APSTriggerBits: u32,
    pub OEMCopyProtection: [u8; 256],
    pub CopyProtectionSupport: D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_SUPPORT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_COPYPROTECTION_SUPPORT {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_ROTATION_SUPPORT {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_SCALING_SUPPORT {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_PRESENT_PATH_TRANSFORMATION {
    pub Scaling: D3DKMDT_VIDPN_PRESENT_PATH_SCALING,
    pub ScalingSupport: D3DKMDT_VIDPN_PRESENT_PATH_SCALING_SUPPORT,
    pub Rotation: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION,
    pub RotationSupport: D3DKMDT_VIDPN_PRESENT_PATH_ROTATION_SUPPORT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_SOURCE_MODE {
    pub Id: u32,
    pub Type: D3DKMDT_VIDPN_SOURCE_MODE_TYPE,
    pub Format: D3DKMDT_VIDPN_SOURCE_MODE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDPN_SOURCE_MODE_0 {
    pub Graphics: D3DKMDT_GRAPHICS_RENDERING_FORMAT,
    pub Text: D3DKMDT_TEXT_RENDERING_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_TARGET_MODE {
    pub Id: u32,
    pub VideoSignalInfo: D3DKMDT_VIDEO_SIGNAL_INFO,
    pub Anonymous: D3DKMDT_VIDPN_TARGET_MODE_0,
    pub MinimumVSyncFreq: D3DDDI_RATIONAL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_VIDPN_TARGET_MODE_0 {
    pub WireFormatAndPreference: D3DKMDT_WIRE_FORMAT_AND_PREFERENCE,
    pub Anonymous: D3DKMDT_VIDPN_TARGET_MODE_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIDPN_TARGET_MODE_0_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_VIRTUALGPUSURFACEDATA {
    pub Size: u64,
    pub Alignment: u32,
    pub DriverSegmentId: u32,
    pub PrivateDriverData: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMDT_WIRE_FORMAT_AND_PREFERENCE {
    pub Anonymous: D3DKMDT_WIRE_FORMAT_AND_PREFERENCE_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_WIRE_FORMAT_AND_PREFERENCE_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ACQUIREKEYEDMUTEX {
    pub hKeyedMutex: u32,
    pub Key: u64,
    pub pTimeout: *mut i64,
    pub FenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ACQUIREKEYEDMUTEX2 {
    pub hKeyedMutex: u32,
    pub Key: u64,
    pub pTimeout: *mut i64,
    pub FenceValue: u64,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ACTIVATE_SPECIFIC_DIAG_ESCAPE {
    pub Type: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE,
    pub Activate: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERADDRESS {
    pub BusNumber: u32,
    pub DeviceNumber: u32,
    pub FunctionNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERINFO {
    pub hAdapter: u32,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub NumOfSources: u32,
    pub bPrecisePresentRegionsPreferred: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERREGISTRYINFO {
    pub AdapterString: [u16; 260],
    pub BiosString: [u16; 260],
    pub DacType: [u16; 260],
    pub ChipType: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERTYPE {
    pub Anonymous: D3DKMT_ADAPTERTYPE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTERTYPE_0 {
    pub Anonymous: D3DKMT_ADAPTERTYPE_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERTYPE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_PERFDATA {
    pub PhysicalAdapterIndex: u32,
    pub MemoryFrequency: u64,
    pub MaxMemoryFrequency: u64,
    pub MaxMemoryFrequencyOC: u64,
    pub MemoryBandwidth: u64,
    pub PCIEBandwidth: u64,
    pub FanRPM: u32,
    pub Power: u32,
    pub Temperature: u32,
    pub PowerStateOverride: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_PERFDATACAPS {
    pub PhysicalAdapterIndex: u32,
    pub MaxMemoryBandwidth: u64,
    pub MaxPCIEBandwidth: u64,
    pub MaxFanRPM: u32,
    pub TemperatureMax: u32,
    pub TemperatureWarning: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_VERIFIER_OPTION {
    pub Type: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE,
    pub Mode: D3DKMT_VERIFIER_OPTION_MODE,
    pub Data: D3DKMT_ADAPTER_VERIFIER_OPTION_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTER_VERIFIER_OPTION_DATA {
    pub VidMmFlags: D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS,
    pub VidMmTrimInterval: D3DKMT_ADAPTER_VERIFIER_VIDMM_TRIM_INTERVAL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS {
    pub Anonymous: D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_VERIFIER_VIDMM_TRIM_INTERVAL {
    pub MinimumTrimInterval: u64,
    pub MaximumTrimInterval: u64,
    pub IdleTrimInterval: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADJUSTFULLSCREENGAMMA {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub Scale: D3DDDI_DXGI_RGB,
    pub Offset: D3DDDI_DXGI_RGB,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_AUXILIARYPRESENTINFO {
    pub size: u32,
    pub r#type: D3DKMT_AUXILIARYPRESENTINFO_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BDDFALLBACK_CTL {
    pub ForceBddHeadlessNextFallback: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BLOCKLIST_INFO {
    pub Size: u32,
    pub BlockList: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BLTMODEL_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
    pub hPhysicalSurface: u64,
    pub EventId: u64,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BRIGHTNESS_INFO {
    pub Type: D3DKMT_BRIGHTNESS_INFO_TYPE,
    pub ChildUid: u32,
    pub Anonymous: D3DKMT_BRIGHTNESS_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_BRIGHTNESS_INFO_0 {
    pub PossibleLevels: D3DKMT_BRIGHTNESS_POSSIBLE_LEVELS,
    pub Brightness: u8,
    pub BrightnessCaps: DXGK_BRIGHTNESS_CAPS,
    pub BrightnessState: DXGK_BRIGHTNESS_STATE,
    pub OptimizationLevel: DXGK_BACKLIGHT_OPTIMIZATION_LEVEL,
    pub ReductionInfo: DXGK_BACKLIGHT_INFO,
    pub VerboseLogging: super::super::super::Win32::Foundation::BOOLEAN,
    pub NitRanges: DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT,
    pub GetBrightnessMillinits: DXGK_BRIGHTNESS_GET_OUT,
    pub SetBrightnessMillinits: DXGK_BRIGHTNESS_SET_IN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BRIGHTNESS_POSSIBLE_LEVELS {
    pub LevelCount: u8,
    pub BrightnessLevels: [u8; 256],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BUDGETCHANGENOTIFICATION {
    pub Context: *mut core::ffi::c_void,
    pub Budget: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CANCEL_PRESENTS {
    pub cbSize: u32,
    pub hDevice: u32,
    pub Flags: D3DKMT_CANCEL_PRESENTS_FLAGS,
    pub Operation: D3DKMT_CANCEL_PRESENTS_OPERATION,
    pub CancelFromPresentId: u64,
    pub CompSurfaceLuid: super::super::super::Win32::Foundation::LUID,
    pub BindId: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CANCEL_PRESENTS_FLAGS {
    pub Anonymous: D3DKMT_CANCEL_PRESENTS_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CANCEL_PRESENTS_FLAGS_0 {
    pub ReprogramInterrupt: D3DKMT_CANCEL_PRESENTS_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CANCEL_PRESENTS_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHANGESURFACEPOINTER {
    pub hDC: super::super::super::Win32::Graphics::Gdi::HDC,
    pub hBitmap: super::super::super::Win32::Foundation::HANDLE,
    pub pSurfacePointer: *mut core::ffi::c_void,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHANGEVIDEOMEMORYRESERVATION {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub hAdapter: u32,
    pub MemorySegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub Reservation: u64,
    pub PhysicalAdapterIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMONITORPOWERSTATE {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT {
    pub hDevice: u32,
    pub PlaneCount: u32,
    pub pOverlayPlanes: *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE,
    pub Supported: super::super::super::Win32::Foundation::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2 {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub PlaneCount: u32,
    pub pOverlayPlanes: *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE2,
    pub Supported: super::super::super::Win32::Foundation::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3 {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub PlaneCount: u32,
    pub ppOverlayPlanes: *mut *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE3,
    pub PostCompositionCount: u32,
    pub ppPostComposition: *mut *mut D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_WITH_SOURCE,
    pub Supported: super::super::super::Win32::Foundation::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKOCCLUSION {
    pub hWindow: super::super::super::Win32::Foundation::HWND,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKSHAREDRESOURCEACCESS {
    pub hResource: u32,
    pub ClientPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE {
    pub hResource: u32,
    pub CompSurfaceLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE2 {
    pub LayerIndex: u32,
    pub hResource: u32,
    pub CompSurfaceLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE3 {
    pub LayerIndex: u32,
    pub hResource: u32,
    pub CompSurfaceLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
    pub pPlaneAttributes: *mut D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO {
    pub Anonymous: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0 {
    pub Anonymous: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CLOSEADAPTER {
    pub hAdapter: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_COMPOSITION_PRESENTHISTORYTOKEN {
    pub hPrivateData: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONFIGURESHAREDRESOURCE {
    pub hDevice: u32,
    pub hResource: u32,
    pub IsDwm: super::super::super::Win32::Foundation::BOOLEAN,
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub AllowAccess: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONNECT_DOORBELL {
    pub hHwQueue: u32,
    pub Flags: D3DKMT_CONNECT_DOORBELL_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONNECT_DOORBELL_FLAGS {
    pub Anonymous: D3DKMT_CONNECT_DOORBELL_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CONNECT_DOORBELL_FLAGS_0 {
    pub Anonymous: D3DKMT_CONNECT_DOORBELL_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONNECT_DOORBELL_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CPDRIVERNAME {
    pub ContentProtectionFileName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEALLOCATION {
    pub hDevice: u32,
    pub hResource: u32,
    pub hGlobalShare: u32,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub Anonymous1: D3DKMT_CREATEALLOCATION_0,
    pub PrivateDriverDataSize: u32,
    pub NumAllocations: u32,
    pub Anonymous2: D3DKMT_CREATEALLOCATION_1,
    pub Flags: D3DKMT_CREATEALLOCATIONFLAGS,
    pub hPrivateRuntimeResourceHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEALLOCATION_0 {
    pub pStandardAllocation: *mut D3DKMT_CREATESTANDARDALLOCATION,
    pub pPrivateDriverData: *const core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEALLOCATION_1 {
    pub pAllocationInfo: *mut D3DDDI_ALLOCATIONINFO,
    pub pAllocationInfo2: *mut D3DDDI_ALLOCATIONINFO2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEALLOCATIONFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATECONTEXT {
    pub hDevice: u32,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: D3DDDI_CREATECONTEXTFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub ClientHint: D3DKMT_CLIENTHINT,
    pub hContext: u32,
    pub pCommandBuffer: *mut core::ffi::c_void,
    pub CommandBufferSize: u32,
    pub pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    pub AllocationListSize: u32,
    pub pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    pub PatchLocationListSize: u32,
    pub CommandBuffer: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATECONTEXTVIRTUAL {
    pub hDevice: u32,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: D3DDDI_CREATECONTEXTFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub ClientHint: D3DKMT_CLIENTHINT,
    pub hContext: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEDCFROMMEMORY {
    pub pMemory: *mut core::ffi::c_void,
    pub Format: D3DDDIFORMAT,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub hDeviceDc: super::super::super::Win32::Graphics::Gdi::HDC,
    pub pColorTable: *mut super::super::super::Win32::Graphics::Gdi::PALETTEENTRY,
    pub hDc: super::super::super::Win32::Graphics::Gdi::HDC,
    pub hBitmap: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEDEVICE {
    pub Anonymous: D3DKMT_CREATEDEVICE_0,
    pub Flags: D3DKMT_CREATEDEVICEFLAGS,
    pub hDevice: u32,
    pub pCommandBuffer: *mut core::ffi::c_void,
    pub CommandBufferSize: u32,
    pub pAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    pub AllocationListSize: u32,
    pub pPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    pub PatchLocationListSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEDEVICE_0 {
    pub hAdapter: u32,
    pub pAdapter: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEDEVICEFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEHWCONTEXT {
    pub hDevice: u32,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: D3DDDI_CREATEHWCONTEXTFLAGS,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub hHwContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEHWQUEUE {
    pub hHwContext: u32,
    pub Flags: D3DDDI_CREATEHWQUEUEFLAGS,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub hHwQueue: u32,
    pub hHwQueueProgressFence: u32,
    pub HwQueueProgressFenceCPUVirtualAddress: *mut core::ffi::c_void,
    pub HwQueueProgressFenceGPUVirtualAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX {
    pub InitialValue: u64,
    pub hSharedHandle: u32,
    pub hKeyedMutex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX2 {
    pub InitialValue: u64,
    pub hSharedHandle: u32,
    pub hKeyedMutex: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub Flags: D3DKMT_CREATEKEYEDMUTEX2_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX2_FLAGS {
    pub Anonymous: D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0 {
    pub Anonymous: D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATENATIVEFENCE {
    pub hDevice: u32,
    pub Info: D3DDDI_CREATENATIVEFENCEINFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEOVERLAY {
    pub VidPnSourceId: u32,
    pub hDevice: u32,
    pub OverlayInfo: D3DDDI_KERNELOVERLAYINFO,
    pub hOverlay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEPAGINGQUEUE {
    pub hDevice: u32,
    pub Priority: D3DDDI_PAGINGQUEUE_PRIORITY,
    pub hPagingQueue: u32,
    pub hSyncObject: u32,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub PhysicalAdapterIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEPROTECTEDSESSION {
    pub hDevice: u32,
    pub hSyncObject: u32,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub hHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESTANDARDALLOCATION {
    pub Type: D3DKMT_STANDARDALLOCATIONTYPE,
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATION_0,
    pub Flags: D3DKMT_CREATESTANDARDALLOCATIONFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATESTANDARDALLOCATION_0 {
    pub ExistingHeapData: D3DKMT_STANDARDALLOCATION_EXISTINGHEAP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESTANDARDALLOCATIONFLAGS {
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0 {
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESYNCFILE {
    pub hDevice: u32,
    pub hMonitoredFence: u32,
    pub FenceValue: u64,
    pub hSyncFile: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESYNCHRONIZATIONOBJECT {
    pub hDevice: u32,
    pub Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO,
    pub hSyncObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESYNCHRONIZATIONOBJECT2 {
    pub hDevice: u32,
    pub Info: D3DDDI_SYNCHRONIZATIONOBJECTINFO2,
    pub hSyncObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_DOORBELL {
    pub hHwQueue: u32,
    pub hRingBuffer: u32,
    pub hRingBufferControl: u32,
    pub Flags: D3DKMT_CREATE_DOORBELL_FLAGS,
    pub PrivateDriverDataSize: u32,
    pub PrivateDriverData: *mut core::ffi::c_void,
    pub DoorbellCPUVirtualAddress: *mut core::ffi::c_void,
    pub DoorbellSecondaryCPUVirtualAddress: *mut core::ffi::c_void,
    pub DoorbellStatusCPUVirtualAddress: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_DOORBELL_FLAGS {
    pub Anonymous: D3DKMT_CREATE_DOORBELL_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATE_DOORBELL_FLAGS_0 {
    pub Anonymous: D3DKMT_CREATE_DOORBELL_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_DOORBELL_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_OUTPUTDUPL {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub KeyedMutexCount: u32,
    pub RequiredKeyedMutexCount: u32,
    pub KeyedMutexs: [D3DKMT_OUTPUTDUPL_KEYEDMUTEX; 3],
    pub Flags: D3DKMT_OUTPUTDUPLCREATIONFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CROSSADAPTERRESOURCE_SUPPORT {
    pub SupportTier: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CURRENTDISPLAYMODE {
    pub VidPnSourceId: u32,
    pub DisplayMode: D3DKMT_DISPLAYMODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEBUG_SNAPSHOT_ESCAPE {
    pub Length: u32,
    pub Buffer: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYALLOCATION {
    pub hDevice: u32,
    pub hResource: u32,
    pub phAllocationList: *const u32,
    pub AllocationCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYALLOCATION2 {
    pub hDevice: u32,
    pub hResource: u32,
    pub phAllocationList: *const u32,
    pub AllocationCount: u32,
    pub Flags: D3DDDICB_DESTROYALLOCATION2FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYCONTEXT {
    pub hContext: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYDCFROMMEMORY {
    pub hDc: super::super::super::Win32::Graphics::Gdi::HDC,
    pub hBitmap: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYDEVICE {
    pub hDevice: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYHWCONTEXT {
    pub hHwContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYHWQUEUE {
    pub hHwQueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYKEYEDMUTEX {
    pub hKeyedMutex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYOVERLAY {
    pub hDevice: u32,
    pub hOverlay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYPROTECTEDSESSION {
    pub hHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYSYNCHRONIZATIONOBJECT {
    pub hSyncObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROY_DOORBELL {
    pub hHwQueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROY_OUTPUTDUPL {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub bDestroyAllContexts: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICEPAGEFAULT_STATE {
    pub FaultedPrimitiveAPISequenceNumber: u64,
    pub FaultedPipelineStage: DXGK_RENDER_PIPELINE_STAGE,
    pub FaultedBindTableEntry: u32,
    pub PageFaultFlags: DXGK_PAGE_FAULT_FLAGS,
    pub FaultErrorCode: DXGK_FAULT_ERROR_CODE,
    pub FaultedVirtualAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICEPRESENT_QUEUE_STATE {
    pub VidPnSourceId: u32,
    pub bQueuedPresentLimitReached: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICEPRESENT_STATE {
    pub VidPnSourceId: u32,
    pub PresentStats: D3DKMT_PRESENT_STATS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICEPRESENT_STATE_DWM {
    pub VidPnSourceId: u32,
    pub PresentStatsDWM: D3DKMT_PRESENT_STATS_DWM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICERESET_STATE {
    pub Anonymous: D3DKMT_DEVICERESET_STATE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_DEVICERESET_STATE_0 {
    pub Anonymous: D3DKMT_DEVICERESET_STATE_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICERESET_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICE_ESCAPE {
    pub Type: D3DKMT_DEVICEESCAPE_TYPE,
    pub Anonymous: D3DKMT_DEVICE_ESCAPE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_DEVICE_ESCAPE_0 {
    pub VidPnFromAllocation: D3DKMT_DEVICE_ESCAPE_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICE_ESCAPE_0_0 {
    pub hPrimaryAllocation: u32,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICE_IDS {
    pub VendorID: u32,
    pub DeviceID: u32,
    pub SubVendorID: u32,
    pub SubSystemID: u32,
    pub RevisionID: u32,
    pub BusType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DIRECTFLIP_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DIRTYREGIONS {
    pub NumRects: u32,
    pub Rects: [super::super::super::Win32::Foundation::RECT; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAYMODE {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3DDDIFORMAT,
    pub IntegerRefreshRate: u32,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub ScanLineOrdering: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
    pub DisplayOrientation: D3DDDI_ROTATION,
    pub DisplayFixedOutput: u32,
    pub Flags: D3DKMDT_DISPLAYMODE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAYMODELIST {
    pub VidPnSourceId: u32,
    pub ModeCount: u32,
    pub pModeList: [D3DKMT_DISPLAYMODE; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAY_CAPS {
    pub Anonymous: D3DKMT_DISPLAY_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_DISPLAY_CAPS_0 {
    pub Anonymous: D3DKMT_DISPLAY_CAPS_0_0,
    pub Value: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAY_CAPS_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAY_UMD_FILENAMEINFO {
    pub Version: KMT_DISPLAY_UMD_VERSION,
    pub UmdFileName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DLIST_DRIVER_NAME {
    pub DListFileName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DMM_ESCAPE {
    pub Type: D3DKMT_DMMESCAPETYPE,
    pub ProvidedBufferSize: usize,
    pub MinRequiredBufferSize: usize,
    pub Data: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DOD_SET_DIRTYRECT_MODE {
    pub bForceFullScreenDirty: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DRIVERCAPS_EXT {
    pub Anonymous: D3DKMT_DRIVERCAPS_EXT_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_DRIVERCAPS_EXT_0 {
    pub Anonymous: D3DKMT_DRIVERCAPS_EXT_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_DRIVERCAPS_EXT_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DRIVER_DESCRIPTION {
    pub DriverDescription: [u16; 4096],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS {
    pub NumAdapters: u32,
    pub Adapters: [D3DKMT_ADAPTERINFO; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS2 {
    pub NumAdapters: u32,
    pub pAdapters: *mut D3DKMT_ADAPTERINFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS3 {
    pub Filter: D3DKMT_ENUMADAPTERS_FILTER,
    pub NumAdapters: u32,
    pub pAdapters: *mut D3DKMT_ADAPTERINFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ENUMADAPTERS_FILTER {
    pub Anonymous: D3DKMT_ENUMADAPTERS_FILTER_0,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS_FILTER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ESCAPE {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub Type: D3DKMT_ESCAPETYPE,
    pub Flags: D3DDDI_ESCAPEFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub hContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE {
    pub Type: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE,
    pub VidPnSourceId: u32,
    pub ProcessBoostEligible: super::super::super::Win32::Foundation::BOOLEAN,
    pub VSyncMultiplier: u32,
    pub BaseDesktopDuration: u32,
    pub Reserved: [u8; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICT {
    pub hDevice: u32,
    pub NumAllocations: u32,
    pub AllocationList: *const u32,
    pub Flags: D3DDDI_EVICT_FLAGS,
    pub NumBytesToTrim: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICTION_CRITERIA {
    pub MinimumSize: u64,
    pub MaximumSize: u64,
    pub Anonymous: D3DKMT_EVICTION_CRITERIA_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICTION_CRITERIA_0 {
    pub Anonymous: D3DKMT_EVICTION_CRITERIA_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_EVICTION_CRITERIA_0_0 {
    pub Flags: D3DKMT_EVICTION_CRITERIA_0_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICTION_CRITERIA_0_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FENCE_PRESENTHISTORYTOKEN {
    pub Key: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPINFOFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMANAGER_AUXILIARYPRESENTINFO {
    pub auxiliaryPresentInfo: D3DKMT_AUXILIARYPRESENTINFO,
    pub flipManagerTracingId: u32,
    pub customDurationChanged: super::super::super::Win32::Foundation::BOOL,
    pub FlipAdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
    pub independentFlipStage: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE,
    pub FlipCompletedQpc: u64,
    pub HwPresentDurationQpc: u32,
    pub WasCanceled: super::super::super::Win32::Foundation::BOOL,
    pub ConvertedToNonIFlip: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN {
    pub hPrivateData: u64,
    pub PresentAtQpc: u64,
    pub Flags: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0 {
    pub Anonymous: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN {
    pub FenceValue: u64,
    pub hLogicalSurface: u64,
    pub dxgContext: usize,
    pub VidPnSourceId: u32,
    pub SwapChainIndex: u32,
    pub PresentLimitSemaphoreId: u64,
    pub FlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS,
    pub hCompSurf: i64,
    pub compSurfLuid: super::super::super::Win32::Foundation::LUID,
    pub confirmationCookie: u64,
    pub CompositionSyncKey: u64,
    pub RemainingTokens: u32,
    pub ScrollRect: super::super::super::Win32::Foundation::RECT,
    pub ScrollOffset: super::super::super::Win32::Foundation::POINT,
    pub PresentCount: u32,
    pub RevealColor: [f32; 4],
    pub Rotation: D3DDDI_ROTATION,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0,
    pub InkCookie: u32,
    pub SourceRect: super::super::super::Win32::Foundation::RECT,
    pub DestWidth: u32,
    pub DestHeight: u32,
    pub TargetRect: super::super::super::Win32::Foundation::RECT,
    pub Transform: [f32; 6],
    pub CustomDuration: u32,
    pub CustomDurationFlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub PlaneIndex: u32,
    pub ColorSpace: D3DDDI_COLOR_SPACE_TYPE,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0 {
    pub ScatterBlts: D3DKMT_SCATTERBLTS,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0 {
    pub hSyncObject: super::super::super::Win32::Foundation::HANDLE,
    pub HDRMetaDataType: D3DDDI_HDR_METADATA_TYPE,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0_0 {
    pub HDRMetaDataHDR10: D3DDDI_HDR_METADATA_HDR10,
    pub HDRMetaDataHDR10Plus: D3DDDI_HDR_METADATA_HDR10PLUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS {
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0 {
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPOVERLAY {
    pub hDevice: u32,
    pub hOverlay: u32,
    pub hSource: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPQUEUEINFO {
    pub MaxHardwareFlipQueueLength: u32,
    pub MaxSoftwareFlipQueueLength: u32,
    pub FlipFlags: D3DKMT_FLIPINFOFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLUSHHEAPTRANSITIONS {
    pub hAdapter: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FREEGPUVIRTUALADDRESS {
    pub hAdapter: u32,
    pub BaseAddress: u64,
    pub Size: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GDIMODEL_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
    pub hPhysicalSurface: u64,
    pub ScrollRect: super::super::super::Win32::Foundation::RECT,
    pub ScrollOffset: super::super::super::Win32::Foundation::POINT,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GDIMODEL_SYSMEM_PRESENTHISTORYTOKEN {
    pub hlsurf: u64,
    pub dwDirtyFlags: u32,
    pub uiCookie: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETALLOCATIONPRIORITY {
    pub hDevice: u32,
    pub hResource: u32,
    pub phAllocationList: *const u32,
    pub AllocationCount: u32,
    pub pPriorities: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY {
    pub hContext: u32,
    pub Priority: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETCONTEXTSCHEDULINGPRIORITY {
    pub hContext: u32,
    pub Priority: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETDEVICESTATE {
    pub hDevice: u32,
    pub StateType: D3DKMT_DEVICESTATE_TYPE,
    pub Anonymous: D3DKMT_GETDEVICESTATE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_GETDEVICESTATE_0 {
    pub ExecutionState: D3DKMT_DEVICEEXECUTION_STATE,
    pub PresentState: D3DKMT_DEVICEPRESENT_STATE,
    pub ResetState: D3DKMT_DEVICERESET_STATE,
    pub PresentStateDWM: D3DKMT_DEVICEPRESENT_STATE_DWM,
    pub PageFaultState: D3DKMT_DEVICEPAGEFAULT_STATE,
    pub PresentQueueState: D3DKMT_DEVICEPRESENT_QUEUE_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETDISPLAYMODELIST {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub pModeList: *mut D3DKMT_DISPLAYMODE,
    pub ModeCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETMULTISAMPLEMETHODLIST {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub Width: u32,
    pub Height: u32,
    pub Format: D3DDDIFORMAT,
    pub pMethodList: *mut D3DKMT_MULTISAMPLEMETHOD,
    pub MethodCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETOVERLAYSTATE {
    pub hDevice: u32,
    pub hOverlay: u32,
    pub OverlayEnabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETPRESENTHISTORY {
    pub hAdapter: u32,
    pub ProvidedSize: u32,
    pub WrittenSize: u32,
    pub pTokens: *mut D3DKMT_PRESENTHISTORYTOKEN,
    pub NumTokens: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub Support: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETRUNTIMEDATA {
    pub hAdapter: u32,
    pub hGlobalShare: u32,
    pub pRuntimeData: *mut core::ffi::c_void,
    pub RuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETSCANLINE {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub InVerticalBlank: super::super::super::Win32::Foundation::BOOLEAN,
    pub ScanLine: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETSHAREDPRIMARYHANDLE {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub hSharedPrimary: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETSHAREDRESOURCEADAPTERLUID {
    pub hGlobalShare: u32,
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETVERTICALBLANKEVENT {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub VidPnSourceId: u32,
    pub phEvent: *mut isize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_DEVICE_VIDPN_OWNERSHIP_INFO {
    pub hDevice: u32,
    pub bFailedDwmAcquireVidPn: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_GPUMMU_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub GpuMmuCaps: DXGK_ESCAPE_GPUMMUCAPS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub MaxPlanes: u32,
    pub MaxRGBPlanes: u32,
    pub MaxYUVPlanes: u32,
    pub OverlayCaps: D3DKMT_MULTIPLANE_OVERLAY_CAPS,
    pub MaxStretchFactor: f32,
    pub MaxShrinkFactor: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_POST_COMPOSITION_CAPS {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub MaxStretchFactor: f32,
    pub MaxShrinkFactor: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_PTE {
    pub PhysicalAdapterIndex: u32,
    pub PageTableLevel: u32,
    pub PageTableIndex: [u32; 6],
    pub b64KBPte: super::super::super::Win32::Foundation::BOOLEAN,
    pub NumPtes: u32,
    pub Pte: [DXGK_PTE; 64],
    pub NumValidEntries: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_SEGMENT_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub NumSegments: u32,
    pub SegmentCaps: [D3DKMT_SEGMENT_CAPS; 32],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GPUMMU_CAPS {
    pub Flags: D3DKMT_GPUMMU_CAPS_0,
    pub VirtualAddressBitCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_GPUMMU_CAPS_0 {
    pub Anonymous: D3DKMT_GPUMMU_CAPS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GPUMMU_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GPUVERSION {
    pub PhysicalAdapterIndex: u32,
    pub BiosVersion: [u16; 32],
    pub GpuArchitecture: [u16; 32],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_HISTORY_BUFFER_STATUS {
    pub Enabled: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_HWDRM_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_HYBRID_DLIST_DLL_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_HYBRID_LIST {
    pub State: D3DKMT_GPU_PREFERENCE_QUERY_STATE,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub bUserPreferenceQuery: super::super::super::Win32::Foundation::BOOL,
    pub QueryType: D3DKMT_GPU_PREFERENCE_QUERY_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_INDEPENDENTFLIP_SECONDARY_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_INDEPENDENTFLIP_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_INVALIDATEACTIVEVIDPN {
    pub hAdapter: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_INVALIDATECACHE {
    pub hDevice: u32,
    pub hAllocation: u32,
    pub Offset: usize,
    pub Length: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ISBADDRIVERFORHWPROTECTIONDISABLED {
    pub Disabled: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_KMD_DRIVER_VERSION {
    pub DriverVersion: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_LOCK {
    pub hDevice: u32,
    pub hAllocation: u32,
    pub PrivateDriverData: u32,
    pub NumPages: u32,
    pub pPages: *const u32,
    pub pData: *mut core::ffi::c_void,
    pub Flags: D3DDDICB_LOCKFLAGS,
    pub GpuVirtualAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_LOCK2 {
    pub hDevice: u32,
    pub hAllocation: u32,
    pub Flags: D3DDDICB_LOCK2FLAGS,
    pub pData: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MARKDEVICEASERROR {
    pub hDevice: u32,
    pub Reason: D3DKMT_DEVICE_ERROR_REASON,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACASTCOMPANIONDRIVERNAME {
    pub MiracastCompanionDriverName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACAST_CHUNK_DATA {
    pub ChunkInfo: DXGK_MIRACAST_CHUNK_INFO,
    pub PrivateDriverDataSize: u32,
    pub PrivateDriverData: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACAST_DISPLAY_DEVICE_CAPS {
    pub HdcpSupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub DefaultControlPort: u32,
    pub UsesIhvSolution: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACAST_DISPLAY_DEVICE_STATUS {
    pub State: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACAST_DISPLAY_STOP_SESSIONS {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub TargetId: u32,
    pub StopReason: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MOVE_RECT {
    pub SourcePoint: super::super::super::Win32::Foundation::POINT,
    pub DestRect: super::super::super::Win32::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MPO3DDI_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MPOKERNELCAPS_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANEOVERLAY_DECODE_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANEOVERLAY_HUD_SUPPORT {
    pub VidPnSourceId: u32,
    pub Update: super::super::super::Win32::Foundation::BOOL,
    pub KernelSupported: super::super::super::Win32::Foundation::BOOL,
    pub HudSupported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANEOVERLAY_SECONDARY_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANEOVERLAY_STRETCH_SUPPORT {
    pub VidPnSourceId: u32,
    pub Update: super::super::super::Win32::Foundation::BOOL,
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANEOVERLAY_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY {
    pub LayerIndex: u32,
    pub Enabled: super::super::super::Win32::Foundation::BOOL,
    pub hAllocation: u32,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY2 {
    pub LayerIndex: u32,
    pub Enabled: super::super::super::Win32::Foundation::BOOL,
    pub hAllocation: u32,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY3 {
    pub LayerIndex: u32,
    pub InputFlags: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS,
    pub FlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub MaxImmediateFlipLine: u32,
    pub AllocationCount: u32,
    pub pAllocationList: *mut u32,
    pub DriverPrivateDataSize: u32,
    pub pDriverPrivateData: *mut core::ffi::c_void,
    pub pPlaneAttributes: *const D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3,
    pub hFlipToFence: u32,
    pub hFlipAwayFence: u32,
    pub FlipToFenceValue: u64,
    pub FlipAwayFenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES {
    pub Flags: u32,
    pub SrcRect: super::super::super::Win32::Foundation::RECT,
    pub DstRect: super::super::super::Win32::Foundation::RECT,
    pub ClipRect: super::super::super::Win32::Foundation::RECT,
    pub Rotation: D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::super::super::Win32::Foundation::RECT,
    pub VideoFrameFormat: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT,
    pub YCbCrFlags: u32,
    pub StereoFormat: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT,
    pub StereoLeftViewFrame0: super::super::super::Win32::Foundation::BOOL,
    pub StereoBaseViewFrame0: super::super::super::Win32::Foundation::BOOL,
    pub StereoFlipMode: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2 {
    pub Flags: u32,
    pub SrcRect: super::super::super::Win32::Foundation::RECT,
    pub DstRect: super::super::super::Win32::Foundation::RECT,
    pub ClipRect: super::super::super::Win32::Foundation::RECT,
    pub Rotation: D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::super::super::Win32::Foundation::RECT,
    pub VideoFrameFormat: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT,
    pub ColorSpace: D3DDDI_COLOR_SPACE_TYPE,
    pub StereoFormat: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT,
    pub StereoLeftViewFrame0: super::super::super::Win32::Foundation::BOOL,
    pub StereoBaseViewFrame0: super::super::super::Win32::Foundation::BOOL,
    pub StereoFlipMode: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
    pub Reserved1: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3 {
    pub Flags: u32,
    pub SrcRect: super::super::super::Win32::Foundation::RECT,
    pub DstRect: super::super::super::Win32::Foundation::RECT,
    pub ClipRect: super::super::super::Win32::Foundation::RECT,
    pub Rotation: D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::super::super::Win32::Foundation::RECT,
    pub ColorSpace: D3DDDI_COLOR_SPACE_TYPE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
    pub SDRWhiteLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_CAPS {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_CAPS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_MULTIPLANE_OVERLAY_CAPS_0 {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_CAPS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION {
    pub Flags: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS,
    pub SrcRect: super::super::super::Win32::Foundation::RECT,
    pub DstRect: super::super::super::Win32::Foundation::RECT,
    pub Rotation: D3DDDI_ROTATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0 {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_WITH_SOURCE {
    pub VidPnSourceId: u32,
    pub PostComposition: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTISAMPLEMETHOD {
    pub NumSamples: u32,
    pub NumQualityLevels: u32,
    pub Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_NODEMETADATA {
    pub NodeOrdinalAndAdapterIndex: u32,
    pub NodeData: DXGK_NODEMETADATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_NODE_PERFDATA {
    pub NodeOrdinal: u32,
    pub PhysicalAdapterIndex: u32,
    pub Frequency: u64,
    pub MaxFrequency: u64,
    pub MaxFrequencyOC: u64,
    pub Voltage: u32,
    pub VoltageMax: u32,
    pub VoltageMaxOC: u32,
    pub MaxTransitionLatency: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION {
    pub hHwQueue: u32,
    pub Flags: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS {
    pub Anonymous: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0 {
    pub Anonymous: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OFFERALLOCATIONS {
    pub hDevice: u32,
    pub pResources: *mut u32,
    pub HandleList: *const u32,
    pub NumAllocations: u32,
    pub Priority: D3DKMT_OFFER_PRIORITY,
    pub Flags: D3DKMT_OFFER_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OFFER_FLAGS {
    pub Anonymous: D3DKMT_OFFER_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OFFER_FLAGS_0 {
    pub Anonymous: D3DKMT_OFFER_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OFFER_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMDEVICENAME {
    pub pDeviceName: windows_sys::core::PCWSTR,
    pub hAdapter: u32,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME {
    pub DeviceName: [u16; 32],
    pub hAdapter: u32,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMHDC {
    pub hDc: super::super::super::Win32::Graphics::Gdi::HDC,
    pub hAdapter: u32,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMLUID {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub hAdapter: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENGLINFO {
    pub UmdOpenGlIcdFileName: [u16; 260],
    pub Version: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENKEYEDMUTEX {
    pub hSharedHandle: u32,
    pub hKeyedMutex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENKEYEDMUTEX2 {
    pub hSharedHandle: u32,
    pub hKeyedMutex: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub hKeyedMutex: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENNATIVEFENCEFROMNTHANDLE {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub hDevice: u32,
    pub EngineAffinity: u32,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub hSyncObject: u32,
    pub NativeFenceMapping: D3DDDI_NATIVEFENCEMAPPING,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENNTHANDLEFROMNAME {
    pub dwDesiredAccess: u32,
    pub pObjAttrib: *mut super::super::Foundation::OBJECT_ATTRIBUTES,
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub hHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENRESOURCE {
    pub hDevice: u32,
    pub hGlobalShare: u32,
    pub NumAllocations: u32,
    pub Anonymous: D3DKMT_OPENRESOURCE_0,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub pResourcePrivateDriverData: *mut core::ffi::c_void,
    pub ResourcePrivateDriverDataSize: u32,
    pub pTotalPrivateDriverDataBuffer: *mut core::ffi::c_void,
    pub TotalPrivateDriverDataBufferSize: u32,
    pub hResource: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OPENRESOURCE_0 {
    pub pOpenAllocationInfo: *mut D3DDDI_OPENALLOCATIONINFO,
    pub pOpenAllocationInfo2: *mut D3DDDI_OPENALLOCATIONINFO2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENRESOURCEFROMNTHANDLE {
    pub hDevice: u32,
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub NumAllocations: u32,
    pub pOpenAllocationInfo2: *mut D3DDDI_OPENALLOCATIONINFO2,
    pub PrivateRuntimeDataSize: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub ResourcePrivateDriverDataSize: u32,
    pub pResourcePrivateDriverData: *mut core::ffi::c_void,
    pub TotalPrivateDriverDataBufferSize: u32,
    pub pTotalPrivateDriverDataBuffer: *mut core::ffi::c_void,
    pub hResource: u32,
    pub hKeyedMutex: u32,
    pub pKeyedMutexPrivateRuntimeData: *mut core::ffi::c_void,
    pub KeyedMutexPrivateRuntimeDataSize: u32,
    pub hSyncObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCHRONIZATIONOBJECT {
    pub hSharedHandle: u32,
    pub hSyncObject: u32,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub hSyncObject: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2 {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub hDevice: u32,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub hSyncObject: u32,
    pub Anonymous: D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0 {
    pub MonitoredFence: D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0_0,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0_0 {
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: u64,
    pub EngineAffinity: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME {
    pub dwDesiredAccess: u32,
    pub pObjAttrib: *mut super::super::Foundation::OBJECT_ATTRIBUTES,
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::super::Win32::Foundation::POINT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLCONTEXTSCOUNT {
    pub VidPnSourceId: u32,
    pub OutputDuplicationCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLCREATIONFLAGS {
    pub Anonymous: D3DKMT_OUTPUTDUPLCREATIONFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OUTPUTDUPLCREATIONFLAGS_0 {
    pub Anonymous: D3DKMT_OUTPUTDUPLCREATIONFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLCREATIONFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENT {
    pub hContext: u32,
    pub hSource: u32,
    pub VidPnSourceId: u32,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub PresentRegions: D3DKMT_PRESENT_RGNS,
    pub Flags: D3DKMT_OUTPUTDUPLPRESENTFLAGS,
    pub hIndirectContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENTFLAGS {
    pub Anonymous: D3DKMT_OUTPUTDUPLPRESENTFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OUTPUTDUPLPRESENTFLAGS_0 {
    pub Anonymous: D3DKMT_OUTPUTDUPLPRESENTFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE {
    pub hSource: u32,
    pub VidPnSourceId: u32,
    pub BroadcastHwQueueCount: u32,
    pub hHwQueues: *mut u32,
    pub PresentRegions: D3DKMT_PRESENT_RGNS,
    pub Flags: D3DKMT_OUTPUTDUPLPRESENTFLAGS,
    pub hIndirectHwQueue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_FRAMEINFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: super::super::super::Win32::Foundation::BOOL,
    pub ProtectedContentMaskedOut: super::super::super::Win32::Foundation::BOOL,
    pub PointerPosition: D3DKMT_OUTPUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_GET_FRAMEINFO {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub FrameInfo: D3DKMT_OUTPUTDUPL_FRAMEINFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub BufferSizeSupplied: u32,
    pub pShapeBuffer: *mut core::ffi::c_void,
    pub BufferSizeRequired: u32,
    pub ShapeInfo: D3DKMT_OUTDUPL_POINTER_SHAPE_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_KEYEDMUTEX {
    pub hSharedSurfaceNt: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_METADATA {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub Type: D3DKMT_OUTPUTDUPL_METADATATYPE,
    pub BufferSizeSupplied: u32,
    pub pBuffer: *mut core::ffi::c_void,
    pub BufferSizeRequired: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_POINTER_POSITION {
    pub Position: super::super::super::Win32::Foundation::POINT,
    pub Visible: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_RELEASE_FRAME {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub NextKeyMutexIdx: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_SNAPSHOT {
    pub Size: u32,
    pub SessionProcessCount: u32,
    pub SessionActiveConnectionsCount: u32,
    pub NumVidPnSources: u32,
    pub NumOutputDuplContexts: u32,
    pub Padding: u32,
    pub OutputDuplDebugInfos: [OUTPUTDUPL_CONTEXT_DEBUG_INFO; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PAGE_TABLE_LEVEL_DESC {
    pub IndexBitCount: u32,
    pub IndexMask: u64,
    pub IndexShift: u64,
    pub LowerLevelsMask: u64,
    pub EntryCoverageInPages: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PANELFITTER_SUPPORT {
    pub Supported: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PARAVIRTUALIZATION {
    pub SecureContainer: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PHYSICAL_ADAPTER_COUNT {
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PINDIRECTFLIPRESOURCES {
    pub hDevice: u32,
    pub ResourceCount: u32,
    pub pResourceList: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0 {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0 {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_POLLDISPLAYCHILDREN {
    pub hAdapter: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT {
    pub Anonymous1: D3DKMT_PRESENT_0,
    pub hWindow: super::super::super::Win32::Foundation::HWND,
    pub VidPnSourceId: u32,
    pub hSource: u32,
    pub hDestination: u32,
    pub Color: u32,
    pub DstRect: super::super::super::Win32::Foundation::RECT,
    pub SrcRect: super::super::super::Win32::Foundation::RECT,
    pub SubRectCnt: u32,
    pub pSrcSubRects: *const super::super::super::Win32::Foundation::RECT,
    pub PresentCount: u32,
    pub FlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub PresentLimitSemaphore: super::super::super::Win32::Foundation::HANDLE,
    pub PresentHistoryToken: D3DKMT_PRESENTHISTORYTOKEN,
    pub pPresentRegions: *mut D3DKMT_PRESENT_RGNS,
    pub Anonymous2: D3DKMT_PRESENT_1,
    pub Duration: u32,
    pub BroadcastSrcAllocation: *mut u32,
    pub BroadcastDstAllocation: *mut u32,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub bOptimizeForComposition: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_0 {
    pub hDevice: u32,
    pub hContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_1 {
    pub hAdapter: u32,
    pub hIndirectContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENTFLAGS {
    pub Anonymous: D3DKMT_PRESENTFLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENTFLAGS_0 {
    pub Anonymous: D3DKMT_PRESENTFLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENTHISTORYTOKEN {
    pub Model: D3DKMT_PRESENT_MODEL,
    pub TokenSize: u32,
    pub CompositionBindingId: u64,
    pub Token: D3DKMT_PRESENTHISTORYTOKEN_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENTHISTORYTOKEN_0 {
    pub Flip: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN,
    pub Blt: D3DKMT_BLTMODEL_PRESENTHISTORYTOKEN,
    pub VistaBlt: u64,
    pub Gdi: D3DKMT_GDIMODEL_PRESENTHISTORYTOKEN,
    pub Fence: D3DKMT_FENCE_PRESENTHISTORYTOKEN,
    pub GdiSysMem: D3DKMT_GDIMODEL_SYSMEM_PRESENTHISTORYTOKEN,
    pub Composition: D3DKMT_COMPOSITION_PRESENTHISTORYTOKEN,
    pub FlipManager: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN,
    pub SurfaceComplete: D3DKMT_SURFACECOMPLETE_PRESENTHISTORYTOKEN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_0,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub VidPnSourceId: u32,
    pub PresentCount: u32,
    pub FlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub PresentPlaneCount: u32,
    pub pPresentPlanes: *mut D3DKMT_MULTIPLANE_OVERLAY,
    pub Duration: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY_0 {
    pub hDevice: u32,
    pub hContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY2 {
    pub hAdapter: u32,
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY2_0,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub VidPnSourceId: u32,
    pub PresentCount: u32,
    pub FlipInterval: D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub PresentPlaneCount: u32,
    pub pPresentPlanes: *mut D3DKMT_MULTIPLANE_OVERLAY2,
    pub Duration: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY2_0 {
    pub hDevice: u32,
    pub hContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY3 {
    pub hAdapter: u32,
    pub ContextCount: u32,
    pub pContextList: *mut u32,
    pub VidPnSourceId: u32,
    pub PresentCount: u32,
    pub Flags: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS,
    pub PresentPlaneCount: u32,
    pub ppPresentPlanes: *mut *mut D3DKMT_MULTIPLANE_OVERLAY3,
    pub pPostComposition: *mut D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION,
    pub Duration: u32,
    pub HDRMetaDataType: D3DDDI_HDR_METADATA_TYPE,
    pub HDRMetaDataSize: u32,
    pub pHDRMetaData: *const core::ffi::c_void,
    pub BoostRefreshRateMultiplier: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0 {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_REDIRECTED {
    pub hSyncObj: u32,
    pub hDevice: u32,
    pub WaitedFenceValue: u64,
    pub PresentHistoryToken: D3DKMT_PRESENTHISTORYTOKEN,
    pub Flags: D3DKMT_PRESENT_REDIRECTED_FLAGS,
    pub hSource: u32,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_REDIRECTED_FLAGS {
    pub Anonymous: D3DKMT_PRESENT_REDIRECTED_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_REDIRECTED_FLAGS_0 {
    pub Anonymous: D3DKMT_PRESENT_REDIRECTED_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_REDIRECTED_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_RGNS {
    pub DirtyRectCount: u32,
    pub pDirtyRects: *const super::super::super::Win32::Foundation::RECT,
    pub MoveRectCount: u32,
    pub pMoveRects: *const D3DKMT_MOVE_RECT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_STATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_STATS_DWM {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub PresentQPCTime: i64,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub CustomPresentDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_STATS_DWM2 {
    pub cbSize: u32,
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub PresentQPCTime: i64,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub CustomPresentDuration: u32,
    pub VirtualSyncRefreshCount: u32,
    pub VirtualSyncQPCTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PROCESS_VERIFIER_OPTION {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub Type: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE,
    pub Mode: D3DKMT_VERIFIER_OPTION_MODE,
    pub Data: D3DKMT_PROCESS_VERIFIER_OPTION_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PROCESS_VERIFIER_OPTION_DATA {
    pub VidMmFlags: D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS,
    pub VidMmRestrictBudget: D3DKMT_PROCESS_VERIFIER_VIDMM_RESTRICT_BUDGET,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS {
    pub Anonymous: D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PROCESS_VERIFIER_VIDMM_RESTRICT_BUDGET {
    pub LocalBudget: u64,
    pub NonLocalBudget: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYADAPTERINFO {
    pub hAdapter: u32,
    pub Type: KMTQUERYADAPTERINFOTYPE,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYALLOCATIONRESIDENCY {
    pub hDevice: u32,
    pub hResource: u32,
    pub phAllocationList: *const u32,
    pub AllocationCount: u32,
    pub pResidencyStatus: *mut D3DKMT_ALLOCATIONRESIDENCYSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYCLOCKCALIBRATION {
    pub hAdapter: u32,
    pub NodeOrdinal: u32,
    pub PhysicalAdapterIndex: u32,
    pub ClockData: DXGK_GPUCLOCKDATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYFSEBLOCK {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub Flags: D3DKMT_QUERYFSEBLOCKFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYFSEBLOCKFLAGS {
    pub Anonymous: D3DKMT_QUERYFSEBLOCKFLAGS_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYFSEBLOCKFLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYPROCESSOFFERINFO {
    pub cbSize: u32,
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub DecommitUniqueness: u64,
    pub DecommittableBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE {
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYPROTECTEDSESSIONSTATUS {
    pub hHandle: u32,
    pub Status: D3DKMT_PROTECTED_SESSION_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME {
    pub DeviceName: [u16; 32],
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYRESOURCEINFO {
    pub hDevice: u32,
    pub hGlobalShare: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub TotalPrivateDriverDataSize: u32,
    pub ResourcePrivateDriverDataSize: u32,
    pub NumAllocations: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE {
    pub hDevice: u32,
    pub hNtHandle: super::super::super::Win32::Foundation::HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub TotalPrivateDriverDataSize: u32,
    pub ResourcePrivateDriverDataSize: u32,
    pub NumAllocations: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS {
    pub Type: D3DKMT_QUERYSTATISTICS_TYPE,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub QueryResult: D3DKMT_QUERYSTATISTICS_RESULT,
    pub Anonymous: D3DKMT_QUERYSTATISTICS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_0 {
    pub QuerySegment: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT,
    pub QueryProcessSegment: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT,
    pub QueryProcessSegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub QueryNode: D3DKMT_QUERYSTATISTICS_QUERY_NODE,
    pub QueryProcessNode: D3DKMT_QUERYSTATISTICS_QUERY_NODE,
    pub QueryVidPnSource: D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE,
    pub QueryProcessVidPnSource: D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE,
    pub QueryPhysAdapter: D3DKMT_QUERYSTATISTICS_QUERY_PHYSICAL_ADAPTER,
    pub QueryAdapter2: D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2,
    pub QuerySegment2: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2,
    pub QueryProcessAdapter2: D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2,
    pub QueryProcessSegment2: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2,
    pub QueryProcessSegmentGroup2: D3DKMT_QUERYSTATISTICS_QUERY_PROCESS_SEGMENT_GROUP2,
    pub QuerySegmentUsage: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_USAGE,
    pub QuerySegmentGroupUsage: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_GROUP_USAGE,
    pub QueryNode2: D3DKMT_QUERYSTATISTICS_QUERY_NODE2,
    pub QueryProcessNode2: D3DKMT_QUERYSTATISTICS_QUERY_NODE2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION {
    pub NbSegments: u32,
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub VSyncEnabled: u32,
    pub TdrDetectedCount: u32,
    pub ZeroLengthDmaBuffers: i64,
    pub RestartedPeriod: u64,
    pub ReferenceDmaBuffer: D3DKMT_QUERYSTATSTICS_REFERENCE_DMA_BUFFER,
    pub Renaming: D3DKMT_QUERYSTATSTICS_RENAMING,
    pub Preparation: D3DKMT_QUERYSTATSTICS_PREPRATION,
    pub PagingFault: D3DKMT_QUERYSTATSTICS_PAGING_FAULT,
    pub PagingTransfer: D3DKMT_QUERYSTATSTICS_PAGING_TRANSFER,
    pub SwizzlingRange: D3DKMT_QUERYSTATSTICS_SWIZZLING_RANGE,
    pub Locks: D3DKMT_QUERYSTATSTICS_LOCKS,
    pub Allocations: D3DKMT_QUERYSTATSTICS_ALLOCATIONS,
    pub Terminations: D3DKMT_QUERYSTATSTICS_TERMINATIONS,
    pub Flags: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS,
    pub Reserved: [u64; 7],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS {
    pub Anonymous: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0 {
    pub Anonymous: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0_0,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_COMMITMENT_DATA {
    pub TotalBytesEvictedFromProcess: u64,
    pub BytesBySegmentPreference: [u64; 5],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_COUNTER {
    pub Count: u32,
    pub Bytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_DMA_BUFFER {
    pub Size: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationListBytes: u32,
    pub PatchLocationListBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_INFORMATION {
    pub PacketSubmited: u32,
    pub PacketCompleted: u32,
    pub PacketPreempted: u32,
    pub PacketFaulted: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_MEMORY {
    pub TotalBytesEvicted: u64,
    pub AllocsCommitted: u32,
    pub AllocsResident: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_MEMORY_USAGE {
    pub AllocatedBytes: u64,
    pub FreeBytes: u64,
    pub ZeroBytes: u64,
    pub ModifiedBytes: u64,
    pub StandbyBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_NODE_INFORMATION {
    pub GlobalInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub SystemInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub NodePerfData: D3DKMT_NODE_PERFDATA,
    pub Reserved: [u32; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PACKET_INFORMATION {
    pub QueuePacket: [D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_INFORMATION; 8],
    pub DmaPacket: [D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_INFORMATION; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER_INFORMATION {
    pub AdapterPerfData: D3DKMT_ADAPTER_PERFDATA,
    pub AdapterPerfDataCaps: D3DKMT_ADAPTER_PERFDATACAPS,
    pub GpuVersion: D3DKMT_GPUVERSION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_POLICY {
    pub PreferApertureForRead: [u64; 5],
    pub PreferAperture: [u64; 5],
    pub MemResetOnPaging: u64,
    pub RemovePagesFromWorkingSetOnPaging: u64,
    pub MigrationEnabled: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PREEMPTION_INFORMATION {
    pub PreemptionCounter: [u32; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER_INFORMATION {
    pub NbSegments: u32,
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub VirtualMemoryUsage: u32,
    pub DmaBuffer: D3DKMT_QUERYSTATISTICS_DMA_BUFFER,
    pub CommitmentData: D3DKMT_QUERYSTATISTICS_COMMITMENT_DATA,
    pub _Policy: D3DKMT_QUERYSTATISTICS_POLICY,
    pub ProcessInterferenceCounters: D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_COUNTERS,
    pub ClientHint: D3DKMT_CLIENTHINT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_INFORMATION {
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub SystemMemory: D3DKMT_QUERYSTATISTICS_SYSTEM_MEMORY,
    pub Reserved: [u64; 7],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_COUNTERS {
    pub InterferenceCount: [u64; 9],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION {
    pub RunningTime: i64,
    pub ContextSwitch: u32,
    pub PreemptionStatistics: D3DKMT_QUERYSTATISTICS_PREEMPTION_INFORMATION,
    pub PacketStatistics: D3DKMT_QUERYSTATISTICS_PACKET_INFORMATION,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP_INFORMATION {
    pub Budget: u64,
    pub Requested: u64,
    pub Usage: u64,
    pub Demoted: [u64; 5],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_INFORMATION {
    pub BytesCommitted: u64,
    pub MaximumWorkingSet: u64,
    pub MinimumWorkingSet: u64,
    pub NbReferencedAllocationEvictedInPeriod: u32,
    pub Padding: u32,
    pub VideoMemory: D3DKMT_QUERYSTATISTICS_VIDEO_MEMORY,
    pub _Policy: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_POLICY,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_POLICY {
    pub UseMRU: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION {
    pub Frame: u32,
    pub CancelledFrame: u32,
    pub QueuedPresent: u32,
    pub Padding: u32,
    pub IsVSyncEnabled: u64,
    pub VSyncOnTotalTimeMs: u64,
    pub VSyncOffKeepPhaseTotalTimeMs: u64,
    pub VSyncOffNoPhaseTotalTimeMs: u64,
    pub Reserved: [u64; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2 {
    pub PhysicalAdapterIndex: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER_INFORMATION2 {
    pub PhysicalAdapterIndex: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_NODE {
    pub NodeId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_NODE2 {
    pub PhysicalAdapterIndex: u16,
    pub NodeOrdinal: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_PHYSICAL_ADAPTER {
    pub PhysicalAdapterIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_PROCESS_SEGMENT_GROUP2 {
    pub PhysicalAdapterIndex: u16,
    pub SegmentGroup: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT {
    pub SegmentId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2 {
    pub PhysicalAdapterIndex: u16,
    pub SegmentId: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_GROUP_USAGE {
    pub PhysicalAdapterIndex: u16,
    pub SegmentGroup: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_USAGE {
    pub PhysicalAdapterIndex: u16,
    pub SegmentId: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE {
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_INFORMATION {
    pub PacketSubmited: u32,
    pub PacketCompleted: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_RESULT {
    pub AdapterInformation: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION,
    pub PhysAdapterInformation: D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER_INFORMATION,
    pub SegmentInformation: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION,
    pub NodeInformation: D3DKMT_QUERYSTATISTICS_NODE_INFORMATION,
    pub VidPnSourceInformation: D3DKMT_QUERYSTATISTICS_VIDPNSOURCE_INFORMATION,
    pub ProcessInformation: D3DKMT_QUERYSTATISTICS_PROCESS_INFORMATION,
    pub ProcessAdapterInformation: D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER_INFORMATION,
    pub ProcessSegmentInformation: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_INFORMATION,
    pub ProcessNodeInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub ProcessVidPnSourceInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub ProcessSegmentGroupInformation: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP_INFORMATION,
    pub SegmentUsageInformation: D3DKMT_QUERYSTATISTICS_MEMORY_USAGE,
    pub SegmentGroupUsageInformation: D3DKMT_QUERYSTATISTICS_MEMORY_USAGE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION {
    pub CommitLimit: u64,
    pub BytesCommitted: u64,
    pub BytesResident: u64,
    pub Memory: D3DKMT_QUERYSTATISTICS_MEMORY,
    pub Aperture: u32,
    pub TotalBytesEvictedByPriority: [u64; 5],
    pub SystemMemoryEndAddress: u64,
    pub PowerFlags: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_0,
    pub SegmentProperties: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_1,
    pub Reserved: [u64; 5],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_SYSTEM_MEMORY {
    pub BytesAllocated: u64,
    pub BytesReserved: u64,
    pub SmallAllocationBlocks: u32,
    pub LargeAllocationBlocks: u32,
    pub WriteCombinedBytesAllocated: u64,
    pub WriteCombinedBytesReserved: u64,
    pub CachedBytesAllocated: u64,
    pub CachedBytesReserved: u64,
    pub SectionBytesAllocated: u64,
    pub SectionBytesReserved: u64,
    pub BytesZeroed: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_VIDEO_MEMORY {
    pub AllocsCommitted: u32,
    pub AllocsResidentInP: [D3DKMT_QUERYSTATISTICS_COUNTER; 5],
    pub AllocsResidentInNonPreferred: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub TotalBytesEvictedDueToPreparation: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_VIDPNSOURCE_INFORMATION {
    pub GlobalInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub SystemInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_ALLOCATIONS {
    pub Created: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Destroyed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Opened: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Closed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedSuccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedFail: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedAbandoned: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_LOCKS {
    pub NbLocks: u32,
    pub NbLocksWaitFlag: u32,
    pub NbLocksDiscardFlag: u32,
    pub NbLocksNoOverwrite: u32,
    pub NbLocksNoReadSync: u32,
    pub NbLocksLinearization: u32,
    pub NbComplexLocks: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_PAGING_FAULT {
    pub Faults: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsFirstTimeAccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsReclaimed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsMigration: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsIncorrectResource: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsLostContent: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsEvicted: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsMEM_RESET: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetSuccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetFail: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetSuccessRead: u32,
    pub AllocationsUnresetFailRead: u32,
    pub Evictions: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToPreparation: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToLock: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToClose: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToPurge: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToSuspendCPUAccess: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_PAGING_TRANSFER {
    pub BytesFilled: u64,
    pub BytesDiscarded: u64,
    pub BytesMappedIntoAperture: u64,
    pub BytesUnmappedFromAperture: u64,
    pub BytesTransferredFromMdlToMemory: u64,
    pub BytesTransferredFromMemoryToMdl: u64,
    pub BytesTransferredFromApertureToMemory: u64,
    pub BytesTransferredFromMemoryToAperture: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_PREPRATION {
    pub BroadcastStall: u32,
    pub NbDMAPrepared: u32,
    pub NbDMAPreparedLongPath: u32,
    pub ImmediateHighestPreparationPass: u32,
    pub AllocationsTrimmed: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_REFERENCE_DMA_BUFFER {
    pub NbCall: u32,
    pub NbAllocationsReferenced: u32,
    pub MaxNbAllocationsReferenced: u32,
    pub NbNULLReference: u32,
    pub NbWriteReference: u32,
    pub NbRenamedAllocationsReferenced: u32,
    pub NbIterationSearchingRenamedAllocation: u32,
    pub NbLockedAllocationReferenced: u32,
    pub NbAllocationWithValidPrepatchingInfoReferenced: u32,
    pub NbAllocationWithInvalidPrepatchingInfoReferenced: u32,
    pub NbDMABufferSuccessfullyPrePatched: u32,
    pub NbPrimariesReferencesOverflow: u32,
    pub NbAllocationWithNonPreferredResources: u32,
    pub NbAllocationInsertedInMigrationTable: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_RENAMING {
    pub NbAllocationsRenamed: u32,
    pub NbAllocationsShrinked: u32,
    pub NbRenamedBuffer: u32,
    pub MaxRenamingListLength: u32,
    pub NbFailuresDueToRenamingLimit: u32,
    pub NbFailuresDueToCreateAllocation: u32,
    pub NbFailuresDueToOpenAllocation: u32,
    pub NbFailuresDueToLowResource: u32,
    pub NbFailuresDueToNonRetiredLimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_SWIZZLING_RANGE {
    pub NbRangesAcquired: u32,
    pub NbRangesReleased: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATSTICS_TERMINATIONS {
    pub TerminatedShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub TerminatedNonShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub DestroyedShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub DestroyedNonShared: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYVIDEOMEMORYINFO {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub hAdapter: u32,
    pub MemorySegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub CurrentReservation: u64,
    pub AvailableForReservation: u64,
    pub PhysicalAdapterIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub hWindow: super::super::super::Win32::Foundation::HWND,
    pub VidPnSourceId: u32,
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub OwnerType: D3DKMT_VIDPNSOURCEOWNER_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_ADAPTER_UNIQUE_GUID {
    pub AdapterUniqueGUID: [u16; 40],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_DEVICE_IDS {
    pub PhysicalAdapterIndex: u32,
    pub DeviceIds: D3DKMT_DEVICE_IDS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_GPUMMU_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub Caps: D3DKMT_GPUMMU_CAPS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_MIRACAST_DRIVER_TYPE {
    pub MiracastDriverType: D3DKMT_MIRACAST_DRIVER_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_PHYSICAL_ADAPTER_PNP_KEY {
    pub PhysicalAdapterIndex: u32,
    pub PnPKeyType: D3DKMT_PNP_KEY_TYPE,
    pub pDest: windows_sys::core::PWSTR,
    pub pCchDest: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_SCANOUT_CAPS {
    pub VidPnSourceId: u32,
    pub Caps: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RECLAIMALLOCATIONS {
    pub hDevice: u32,
    pub pResources: *mut u32,
    pub HandleList: *const u32,
    pub pDiscarded: *mut super::super::super::Win32::Foundation::BOOL,
    pub NumAllocations: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RECLAIMALLOCATIONS2 {
    pub hPagingQueue: u32,
    pub NumAllocations: u32,
    pub pResources: *mut u32,
    pub HandleList: *const u32,
    pub Anonymous: D3DKMT_RECLAIMALLOCATIONS2_0,
    pub PagingFenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_RECLAIMALLOCATIONS2_0 {
    pub pDiscarded: *mut super::super::super::Win32::Foundation::BOOL,
    pub pResults: *mut D3DDDI_RECLAIM_RESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_REGISTERBUDGETCHANGENOTIFICATION {
    pub hDevice: u32,
    pub Callback: PFND3DKMT_BUDGETCHANGENOTIFICATIONCALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Handle: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_REGISTERTRIMNOTIFICATION {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub hDevice: u32,
    pub Callback: PFND3DKMT_TRIMNOTIFICATIONCALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Handle: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RELEASEKEYEDMUTEX {
    pub hKeyedMutex: u32,
    pub Key: u64,
    pub FenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RELEASEKEYEDMUTEX2 {
    pub hKeyedMutex: u32,
    pub Key: u64,
    pub FenceValue: u64,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RENDER {
    pub Anonymous: D3DKMT_RENDER_0,
    pub CommandOffset: u32,
    pub CommandLength: u32,
    pub AllocationCount: u32,
    pub PatchLocationCount: u32,
    pub pNewCommandBuffer: *mut core::ffi::c_void,
    pub NewCommandBufferSize: u32,
    pub pNewAllocationList: *mut D3DDDI_ALLOCATIONLIST,
    pub NewAllocationListSize: u32,
    pub pNewPatchLocationList: *mut D3DDDI_PATCHLOCATIONLIST,
    pub NewPatchLocationListSize: u32,
    pub Flags: D3DKMT_RENDERFLAGS,
    pub PresentHistoryToken: u64,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub QueuedBufferCount: u32,
    pub NewCommandBuffer: u64,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_RENDER_0 {
    pub hDevice: u32,
    pub hContext: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RENDERFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_REQUEST_MACHINE_CRASH_ESCAPE {
    pub Param1: usize,
    pub Param2: usize,
    pub Param3: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SCATTERBLT {
    pub hLogicalSurfaceDestination: u64,
    pub hDestinationCompSurfDWM: i64,
    pub DestinationCompositionBindingId: u64,
    pub SourceRect: super::super::super::Win32::Foundation::RECT,
    pub DestinationOffset: super::super::super::Win32::Foundation::POINT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SCATTERBLTS {
    pub NumBlts: u32,
    pub Blts: [D3DKMT_SCATTERBLT; 12],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SEGMENTGROUPSIZEINFO {
    pub PhysicalAdapterIndex: u32,
    pub LegacyInfo: D3DKMT_SEGMENTSIZEINFO,
    pub LocalMemory: u64,
    pub NonLocalMemory: u64,
    pub NonBudgetMemory: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SEGMENTSIZEINFO {
    pub DedicatedVideoMemorySize: u64,
    pub DedicatedSystemMemorySize: u64,
    pub SharedSystemMemorySize: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SEGMENT_CAPS {
    pub Size: u64,
    pub PageSize: u32,
    pub SegmentId: u32,
    pub bAperture: super::super::super::Win32::Foundation::BOOLEAN,
    pub bReservedSysMem: super::super::super::Win32::Foundation::BOOLEAN,
    pub BudgetGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETALLOCATIONPRIORITY {
    pub hDevice: u32,
    pub hResource: u32,
    pub phAllocationList: *const u32,
    pub AllocationCount: u32,
    pub pPriorities: *const u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY {
    pub hContext: u32,
    pub Priority: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETCONTEXTSCHEDULINGPRIORITY {
    pub hContext: u32,
    pub Priority: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETDISPLAYMODE {
    pub hDevice: u32,
    pub hPrimaryAllocation: u32,
    pub ScanLineOrdering: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
    pub DisplayOrientation: D3DDDI_ROTATION,
    pub PrivateDriverFormatAttribute: u32,
    pub Flags: D3DKMT_SETDISPLAYMODE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETDISPLAYMODE_FLAGS {
    pub _bitfield1: u8,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT {
    pub hDevice: u32,
    pub VidPnSourceId: u32,
    pub PrivateDriverFormatAttribute: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETFSEBLOCK {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub Flags: D3DKMT_SETFSEBLOCKFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SETFSEBLOCKFLAGS {
    pub Anonymous: D3DKMT_SETFSEBLOCKFLAGS_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETFSEBLOCKFLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETGAMMARAMP {
    pub hDevice: u32,
    pub VidPnSourceId: u32,
    pub Type: D3DDDI_GAMMARAMP_TYPE,
    pub Anonymous: D3DKMT_SETGAMMARAMP_0,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SETGAMMARAMP_0 {
    pub pGammaRampRgb256x3x16: *mut D3DDDI_GAMMA_RAMP_RGB256x3x16,
    pub pGammaRampDXGI1: *mut D3DDDI_GAMMA_RAMP_DXGI_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY {
    pub hAdapter: u32,
    pub Recovered: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETQUEUEDLIMIT {
    pub hDevice: u32,
    pub Type: D3DKMT_QUEUEDLIMIT_TYPE,
    pub Anonymous: D3DKMT_SETQUEUEDLIMIT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SETQUEUEDLIMIT_0 {
    pub QueuedPresentLimit: u32,
    pub Anonymous: D3DKMT_SETQUEUEDLIMIT_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETQUEUEDLIMIT_0_0 {
    pub VidPnSourceId: u32,
    pub QueuedPendingFlipLimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETSTABLEPOWERSTATE {
    pub hAdapter: u32,
    pub Enabled: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub VidPnSourceId: u32,
    pub TargetSyncRefreshCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEHWPROTECTION {
    pub hAdapter: u32,
    pub VidPnSourceId: u32,
    pub HwProtected: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER {
    pub hDevice: u32,
    pub pType: *const D3DKMT_VIDPNSOURCEOWNER_TYPE,
    pub pVidPnSourceId: *const u32,
    pub VidPnSourceCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER1 {
    pub Version0: D3DKMT_SETVIDPNSOURCEOWNER,
    pub Flags: D3DKMT_VIDPNSOURCEOWNER_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER2 {
    pub Version1: D3DKMT_SETVIDPNSOURCEOWNER1,
    pub pVidPnSourceNtHandles: *const isize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SET_COLORSPACE_TRANSFORM {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnTargetId: u32,
    pub Type: D3DDDI_GAMMARAMP_TYPE,
    pub Size: u32,
    pub Anonymous: D3DKMT_SET_COLORSPACE_TRANSFORM_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SET_COLORSPACE_TRANSFORM_0 {
    pub pColorSpaceTransform: *mut D3DKMDT_3x4_COLORSPACE_TRANSFORM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
    pub LockRect: super::super::super::Win32::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SHAREOBJECTWITHHOST {
    pub hDevice: u32,
    pub hObject: u32,
    pub Reserved: u64,
    pub hVailProcessNtHandle: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [u32; 32],
    pub Flags: D3DDDICB_SIGNALFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [u32; 32],
    pub Flags: D3DDDICB_SIGNALFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0 {
    pub Fence: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0_0,
    pub CpuEventHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0_0 {
    pub FenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU {
    pub hDevice: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub FenceValueArray: *const u64,
    pub Flags: D3DDDICB_SIGNALFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU_0 {
    pub MonitoredFenceValueArray: *const u64,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2 {
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub Flags: D3DDDICB_SIGNALFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContextArray: *const u32,
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2_0 {
    pub FenceValue: u64,
    pub CpuEventHandle: super::super::super::Win32::Foundation::HANDLE,
    pub MonitoredFenceValueArray: *const u64,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_STANDARDALLOCATION_EXISTINGHEAP {
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITCOMMAND {
    pub Commands: u64,
    pub CommandLength: u32,
    pub Flags: D3DKMT_SUBMITCOMMANDFLAGS,
    pub PresentHistoryToken: u64,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [u32; 64],
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub NumPrimaries: u32,
    pub WrittenPrimaries: [u32; 16],
    pub NumHistoryBuffers: u32,
    pub HistoryBufferArray: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITCOMMANDFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITCOMMANDTOHWQUEUE {
    pub hHwQueue: u32,
    pub HwQueueProgressFenceId: u64,
    pub CommandBuffer: u64,
    pub CommandLength: u32,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub NumPrimaries: u32,
    pub WrittenPrimaries: *const u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITPRESENTBLTTOHWQUEUE {
    pub hHwQueue: u32,
    pub HwQueueProgressFenceId: u64,
    pub PrivatePresentData: D3DKMT_PRESENT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITPRESENTTOHWQUEUE {
    pub hHwQueues: *mut u32,
    pub PrivatePresentData: D3DKMT_PRESENT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE {
    pub Flags: D3DDDICB_SIGNALFLAGS,
    pub BroadcastHwQueueCount: u32,
    pub BroadcastHwQueueArray: *const u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub FenceValueArray: *const u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE {
    pub hHwQueue: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub FenceValueArray: *const u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_SURFACECOMPLETE_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_TDRDBGCTRL_ESCAPE {
    pub TdrControl: D3DKMT_TDRDBGCTRLTYPE,
    pub Anonymous: D3DKMT_TDRDBGCTRL_ESCAPE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_TDRDBGCTRL_ESCAPE_0 {
    pub NodeOrdinal: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRACKEDWORKLOAD_SUPPORT {
    pub PhysicalAdapterIndex: u32,
    pub EngineType: DXGK_ENGINE_TYPE,
    pub Support: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRIMNOTIFICATION {
    pub Context: *mut core::ffi::c_void,
    pub Flags: D3DDDI_TRIMRESIDENCYSET_FLAGS,
    pub NumBytesToTrim: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRIMPROCESSCOMMITMENT {
    pub cbSize: u32,
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub Flags: D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS,
    pub DecommitRequested: u64,
    pub NumBytesDecommitted: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS {
    pub Anonymous: D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UMDFILENAMEINFO {
    pub Version: KMTUMDVERSION,
    pub UmdFileName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UMD_DRIVER_VERSION {
    pub DriverVersion: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNLOCK {
    pub hDevice: u32,
    pub NumAllocations: u32,
    pub phAllocations: *const u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNLOCK2 {
    pub hDevice: u32,
    pub hAllocation: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNPINDIRECTFLIPRESOURCES {
    pub hDevice: u32,
    pub ResourceCount: u32,
    pub pResourceList: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION {
    pub Handle: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNREGISTERTRIMNOTIFICATION {
    pub Handle: *mut core::ffi::c_void,
    pub Callback: PFND3DKMT_TRIMNOTIFICATIONCALLBACK,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UPDATEGPUVIRTUALADDRESS {
    pub hDevice: u32,
    pub hContext: u32,
    pub hFenceObject: u32,
    pub NumOperations: u32,
    pub Operations: *mut D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION,
    pub Reserved0: usize,
    pub Reserved1: u64,
    pub FenceValue: u64,
    pub Flags: D3DKMT_UPDATEGPUVIRTUALADDRESS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_UPDATEGPUVIRTUALADDRESS_0 {
    pub Anonymous: D3DKMT_UPDATEGPUVIRTUALADDRESS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UPDATEGPUVIRTUALADDRESS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UPDATEOVERLAY {
    pub hDevice: u32,
    pub hOverlay: u32,
    pub OverlayInfo: D3DDDI_KERNELOVERLAYINFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VAD_DESC {
    pub VadIndex: u32,
    pub VadAddress: u64,
    pub NumMappedRanges: u32,
    pub VadType: u32,
    pub StartAddress: u64,
    pub EndAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VA_RANGE_DESC {
    pub VadAddress: u64,
    pub VaRangeIndex: u32,
    pub PhysicalAdapterIndex: u32,
    pub StartAddress: u64,
    pub EndAddress: u64,
    pub DriverProtection: u64,
    pub OwnerType: u32,
    pub pOwner: u64,
    pub OwnerOffset: u64,
    pub Protection: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VGPUINTERFACEID {
    pub VirtualGpuIntefaceId: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE {
    pub Type: D3DKMT_VIDMMESCAPETYPE,
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0 {
    pub SetFault: D3DKMT_VIDMM_ESCAPE_0_9,
    pub Evict: D3DKMT_VIDMM_ESCAPE_0_4,
    pub EvictByNtHandle: D3DKMT_VIDMM_ESCAPE_0_3,
    pub GetVads: D3DKMT_VIDMM_ESCAPE_0_6,
    pub SetBudget: D3DKMT_VIDMM_ESCAPE_0_8,
    pub SuspendProcess: D3DKMT_VIDMM_ESCAPE_0_11,
    pub ResumeProcess: D3DKMT_VIDMM_ESCAPE_0_7,
    pub GetBudget: D3DKMT_VIDMM_ESCAPE_0_5,
    pub SetTrimIntervals: D3DKMT_VIDMM_ESCAPE_0_10,
    pub EvictByCriteria: D3DKMT_EVICTION_CRITERIA,
    pub Wake: D3DKMT_VIDMM_ESCAPE_0_13,
    pub Defrag: D3DKMT_VIDMM_ESCAPE_0_0,
    pub DelayExecution: D3DKMT_VIDMM_ESCAPE_0_1,
    pub VerifyIntegrity: D3DKMT_VIDMM_ESCAPE_0_12,
    pub DelayedEvictionConfig: D3DKMT_VIDMM_ESCAPE_0_2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_0 {
    pub Operation: D3DKMT_DEFRAG_ESCAPE_OPERATION,
    pub SegmentId: u32,
    pub TotalCommitted: u64,
    pub TotalFree: u64,
    pub LargestGapBefore: u64,
    pub LargestGapAfter: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_1 {
    pub hPagingQueue: u32,
    pub PhysicalAdapterIndex: u32,
    pub Milliseconds: u32,
    pub PagingFenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_2 {
    pub TimerValue: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_3 {
    pub NtHandle: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_4 {
    pub ResourceHandle: u32,
    pub AllocationHandle: u32,
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_5 {
    pub NumBytesToTrim: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_6 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_6_0,
    pub Command: D3DKMT_VAD_ESCAPE_COMMAND,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_6_0 {
    pub GetNumVads: D3DKMT_VIDMM_ESCAPE_0_6_0_0,
    pub GetVad: D3DKMT_VAD_DESC,
    pub GetVadRange: D3DKMT_VA_RANGE_DESC,
    pub GetGpuMmuCaps: D3DKMT_GET_GPUMMU_CAPS,
    pub GetPte: D3DKMT_GET_PTE,
    pub GetSegmentCaps: D3DKMT_GET_SEGMENT_CAPS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_6_0_0 {
    pub NumVads: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_7 {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_8 {
    pub LocalMemoryBudget: u64,
    pub SystemMemoryBudget: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_9 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_9_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_9_0 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_9_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_9_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_10 {
    pub MinTrimInterval: u32,
    pub MaxTrimInterval: u32,
    pub IdleTrimInterval: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_11 {
    pub hProcess: super::super::super::Win32::Foundation::HANDLE,
    pub bAllowWakeOnSubmission: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_12 {
    pub SegmentId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_13 {
    pub bFlush: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDPNSOURCEOWNER_FLAGS {
    pub Anonymous: D3DKMT_VIDPNSOURCEOWNER_FLAGS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDPNSOURCEOWNER_FLAGS_0 {
    pub Anonymous: D3DKMT_VIDPNSOURCEOWNER_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDPNSOURCEOWNER_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDSCH_ESCAPE {
    pub Type: D3DKMT_VIDSCHESCAPETYPE,
    pub Anonymous: D3DKMT_VIDSCH_ESCAPE_0,
    pub VirtualRefreshRateControl: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDSCH_ESCAPE_0 {
    pub PreemptionControl: super::super::super::Win32::Foundation::BOOL,
    pub EnableContextDelay: super::super::super::Win32::Foundation::BOOL,
    pub TdrControl2: D3DKMT_VIDSCH_ESCAPE_0_0,
    pub SuspendScheduler: super::super::super::Win32::Foundation::BOOL,
    pub TdrControl: u32,
    pub SuspendTime: u32,
    pub TdrLimit: D3DKMT_VIDSCH_ESCAPE_0_1,
    pub PfnControl: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDSCH_ESCAPE_0_0 {
    pub TdrControl: u32,
    pub Anonymous: D3DKMT_VIDSCH_ESCAPE_0_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDSCH_ESCAPE_0_0_0 {
    pub NodeOrdinal: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDSCH_ESCAPE_0_1 {
    pub Count: u32,
    pub Time: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIRTUALADDRESSFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIRTUALADDRESSINFO {
    pub VirtualAddressFlags: D3DKMT_VIRTUALADDRESSFLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORIDLE {
    pub hDevice: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [u32; 32],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [u32; 32],
    pub Anonymous: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0 {
    pub Fence: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0_0,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0_0 {
    pub FenceValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU {
    pub hDevice: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub FenceValueArray: *const u64,
    pub hAsyncEvent: super::super::super::Win32::Foundation::HANDLE,
    pub Flags: D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU {
    pub hContext: u32,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const u32,
    pub Anonymous: D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU_0 {
    pub MonitoredFenceValueArray: *const u64,
    pub FenceValue: u64,
    pub Reserved: [u64; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORVERTICALBLANKEVENT {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub VidPnSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORVERTICALBLANKEVENT2 {
    pub hAdapter: u32,
    pub hDevice: u32,
    pub VidPnSourceId: u32,
    pub NumObjects: u32,
    pub ObjectHandleArray: [isize; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_2_CAPS {
    pub PreemptionCaps: D3DKMDT_PREEMPTION_CAPS,
    pub Anonymous: D3DKMT_WDDM_1_2_CAPS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_1_2_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_1_2_CAPS_0_0,
    pub Value: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_2_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_3_CAPS {
    pub Anonymous: D3DKMT_WDDM_1_3_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_1_3_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_1_3_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_1_3_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_0_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_0_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_0_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_0_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_0_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_7_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_7_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_7_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_7_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_7_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_9_CAPS {
    pub Anonymous: D3DKMT_WDDM_2_9_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_2_9_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_2_9_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_2_9_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_0_CAPS {
    pub Anonymous: D3DKMT_WDDM_3_0_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_3_0_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_3_0_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_0_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_1_CAPS {
    pub Anonymous: D3DKMT_WDDM_3_1_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union D3DKMT_WDDM_3_1_CAPS_0 {
    pub Anonymous: D3DKMT_WDDM_3_1_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WDDM_3_1_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WORKINGSETFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WORKINGSETINFO {
    pub Flags: D3DKMT_WORKINGSETFLAGS,
    pub MinimumWorkingSetPercentile: u32,
    pub MaximumWorkingSetPercentile: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WSAUMDIMAGENAME {
    pub WsaUmdImageName: [u16; 260],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_XBOX {
    pub IsXBOX: super::super::super::Win32::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DLINEPATTERN {
    pub wRepeatFactor: u16,
    pub wLinePattern: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_CLIPPEDTRIANGLEFAN {
    pub FirstVertexOffset: u32,
    pub dwEdgeFlags: u32,
    pub PrimitiveCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_CONTEXTDESTROYALLDATA {
    pub dwPID: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_CONTEXTDESTROYDATA {
    pub dwhContext: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_D3DDX6EXTENDEDCAPS {
    pub dwSize: u32,
    pub dwMinTextureWidth: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMinStippleWidth: u32,
    pub dwMaxStippleWidth: u32,
    pub dwMinStippleHeight: u32,
    pub dwMaxStippleHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_D3DEXTENDEDCAPS {
    pub dwSize: u32,
    pub dwMinTextureWidth: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMinStippleWidth: u32,
    pub dwMaxStippleWidth: u32,
    pub dwMinStippleHeight: u32,
    pub dwMaxStippleHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
    pub dwMaxActiveLights: u32,
    pub dvMaxVertexW: f32,
    pub wMaxUserClipPlanes: u16,
    pub wMaxVertexBlendMatrices: u16,
    pub dwVertexProcessingCaps: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2ADDDIRTYRECT {
    pub dwSurface: u32,
    pub rDirtyArea: super::super::super::Win32::Foundation::RECTL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2BLT {
    pub dwSource: u32,
    pub rSource: super::super::super::Win32::Foundation::RECTL,
    pub dwSourceMipLevel: u32,
    pub dwDest: u32,
    pub rDest: super::super::super::Win32::Foundation::RECTL,
    pub dwDestMipLevel: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CLEAR {
    pub dwFlags: u32,
    pub dwFillColor: u32,
    pub dvFillDepth: f32,
    pub dwFillStencil: u32,
    pub Rects: [super::super::super::Win32::Foundation::RECT; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2COLORFILL {
    pub dwSurface: u32,
    pub rRect: super::super::super::Win32::Foundation::RECTL,
    pub Color: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2COMMAND {
    pub bCommand: u8,
    pub bReserved: u8,
    pub Anonymous: D3DNTHAL_DP2COMMAND_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DNTHAL_DP2COMMAND_0 {
    pub wPrimitiveCount: u16,
    pub wStateCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CREATELIGHT {
    pub dwIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CREATEPIXELSHADER {
    pub dwHandle: u32,
    pub dwCodeSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CREATEVERTEXSHADER {
    pub dwHandle: u32,
    pub dwDeclSize: u32,
    pub dwCodeSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CREATEVERTEXSHADERDECL {
    pub dwHandle: u32,
    pub dwNumVertexElements: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2CREATEVERTEXSHADERFUNC {
    pub dwHandle: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2DELETEQUERY {
    pub dwQueryID: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2DRAWRECTPATCH {
    pub Handle: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2DRAWTRIPATCH {
    pub Handle: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2EXT {
    pub dwExtToken: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDLINELIST {
    pub wV1: u16,
    pub wV2: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDLINESTRIP {
    pub wV: [u16; 2],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDTRIANGLEFAN {
    pub wV: [u16; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDTRIANGLELIST {
    pub wV1: u16,
    pub wV2: u16,
    pub wV3: u16,
    pub wFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDTRIANGLELIST2 {
    pub wV1: u16,
    pub wV2: u16,
    pub wV3: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2INDEXEDTRIANGLESTRIP {
    pub wV: [u16; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2ISSUEQUERY {
    pub dwQueryID: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2LINELIST {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2LINESTRIP {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2PIXELSHADER {
    pub dwHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2POINTS {
    pub wCount: u16,
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2RESPONSE {
    pub bCommand: u8,
    pub bReserved: u8,
    pub wStateCount: u16,
    pub dwTotalSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2RESPONSEQUERY {
    pub dwQueryID: u32,
    pub dwSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETCLIPPLANE {
    pub dwIndex: u32,
    pub plane: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETCONVOLUTIONKERNELMONO {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETDEPTHSTENCIL {
    pub hZBuffer: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETINDICES {
    pub dwVBHandle: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETLIGHT {
    pub dwIndex: u32,
    pub Anonymous: D3DNTHAL_DP2SETLIGHT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DNTHAL_DP2SETLIGHT_0 {
    pub lightData: u32,
    pub dwDataType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETPALETTE {
    pub dwPaletteHandle: u32,
    pub dwPaletteFlags: u32,
    pub dwSurfaceHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETPIXELSHADERCONST {
    pub dwRegister: u32,
    pub dwCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETPRIORITY {
    pub dwDDDestSurface: u32,
    pub dwPriority: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETRENDERTARGET {
    pub hRenderTarget: u32,
    pub hZBuffer: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETRENDERTARGET2 {
    pub RTIndex: u32,
    pub hRenderTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETSTREAMSOURCE {
    pub dwStream: u32,
    pub dwVBHandle: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETSTREAMSOURCE2 {
    pub dwStream: u32,
    pub dwVBHandle: u32,
    pub dwOffset: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETSTREAMSOURCEFREQ {
    pub dwStream: u32,
    pub dwDivider: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETSTREAMSOURCEUM {
    pub dwStream: u32,
    pub dwStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETTEXLOD {
    pub dwDDSurface: u32,
    pub dwLOD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SETVERTEXSHADERCONST {
    pub dwRegister: u32,
    pub dwCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2STARTVERTEX {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2SURFACEBLT {
    pub dwSource: u32,
    pub rSource: super::super::super::Win32::Foundation::RECTL,
    pub dwSourceMipLevel: u32,
    pub dwDest: u32,
    pub rDest: super::super::super::Win32::Foundation::RECTL,
    pub dwDestMipLevel: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TEXBLT {
    pub dwDDDestSurface: u32,
    pub dwDDSrcSurface: u32,
    pub pDest: super::super::super::Win32::Foundation::POINT,
    pub rSrc: super::super::super::Win32::Foundation::RECTL,
    pub dwFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TEXTURESTAGESTATE {
    pub wStage: u16,
    pub TSState: u16,
    pub dwValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TRIANGLEFAN {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TRIANGLEFAN_IMM {
    pub dwEdgeFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TRIANGLELIST {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2TRIANGLESTRIP {
    pub wVStart: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2UPDATEPALETTE {
    pub dwPaletteHandle: u32,
    pub wStartIndex: u16,
    pub wNumEntries: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2VERTEXSHADER {
    pub dwHandle: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2VIEWPORTINFO {
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2WINFO {
    pub dvWNear: f32,
    pub dvWFar: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_DP2ZRANGE {
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_SCENECAPTUREDATA {
    pub dwhContext: usize,
    pub dwFlag: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_TEXTURECREATEDATA {
    pub dwhContext: usize,
    pub hDDS: super::super::super::Win32::Foundation::HANDLE,
    pub dwHandle: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_TEXTUREDESTROYDATA {
    pub dwhContext: usize,
    pub dwHandle: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_TEXTUREGETSURFDATA {
    pub dwhContext: usize,
    pub hDDS: super::super::super::Win32::Foundation::HANDLE,
    pub dwHandle: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_TEXTURESWAPDATA {
    pub dwhContext: usize,
    pub dwHandle1: usize,
    pub dwHandle2: usize,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DNTHAL_VALIDATETEXTURESTAGESTATEDATA {
    pub dwhContext: usize,
    pub dwFlags: u32,
    pub dwReserved: usize,
    pub dwNumPasses: u32,
    pub ddrval: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_DEFERRED_AGP_AWARE_DATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_DXVERSION {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwDXVersion: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_FREE_DEFERRED_AGP_DATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwProcessId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETADAPTERGROUPDATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub ulUniqueAdapterGroupId: usize,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETD3DQUERYCOUNTDATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwNumQueries: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETDDIVERSIONDATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwDXVersion: u32,
    pub dwDDIVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETDRIVERINFO2DATA {
    pub dwReserved: u32,
    pub dwMagic: u32,
    pub dwType: u32,
    pub dwExpectedSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETEXTENDEDMODECOUNTDATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwModeCount: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DDNT_GETFORMATCOUNTDATA {
    pub gdi2: DDNT_GETDRIVERINFO2DATA,
    pub dwFormatCount: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_DEFERRED_AGP_AWARE_DATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_DXVERSION {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwDXVersion: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_FREE_DEFERRED_AGP_DATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwProcessId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETADAPTERGROUPDATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub ulUniqueAdapterGroupId: usize,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETD3DQUERYCOUNTDATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwNumQueries: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETDDIVERSIONDATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwDXVersion: u32,
    pub dwDDIVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETDRIVERINFO2DATA {
    pub dwReserved: u32,
    pub dwMagic: u32,
    pub dwType: u32,
    pub dwExpectedSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETEXTENDEDMODECOUNTDATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwModeCount: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DD_GETFORMATCOUNTDATA {
    pub gdi2: DD_GETDRIVERINFO2DATA,
    pub dwFormatCount: u32,
    pub dwReserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I {
    pub Anonymous1: DISPLAYID_DETAILED_TIMING_TYPE_I_0,
    pub HorizontalActivePixels: u16,
    pub HorizontalBlankPixels: u16,
    pub Anonymous2: DISPLAYID_DETAILED_TIMING_TYPE_I_1,
    pub HorizontalSyncWidth: u16,
    pub VerticalActiveLines: u16,
    pub VerticalBlankLines: u16,
    pub Anonymous3: DISPLAYID_DETAILED_TIMING_TYPE_I_2,
    pub VerticalSyncWidth: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_1 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DISPLAYID_DETAILED_TIMING_TYPE_I_2 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKARG_SETPALETTE {
    pub VidPnSourceId: u32,
    pub FirstEntry: u32,
    pub NumEntries: u32,
    pub pLookupTable: *mut D3DKMDT_PALETTEDATA,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_ACP_AND_CGMSA_SIGNALING {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulAvailableTVProtectionStandards: u32,
    pub ulActiveTVProtectionStandard: u32,
    pub ulReserved: u32,
    pub ulAspectRatioValidMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioValidMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioValidMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved2: [u32; 4],
    pub ulReserved3: [u32; 4],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_ACTUAL_OUTPUT_FORMAT {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulDisplayWidth: u32,
    pub ulDisplayHeight: u32,
    pub ifInterleaveFormat: DXGKMDT_OPM_INTERLEAVE_FORMAT,
    pub d3dFormat: u32,
    pub ulFrequencyNumerator: u32,
    pub ulFrequencyDenominator: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_CONFIGURE_PARAMETERS {
    pub omac: DXGKMDT_OPM_OMAC,
    pub guidSetting: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulHDCPFlags: u32,
    pub ksvB: DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR,
    pub Reserved: [u8; 11],
    pub Reserved2: [u8; 16],
    pub Reserved3: [u8; 16],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub guidInformation: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_CREATE_VIDEO_OUTPUT_FOR_TARGET_PARAMETERS {
    pub AdapterLuid: super::super::super::Win32::Foundation::LUID,
    pub TargetId: u32,
    pub Vos: DXGKMDT_OPM_VIDEO_OUTPUT_SEMANTICS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_ENCRYPTED_PARAMETERS {
    pub abEncryptedParameters: [u8; 256],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_GET_INFO_PARAMETERS {
    pub omac: DXGKMDT_OPM_OMAC,
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub guidInformation: windows_sys::core::GUID,
    pub ulSequenceNumber: u32,
    pub cbParametersSize: u32,
    pub abParameters: [u8; 4056],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_HDCP_KEY_SELECTION_VECTOR {
    pub abKeySelectionVector: [u8; 5],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_OMAC {
    pub abOMAC: [u8; 16],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_OUTPUT_ID {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub OutputId: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_RANDOM_NUMBER {
    pub abRandomNumber: [u8; 16],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_REQUESTED_INFORMATION {
    pub omac: DXGKMDT_OPM_OMAC,
    pub cbRequestedInformationSize: u32,
    pub abRequestedInformation: [u8; 4076],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    pub ulNewTVProtectionStandard: u32,
    pub ulAspectRatioChangeMask1: u32,
    pub ulAspectRatioData1: u32,
    pub ulAspectRatioChangeMask2: u32,
    pub ulAspectRatioData2: u32,
    pub ulAspectRatioChangeMask3: u32,
    pub ulAspectRatioData3: u32,
    pub ulReserved: [u32; 4],
    pub ulReserved2: [u32; 4],
    pub ulReserved3: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_SET_HDCP_SRM_PARAMETERS {
    pub ulSRMVersion: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    pub ulProtectionType: u32,
    pub ulProtectionLevel: u32,
    pub Reserved: u32,
    pub Reserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGKMDT_OPM_STANDARD_INFORMATION {
    pub rnRandomNumber: DXGKMDT_OPM_RANDOM_NUMBER,
    pub ulStatusFlags: u32,
    pub ulInformation: u32,
    pub ulReserved: u32,
    pub ulReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_HEAD {
    pub Luid: GPUP_DRIVER_ESCAPE_INPUT,
    pub Type: DXGKVGPU_ESCAPE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_INITIALIZE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub VmGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_PAUSE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub DeviceLuid: super::super::super::Win32::Foundation::LUID,
    pub Anonymous: DXGKVGPU_ESCAPE_PAUSE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGKVGPU_ESCAPE_PAUSE_0 {
    pub Anonymous: DXGKVGPU_ESCAPE_PAUSE_0_0,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_PAUSE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_POWERTRANSITIONCOMPLETE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub PowerState: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_READ_PCI_CONFIG {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_READ_VGPU_TYPE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_RELEASE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_RESUME {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub DeviceLuid: super::super::super::Win32::Foundation::LUID,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_WRITE_PCI_CONFIG {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_ADAPTER_PERFDATA {
    pub MemoryFrequency: u64,
    pub MaxMemoryFrequency: u64,
    pub MaxMemoryFrequencyOC: u64,
    pub MemoryBandwidth: u64,
    pub PCIEBandwidth: u64,
    pub FanRPM: u32,
    pub Power: u32,
    pub Temperature: u32,
    pub PowerStateOverride: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_ADAPTER_PERFDATACAPS {
    pub MaxMemoryBandwidth: u64,
    pub MaxPCIEBandwidth: u64,
    pub MaxFanRPM: u32,
    pub TemperatureMax: u32,
    pub TemperatureWarning: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BACKLIGHT_INFO {
    pub BacklightUsersetting: u16,
    pub BacklightEffective: u16,
    pub GammaRamp: D3DDDI_GAMMA_RAMP_RGB256x3x16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_CAPS {
    pub Anonymous: DXGK_BRIGHTNESS_CAPS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_CAPS_0 {
    pub Anonymous: DXGK_BRIGHTNESS_CAPS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT {
    pub NormalRangeCount: u32,
    pub RangeCount: u32,
    pub PreferredMaximumBrightness: u32,
    pub SupportedRanges: [DXGK_BRIGHTNESS_NIT_RANGE; 16],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_GET_OUT {
    pub CurrentBrightnessMillinits: u32,
    pub TargetBrightnessMillinits: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_NIT_RANGE {
    pub MinimumLevelMillinit: u32,
    pub MaximumLevelMillinit: u32,
    pub StepSizeMillinit: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA {
    pub Size: u32,
    pub Anonymous: DXGK_BRIGHTNESS_SENSOR_DATA_0,
    pub AlsReading: f32,
    pub Chromaticity: DXGK_BRIGHTNESS_SENSOR_DATA_CHROMATICITY,
    pub ColorTemperature: f32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_SENSOR_DATA_0 {
    pub Flags: DXGK_BRIGHTNESS_SENSOR_DATA_0_0,
    pub ValidSensorValues: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SENSOR_DATA_CHROMATICITY {
    pub ChromaticityX: f32,
    pub ChromaticityY: f32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_SET_IN {
    pub BrightnessMillinits: u32,
    pub TransitionTimeMs: u32,
    pub SensorReadings: DXGK_BRIGHTNESS_SENSOR_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_STATE {
    pub Anonymous: DXGK_BRIGHTNESS_STATE_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_BRIGHTNESS_STATE_0 {
    pub Anonymous: DXGK_BRIGHTNESS_STATE_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_BRIGHTNESS_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_DISPLAY_INFORMATION {
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub ColorFormat: D3DDDIFORMAT,
    pub PhysicAddress: i64,
    pub TargetId: u32,
    pub AcpiId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_ESCAPE_GPUMMUCAPS {
    pub ReadOnlyMemorySupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub NoExecuteMemorySupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub ZeroInPteSupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub CacheCoherentMemorySupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub LargePageSupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub DualPteSupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub AllowNonAlignedLargePageAddress: super::super::super::Win32::Foundation::BOOLEAN,
    pub VirtualAddressBitCount: u32,
    pub PageTableLevelCount: u32,
    pub PageTableLevelDesk: [D3DKMT_PAGE_TABLE_LEVEL_DESC; 6],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_FAULT_ERROR_CODE {
    pub Anonymous: DXGK_FAULT_ERROR_CODE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_FAULT_ERROR_CODE_0 {
    pub Anonymous1: DXGK_FAULT_ERROR_CODE_0_0,
    pub Anonymous2: DXGK_FAULT_ERROR_CODE_0_1,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_FAULT_ERROR_CODE_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_FAULT_ERROR_CODE_0_1 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_GPUCLOCKDATA {
    pub GpuFrequency: u64,
    pub GpuClockCounter: u64,
    pub CpuClockCounter: u64,
    pub Flags: DXGK_GPUCLOCKDATA_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_GPUCLOCKDATA_FLAGS {
    pub Anonymous: DXGK_GPUCLOCKDATA_FLAGS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_GPUCLOCKDATA_FLAGS_0 {
    pub Anonymous: DXGK_GPUCLOCKDATA_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_GPUCLOCKDATA_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_GPUVERSION {
    pub BiosVersion: [u16; 32],
    pub GpuArchitecture: [u16; 32],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Power")]
#[derive(Clone, Copy)]
pub struct DXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2 {
    pub Version: u32,
    pub PrivateHandle: *mut core::ffi::c_void,
    pub PowerNotificationCb: PDXGK_POWER_NOTIFICATION,
    pub RemovalNotificationCb: PDXGK_REMOVAL_NOTIFICATION,
    pub FStateNotificationCb: PDXGK_FSTATE_NOTIFICATION,
    pub InitialComponentStateCb: PDXGK_INITIAL_COMPONENT_STATE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Power")]
#[derive(Clone, Copy)]
pub struct DXGK_GRAPHICSPOWER_REGISTER_OUTPUT {
    pub DeviceHandle: *mut core::ffi::c_void,
    pub InitialGrfxPowerState: super::super::super::Win32::System::Power::DEVICE_POWER_STATE,
    pub SetSharedPowerComponentStateCb: PDXGK_SET_SHARED_POWER_COMPONENT_STATE,
    pub UnregisterCb: PDXGK_GRAPHICSPOWER_UNREGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_MIRACAST_CHUNK_ID {
    pub Anonymous: DXGK_MIRACAST_CHUNK_ID_0,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_MIRACAST_CHUNK_ID_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_MIRACAST_CHUNK_INFO {
    pub ChunkType: DXGK_MIRACAST_CHUNK_TYPE,
    pub ChunkId: DXGK_MIRACAST_CHUNK_ID,
    pub ProcessingTime: u32,
    pub EncodeRate: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_MONITORLINKINFO_CAPABILITIES {
    pub Anonymous: DXGK_MONITORLINKINFO_CAPABILITIES_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_MONITORLINKINFO_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_MONITORLINKINFO_USAGEHINTS {
    pub Anonymous: DXGK_MONITORLINKINFO_USAGEHINTS_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_MONITORLINKINFO_USAGEHINTS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_NODEMETADATA {
    pub EngineType: DXGK_ENGINE_TYPE,
    pub FriendlyName: [u16; 32],
    pub Flags: DXGK_NODEMETADATA_FLAGS,
    pub GpuMmuSupported: super::super::super::Win32::Foundation::BOOLEAN,
    pub IoMmuSupported: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_NODEMETADATA_FLAGS {
    pub Anonymous: DXGK_NODEMETADATA_FLAGS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXGK_NODEMETADATA_FLAGS_0 {
    pub Anonymous: DXGK_NODEMETADATA_FLAGS_0_0,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_NODEMETADATA_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXGK_NODE_PERFDATA {
    pub Frequency: u64,
    pub MaxFrequency: u64,
    pub MaxFrequencyOC: u64,
    pub Voltage: u32,
    pub VoltageMax: u32,
    pub VoltageMaxOC: u32,
    pub MaxTransitionLatency: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_PTE {
    pub Anonymous1: DXGK_PTE_0,
    pub Anonymous2: DXGK_PTE_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_PTE_0 {
    pub Anonymous: DXGK_PTE_0_0,
    pub Flags: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_PTE_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_PTE_1 {
    pub PageAddress: u64,
    pub PageTableAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_TARGETMODE_DETAIL_TIMING {
    pub VideoStandard: D3DKMDT_VIDEO_SIGNAL_STANDARD,
    pub TimingId: u32,
    pub DetailTiming: DISPLAYID_DETAILED_TIMING_TYPE_I,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GPUP_DRIVER_ESCAPE_INPUT {
    pub vfLUID: super::super::super::Win32::Foundation::LUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OUTPUTDUPL_CONTEXT_DEBUG_INFO {
    pub Status: OUTPUTDUPL_CONTEXT_DEBUG_STATUS,
    pub ProcessID: super::super::super::Win32::Foundation::HANDLE,
    pub AccumulatedPresents: u32,
    pub LastPresentTime: i64,
    pub LastMouseTime: i64,
    pub ProcessName: [i8; 16],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _NT_D3DLINEPATTERN {
    pub wRepeatFactor: u16,
    pub wLinePattern: u16,
}
pub type LPD3DHAL_CONTEXTDESTROYALLCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_CONTEXTDESTROYALLDATA) -> u32>;
pub type LPD3DHAL_CONTEXTDESTROYCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_CONTEXTDESTROYDATA) -> u32>;
pub type LPD3DHAL_DRAWPRIMITIVESCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_DRAWPRIMITIVESDATA) -> u32>;
pub type LPD3DHAL_RENDERSTATECB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_RENDERSTATEDATA) -> u32>;
pub type LPD3DHAL_SCENECAPTURECB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_SCENECAPTUREDATA) -> u32>;
pub type LPD3DHAL_TEXTURECREATECB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_TEXTURECREATEDATA) -> u32>;
pub type LPD3DHAL_TEXTUREDESTROYCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_TEXTUREDESTROYDATA) -> u32>;
pub type LPD3DHAL_TEXTUREGETSURFCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_TEXTUREGETSURFDATA) -> u32>;
pub type LPD3DHAL_TEXTURESWAPCB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_TEXTURESWAPDATA) -> u32>;
pub type LPD3DHAL_VALIDATETEXTURESTAGESTATECB = Option<unsafe extern "system" fn(param0: *mut D3DHAL_VALIDATETEXTURESTAGESTATEDATA) -> u32>;
pub type LPD3DNTHAL_CONTEXTDESTROYALLCB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_CONTEXTDESTROYALLDATA) -> u32>;
pub type LPD3DNTHAL_CONTEXTDESTROYCB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_CONTEXTDESTROYDATA) -> u32>;
pub type LPD3DNTHAL_SCENECAPTURECB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_SCENECAPTUREDATA) -> u32>;
pub type LPD3DNTHAL_TEXTURECREATECB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_TEXTURECREATEDATA) -> u32>;
pub type LPD3DNTHAL_TEXTUREDESTROYCB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_TEXTUREDESTROYDATA) -> u32>;
pub type LPD3DNTHAL_TEXTUREGETSURFCB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_TEXTUREGETSURFDATA) -> u32>;
pub type LPD3DNTHAL_TEXTURESWAPCB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_TEXTURESWAPDATA) -> u32>;
pub type LPD3DNTHAL_VALIDATETEXTURESTAGESTATECB = Option<unsafe extern "system" fn(param0: *mut D3DNTHAL_VALIDATETEXTURESTAGESTATEDATA) -> u32>;
pub type PDXGK_FSTATE_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, componentindex: u32, newfstate: u32, prenotification: super::super::super::Win32::Foundation::BOOLEAN, privatehandle: *mut core::ffi::c_void)>;
pub type PDXGK_GRAPHICSPOWER_UNREGISTER = Option<unsafe extern "system" fn(devicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PDXGK_INITIAL_COMPONENT_STATE = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void, componentindex: u32, isblockingtype: super::super::super::Win32::Foundation::BOOLEAN, initialfstate: u32, componentguid: windows_sys::core::GUID, powercomponentmappingflag: u32)>;
#[cfg(feature = "Win32_System_Power")]
pub type PDXGK_POWER_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, newgrfxpowerstate: super::super::super::Win32::System::Power::DEVICE_POWER_STATE, prenotification: super::super::super::Win32::Foundation::BOOLEAN, privatehandle: *mut core::ffi::c_void)>;
pub type PDXGK_REMOVAL_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void)>;
pub type PDXGK_SET_SHARED_POWER_COMPONENT_STATE = Option<unsafe extern "system" fn(devicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void, componentindex: u32, active: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ACQUIREKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ACQUIREKEYEDMUTEX) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ACQUIREKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ACQUIREKEYEDMUTEX2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ADJUSTFULLSCREENGAMMA = Option<unsafe extern "system" fn(param0: *const D3DKMT_ADJUSTFULLSCREENGAMMA) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_BUDGETCHANGENOTIFICATIONCALLBACK = Option<unsafe extern "system" fn(param0: *const D3DKMT_BUDGETCHANGENOTIFICATION)>;
pub type PFND3DKMT_CANCELPRESENTS = Option<unsafe extern "system" fn(param0: *const D3DKMT_CANCEL_PRESENTS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFND3DKMT_CHANGESURFACEPOINTER = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHANGESURFACEPOINTER) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHANGEVIDEOMEMORYRESERVATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHANGEVIDEOMEMORYRESERVATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PFND3DKMT_CHECKMONITORPOWERSTATE = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKMONITORPOWERSTATE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKOCCLUSION = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKOCCLUSION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKSHAREDRESOURCEACCESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKSHAREDRESOURCEACCESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CLOSEADAPTER = Option<unsafe extern "system" fn(param0: *const D3DKMT_CLOSEADAPTER) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CONFIGURESHAREDRESOURCE = Option<unsafe extern "system" fn(param0: *const D3DKMT_CONFIGURESHAREDRESOURCE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CONNECTDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CONNECT_DOORBELL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEALLOCATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEALLOCATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEALLOCATION2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEALLOCATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATECONTEXT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATECONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATECONTEXTVIRTUAL = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATECONTEXTVIRTUAL) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFND3DKMT_CREATEDCFROMMEMORY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEDCFROMMEMORY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEDEVICE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEDEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CREATE_DOORBELL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEHWQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEKEYEDMUTEX) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEKEYEDMUTEX2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATENATIVEFENCE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATENATIVEFENCE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEOUTPUTDUPL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CREATE_OUTPUTDUPL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEOVERLAY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEOVERLAY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEPAGINGQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEPAGINGQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATEPROTECTEDSESSION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEPROTECTEDSESSION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATESYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_CREATESYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYALLOCATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYALLOCATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYALLOCATION2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYALLOCATION2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYCONTEXT = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYCONTEXT) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFND3DKMT_DESTROYDCFROMMEMORY = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYDCFROMMEMORY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYDEVICE = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYDEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROY_DOORBELL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYKEYEDMUTEX) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYOUTPUTDUPL = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROY_OUTPUTDUPL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYOVERLAY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYPAGINGQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DDDI_DESTROYPAGINGQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYPROTECTEDSESSION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_DESTROYPROTECTEDSESSION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_DESTROYSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ENUMADAPTERS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ENUMADAPTERS2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ENUMADAPTERS3 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS3) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_ESCAPE = Option<unsafe extern "system" fn(param0: *const D3DKMT_ESCAPE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_EVICT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_EVICT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_FLIPOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_FLIPOVERLAY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_FLUSHHEAPTRANSITIONS = Option<unsafe extern "system" fn(param0: *const D3DKMT_FLUSHHEAPTRANSITIONS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_FREEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_FREEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETALLOCATIONPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_GETALLOCATIONPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETCONTEXTSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETCONTEXTSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETDEVICESTATE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETDEVICESTATE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETDISPLAYMODELIST = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETDISPLAYMODELIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETDWMVERTICALBLANKEVENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_GETVERTICALBLANKEVENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETMULTIPLANEOVERLAYCAPS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETMULTISAMPLEMETHODLIST = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETMULTISAMPLEMETHODLIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETOVERLAYSTATE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETOVERLAYSTATE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETPOSTCOMPOSITIONCAPS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GET_POST_COMPOSITION_CAPS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETPRESENTHISTORY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETPRESENTHISTORY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETPROCESSDEVICEREMOVALSUPPORT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETPROCESSSCHEDULINGPRIORITYCLASS = Option<unsafe extern "system" fn(param0: super::super::super::Win32::Foundation::HANDLE, param1: *mut D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETRESOURCEPRESENTPRIVATEDRIVERDATA = Option<unsafe extern "system" fn(param0: *mut D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETRUNTIMEDATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETRUNTIMEDATA) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETSCANLINE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSCANLINE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETSHAREDPRIMARYHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSHAREDPRIMARYHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_GETSHAREDRESOURCEADAPTERLUID = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSHAREDRESOURCEADAPTERLUID) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_INVALIDATEACTIVEVIDPN = Option<unsafe extern "system" fn(param0: *const D3DKMT_INVALIDATEACTIVEVIDPN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_INVALIDATECACHE = Option<unsafe extern "system" fn(param0: *const D3DKMT_INVALIDATECACHE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_LOCK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_LOCK) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_LOCK2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_LOCK2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_MAKERESIDENT = Option<unsafe extern "system" fn(param0: *mut D3DDDI_MAKERESIDENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_MAPGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *mut D3DDDI_MAPGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_MARKDEVICEASERROR = Option<unsafe extern "system" fn(param0: *const D3DKMT_MARKDEVICEASERROR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_NOTIFYWORKSUBMISSION = Option<unsafe extern "system" fn(param0: *const D3DKMT_NOTIFY_WORK_SUBMISSION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OFFERALLOCATIONS = Option<unsafe extern "system" fn(param0: *const D3DKMT_OFFERALLOCATIONS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENADAPTERFROMDEVICENAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMDEVICENAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENADAPTERFROMGDIDISPLAYNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFND3DKMT_OPENADAPTERFROMHDC = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMHDC) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENADAPTERFROMLUID = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMLUID) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEX) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEX2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENKEYEDMUTEXFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENNATIVEFENCEFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENNATIVEFENCEFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFND3DKMT_OPENNTHANDLEFROMNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENNTHANDLEFROMNAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENRESOURCE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENRESOURCE2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENRESOURCEFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCEFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENSYNCOBJECTFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OPENSYNCOBJECTFROMNTHANDLE2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFND3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLGETFRAMEINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_GET_FRAMEINFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLGETMETADATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_METADATA) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLGETPOINTERSHAPEDATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLPRESENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPLPRESENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_OUTPUTDUPLRELEASEFRAME = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPL_RELEASE_FRAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_PINDIRECTFLIPRESOURCES = Option<unsafe extern "system" fn(param0: *const D3DKMT_PINDIRECTFLIPRESOURCES) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_POLLDISPLAYCHILDREN = Option<unsafe extern "system" fn(param0: *const D3DKMT_POLLDISPLAYCHILDREN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_PRESENT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_PRESENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY3 = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY3) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYADAPTERINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYADAPTERINFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYALLOCATIONRESIDENCY = Option<unsafe extern "system" fn(param0: *const D3DKMT_QUERYALLOCATIONRESIDENCY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYCLOCKCALIBRATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYCLOCKCALIBRATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYFSEBLOCK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYFSEBLOCK) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYHYBRIDLISTVALUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_HYBRID_LIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYPROCESSOFFERINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROCESSOFFERINFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYPROTECTEDSESSIONSTATUS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROTECTEDSESSIONSTATUS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYRESOURCEINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYRESOURCEINFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYRESOURCEINFOFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYSTATISTICS = Option<unsafe extern "system" fn(param0: *const D3DKMT_QUERYSTATISTICS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYVIDEOMEMORYINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYVIDEOMEMORYINFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RECLAIMALLOCATIONS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RECLAIMALLOCATIONS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RECLAIMALLOCATIONS2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RECLAIMALLOCATIONS2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_REGISTERBUDGETCHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_REGISTERBUDGETCHANGENOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_REGISTERTRIMNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_REGISTERTRIMNOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RELEASEKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RELEASEKEYEDMUTEX) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RELEASEKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RELEASEKEYEDMUTEX2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RELEASEPROCESSVIDPNSOURCEOWNERS = Option<unsafe extern "system" fn(param0: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RENDER = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RENDER) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_RESERVEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *mut D3DDDI_RESERVEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETALLOCATIONPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETALLOCATIONPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETCONTEXTSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETDISPLAYMODE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_SETDISPLAYMODE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETFSEBLOCK = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETFSEBLOCK) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETGAMMARAMP = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETGAMMARAMP) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETHYBRIDLISTVVALUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_HYBRID_LIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETPROCESSSCHEDULINGPRIORITYCLASS = Option<unsafe extern "system" fn(param0: super::super::super::Win32::Foundation::HANDLE, param1: D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETQUEUEDLIMIT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETQUEUEDLIMIT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETSTABLEPOWERSTATE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETSTABLEPOWERSTATE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETSTEREOENABLED = Option<unsafe extern "system" fn(param0: super::super::super::Win32::Foundation::BOOL) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETSYNCREFRESHCOUNTWAITTARGET = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETVIDPNSOURCEHWPROTECTION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEHWPROTECTION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETVIDPNSOURCEOWNER = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETVIDPNSOURCEOWNER1 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER1) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SETVIDPNSOURCEOWNER2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SHAREDPRIMARYLOCKNOTIFICATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFND3DKMT_SHAREOBJECTS = Option<unsafe extern "system" fn(cobjects: u32, hobjects: *const u32, pobjectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, dwdesiredaccess: u32, phsharednthandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITCOMMAND = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITCOMMAND) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITCOMMANDTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITCOMMANDTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITPRESENTBLTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITPRESENTBLTTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITPRESENTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_SUBMITPRESENTTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_TRIMNOTIFICATIONCALLBACK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_TRIMNOTIFICATION)>;
pub type PFND3DKMT_TRIMPROCESSCOMMITMENT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_TRIMPROCESSCOMMITMENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UNLOCK = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNLOCK) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UNLOCK2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNLOCK2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UNPINDIRECTFLIPRESOURCES = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNPINDIRECTFLIPRESOURCES) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UNREGISTERTRIMNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_UNREGISTERTRIMNOTIFICATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UPDATEALLOCATIONPROPERTY = Option<unsafe extern "system" fn(param0: *mut D3DDDI_UPDATEALLOCPROPERTY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UPDATEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_UPDATEGPUVIRTUALADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_UPDATEOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_UPDATEOVERLAY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORIDLE = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORIDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORVERTICALBLANKEVENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORVERTICALBLANKEVENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DKMT_WAITFORVERTICALBLANKEVENT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORVERTICALBLANKEVENT2) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFND3DNTPARSEUNKNOWNCOMMAND = Option<unsafe extern "system" fn(lpvcommands: *mut core::ffi::c_void, lplpvreturnedcommand: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type PFND3DPARSEUNKNOWNCOMMAND = Option<unsafe extern "system" fn(lpvcommands: *mut core::ffi::c_void, lplpvreturnedcommand: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
