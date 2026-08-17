#[inline]
pub unsafe fn AddClusterGroupDependency(hdependentgroup: HGROUP, hprovidergroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupDependency(hdependentgroup : HGROUP, hprovidergroup : HGROUP) -> u32);
    unsafe { AddClusterGroupDependency(hdependentgroup, hprovidergroup) }
}
#[inline]
pub unsafe fn AddClusterGroupDependencyEx<P2>(hdependentgroup: HGROUP, hprovidergroup: HGROUP, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupDependencyEx(hdependentgroup : HGROUP, hprovidergroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupDependencyEx(hdependentgroup, hprovidergroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterGroupSetDependency(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupSetDependency(hdependentgroupset : HGROUPSET, hprovidergroupset : HGROUPSET) -> u32);
    unsafe { AddClusterGroupSetDependency(hdependentgroupset, hprovidergroupset) }
}
#[inline]
pub unsafe fn AddClusterGroupSetDependencyEx<P2>(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupSetDependencyEx(hdependentgroupset : HGROUPSET, hprovidergroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupSetDependencyEx(hdependentgroupset, hprovidergroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependency(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependency(hdependentgroup : HGROUP, hprovidergroupset : HGROUPSET) -> u32);
    unsafe { AddClusterGroupToGroupSetDependency(hdependentgroup, hprovidergroupset) }
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependencyEx<P2>(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependencyEx(hdependentgroup : HGROUP, hprovidergroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupToGroupSetDependencyEx(hdependentgroup, hprovidergroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterNode<P1>(hcluster: HCLUSTER, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    unsafe { AddClusterNode(hcluster, lpsznodename.param().abi(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddClusterNodeEx<P1>(hcluster: HCLUSTER, lpsznodename: P1, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterNodeEx(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, dwflags : u32, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    unsafe { AddClusterNodeEx(hcluster, lpsznodename.param().abi(), dwflags, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddClusterResourceDependency(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceDependency(hresource : HRESOURCE, hdependson : HRESOURCE) -> u32);
    unsafe { AddClusterResourceDependency(hresource, hdependson) }
}
#[inline]
pub unsafe fn AddClusterResourceDependencyEx<P2>(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceDependencyEx(hresource : HRESOURCE, hdependson : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterResourceDependencyEx(hresource, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterResourceNode(hresource: HRESOURCE, hnode: HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceNode(hresource : HRESOURCE, hnode : HNODE) -> u32);
    unsafe { AddClusterResourceNode(hresource, hnode) }
}
#[inline]
pub unsafe fn AddClusterResourceNodeEx<P2>(hresource: HRESOURCE, hnode: HNODE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceNodeEx(hresource : HRESOURCE, hnode : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterResourceNodeEx(hresource, hnode, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterStorageNode<P1, P4, P5>(hcluster: HCLUSTER, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, lpszclusterstoragenodedescription: P4, lpszclusterstoragenodelocation: P5) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterStorageNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, lpszclusterstoragenodedescription : windows_core::PCWSTR, lpszclusterstoragenodelocation : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterStorageNode(hcluster, lpsznodename.param().abi(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _, lpszclusterstoragenodedescription.param().abi(), lpszclusterstoragenodelocation.param().abi()) }
}
#[inline]
pub unsafe fn AddCrossClusterGroupSetDependency<P1, P2>(hdependentgroupset: HGROUPSET, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddCrossClusterGroupSetDependency(hdependentgroupset : HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    unsafe { AddCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn AddResourceToClusterSharedVolumes(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddResourceToClusterSharedVolumes(hresource : HRESOURCE) -> u32);
    unsafe { AddResourceToClusterSharedVolumes(hresource) }
}
#[inline]
pub unsafe fn BackupClusterDatabase<P1>(hcluster: HCLUSTER, lpszpathname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn BackupClusterDatabase(hcluster : HCLUSTER, lpszpathname : windows_core::PCWSTR) -> u32);
    unsafe { BackupClusterDatabase(hcluster, lpszpathname.param().abi()) }
}
#[inline]
pub unsafe fn CanResourceBeDependent(hresource: HRESOURCE, hresourcedependent: HRESOURCE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CanResourceBeDependent(hresource : HRESOURCE, hresourcedependent : HRESOURCE) -> windows_core::BOOL);
    unsafe { CanResourceBeDependent(hresource, hresourcedependent) }
}
#[inline]
pub unsafe fn CancelClusterGroupOperation(hgroup: HGROUP, dwcancelflags_reserved: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn CancelClusterGroupOperation(hgroup : HGROUP, dwcancelflags_reserved : u32) -> u32);
    unsafe { CancelClusterGroupOperation(hgroup, dwcancelflags_reserved) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroup(hresource: HRESOURCE, hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroup(hresource : HRESOURCE, hgroup : HGROUP) -> u32);
    unsafe { ChangeClusterResourceGroup(hresource, hgroup) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx(hresource: HRESOURCE, hgroup: HGROUP, flags: u64) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx(hresource : HRESOURCE, hgroup : HGROUP, flags : u64) -> u32);
    unsafe { ChangeClusterResourceGroupEx(hresource, hgroup, flags) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx2<P3>(hresource: HRESOURCE, hgroup: HGROUP, flags: u64, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx2(hresource : HRESOURCE, hgroup : HGROUP, flags : u64, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ChangeClusterResourceGroupEx2(hresource, hgroup, flags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn CloseCluster(hcluster: HCLUSTER) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseCluster(hcluster : HCLUSTER) -> windows_core::BOOL);
    unsafe { CloseCluster(hcluster) }
}
#[inline]
pub unsafe fn CloseClusterCryptProvider(hcluscryptprovider: HCLUSCRYPTPROVIDER) -> u32 {
    windows_core::link!("resutils.dll" "system" fn CloseClusterCryptProvider(hcluscryptprovider : HCLUSCRYPTPROVIDER) -> u32);
    unsafe { CloseClusterCryptProvider(hcluscryptprovider) }
}
#[inline]
pub unsafe fn CloseClusterGroup(hgroup: HGROUP) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterGroup(hgroup : HGROUP) -> windows_core::BOOL);
    unsafe { CloseClusterGroup(hgroup).ok() }
}
#[inline]
pub unsafe fn CloseClusterGroupSet(hgroupset: HGROUPSET) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterGroupSet(hgroupset : HGROUPSET) -> windows_core::BOOL);
    unsafe { CloseClusterGroupSet(hgroupset).ok() }
}
#[inline]
pub unsafe fn CloseClusterNetInterface(hnetinterface: HNETINTERFACE) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNetInterface(hnetinterface : HNETINTERFACE) -> windows_core::BOOL);
    unsafe { CloseClusterNetInterface(hnetinterface).ok() }
}
#[inline]
pub unsafe fn CloseClusterNetwork(hnetwork: HNETWORK) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNetwork(hnetwork : HNETWORK) -> windows_core::BOOL);
    unsafe { CloseClusterNetwork(hnetwork).ok() }
}
#[inline]
pub unsafe fn CloseClusterNode(hnode: HNODE) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNode(hnode : HNODE) -> windows_core::BOOL);
    unsafe { CloseClusterNode(hnode).ok() }
}
#[inline]
pub unsafe fn CloseClusterNotifyPort(hchange: HCHANGE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNotifyPort(hchange : HCHANGE) -> windows_core::BOOL);
    unsafe { CloseClusterNotifyPort(hchange) }
}
#[inline]
pub unsafe fn CloseClusterResource(hresource: HRESOURCE) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterResource(hresource : HRESOURCE) -> windows_core::BOOL);
    unsafe { CloseClusterResource(hresource).ok() }
}
#[inline]
pub unsafe fn ClusAddClusterHealthFault(hcluster: HCLUSTER, failure: *const CLUSTER_HEALTH_FAULT, param2: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusAddClusterHealthFault(hcluster : HCLUSTER, failure : *const CLUSTER_HEALTH_FAULT, param2 : u32) -> u32);
    unsafe { ClusAddClusterHealthFault(hcluster, failure, param2) }
}
#[inline]
pub unsafe fn ClusGetClusterHealthFaults(hcluster: HCLUSTER, objects: *mut CLUSTER_HEALTH_FAULT_ARRAY, flags: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusGetClusterHealthFaults(hcluster : HCLUSTER, objects : *mut CLUSTER_HEALTH_FAULT_ARRAY, flags : u32) -> u32);
    unsafe { ClusGetClusterHealthFaults(hcluster, objects as _, flags) }
}
#[inline]
pub unsafe fn ClusRemoveClusterHealthFault<P1>(hcluster: HCLUSTER, id: P1, flags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusRemoveClusterHealthFault(hcluster : HCLUSTER, id : windows_core::PCWSTR, flags : u32) -> u32);
    unsafe { ClusRemoveClusterHealthFault(hcluster, id.param().abi(), flags) }
}
#[inline]
pub unsafe fn ClusWorkerCheckTerminate(lpworker: *mut CLUS_WORKER) -> windows_core::BOOL {
    windows_core::link!("resutils.dll" "system" fn ClusWorkerCheckTerminate(lpworker : *mut CLUS_WORKER) -> windows_core::BOOL);
    unsafe { ClusWorkerCheckTerminate(lpworker as _) }
}
#[inline]
pub unsafe fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusWorkerCreate(lpworker : *mut CLUS_WORKER, lpstartaddress : PWORKER_START_ROUTINE, lpparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ClusWorkerCreate(lpworker as _, lpstartaddress, lpparameter as _) }
}
#[inline]
pub unsafe fn ClusWorkerTerminate(lpworker: *const CLUS_WORKER) {
    windows_core::link!("resutils.dll" "system" fn ClusWorkerTerminate(lpworker : *const CLUS_WORKER));
    unsafe { ClusWorkerTerminate(lpworker) }
}
#[inline]
pub unsafe fn ClusWorkerTerminateEx(clusworker: *mut CLUS_WORKER, timeoutinmilliseconds: u32, waitonly: bool) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusWorkerTerminateEx(clusworker : *mut CLUS_WORKER, timeoutinmilliseconds : u32, waitonly : windows_core::BOOL) -> u32);
    unsafe { ClusWorkerTerminateEx(clusworker as _, timeoutinmilliseconds, waitonly.into()) }
}
#[inline]
pub unsafe fn ClusWorkersTerminate(clusworkers: &mut [*mut CLUS_WORKER], timeoutinmilliseconds: u32, waitonly: bool) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusWorkersTerminate(clusworkers : *mut *mut CLUS_WORKER, clusworkerscount : usize, timeoutinmilliseconds : u32, waitonly : windows_core::BOOL) -> u32);
    unsafe { ClusWorkersTerminate(core::mem::transmute(clusworkers.as_ptr()), clusworkers.len().try_into().unwrap(), timeoutinmilliseconds, waitonly.into()) }
}
#[inline]
pub unsafe fn ClusapiSetReasonHandler(lphandler: *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER {
    windows_core::link!("clusapi.dll" "system" fn ClusapiSetReasonHandler(lphandler : *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER);
    unsafe { ClusapiSetReasonHandler(lphandler) }
}
#[inline]
pub unsafe fn ClusterAddGroupToAffinityRule<P1>(hcluster: HCLUSTER, rulename: P1, hgroup: HGROUP) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, hgroup : HGROUP) -> u32);
    unsafe { ClusterAddGroupToAffinityRule(hcluster, rulename.param().abi(), hgroup) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSet(hgroupset: HGROUPSET, hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSet(hgroupset : HGROUPSET, hgroup : HGROUP) -> u32);
    unsafe { ClusterAddGroupToGroupSet(hgroupset, hgroup) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomains(hgroupset: HGROUPSET, hgroup: HGROUP, faultdomain: u32, updatedomain: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomains(hgroupset : HGROUPSET, hgroup : HGROUP, faultdomain : u32, updatedomain : u32) -> u32);
    unsafe { ClusterAddGroupToGroupSetWithDomains(hgroupset, hgroup, faultdomain, updatedomain) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomainsEx<P4>(hgroupset: HGROUPSET, hgroup: HGROUP, faultdomain: u32, updatedomain: u32, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomainsEx(hgroupset : HGROUPSET, hgroup : HGROUP, faultdomain : u32, updatedomain : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterAddGroupToGroupSetWithDomainsEx(hgroupset, hgroup, faultdomain, updatedomain, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterAffinityRuleControl<P1>(hcluster: HCLUSTER, affinityrulename: P1, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAffinityRuleControl(hcluster : HCLUSTER, affinityrulename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterAffinityRuleControl(hcluster, affinityrulename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterClearBackupStateForSharedVolume<P0>(lpszvolumepathname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusterClearBackupStateForSharedVolume(lpszvolumepathname : windows_core::PCWSTR) -> u32);
    unsafe { ClusterClearBackupStateForSharedVolume(lpszvolumepathname.param().abi()) }
}
#[inline]
pub unsafe fn ClusterCloseEnum(henum: HCLUSENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterCloseEnum(henum : HCLUSENUM) -> u32);
    unsafe { ClusterCloseEnum(henum) }
}
#[inline]
pub unsafe fn ClusterCloseEnumEx(hclusterenum: HCLUSENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterCloseEnumEx(hclusterenum : HCLUSENUMEX) -> u32);
    unsafe { ClusterCloseEnumEx(hclusterenum) }
}
#[inline]
pub unsafe fn ClusterControl(hcluster: HCLUSTER, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterControl(hcluster : HCLUSTER, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterControl(hcluster, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterControlEx<P8>(hcluster: HCLUSTER, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterControlEx(hcluster : HCLUSTER, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterControlEx(hcluster, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterCreateAffinityRule<P1>(hcluster: HCLUSTER, rulename: P1, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterCreateAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, ruletype : CLUS_AFFINITY_RULE_TYPE) -> u32);
    unsafe { ClusterCreateAffinityRule(hcluster, rulename.param().abi(), ruletype) }
}
#[inline]
pub unsafe fn ClusterDecrypt(hcluscryptprovider: HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusterDecrypt(hcluscryptprovider : HCLUSCRYPTPROVIDER, pcryptinput : *const u8, cbcryptinput : u32, ppcryptoutput : *mut *mut u8, pcbcryptoutput : *mut u32) -> u32);
    unsafe { ClusterDecrypt(hcluscryptprovider, pcryptinput, cbcryptinput, ppcryptoutput as _, pcbcryptoutput as _) }
}
#[inline]
pub unsafe fn ClusterEncrypt(hcluscryptprovider: HCLUSCRYPTPROVIDER, pdata: &[u8], ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ClusterEncrypt(hcluscryptprovider : HCLUSCRYPTPROVIDER, pdata : *const u8, cbdata : u32, ppdata : *mut *mut u8, pcbdata : *mut u32) -> u32);
    unsafe { ClusterEncrypt(hcluscryptprovider, core::mem::transmute(pdata.as_ptr()), pdata.len().try_into().unwrap(), ppdata as _, pcbdata as _) }
}
#[inline]
pub unsafe fn ClusterEnum(henum: HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterEnum(henum : HCLUSENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterEnum(henum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterEnumEx(hclusterenum: HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterEnumEx(hclusterenum : HCLUSENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterEnumEx(hclusterenum, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterGetEnumCount(henum: HCLUSENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGetEnumCount(henum : HCLUSENUM) -> u32);
    unsafe { ClusterGetEnumCount(henum) }
}
#[inline]
pub unsafe fn ClusterGetEnumCountEx(hclusterenum: HCLUSENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGetEnumCountEx(hclusterenum : HCLUSENUMEX) -> u32);
    unsafe { ClusterGetEnumCountEx(hclusterenum) }
}
#[inline]
pub unsafe fn ClusterGetVolumeNameForVolumeMountPoint<P0>(lpszvolumemountpoint: P0, lpszvolumename: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PWSTR, cchbufferlength : u32) -> windows_core::BOOL);
    unsafe { ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint.param().abi(), core::mem::transmute(lpszvolumename), cchbufferlength).ok() }
}
#[inline]
pub unsafe fn ClusterGetVolumePathName<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusterGetVolumePathName(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, cchbufferlength : u32) -> windows_core::BOOL);
    unsafe { ClusterGetVolumePathName(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname), cchbufferlength).ok() }
}
#[inline]
pub unsafe fn ClusterGroupCloseEnum(hgroupenum: HGROUPENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupCloseEnum(hgroupenum : HGROUPENUM) -> u32);
    unsafe { ClusterGroupCloseEnum(hgroupenum) }
}
#[inline]
pub unsafe fn ClusterGroupCloseEnumEx(hgroupenumex: HGROUPENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupCloseEnumEx(hgroupenumex : HGROUPENUMEX) -> u32);
    unsafe { ClusterGroupCloseEnumEx(hgroupenumex) }
}
#[inline]
pub unsafe fn ClusterGroupControl(hgroup: HGROUP, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupControl(hgroup : HGROUP, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterGroupControl(hgroup, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterGroupControlEx<P8>(hgroup: HGROUP, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupControlEx(hgroup : HGROUP, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterGroupControlEx(hgroup, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterGroupEnum(hgroupenum: HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupEnum(hgroupenum : HGROUPENUM, dwindex : u32, lpdwtype : *mut u32, lpszresourcename : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterGroupEnum(hgroupenum, dwindex, lpdwtype as _, core::mem::transmute(lpszresourcename), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterGroupEnumEx(hgroupenumex: HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupEnumEx(hgroupenumex : HGROUPENUMEX, dwindex : u32, pitem : *mut CLUSTER_GROUP_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterGroupEnumEx(hgroupenumex, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCount(hgroupenum: HGROUPENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCount(hgroupenum : HGROUPENUM) -> u32);
    unsafe { ClusterGroupGetEnumCount(hgroupenum) }
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCountEx(hgroupenumex: HGROUPENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCountEx(hgroupenumex : HGROUPENUMEX) -> u32);
    unsafe { ClusterGroupGetEnumCountEx(hgroupenumex) }
}
#[inline]
pub unsafe fn ClusterGroupOpenEnum(hgroup: HGROUP, dwtype: u32) -> HGROUPENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupOpenEnum(hgroup : HGROUP, dwtype : u32) -> HGROUPENUM);
    unsafe { ClusterGroupOpenEnum(hgroup, dwtype) }
}
#[inline]
pub unsafe fn ClusterGroupOpenEnumEx<P1, P3>(hcluster: HCLUSTER, lpszproperties: P1, cbproperties: u32, lpszroproperties: P3, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupOpenEnumEx(hcluster : HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HGROUPENUMEX);
    unsafe { ClusterGroupOpenEnumEx(hcluster, lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags) }
}
#[inline]
pub unsafe fn ClusterGroupSetCloseEnum(hgroupsetenum: HGROUPSETENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetCloseEnum(hgroupsetenum : HGROUPSETENUM) -> u32);
    unsafe { ClusterGroupSetCloseEnum(hgroupsetenum) }
}
#[inline]
pub unsafe fn ClusterGroupSetControl(hgroupset: HGROUPSET, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetControl(hgroupset : HGROUPSET, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterGroupSetControl(hgroupset, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetControlEx<P8>(hgroupset: HGROUPSET, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetControlEx(hgroupset : HGROUPSET, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterGroupSetControlEx(hgroupset, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterGroupSetEnum(hgroupsetenum: HGROUPSETENUM, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetEnum(hgroupsetenum : HGROUPSETENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterGroupSetEnum(hgroupsetenum, dwindex, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetGetEnumCount(hgroupsetenum: HGROUPSETENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetGetEnumCount(hgroupsetenum : HGROUPSETENUM) -> u32);
    unsafe { ClusterGroupSetGetEnumCount(hgroupsetenum) }
}
#[inline]
pub unsafe fn ClusterGroupSetOpenEnum(hcluster: HCLUSTER) -> HGROUPSETENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetOpenEnum(hcluster : HCLUSTER) -> HGROUPSETENUM);
    unsafe { ClusterGroupSetOpenEnum(hcluster) }
}
#[inline]
pub unsafe fn ClusterIsPathOnSharedVolume<P0>(lpszpathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusterIsPathOnSharedVolume(lpszpathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ClusterIsPathOnSharedVolume(lpszpathname.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum: HNETINTERFACEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum : HNETINTERFACEENUM) -> u32);
    unsafe { ClusterNetInterfaceCloseEnum(hnetinterfaceenum) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceControl(hnetinterface: HNETINTERFACE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceControl(hnetinterface : HNETINTERFACE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNetInterfaceControl(hnetinterface, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceControlEx<P8>(hnetinterface: HNETINTERFACE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceControlEx(hnetinterface : HNETINTERFACE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNetInterfaceControlEx(hnetinterface, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceEnum(hnetinterfaceenum: HNETINTERFACEENUM, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceEnum(hnetinterfaceenum : HNETINTERFACEENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNetInterfaceEnum(hnetinterfaceenum, dwindex, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceOpenEnum<P1, P2>(hcluster: HCLUSTER, lpsznodename: P1, lpsznetworkname: P2) -> HNETINTERFACEENUM
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceOpenEnum(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR) -> HNETINTERFACEENUM);
    unsafe { ClusterNetInterfaceOpenEnum(hcluster, lpsznodename.param().abi(), lpsznetworkname.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetworkCloseEnum(hnetworkenum: HNETWORKENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkCloseEnum(hnetworkenum : HNETWORKENUM) -> u32);
    unsafe { ClusterNetworkCloseEnum(hnetworkenum) }
}
#[inline]
pub unsafe fn ClusterNetworkControl(hnetwork: HNETWORK, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkControl(hnetwork : HNETWORK, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNetworkControl(hnetwork, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNetworkControlEx<P8>(hnetwork: HNETWORK, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkControlEx(hnetwork : HNETWORK, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNetworkControlEx(hnetwork, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetworkEnum(hnetworkenum: HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkEnum(hnetworkenum : HNETWORKENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNetworkEnum(hnetworkenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNetworkGetEnumCount(hnetworkenum: HNETWORKENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkGetEnumCount(hnetworkenum : HNETWORKENUM) -> u32);
    unsafe { ClusterNetworkGetEnumCount(hnetworkenum) }
}
#[inline]
pub unsafe fn ClusterNetworkOpenEnum(hnetwork: HNETWORK, dwtype: u32) -> HNETWORKENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkOpenEnum(hnetwork : HNETWORK, dwtype : u32) -> HNETWORKENUM);
    unsafe { ClusterNetworkOpenEnum(hnetwork, dwtype) }
}
#[inline]
pub unsafe fn ClusterNodeCloseEnum(hnodeenum: HNODEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeCloseEnum(hnodeenum : HNODEENUM) -> u32);
    unsafe { ClusterNodeCloseEnum(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeCloseEnumEx(hnodeenum: HNODEENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeCloseEnumEx(hnodeenum : HNODEENUMEX) -> u32);
    unsafe { ClusterNodeCloseEnumEx(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeControl(hnode: HNODE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeControl(hnode : HNODE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNodeControl(hnode, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNodeControlEx<P8>(hnode: HNODE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeControlEx(hnode : HNODE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNodeControlEx(hnode, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNodeEnum(hnodeenum: HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeEnum(hnodeenum : HNODEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNodeEnum(hnodeenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNodeEnumEx(hnodeenum: HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeEnumEx(hnodeenum : HNODEENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterNodeEnumEx(hnodeenum, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCount(hnodeenum: HNODEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCount(hnodeenum : HNODEENUM) -> u32);
    unsafe { ClusterNodeGetEnumCount(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCountEx(hnodeenum: HNODEENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCountEx(hnodeenum : HNODEENUMEX) -> u32);
    unsafe { ClusterNodeGetEnumCountEx(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeOpenEnum(hnode: HNODE, dwtype: u32) -> HNODEENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeOpenEnum(hnode : HNODE, dwtype : u32) -> HNODEENUM);
    unsafe { ClusterNodeOpenEnum(hnode, dwtype) }
}
#[inline]
pub unsafe fn ClusterNodeOpenEnumEx(hnode: HNODE, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HNODEENUMEX {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeOpenEnumEx(hnode : HNODE, dwtype : u32, poptions : *const core::ffi::c_void) -> HNODEENUMEX);
    unsafe { ClusterNodeOpenEnumEx(hnode, dwtype, poptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNodeReplacement<P1, P2>(hcluster: HCLUSTER, lpsznodenamecurrent: P1, lpsznodenamenew: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeReplacement(hcluster : HCLUSTER, lpsznodenamecurrent : windows_core::PCWSTR, lpsznodenamenew : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNodeReplacement(hcluster, lpsznodenamecurrent.param().abi(), lpsznodenamenew.param().abi()) }
}
#[inline]
pub unsafe fn ClusterOpenEnum(hcluster: HCLUSTER, dwtype: u32) -> HCLUSENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterOpenEnum(hcluster : HCLUSTER, dwtype : u32) -> HCLUSENUM);
    unsafe { ClusterOpenEnum(hcluster, dwtype) }
}
#[inline]
pub unsafe fn ClusterOpenEnumEx(hcluster: HCLUSTER, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HCLUSENUMEX {
    windows_core::link!("clusapi.dll" "system" fn ClusterOpenEnumEx(hcluster : HCLUSTER, dwtype : u32, poptions : *const core::ffi::c_void) -> HCLUSENUMEX);
    unsafe { ClusterOpenEnumEx(hcluster, dwtype, poptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterPrepareSharedVolumeForBackup<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: windows_core::PWSTR, lpcchvolumename: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ClusterPrepareSharedVolumeForBackup(lpszfilename : windows_core::PCWSTR, lpszvolumepathname : windows_core::PWSTR, lpcchvolumepathname : *mut u32, lpszvolumename : windows_core::PWSTR, lpcchvolumename : *mut u32) -> u32);
    unsafe { ClusterPrepareSharedVolumeForBackup(lpszfilename.param().abi(), core::mem::transmute(lpszvolumepathname), lpcchvolumepathname as _, core::mem::transmute(lpszvolumename), lpcchvolumename as _) }
}
#[inline]
pub unsafe fn ClusterRegBatchAddCommand<P2>(hregbatch: HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: P2, dwoptions: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchAddCommand(hregbatch : HREGBATCH, dwcommand : CLUSTER_REG_COMMAND, wzname : windows_core::PCWSTR, dwoptions : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> i32);
    unsafe { ClusterRegBatchAddCommand(hregbatch, dwcommand, wzname.param().abi(), dwoptions, lpdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[inline]
pub unsafe fn ClusterRegBatchCloseNotification(hbatchnotification: HREGBATCHNOTIFICATION) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchCloseNotification(hbatchnotification : HREGBATCHNOTIFICATION) -> i32);
    unsafe { ClusterRegBatchCloseNotification(hbatchnotification) }
}
#[inline]
pub unsafe fn ClusterRegBatchReadCommand(hbatchnotification: HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchReadCommand(hbatchnotification : HREGBATCHNOTIFICATION, pbatchcommand : *mut CLUSTER_BATCH_COMMAND) -> i32);
    unsafe { ClusterRegBatchReadCommand(hbatchnotification, pbatchcommand as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatch(hregbatch: HREGBATCH, bcommit: bool, failedcommandnumber: Option<*mut i32>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatch(hregbatch : HREGBATCH, bcommit : windows_core::BOOL, failedcommandnumber : *mut i32) -> i32);
    unsafe { ClusterRegCloseBatch(hregbatch, bcommit.into(), failedcommandnumber.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatchEx(hregbatch: HREGBATCH, flags: u32, failedcommandnumber: Option<*mut i32>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatchEx(hregbatch : HREGBATCH, flags : u32, failedcommandnumber : *mut i32) -> i32);
    unsafe { ClusterRegCloseBatchEx(hregbatch, flags, failedcommandnumber.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport: HREGBATCHPORT) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport : HREGBATCHPORT) -> i32);
    unsafe { ClusterRegCloseBatchNotifyPort(hbatchnotifyport) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCloseKey(hkey: super::super::System::Registry::HKEY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseKey(hkey : super::super::System::Registry:: HKEY) -> i32);
    unsafe { ClusterRegCloseKey(hkey) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatch(hregreadbatch: HREGREADBATCH, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatch(hregreadbatch : HREGREADBATCH, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatch(hregreadbatch, phregreadbatchreply as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchEx(hregreadbatch: HREGREADBATCH, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchEx(hregreadbatch : HREGREADBATCH, flags : u32, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatchEx(hregreadbatch, flags, phregreadbatchreply as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchReply(hregreadbatchreply: HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchReply(hregreadbatchreply : HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatchReply(hregreadbatchreply) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatch(hkey: Option<super::super::System::Registry::HKEY>, phregbatch: *mut HREGBATCH) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateBatch(hkey : super::super::System::Registry:: HKEY, phregbatch : *mut HREGBATCH) -> i32);
    unsafe { ClusterRegCreateBatch(hkey.unwrap_or(core::mem::zeroed()) as _, phregbatch as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateBatchNotifyPort(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut HREGBATCHPORT) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateBatchNotifyPort(hkey : super::super::System::Registry:: HKEY, phbatchnotifyport : *mut HREGBATCHPORT) -> i32);
    unsafe { ClusterRegCreateBatchNotifyPort(hkey, phbatchnotifyport as _) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegCreateKey<P1>(hkey: super::super::System::Registry::HKEY, lpszsubkey: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32) -> i32);
    unsafe { ClusterRegCreateKey(hkey, lpszsubkey.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegCreateKeyEx<P1, P7>(hkey: super::super::System::Registry::HKEY, lpsubkey: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: Option<*mut u32>, lpszreason: P7) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateKeyEx(hkey : super::super::System::Registry:: HKEY, lpsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegCreateKeyEx(hkey, lpsubkey.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegCreateReadBatch(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut HREGREADBATCH) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateReadBatch(hkey : super::super::System::Registry:: HKEY, phregreadbatch : *mut HREGREADBATCH) -> i32);
    unsafe { ClusterRegCreateReadBatch(hkey, phregreadbatch as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteKey<P1>(hkey: super::super::System::Registry::HKEY, lpszsubkey: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegDeleteKey(hkey, lpszsubkey.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteKeyEx<P1, P2>(hkey: super::super::System::Registry::HKEY, lpsubkey: P1, lpszreason: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteKeyEx(hkey : super::super::System::Registry:: HKEY, lpsubkey : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegDeleteKeyEx(hkey, lpsubkey.param().abi(), lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteValue<P1>(hkey: super::super::System::Registry::HKEY, lpszvaluename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegDeleteValue(hkey, lpszvaluename.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegDeleteValueEx<P1, P2>(hkey: super::super::System::Registry::HKEY, lpszvaluename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteValueEx(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegDeleteValueEx(hkey, lpszvaluename.param().abi(), lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegEnumKey(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: Option<*mut super::super::Foundation::FILETIME>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegEnumKey(hkey : super::super::System::Registry:: HKEY, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpftlastwritetime : *mut super::super::Foundation:: FILETIME) -> i32);
    unsafe { ClusterRegEnumKey(hkey, dwindex, core::mem::transmute(lpszname), lpcchname as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegEnumValue(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegEnumValue(hkey : super::super::System::Registry:: HKEY, dwindex : u32, lpszvaluename : windows_core::PWSTR, lpcchvaluename : *mut u32, lpdwtype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> u32);
    unsafe { ClusterRegEnumValue(hkey, dwindex, core::mem::transmute(lpszvaluename), lpcchvaluename as _, lpdwtype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegGetBatchNotification(hbatchnotify: HREGBATCHPORT, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegGetBatchNotification(hbatchnotify : HREGBATCHPORT, phbatchnotification : *mut HREGBATCHNOTIFICATION) -> i32);
    unsafe { ClusterRegGetBatchNotification(hbatchnotify, phbatchnotification as _) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegGetKeySecurity(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegGetKeySecurity(hkey : super::super::System::Registry:: HKEY, requestedinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> i32);
    unsafe { ClusterRegGetKeySecurity(hkey, requestedinformation, psecuritydescriptor as _, lpcbsecuritydescriptor as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegOpenKey<P1>(hkey: super::super::System::Registry::HKEY, lpszsubkey: P1, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegOpenKey(hkey : super::super::System::Registry:: HKEY, lpszsubkey : windows_core::PCWSTR, samdesired : u32, phkresult : *mut super::super::System::Registry:: HKEY) -> i32);
    unsafe { ClusterRegOpenKey(hkey, lpszsubkey.param().abi(), samdesired, phkresult as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegQueryInfoKey(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::super::Foundation::FILETIME) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegQueryInfoKey(hkey : super::super::System::Registry:: HKEY, lpcsubkeys : *const u32, lpcchmaxsubkeylen : *const u32, lpcvalues : *const u32, lpcchmaxvaluenamelen : *const u32, lpcbmaxvaluelen : *const u32, lpcbsecuritydescriptor : *const u32, lpftlastwritetime : *const super::super::Foundation:: FILETIME) -> i32);
    unsafe { ClusterRegQueryInfoKey(hkey, lpcsubkeys, lpcchmaxsubkeylen, lpcvalues, lpcchmaxvaluenamelen, lpcbmaxvaluelen, lpcbsecuritydescriptor, lpftlastwritetime) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegQueryValue<P1>(hkey: super::super::System::Registry::HKEY, lpszvaluename: P1, lpdwvaluetype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegQueryValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, lpdwvaluetype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> i32);
    unsafe { ClusterRegQueryValue(hkey, lpszvaluename.param().abi(), lpdwvaluetype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegReadBatchAddCommand<P1, P2>(hregreadbatch: HREGREADBATCH, wzsubkeyname: P1, wzvaluename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegReadBatchAddCommand(hregreadbatch : HREGREADBATCH, wzsubkeyname : windows_core::PCWSTR, wzvaluename : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegReadBatchAddCommand(hregreadbatch, wzsubkeyname.param().abi(), wzvaluename.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply: HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply : HREGREADBATCHREPLY, pbatchcommand : *mut CLUSTER_READ_BATCH_COMMAND) -> i32);
    unsafe { ClusterRegReadBatchReplyNextCommand(hregreadbatchreply, pbatchcommand as _) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurity(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurity(hkey : super::super::System::Registry:: HKEY, securityinformation : u32, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> i32);
    unsafe { ClusterRegSetKeySecurity(hkey, securityinformation, psecuritydescriptor) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurityEx<P3>(hkey: super::super::System::Registry::HKEY, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpszreason: P3) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurityEx(hkey : super::super::System::Registry:: HKEY, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegSetKeySecurityEx(hkey, securityinformation, psecuritydescriptor, lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegSetValue<P1>(hkey: super::super::System::Registry::HKEY, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetValue(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
    unsafe { ClusterRegSetValue(hkey, lpszvaluename.param().abi(), dwtype, lpdata, cbdata) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ClusterRegSetValueEx<P1, P5>(hkey: super::super::System::Registry::HKEY, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: P5) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetValueEx(hkey : super::super::System::Registry:: HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegSetValueEx(hkey, lpszvaluename.param().abi(), dwtype, lpdata, cbdata, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRegSyncDatabase(hcluster: HCLUSTER, flags: u32) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSyncDatabase(hcluster : HCLUSTER, flags : u32) -> i32);
    unsafe { ClusterRegSyncDatabase(hcluster, flags) }
}
#[inline]
pub unsafe fn ClusterRemoveAffinityRule<P1>(hcluster: HCLUSTER, rulename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRemoveAffinityRule(hcluster, rulename.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromAffinityRule<P1>(hcluster: HCLUSTER, rulename: P1, hgroup: HGROUP) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromAffinityRule(hcluster : HCLUSTER, rulename : windows_core::PCWSTR, hgroup : HGROUP) -> u32);
    unsafe { ClusterRemoveGroupFromAffinityRule(hcluster, rulename.param().abi(), hgroup) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSet(hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSet(hgroup : HGROUP) -> u32);
    unsafe { ClusterRemoveGroupFromGroupSet(hgroup) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSetEx<P1>(hgroup: HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSetEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRemoveGroupFromGroupSetEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceCloseEnum(hresenum: HRESENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceCloseEnum(hresenum : HRESENUM) -> u32);
    unsafe { ClusterResourceCloseEnum(hresenum) }
}
#[inline]
pub unsafe fn ClusterResourceCloseEnumEx(hresourceenumex: HRESENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceCloseEnumEx(hresourceenumex : HRESENUMEX) -> u32);
    unsafe { ClusterResourceCloseEnumEx(hresourceenumex) }
}
#[inline]
pub unsafe fn ClusterResourceControl(hresource: HRESOURCE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControl(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceControl(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceControlAsUser(hresource: HRESOURCE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlAsUser(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceControlAsUser(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceControlAsUserEx<P8>(hresource: HRESOURCE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlAsUserEx(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceControlAsUserEx(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceControlEx<P8>(hresource: HRESOURCE, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlEx(hresource : HRESOURCE, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceControlEx(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceEnum(hresenum: HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceEnum(hresenum : HRESENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterResourceEnum(hresenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterResourceEnumEx(hresourceenumex: HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceEnumEx(hresourceenumex : HRESENUMEX, dwindex : u32, pitem : *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterResourceEnumEx(hresourceenumex, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCount(hresenum: HRESENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCount(hresenum : HRESENUM) -> u32);
    unsafe { ClusterResourceGetEnumCount(hresenum) }
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCountEx(hresourceenumex: HRESENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCountEx(hresourceenumex : HRESENUMEX) -> u32);
    unsafe { ClusterResourceGetEnumCountEx(hresourceenumex) }
}
#[inline]
pub unsafe fn ClusterResourceOpenEnum(hresource: HRESOURCE, dwtype: u32) -> HRESENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceOpenEnum(hresource : HRESOURCE, dwtype : u32) -> HRESENUM);
    unsafe { ClusterResourceOpenEnum(hresource, dwtype) }
}
#[inline]
pub unsafe fn ClusterResourceOpenEnumEx<P1, P3>(hcluster: HCLUSTER, lpszproperties: P1, cbproperties: u32, lpszroproperties: P3, cbroproperties: u32, dwflags: u32) -> HRESENUMEX
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceOpenEnumEx(hcluster : HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HRESENUMEX);
    unsafe { ClusterResourceOpenEnumEx(hcluster, lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags) }
}
#[inline]
pub unsafe fn ClusterResourceTypeCloseEnum(hrestypeenum: HRESTYPEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeCloseEnum(hrestypeenum : HRESTYPEENUM) -> u32);
    unsafe { ClusterResourceTypeCloseEnum(hrestypeenum) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControl<P1>(hcluster: HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControl(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceTypeControl(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUser<P1>(hcluster: HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUser(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceTypeControlAsUser(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUserEx<P1, P9>(hcluster: HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P9) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P9: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUserEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceTypeControlAsUserEx(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlEx<P1, P9>(hcluster: HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P9) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P9: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceTypeControlEx(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceTypeEnum(hrestypeenum: HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeEnum(hrestypeenum : HRESTYPEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterResourceTypeEnum(hrestypeenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeGetEnumCount(hrestypeenum: HRESTYPEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeGetEnumCount(hrestypeenum : HRESTYPEENUM) -> u32);
    unsafe { ClusterResourceTypeGetEnumCount(hrestypeenum) }
}
#[inline]
pub unsafe fn ClusterResourceTypeOpenEnum<P1>(hcluster: HCLUSTER, lpszresourcetypename: P1, dwtype: u32) -> HRESTYPEENUM
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeOpenEnum(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, dwtype : u32) -> HRESTYPEENUM);
    unsafe { ClusterResourceTypeOpenEnum(hcluster, lpszresourcetypename.param().abi(), dwtype) }
}
#[inline]
pub unsafe fn ClusterSetAccountAccess<P1>(hcluster: HCLUSTER, szaccountsid: P1, dwaccess: u32, dwcontroltype: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterSetAccountAccess(hcluster : HCLUSTER, szaccountsid : windows_core::PCWSTR, dwaccess : u32, dwcontroltype : u32) -> u32);
    unsafe { ClusterSetAccountAccess(hcluster, szaccountsid.param().abi(), dwaccess, dwcontroltype) }
}
#[inline]
pub unsafe fn ClusterSharedVolumeSetSnapshotState<P1>(guidsnapshotset: windows_core::GUID, lpszvolumename: P1, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset : windows_core::GUID, lpszvolumename : windows_core::PCWSTR, state : CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32);
    unsafe { ClusterSharedVolumeSetSnapshotState(core::mem::transmute(guidsnapshotset), lpszvolumename.param().abi(), state) }
}
#[inline]
pub unsafe fn ClusterUpgradeFunctionalLevel(hcluster: HCLUSTER, perform: bool, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterUpgradeFunctionalLevel(hcluster : HCLUSTER, perform : windows_core::BOOL, pfnprogresscallback : PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    unsafe { ClusterUpgradeFunctionalLevel(hcluster, perform.into(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn CreateCluster(pconfig : *const CREATE_CLUSTER_CONFIG, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HCLUSTER);
    unsafe { CreateCluster(pconfig, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterAvailabilitySet<P1>(hcluster: HCLUSTER, lpavailabilitysetname: P1, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterAvailabilitySet(hcluster : HCLUSTER, lpavailabilitysetname : windows_core::PCWSTR, pavailabilitysetconfig : *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET);
    unsafe { CreateClusterAvailabilitySet(hcluster, lpavailabilitysetname.param().abi(), pavailabilitysetconfig) }
}
#[inline]
pub unsafe fn CreateClusterGroup<P1>(hcluster: HCLUSTER, lpszgroupname: P1) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroup(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    unsafe { CreateClusterGroup(hcluster, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterGroupEx<P1>(hcluster: HCLUSTER, lpszgroupname: P1, pgroupinfo: Option<*const CLUSTER_CREATE_GROUP_INFO>) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroupEx(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR, pgroupinfo : *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP);
    unsafe { CreateClusterGroupEx(hcluster, lpszgroupname.param().abi(), pgroupinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterGroupSet<P1>(hcluster: HCLUSTER, groupsetname: P1) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroupSet(hcluster : HCLUSTER, groupsetname : windows_core::PCWSTR) -> HGROUPSET);
    unsafe { CreateClusterGroupSet(hcluster, groupsetname.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterNameAccount(hcluster: HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNameAccount(hcluster : HCLUSTER, pconfig : *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    unsafe { CreateClusterNameAccount(hcluster, pconfig, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterNotifyPort(hchange: HCHANGE, hcluster: HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> HCHANGE {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNotifyPort(hchange : HCHANGE, hcluster : HCLUSTER, dwfilter : u32, dwnotifykey : usize) -> HCHANGE);
    unsafe { CreateClusterNotifyPort(hchange, hcluster, dwfilter, dwnotifykey) }
}
#[inline]
pub unsafe fn CreateClusterNotifyPortV2(hchange: HCHANGE, hcluster: HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNotifyPortV2(hchange : HCHANGE, hcluster : HCLUSTER, filters : *const NOTIFY_FILTER_AND_TYPE, dwfiltercount : u32, dwnotifykey : usize) -> HCHANGE);
    unsafe { CreateClusterNotifyPortV2(hchange, hcluster, filters, dwfiltercount, dwnotifykey) }
}
#[inline]
pub unsafe fn CreateClusterResource<P1, P2>(hgroup: HGROUP, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResource(hgroup : HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32) -> HRESOURCE);
    unsafe { CreateClusterResource(hgroup, lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CreateClusterResourceEx<P1, P2, P4>(hgroup: HGROUP, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32, lpszreason: P4) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceEx(hgroup : HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32, lpszreason : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { CreateClusterResourceEx(hgroup, lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterResourceType<P1, P2, P3>(hcluster: HCLUSTER, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceType(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32) -> u32);
    unsafe { CreateClusterResourceType(hcluster, lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval) }
}
#[inline]
pub unsafe fn CreateClusterResourceTypeEx<P1, P2, P3, P6>(hcluster: HCLUSTER, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: P6) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceTypeEx(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { CreateClusterResourceTypeEx(hcluster, lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterGroup(hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroup(hgroup : HGROUP) -> u32);
    unsafe { DeleteClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn DeleteClusterGroupEx<P1>(hgroup: HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterGroupEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterGroupSet(hgroupset: HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupSet(hgroupset : HGROUPSET) -> u32);
    unsafe { DeleteClusterGroupSet(hgroupset) }
}
#[inline]
pub unsafe fn DeleteClusterGroupSetEx<P1>(hgroupset: HGROUPSET, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupSetEx(hgroupset : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterGroupSetEx(hgroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResource(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResource(hresource : HRESOURCE) -> u32);
    unsafe { DeleteClusterResource(hresource) }
}
#[inline]
pub unsafe fn DeleteClusterResourceEx<P1>(hresource: HRESOURCE, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceEx(hresource : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceEx(hresource, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResourceType<P1>(hcluster: HCLUSTER, lpszresourcetypename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceType(hcluster : HCLUSTER, lpszresourcetypename : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceType(hcluster, lpszresourcetypename.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResourceTypeEx<P1, P2>(hcluster: HCLUSTER, lpsztypename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceTypeEx(hcluster : HCLUSTER, lpsztypename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceTypeEx(hcluster, lpsztypename.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DestroyCluster(hcluster: HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, fdeletevirtualcomputerobjects: bool) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DestroyCluster(hcluster : HCLUSTER, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, fdeletevirtualcomputerobjects : windows_core::BOOL) -> u32);
    unsafe { DestroyCluster(hcluster, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _, fdeletevirtualcomputerobjects.into()) }
}
#[inline]
pub unsafe fn DestroyClusterGroup(hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DestroyClusterGroup(hgroup : HGROUP) -> u32);
    unsafe { DestroyClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn DestroyClusterGroupEx<P1>(hgroup: HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DestroyClusterGroupEx(hgroup : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DestroyClusterGroupEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromCluster(hcluster: HCLUSTER, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromCluster(hcluster : HCLUSTER, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    unsafe { DetermineCNOResTypeFromCluster(hcluster, pcnorestype as _) }
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    unsafe { DetermineCNOResTypeFromNodelist(cnodes, ppsznodenames, pcnorestype as _) }
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromCluster(hcluster: HCLUSTER, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromCluster(hcluster : HCLUSTER, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    unsafe { DetermineClusterCloudTypeFromCluster(hcluster, pcloudtype as _) }
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    unsafe { DetermineClusterCloudTypeFromNodelist(cnodes, ppsznodenames, pcloudtype as _) }
}
#[inline]
pub unsafe fn EvictClusterNode(hnode: HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNode(hnode : HNODE) -> u32);
    unsafe { EvictClusterNode(hnode) }
}
#[inline]
pub unsafe fn EvictClusterNodeEx(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNodeEx(hnode : HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT) -> u32);
    unsafe { EvictClusterNodeEx(hnode, dwtimeout, phrcleanupstatus as _) }
}
#[inline]
pub unsafe fn EvictClusterNodeEx2<P3>(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNodeEx2(hnode : HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { EvictClusterNodeEx2(hnode, dwtimeout, phrcleanupstatus as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn FailClusterResource(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn FailClusterResource(hresource : HRESOURCE) -> u32);
    unsafe { FailClusterResource(hresource) }
}
#[inline]
pub unsafe fn FailClusterResourceEx<P1>(hresource: HRESOURCE, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn FailClusterResourceEx(hresource : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { FailClusterResourceEx(hresource, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn FreeClusterCrypt(pcryptinfo: *const core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn FreeClusterCrypt(pcryptinfo : *const core::ffi::c_void) -> u32);
    unsafe { FreeClusterCrypt(pcryptinfo) }
}
#[inline]
pub unsafe fn FreeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    windows_core::link!("resutils.dll" "system" fn FreeClusterHealthFault(clusterhealthfault : *mut CLUSTER_HEALTH_FAULT) -> u32);
    unsafe { FreeClusterHealthFault(clusterhealthfault as _) }
}
#[inline]
pub unsafe fn FreeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    windows_core::link!("resutils.dll" "system" fn FreeClusterHealthFaultArray(clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32);
    unsafe { FreeClusterHealthFaultArray(clusterhealthfaultarray as _) }
}
#[inline]
pub unsafe fn GetClusterFromGroup(hgroup: HGROUP) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromGroup(hgroup : HGROUP) -> HCLUSTER);
    unsafe { GetClusterFromGroup(hgroup) }
}
#[inline]
pub unsafe fn GetClusterFromNetInterface(hnetinterface: HNETINTERFACE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNetInterface(hnetinterface : HNETINTERFACE) -> HCLUSTER);
    unsafe { GetClusterFromNetInterface(hnetinterface) }
}
#[inline]
pub unsafe fn GetClusterFromNetwork(hnetwork: HNETWORK) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNetwork(hnetwork : HNETWORK) -> HCLUSTER);
    unsafe { GetClusterFromNetwork(hnetwork) }
}
#[inline]
pub unsafe fn GetClusterFromNode(hnode: HNODE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNode(hnode : HNODE) -> HCLUSTER);
    unsafe { GetClusterFromNode(hnode) }
}
#[inline]
pub unsafe fn GetClusterFromResource(hresource: HRESOURCE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromResource(hresource : HRESOURCE) -> HCLUSTER);
    unsafe { GetClusterFromResource(hresource) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterGroupKey(hgroup: HGROUP, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterGroupKey(hgroup : HGROUP, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterGroupKey(hgroup, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterGroupState(hgroup: HGROUP, lpsznodename: Option<windows_core::PWSTR>, lpcchnodename: Option<*mut u32>) -> CLUSTER_GROUP_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterGroupState(hgroup : HGROUP, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32) -> CLUSTER_GROUP_STATE);
    unsafe { GetClusterGroupState(hgroup, lpsznodename.unwrap_or(core::mem::zeroed()) as _, lpcchnodename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetClusterInformation(hcluster: HCLUSTER, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: Option<*mut CLUSTERVERSIONINFO>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterInformation(hcluster : HCLUSTER, lpszclustername : windows_core::PWSTR, lpcchclustername : *mut u32, lpclusterinfo : *mut CLUSTERVERSIONINFO) -> u32);
    unsafe { GetClusterInformation(hcluster, core::mem::transmute(lpszclustername), lpcchclustername as _, lpclusterinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterKey(hcluster: HCLUSTER, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterKey(hcluster : HCLUSTER, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterKey(hcluster, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterNetInterface<P1, P2>(hcluster: HCLUSTER, lpsznodename: P1, lpsznetworkname: P2, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterface(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR, lpszinterfacename : windows_core::PWSTR, lpcchinterfacename : *mut u32) -> u32);
    unsafe { GetClusterNetInterface(hcluster, lpsznodename.param().abi(), lpsznetworkname.param().abi(), core::mem::transmute(lpszinterfacename), lpcchinterfacename as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetInterfaceKey(hnetinterface: HNETINTERFACE, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterfaceKey(hnetinterface : HNETINTERFACE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterNetInterfaceKey(hnetinterface, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterNetInterfaceState(hnetinterface: HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterfaceState(hnetinterface : HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE);
    unsafe { GetClusterNetInterfaceState(hnetinterface) }
}
#[inline]
pub unsafe fn GetClusterNetworkId(hnetwork: HNETWORK, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkId(hnetwork : HNETWORK, lpsznetworkid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { GetClusterNetworkId(hnetwork, core::mem::transmute(lpsznetworkid), lpcchname as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNetworkKey(hnetwork: HNETWORK, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkKey(hnetwork : HNETWORK, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterNetworkKey(hnetwork, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterNetworkState(hnetwork: HNETWORK) -> CLUSTER_NETWORK_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkState(hnetwork : HNETWORK) -> CLUSTER_NETWORK_STATE);
    unsafe { GetClusterNetworkState(hnetwork) }
}
#[inline]
pub unsafe fn GetClusterNodeId(hnode: Option<HNODE>, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeId(hnode : HNODE, lpsznodeid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { GetClusterNodeId(hnode.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpsznodeid), lpcchname as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterNodeKey(hnode: HNODE, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeKey(hnode : HNODE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterNodeKey(hnode, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterNodeState(hnode: HNODE) -> CLUSTER_NODE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeState(hnode : HNODE) -> CLUSTER_NODE_STATE);
    unsafe { GetClusterNodeState(hnode) }
}
#[inline]
pub unsafe fn GetClusterNotify(hchange: HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNotify(hchange : HCHANGE, lpdwnotifykey : *mut usize, lpdwfiltertype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, dwmilliseconds : u32) -> u32);
    unsafe { GetClusterNotify(hchange, lpdwnotifykey as _, lpdwfiltertype as _, core::mem::transmute(lpszname), lpcchname as _, dwmilliseconds) }
}
#[inline]
pub unsafe fn GetClusterNotifyV2(hchange: HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: Option<*mut NOTIFY_FILTER_AND_TYPE>, buffer: Option<*mut u8>, lpbbuffersize: Option<*mut u32>, lpszobjectid: Option<windows_core::PWSTR>, lpcchobjectid: Option<*mut u32>, lpszparentid: Option<windows_core::PWSTR>, lpcchparentid: Option<*mut u32>, lpszname: Option<windows_core::PWSTR>, lpcchname: Option<*mut u32>, lpsztype: Option<windows_core::PWSTR>, lpcchtype: Option<*mut u32>, dwmilliseconds: Option<u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNotifyV2(hchange : HCHANGE, lpdwnotifykey : *mut usize, pfilterandtype : *mut NOTIFY_FILTER_AND_TYPE, buffer : *mut u8, lpbbuffersize : *mut u32, lpszobjectid : windows_core::PWSTR, lpcchobjectid : *mut u32, lpszparentid : windows_core::PWSTR, lpcchparentid : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpsztype : windows_core::PWSTR, lpcchtype : *mut u32, dwmilliseconds : u32) -> u32);
    unsafe {
        GetClusterNotifyV2(
            hchange,
            lpdwnotifykey as _,
            pfilterandtype.unwrap_or(core::mem::zeroed()) as _,
            buffer.unwrap_or(core::mem::zeroed()) as _,
            lpbbuffersize.unwrap_or(core::mem::zeroed()) as _,
            lpszobjectid.unwrap_or(core::mem::zeroed()) as _,
            lpcchobjectid.unwrap_or(core::mem::zeroed()) as _,
            lpszparentid.unwrap_or(core::mem::zeroed()) as _,
            lpcchparentid.unwrap_or(core::mem::zeroed()) as _,
            lpszname.unwrap_or(core::mem::zeroed()) as _,
            lpcchname.unwrap_or(core::mem::zeroed()) as _,
            lpsztype.unwrap_or(core::mem::zeroed()) as _,
            lpcchtype.unwrap_or(core::mem::zeroed()) as _,
            dwmilliseconds.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetClusterQuorumResource(hcluster: HCLUSTER, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterQuorumResource(hcluster : HCLUSTER, lpszresourcename : windows_core::PWSTR, lpcchresourcename : *mut u32, lpszdevicename : windows_core::PWSTR, lpcchdevicename : *mut u32, lpdwmaxquorumlogsize : *mut u32) -> u32);
    unsafe { GetClusterQuorumResource(hcluster, core::mem::transmute(lpszresourcename), lpcchresourcename as _, core::mem::transmute(lpszdevicename), lpcchdevicename as _, lpdwmaxquorumlogsize as _) }
}
#[inline]
pub unsafe fn GetClusterResourceDependencyExpression(hresource: HRESOURCE, lpszdependencyexpression: Option<windows_core::PWSTR>, lpcchdependencyexpression: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceDependencyExpression(hresource : HRESOURCE, lpszdependencyexpression : windows_core::PWSTR, lpcchdependencyexpression : *mut u32) -> u32);
    unsafe { GetClusterResourceDependencyExpression(hresource, lpszdependencyexpression.unwrap_or(core::mem::zeroed()) as _, lpcchdependencyexpression as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceKey(hresource: HRESOURCE, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceKey(hresource : HRESOURCE, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterResourceKey(hresource, samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetClusterResourceNetworkName(hresource: HRESOURCE, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceNetworkName(hresource : HRESOURCE, lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetClusterResourceNetworkName(hresource, core::mem::transmute(lpbuffer), nsize as _).ok() }
}
#[inline]
pub unsafe fn GetClusterResourceState(hresource: HRESOURCE, lpsznodename: Option<windows_core::PWSTR>, lpcchnodename: Option<*mut u32>, lpszgroupname: Option<windows_core::PWSTR>, lpcchgroupname: Option<*mut u32>) -> CLUSTER_RESOURCE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceState(hresource : HRESOURCE, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32, lpszgroupname : windows_core::PWSTR, lpcchgroupname : *mut u32) -> CLUSTER_RESOURCE_STATE);
    unsafe { GetClusterResourceState(hresource, lpsznodename.unwrap_or(core::mem::zeroed()) as _, lpcchnodename.unwrap_or(core::mem::zeroed()) as _, lpszgroupname.unwrap_or(core::mem::zeroed()) as _, lpcchgroupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetClusterResourceTypeKey<P1>(hcluster: HCLUSTER, lpsztypename: P1, samdesired: u32) -> windows_core::Result<super::super::System::Registry::HKEY>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceTypeKey(hcluster : HCLUSTER, lpsztypename : windows_core::PCWSTR, samdesired : u32) -> super::super::System::Registry:: HKEY);
    let result__ = unsafe { GetClusterResourceTypeKey(hcluster, lpsztypename.param().abi(), samdesired) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn GetNodeCloudTypeDW<P0>(ppsznodename: P0, nodecloudtype: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetNodeCloudTypeDW(ppsznodename : windows_core::PCWSTR, nodecloudtype : *mut u32) -> u32);
    unsafe { GetNodeCloudTypeDW(ppsznodename.param().abi(), nodecloudtype as _) }
}
#[inline]
pub unsafe fn GetNodeClusterState<P0>(lpsznodename: P0, pdwclusterstate: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetNodeClusterState(lpsznodename : windows_core::PCWSTR, pdwclusterstate : *mut u32) -> u32);
    unsafe { GetNodeClusterState(lpsznodename.param().abi(), pdwclusterstate as _) }
}
#[inline]
pub unsafe fn GetNotifyEventHandle(hchange: HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetNotifyEventHandle(hchange : HCHANGE, lphtargetevent : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { GetNotifyEventHandle(hchange, lphtargetevent as _) }
}
#[inline]
pub unsafe fn InitializeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32 {
    windows_core::link!("resutils.dll" "system" fn InitializeClusterHealthFault(clusterhealthfault : *mut CLUSTER_HEALTH_FAULT) -> u32);
    unsafe { InitializeClusterHealthFault(clusterhealthfault as _) }
}
#[inline]
pub unsafe fn InitializeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32 {
    windows_core::link!("resutils.dll" "system" fn InitializeClusterHealthFaultArray(clusterhealthfaultarray : *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32);
    unsafe { InitializeClusterHealthFaultArray(clusterhealthfaultarray as _) }
}
#[inline]
pub unsafe fn IsFileOnClusterSharedVolume<P0>(lpszpathname: P0, pbfileisonsharedvolume: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn IsFileOnClusterSharedVolume(lpszpathname : windows_core::PCWSTR, pbfileisonsharedvolume : *mut windows_core::BOOL) -> u32);
    unsafe { IsFileOnClusterSharedVolume(lpszpathname.param().abi(), pbfileisonsharedvolume as _) }
}
#[inline]
pub unsafe fn MoveClusterGroup(hgroup: HGROUP, hdestinationnode: Option<HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroup(hgroup : HGROUP, hdestinationnode : HNODE) -> u32);
    unsafe { MoveClusterGroup(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MoveClusterGroupEx(hgroup: HGROUP, hdestinationnode: Option<HNODE>, dwmoveflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroupEx(hgroup : HGROUP, hdestinationnode : HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { MoveClusterGroupEx(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwmoveflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn MoveClusterGroupEx2<P5>(hgroup: HGROUP, hdestinationnode: Option<HNODE>, dwmoveflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroupEx2(hgroup : HGROUP, hdestinationnode : HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { MoveClusterGroupEx2(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwmoveflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OfflineClusterGroup(hgroup: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroup(hgroup : HGROUP) -> u32);
    unsafe { OfflineClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn OfflineClusterGroupEx(hgroup: HGROUP, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroupEx(hgroup : HGROUP, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OfflineClusterGroupEx(hgroup, dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OfflineClusterGroupEx2<P4>(hgroup: HGROUP, dwofflineflags: u32, lpinbuffer: Option<*const u8>, cbinbuffersize: u32, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroupEx2(hgroup : HGROUP, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OfflineClusterGroupEx2(hgroup, dwofflineflags, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OfflineClusterResource(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResource(hresource : HRESOURCE) -> u32);
    unsafe { OfflineClusterResource(hresource) }
}
#[inline]
pub unsafe fn OfflineClusterResourceEx(hresource: HRESOURCE, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResourceEx(hresource : HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OfflineClusterResourceEx(hresource, dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OfflineClusterResourceEx2<P4>(hresource: HRESOURCE, dwofflineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResourceEx2(hresource : HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OfflineClusterResourceEx2(hresource, dwofflineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OnlineClusterGroup(hgroup: HGROUP, hdestinationnode: Option<HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroup(hgroup : HGROUP, hdestinationnode : HNODE) -> u32);
    unsafe { OnlineClusterGroup(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OnlineClusterGroupEx(hgroup: HGROUP, hdestinationnode: Option<HNODE>, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroupEx(hgroup : HGROUP, hdestinationnode : HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OnlineClusterGroupEx(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OnlineClusterGroupEx2<P5>(hgroup: HGROUP, hdestinationnode: Option<HNODE>, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroupEx2(hgroup : HGROUP, hdestinationnode : HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OnlineClusterGroupEx2(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OnlineClusterResource(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResource(hresource : HRESOURCE) -> u32);
    unsafe { OnlineClusterResource(hresource) }
}
#[inline]
pub unsafe fn OnlineClusterResourceEx(hresource: HRESOURCE, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResourceEx(hresource : HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OnlineClusterResourceEx(hresource, dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OnlineClusterResourceEx2<P4>(hresource: HRESOURCE, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResourceEx2(hresource : HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OnlineClusterResourceEx2(hresource, dwonlineflags, core::mem::transmute(lpinbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OpenCluster<P0>(lpszclustername: P0) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenCluster(lpszclustername : windows_core::PCWSTR) -> HCLUSTER);
    unsafe { OpenCluster(lpszclustername.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterCryptProvider<P0>(lpszresource: P0, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn OpenClusterCryptProvider(lpszresource : windows_core::PCWSTR, lpszprovider : *const i8, dwtype : u32, dwflags : u32) -> HCLUSCRYPTPROVIDER);
    unsafe { OpenClusterCryptProvider(lpszresource.param().abi(), lpszprovider, dwtype, dwflags) }
}
#[inline]
pub unsafe fn OpenClusterCryptProviderEx<P0, P1>(lpszresource: P0, lpszkeyname: P1, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn OpenClusterCryptProviderEx(lpszresource : windows_core::PCWSTR, lpszkeyname : windows_core::PCWSTR, lpszprovider : *const i8, dwtype : u32, dwflags : u32) -> HCLUSCRYPTPROVIDER);
    unsafe { OpenClusterCryptProviderEx(lpszresource.param().abi(), lpszkeyname.param().abi(), lpszprovider, dwtype, dwflags) }
}
#[inline]
pub unsafe fn OpenClusterEx<P0>(lpszclustername: P0, desiredaccess: u32, grantedaccess: Option<*mut u32>) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterEx(lpszclustername : windows_core::PCWSTR, desiredaccess : u32, grantedaccess : *mut u32) -> HCLUSTER);
    unsafe { OpenClusterEx(lpszclustername.param().abi(), desiredaccess, grantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterGroup<P1>(hcluster: HCLUSTER, lpszgroupname: P1) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroup(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    unsafe { OpenClusterGroup(hcluster, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterGroupEx<P1>(hcluster: HCLUSTER, lpszgroupname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroupEx(hcluster : HCLUSTER, lpszgroupname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HGROUP);
    unsafe { OpenClusterGroupEx(hcluster, lpszgroupname.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterGroupSet<P1>(hcluster: HCLUSTER, lpszgroupsetname: P1) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroupSet(hcluster : HCLUSTER, lpszgroupsetname : windows_core::PCWSTR) -> HGROUPSET);
    unsafe { OpenClusterGroupSet(hcluster, lpszgroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetInterface<P1>(hcluster: HCLUSTER, lpszinterfacename: P1) -> HNETINTERFACE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetInterface(hcluster : HCLUSTER, lpszinterfacename : windows_core::PCWSTR) -> HNETINTERFACE);
    unsafe { OpenClusterNetInterface(hcluster, lpszinterfacename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetInterfaceEx<P1>(hcluster: HCLUSTER, lpszinterfacename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETINTERFACE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetInterfaceEx(hcluster : HCLUSTER, lpszinterfacename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETINTERFACE);
    unsafe { OpenClusterNetInterfaceEx(hcluster, lpszinterfacename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterNetwork<P1>(hcluster: HCLUSTER, lpsznetworkname: P1) -> HNETWORK
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetwork(hcluster : HCLUSTER, lpsznetworkname : windows_core::PCWSTR) -> HNETWORK);
    unsafe { OpenClusterNetwork(hcluster, lpsznetworkname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetworkEx<P1>(hcluster: HCLUSTER, lpsznetworkname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETWORK
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetworkEx(hcluster : HCLUSTER, lpsznetworkname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETWORK);
    unsafe { OpenClusterNetworkEx(hcluster, lpsznetworkname.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterNode<P1>(hcluster: HCLUSTER, lpsznodename: P1) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNode(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR) -> HNODE);
    unsafe { OpenClusterNode(hcluster, lpsznodename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNodeById(hcluster: HCLUSTER, nodeid: u32) -> HNODE {
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNodeById(hcluster : HCLUSTER, nodeid : u32) -> HNODE);
    unsafe { OpenClusterNodeById(hcluster, nodeid) }
}
#[inline]
pub unsafe fn OpenClusterNodeEx<P1>(hcluster: HCLUSTER, lpsznodename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNodeEx(hcluster : HCLUSTER, lpsznodename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNODE);
    unsafe { OpenClusterNodeEx(hcluster, lpsznodename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterResource<P1>(hcluster: HCLUSTER, lpszresourcename: P1) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterResource(hcluster : HCLUSTER, lpszresourcename : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { OpenClusterResource(hcluster, lpszresourcename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterResourceEx<P1>(hcluster: HCLUSTER, lpszresourcename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterResourceEx(hcluster : HCLUSTER, lpszresourcename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HRESOURCE);
    unsafe { OpenClusterResourceEx(hcluster, lpszresourcename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PauseClusterNode(hnode: HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNode(hnode : HNODE) -> u32);
    unsafe { PauseClusterNode(hnode) }
}
#[inline]
pub unsafe fn PauseClusterNodeEx(hnode: HNODE, bdrainnode: bool, dwpauseflags: u32, hnodedraintarget: Option<HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNodeEx(hnode : HNODE, bdrainnode : windows_core::BOOL, dwpauseflags : u32, hnodedraintarget : HNODE) -> u32);
    unsafe { PauseClusterNodeEx(hnode, bdrainnode.into(), dwpauseflags, hnodedraintarget.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PauseClusterNodeEx2<P4>(hnode: HNODE, bdrainnode: bool, dwpauseflags: u32, hnodedraintarget: Option<HNODE>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNodeEx2(hnode : HNODE, bdrainnode : windows_core::BOOL, dwpauseflags : u32, hnodedraintarget : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { PauseClusterNodeEx2(hnode, bdrainnode.into(), dwpauseflags, hnodedraintarget.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn QueryAppInstanceVersion(appinstanceid: *const windows_core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32 {
    windows_core::link!("ntlanman.dll" "system" fn QueryAppInstanceVersion(appinstanceid : *const windows_core::GUID, instanceversionhigh : *mut u64, instanceversionlow : *mut u64, versionstatus : *mut super::super::Foundation:: NTSTATUS) -> u32);
    unsafe { QueryAppInstanceVersion(appinstanceid, instanceversionhigh as _, instanceversionlow as _, versionstatus as _) }
}
#[inline]
pub unsafe fn RegisterAppInstance(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const windows_core::GUID, childreninheritappinstance: bool) -> u32 {
    windows_core::link!("ntlanman.dll" "system" fn RegisterAppInstance(processhandle : super::super::Foundation:: HANDLE, appinstanceid : *const windows_core::GUID, childreninheritappinstance : windows_core::BOOL) -> u32);
    unsafe { RegisterAppInstance(processhandle, appinstanceid, childreninheritappinstance.into()) }
}
#[inline]
pub unsafe fn RegisterAppInstanceVersion(appinstanceid: *const windows_core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32 {
    windows_core::link!("ntlanman.dll" "system" fn RegisterAppInstanceVersion(appinstanceid : *const windows_core::GUID, instanceversionhigh : u64, instanceversionlow : u64) -> u32);
    unsafe { RegisterAppInstanceVersion(appinstanceid, instanceversionhigh, instanceversionlow) }
}
#[inline]
pub unsafe fn RegisterClusterNotify(hchange: HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterNotify(hchange : HCHANGE, dwfiltertype : u32, hobject : super::super::Foundation:: HANDLE, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterNotify(hchange, dwfiltertype, hobject, dwnotifykey) }
}
#[inline]
pub unsafe fn RegisterClusterNotifyV2(hchange: HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterNotifyV2(hchange : HCHANGE, filter : NOTIFY_FILTER_AND_TYPE, hobject : super::super::Foundation:: HANDLE, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterNotifyV2(hchange, core::mem::transmute(filter), hobject, dwnotifykey) }
}
#[inline]
pub unsafe fn RegisterClusterResourceTypeNotifyV2<P3>(hchange: HCHANGE, hcluster: HCLUSTER, flags: i64, restypename: P3, dwnotifykey: usize) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterResourceTypeNotifyV2(hchange : HCHANGE, hcluster : HCLUSTER, flags : i64, restypename : windows_core::PCWSTR, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterResourceTypeNotifyV2(hchange, hcluster, flags, restypename.param().abi(), dwnotifykey) }
}
#[inline]
pub unsafe fn RemoveClusterGroupDependency(hgroup: HGROUP, hdependson: HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupDependency(hgroup : HGROUP, hdependson : HGROUP) -> u32);
    unsafe { RemoveClusterGroupDependency(hgroup, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupDependencyEx<P2>(hgroup: HGROUP, hdependson: HGROUP, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupDependencyEx(hgroup : HGROUP, hdependson : HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupDependencyEx(hgroup, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependency(hgroupset: HGROUPSET, hdependson: HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependency(hgroupset : HGROUPSET, hdependson : HGROUPSET) -> u32);
    unsafe { RemoveClusterGroupSetDependency(hgroupset, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependencyEx<P2>(hgroupset: HGROUPSET, hdependson: HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependencyEx(hgroupset : HGROUPSET, hdependson : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupSetDependencyEx(hgroupset, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependency(hgroup: HGROUP, hdependson: HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependency(hgroup : HGROUP, hdependson : HGROUPSET) -> u32);
    unsafe { RemoveClusterGroupToGroupSetDependency(hgroup, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependencyEx<P2>(hgroup: HGROUP, hdependson: HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependencyEx(hgroup : HGROUP, hdependson : HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupToGroupSetDependencyEx(hgroup, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterNameAccount(hcluster: HCLUSTER, bdeletecomputerobjects: bool) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterNameAccount(hcluster : HCLUSTER, bdeletecomputerobjects : windows_core::BOOL) -> u32);
    unsafe { RemoveClusterNameAccount(hcluster, bdeletecomputerobjects.into()) }
}
#[inline]
pub unsafe fn RemoveClusterResourceDependency(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceDependency(hresource : HRESOURCE, hdependson : HRESOURCE) -> u32);
    unsafe { RemoveClusterResourceDependency(hresource, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterResourceDependencyEx<P2>(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceDependencyEx(hresource : HRESOURCE, hdependson : HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterResourceDependencyEx(hresource, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterResourceNode(hresource: HRESOURCE, hnode: HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceNode(hresource : HRESOURCE, hnode : HNODE) -> u32);
    unsafe { RemoveClusterResourceNode(hresource, hnode) }
}
#[inline]
pub unsafe fn RemoveClusterResourceNodeEx<P2>(hresource: HRESOURCE, hnode: HNODE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceNodeEx(hresource : HRESOURCE, hnode : HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterResourceNodeEx(hresource, hnode, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterStorageNode<P1>(hcluster: HCLUSTER, lpszclusterstorageenclosurename: P1, dwtimeout: u32, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterStorageNode(hcluster : HCLUSTER, lpszclusterstorageenclosurename : windows_core::PCWSTR, dwtimeout : u32, dwflags : u32) -> u32);
    unsafe { RemoveClusterStorageNode(hcluster, lpszclusterstorageenclosurename.param().abi(), dwtimeout, dwflags) }
}
#[inline]
pub unsafe fn RemoveCrossClusterGroupSetDependency<P1, P2>(hdependentgroupset: HGROUPSET, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveCrossClusterGroupSetDependency(hdependentgroupset : HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    unsafe { RemoveCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn RemoveResourceFromClusterSharedVolumes(hresource: HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveResourceFromClusterSharedVolumes(hresource : HRESOURCE) -> u32);
    unsafe { RemoveResourceFromClusterSharedVolumes(hresource) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilAddUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilAddUnknownProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, pcboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilAddUnknownProperties(hkeyclusterkey, ppropertytable, poutpropertylist as _, pcboutpropertylistsize, pcbbytesreturned as _, pcbrequired as _) }
}
#[inline]
pub unsafe fn ResUtilCreateDirectoryTree<P0>(pszpath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilCreateDirectoryTree(pszpath : windows_core::PCWSTR) -> u32);
    unsafe { ResUtilCreateDirectoryTree(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilDupGroup(group: HGROUP, copy: *mut HGROUP) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilDupGroup(group : HGROUP, copy : *mut HGROUP) -> u32);
    unsafe { ResUtilDupGroup(group, copy as _) }
}
#[inline]
pub unsafe fn ResUtilDupParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilDupParameterBlock(poutparams : *mut u8, pinparams : *const u8, ppropertytable : *const RESUTIL_PROPERTY_ITEM) -> u32);
    unsafe { ResUtilDupParameterBlock(poutparams as _, pinparams, ppropertytable) }
}
#[inline]
pub unsafe fn ResUtilDupResource(group: HRESOURCE, copy: *mut HRESOURCE) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilDupResource(group : HRESOURCE, copy : *mut HRESOURCE) -> u32);
    unsafe { ResUtilDupResource(group, copy as _) }
}
#[inline]
pub unsafe fn ResUtilDupString<P0>(pszinstring: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilDupString(pszinstring : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { ResUtilDupString(pszinstring.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilEnumGroups(hcluster: HCLUSTER, hself: HGROUP, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumGroups(hcluster : HCLUSTER, hself : HGROUP, prescallback : LPGROUP_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilEnumGroups(hcluster, hself, prescallback, pparameter as _) }
}
#[inline]
pub unsafe fn ResUtilEnumGroupsEx(hcluster: HCLUSTER, hself: HGROUP, grouptype: CLUSGROUP_TYPE, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumGroupsEx(hcluster : HCLUSTER, hself : HGROUP, grouptype : CLUSGROUP_TYPE, prescallback : LPGROUP_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilEnumGroupsEx(hcluster, hself, grouptype, prescallback, pparameter as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilEnumPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumPrivateProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, pszoutproperties : windows_core::PWSTR, cboutpropertiessize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilEnumPrivateProperties(hkeyclusterkey, core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned as _, pcbrequired as _) }
}
#[inline]
pub unsafe fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumProperties(ppropertytable : *const RESUTIL_PROPERTY_ITEM, pszoutproperties : windows_core::PWSTR, cboutpropertiessize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilEnumProperties(ppropertytable, core::mem::transmute(pszoutproperties), cboutpropertiessize, pcbbytesreturned as _, pcbrequired as _) }
}
#[inline]
pub unsafe fn ResUtilEnumResources<P1>(hself: HRESOURCE, lpszrestypename: P1, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumResources(hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilEnumResources(hself, lpszrestypename.param().abi(), prescallback, pparameter as _) }
}
#[inline]
pub unsafe fn ResUtilEnumResourcesEx<P2>(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: P2, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumResourcesEx(hcluster : HCLUSTER, hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilEnumResourcesEx(hcluster, hself, lpszrestypename.param().abi(), prescallback, pparameter as _) }
}
#[inline]
pub unsafe fn ResUtilEnumResourcesEx2<P2>(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: P2, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void, dwdesiredaccess: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilEnumResourcesEx2(hcluster : HCLUSTER, hself : HRESOURCE, lpszrestypename : windows_core::PCWSTR, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void, dwdesiredaccess : u32) -> u32);
    unsafe { ResUtilEnumResourcesEx2(hcluster, hself, lpszrestypename.param().abi(), prescallback, pparameter as _, dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilExpandEnvironmentStrings<P0>(pszsrc: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilExpandEnvironmentStrings(pszsrc : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { ResUtilExpandEnvironmentStrings(pszsrc.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilFindBinaryProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pbpropertyvalue: Option<*mut *mut u8>, pcbpropertyvaluesize: Option<*mut u32>) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindBinaryProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pbpropertyvalue : *mut *mut u8, pcbpropertyvaluesize : *mut u32) -> u32);
    unsafe { ResUtilFindBinaryProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pbpropertyvalue.unwrap_or(core::mem::zeroed()) as _, pcbpropertyvaluesize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilFindDependentDiskResourceDriveLetter(hcluster: HCLUSTER, hresource: HRESOURCE, pszdriveletter: windows_core::PWSTR, pcchdriveletter: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilFindDependentDiskResourceDriveLetter(hcluster : HCLUSTER, hresource : HRESOURCE, pszdriveletter : windows_core::PWSTR, pcchdriveletter : *mut u32) -> u32);
    unsafe { ResUtilFindDependentDiskResourceDriveLetter(hcluster, hresource, core::mem::transmute(pszdriveletter), pcchdriveletter as _) }
}
#[inline]
pub unsafe fn ResUtilFindDwordProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pdwpropertyvalue: *mut u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindDwordProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pdwpropertyvalue : *mut u32) -> u32);
    unsafe { ResUtilFindDwordProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pdwpropertyvalue as _) }
}
#[inline]
pub unsafe fn ResUtilFindExpandSzProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindExpandSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilFindExpandSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pszpropertyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilFindExpandedSzProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindExpandedSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilFindExpandedSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pszpropertyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilFindFileTimeProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindFileTimeProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pftpropertyvalue : *mut super::super::Foundation:: FILETIME) -> u32);
    unsafe { ResUtilFindFileTimeProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pftpropertyvalue as _) }
}
#[inline]
pub unsafe fn ResUtilFindLongProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, plpropertyvalue: *mut i32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindLongProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, plpropertyvalue : *mut i32) -> u32);
    unsafe { ResUtilFindLongProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), plpropertyvalue as _) }
}
#[inline]
pub unsafe fn ResUtilFindMultiSzProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pszpropertyvalue: *mut windows_core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindMultiSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR, pcbpropertyvaluesize : *mut u32) -> u32);
    unsafe { ResUtilFindMultiSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pszpropertyvalue as _, pcbpropertyvaluesize as _) }
}
#[inline]
pub unsafe fn ResUtilFindSzProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, pszpropertyvalue: Option<*mut windows_core::PWSTR>) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindSzProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, pszpropertyvalue : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilFindSzProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), pszpropertyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilFindULargeIntegerProperty<P2>(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: P2, plpropertyvalue: *mut u64) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilFindULargeIntegerProperty(ppropertylist : *const core::ffi::c_void, cbpropertylistsize : u32, pszpropertyname : windows_core::PCWSTR, plpropertyvalue : *mut u64) -> u32);
    unsafe { ResUtilFindULargeIntegerProperty(ppropertylist, cbpropertylistsize, pszpropertyname.param().abi(), plpropertyvalue as _) }
}
#[inline]
pub unsafe fn ResUtilFreeEnvironment(lpenvironment: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilFreeEnvironment(lpenvironment : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilFreeEnvironment(lpenvironment as _) }
}
#[inline]
pub unsafe fn ResUtilFreeParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) {
    windows_core::link!("resutils.dll" "system" fn ResUtilFreeParameterBlock(poutparams : *mut u8, pinparams : *const u8, ppropertytable : *const RESUTIL_PROPERTY_ITEM));
    unsafe { ResUtilFreeParameterBlock(poutparams as _, pinparams, ppropertytable) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetAllProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetAllProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilGetAllProperties(hkeyclusterkey, ppropertytable, poutpropertylist as _, cboutpropertylistsize, pcbbytesreturned as _, pcbrequired as _) }
}
#[inline]
pub unsafe fn ResUtilGetBinaryProperty(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: Option<&[u8]>, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetBinaryProperty(ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32, pvaluestruct : *const CLUSPROP_BINARY, pboldvalue : *const u8, cboldvaluesize : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetBinaryProperty(ppboutvalue as _, pcboutvaluesize as _, pvaluestruct, core::mem::transmute(pboldvalue.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pboldvalue.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pppropertylist as _, pcbpropertylistsize as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetBinaryValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetBinaryValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32) -> u32);
    unsafe { ResUtilGetBinaryValue(hkeyclusterkey, pszvaluename.param().abi(), ppboutvalue as _, pcboutvaluesize as _) }
}
#[inline]
pub unsafe fn ResUtilGetClusterGroupType(hgroup: HGROUP, grouptype: *mut CLUSGROUP_TYPE) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetClusterGroupType(hgroup : HGROUP, grouptype : *mut CLUSGROUP_TYPE) -> u32);
    unsafe { ResUtilGetClusterGroupType(hgroup, grouptype as _) }
}
#[inline]
pub unsafe fn ResUtilGetClusterId(hcluster: HCLUSTER, guid: *mut windows_core::GUID) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetClusterId(hcluster : HCLUSTER, guid : *mut windows_core::GUID) -> u32);
    unsafe { ResUtilGetClusterId(hcluster, guid as _) }
}
#[inline]
pub unsafe fn ResUtilGetClusterRoleState(hcluster: HCLUSTER, eclusterrole: CLUSTER_ROLE) -> CLUSTER_ROLE_STATE {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetClusterRoleState(hcluster : HCLUSTER, eclusterrole : CLUSTER_ROLE) -> CLUSTER_ROLE_STATE);
    unsafe { ResUtilGetClusterRoleState(hcluster, eclusterrole) }
}
#[inline]
pub unsafe fn ResUtilGetCoreClusterResources(hcluster: HCLUSTER, phclusternameresource: *mut HRESOURCE, phclusteripaddressresource: *mut HRESOURCE, phclusterquorumresource: *mut HRESOURCE) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetCoreClusterResources(hcluster : HCLUSTER, phclusternameresource : *mut HRESOURCE, phclusteripaddressresource : *mut HRESOURCE, phclusterquorumresource : *mut HRESOURCE) -> u32);
    unsafe { ResUtilGetCoreClusterResources(hcluster, phclusternameresource as _, phclusteripaddressresource as _, phclusterquorumresource as _) }
}
#[inline]
pub unsafe fn ResUtilGetCoreClusterResourcesEx(hclusterin: HCLUSTER, phclusternameresourceout: Option<*mut HRESOURCE>, phclusterquorumresourceout: Option<*mut HRESOURCE>, dwdesiredaccess: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetCoreClusterResourcesEx(hclusterin : HCLUSTER, phclusternameresourceout : *mut HRESOURCE, phclusterquorumresourceout : *mut HRESOURCE, dwdesiredaccess : u32) -> u32);
    unsafe { ResUtilGetCoreClusterResourcesEx(hclusterin, phclusternameresourceout.unwrap_or(core::mem::zeroed()) as _, phclusterquorumresourceout.unwrap_or(core::mem::zeroed()) as _, dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilGetCoreGroup(hcluster: HCLUSTER) -> HGROUP {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetCoreGroup(hcluster : HCLUSTER) -> HGROUP);
    unsafe { ResUtilGetCoreGroup(hcluster) }
}
#[inline]
pub unsafe fn ResUtilGetDwordProperty(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetDwordProperty(pdwoutvalue : *mut u32, pvaluestruct : *const CLUSPROP_DWORD, dwoldvalue : u32, dwminimum : u32, dwmaximum : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetDwordProperty(pdwoutvalue as _, pvaluestruct, dwoldvalue, dwminimum, dwmaximum, pppropertylist as _, pcbpropertylistsize as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetDwordValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetDwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pdwoutvalue : *mut u32, dwdefaultvalue : u32) -> u32);
    unsafe { ResUtilGetDwordValue(hkeyclusterkey, pszvaluename.param().abi(), pdwoutvalue as _, dwdefaultvalue) }
}
#[inline]
pub unsafe fn ResUtilGetEnvironmentWithNetName(hresource: HRESOURCE) -> *mut core::ffi::c_void {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetEnvironmentWithNetName(hresource : HRESOURCE) -> *mut core::ffi::c_void);
    unsafe { ResUtilGetEnvironmentWithNetName(hresource) }
}
#[inline]
pub unsafe fn ResUtilGetFileTimeProperty(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetFileTimeProperty(pftoutvalue : *mut super::super::Foundation:: FILETIME, pvaluestruct : *const CLUSPROP_FILETIME, ftoldvalue : super::super::Foundation:: FILETIME, ftminimum : super::super::Foundation:: FILETIME, ftmaximum : super::super::Foundation:: FILETIME, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetFileTimeProperty(pftoutvalue as _, pvaluestruct, core::mem::transmute(ftoldvalue), core::mem::transmute(ftminimum), core::mem::transmute(ftmaximum), pppropertylist as _, pcbpropertylistsize as _) }
}
#[inline]
pub unsafe fn ResUtilGetLongProperty(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetLongProperty(ploutvalue : *mut i32, pvaluestruct : *const CLUSPROP_LONG, loldvalue : i32, lminimum : i32, lmaximum : i32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetLongProperty(ploutvalue as _, pvaluestruct, loldvalue, lminimum, lmaximum, pppropertylist as _, pcbpropertylistsize as _) }
}
#[inline]
pub unsafe fn ResUtilGetMultiSzProperty<P3>(ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P3, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetMultiSzProperty(ppszoutvalue : *mut windows_core::PWSTR, pcboutvaluesize : *mut u32, pvaluestruct : *const CLUSPROP_SZ, pszoldvalue : windows_core::PCWSTR, cboldvaluesize : u32, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetMultiSzProperty(ppszoutvalue as _, pcboutvaluesize as _, pvaluestruct, pszoldvalue.param().abi(), cboldvaluesize, pppropertylist as _, pcbpropertylistsize as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetPrivateProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilGetPrivateProperties(hkeyclusterkey, poutpropertylist as _, cboutpropertylistsize, pcbbytesreturned as _, pcbrequired as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, cboutpropertylistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilGetProperties(hkeyclusterkey, ppropertytable, poutpropertylist as _, cboutpropertylistsize, pcbbytesreturned as _, pcbrequired as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPropertiesToParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: bool, psznameofpropinerror: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetPropertiesToParameterBlock(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutparams : *mut u8, bcheckforrequiredproperties : windows_core::BOOL, psznameofpropinerror : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilGetPropertiesToParameterBlock(hkeyclusterkey, ppropertytable, poutparams as _, bcheckforrequiredproperties.into(), psznameofpropinerror as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetProperty(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetProperty(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytableitem : *const RESUTIL_PROPERTY_ITEM, poutpropertyitem : *mut *mut core::ffi::c_void, pcboutpropertyitemsize : *mut u32) -> u32);
    unsafe { ResUtilGetProperty(hkeyclusterkey, ppropertytableitem, poutpropertyitem as _, pcboutpropertyitemsize as _) }
}
#[inline]
pub unsafe fn ResUtilGetPropertyFormats(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetPropertyFormats(ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist : *mut core::ffi::c_void, cbpropertyformatlistsize : u32, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilGetPropertyFormats(ppropertytable, poutpropertyformatlist as _, cbpropertyformatlistsize, pcbbytesreturned as _, pcbrequired as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetPropertySize(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetPropertySize(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytableitem : *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize : *mut u32, pnpropertycount : *mut u32) -> u32);
    unsafe { ResUtilGetPropertySize(hkeyclusterkey, ppropertytableitem, pcboutpropertylistsize as _, pnpropertycount as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetQwordValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetQwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pqwoutvalue : *mut u64, qwdefaultvalue : u64) -> u32);
    unsafe { ResUtilGetQwordValue(hkeyclusterkey, pszvaluename.param().abi(), pqwoutvalue as _, qwdefaultvalue) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependency<P1>(hself: super::super::Foundation::HANDLE, lpszresourcetype: P1) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependency(hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependency(hself, lpszresourcetype.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClass(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: bool) -> HRESOURCE {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByClass(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, prci : *mut CLUS_RESOURCE_CLASS_INFO, brecurse : windows_core::BOOL) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependencyByClass(hcluster, hself, prci as _, brecurse.into()) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByClassEx(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: bool, dwdesiredaccess: u32) -> HRESOURCE {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByClassEx(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, prci : *mut CLUS_RESOURCE_CLASS_INFO, brecurse : windows_core::BOOL, dwdesiredaccess : u32) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependencyByClassEx(hcluster, hself, prci as _, brecurse.into(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByName<P2>(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: P2, brecurse: bool) -> HRESOURCE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByName(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, brecurse : windows_core::BOOL) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependencyByName(hcluster, hself, lpszresourcetype.param().abi(), brecurse.into()) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyByNameEx<P2>(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: P2, brecurse: bool, dwdesiredaccess: u32) -> HRESOURCE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyByNameEx(hcluster : HCLUSTER, hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, brecurse : windows_core::BOOL, dwdesiredaccess : u32) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependencyByNameEx(hcluster, hself, lpszresourcetype.param().abi(), brecurse.into(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependencyEx<P1>(hself: super::super::Foundation::HANDLE, lpszresourcetype: P1, dwdesiredaccess: u32) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependencyEx(hself : super::super::Foundation:: HANDLE, lpszresourcetype : windows_core::PCWSTR, dwdesiredaccess : u32) -> HRESOURCE);
    unsafe { ResUtilGetResourceDependencyEx(hself, lpszresourcetype.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilGetResourceDependentIPAddressProps(hresource: HRESOURCE, pszaddress: windows_core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: windows_core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: windows_core::PWSTR, pcchnetwork: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceDependentIPAddressProps(hresource : HRESOURCE, pszaddress : windows_core::PWSTR, pcchaddress : *mut u32, pszsubnetmask : windows_core::PWSTR, pcchsubnetmask : *mut u32, psznetwork : windows_core::PWSTR, pcchnetwork : *mut u32) -> u32);
    unsafe { ResUtilGetResourceDependentIPAddressProps(hresource, core::mem::transmute(pszaddress), pcchaddress as _, core::mem::transmute(pszsubnetmask), pcchsubnetmask as _, core::mem::transmute(psznetwork), pcchnetwork as _) }
}
#[inline]
pub unsafe fn ResUtilGetResourceName(hresource: HRESOURCE, pszresourcename: windows_core::PWSTR, pcchresourcenameinout: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceName(hresource : HRESOURCE, pszresourcename : windows_core::PWSTR, pcchresourcenameinout : *mut u32) -> u32);
    unsafe { ResUtilGetResourceName(hresource, core::mem::transmute(pszresourcename), pcchresourcenameinout as _) }
}
#[inline]
pub unsafe fn ResUtilGetResourceNameDependency<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1) -> HRESOURCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceNameDependency(lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { ResUtilGetResourceNameDependency(lpszresourcename.param().abi(), lpszresourcetype.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilGetResourceNameDependencyEx<P0, P1>(lpszresourcename: P0, lpszresourcetype: P1, dwdesiredaccess: u32) -> HRESOURCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetResourceNameDependencyEx(lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwdesiredaccess : u32) -> HRESOURCE);
    unsafe { ResUtilGetResourceNameDependencyEx(lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn ResUtilGetSzProperty<P2>(ppszoutvalue: *mut windows_core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: P2, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetSzProperty(ppszoutvalue : *mut windows_core::PWSTR, pvaluestruct : *const CLUSPROP_SZ, pszoldvalue : windows_core::PCWSTR, pppropertylist : *mut *mut u8, pcbpropertylistsize : *mut u32) -> u32);
    unsafe { ResUtilGetSzProperty(ppszoutvalue as _, pvaluestruct, pszoldvalue.param().abi(), pppropertylist as _, pcbpropertylistsize as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilGetSzValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilGetSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { ResUtilGetSzValue(hkeyclusterkey, pszvaluename.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilGroupsEqual(hself: HGROUP, hgroup: HGROUP, pequal: *mut windows_core::BOOL) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilGroupsEqual(hself : HGROUP, hgroup : HGROUP, pequal : *mut windows_core::BOOL) -> u32);
    unsafe { ResUtilGroupsEqual(hself, hgroup, pequal as _) }
}
#[inline]
pub unsafe fn ResUtilIsPathValid<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilIsPathValid(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ResUtilIsPathValid(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn ResUtilIsResourceClassEqual(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: HRESOURCE) -> windows_core::BOOL {
    windows_core::link!("resutils.dll" "system" fn ResUtilIsResourceClassEqual(prci : *mut CLUS_RESOURCE_CLASS_INFO, hresource : HRESOURCE) -> windows_core::BOOL);
    unsafe { ResUtilIsResourceClassEqual(prci as _, hresource) }
}
#[inline]
pub unsafe fn ResUtilLeftPaxosIsLessThanRight(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> windows_core::BOOL {
    windows_core::link!("resutils.dll" "system" fn ResUtilLeftPaxosIsLessThanRight(left : *const PaxosTagCStruct, right : *const PaxosTagCStruct) -> windows_core::BOOL);
    unsafe { ResUtilLeftPaxosIsLessThanRight(left, right) }
}
#[inline]
pub unsafe fn ResUtilNodeEnum(hcluster: HCLUSTER, pnodecallback: LPNODE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilNodeEnum(hcluster : HCLUSTER, pnodecallback : LPNODE_CALLBACK, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilNodeEnum(hcluster, pnodecallback, pparameter as _) }
}
#[inline]
pub unsafe fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> windows_core::BOOL {
    windows_core::link!("resutils.dll" "system" fn ResUtilPaxosComparer(left : *const PaxosTagCStruct, right : *const PaxosTagCStruct) -> windows_core::BOOL);
    unsafe { ResUtilPaxosComparer(left, right) }
}
#[inline]
pub unsafe fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: Option<*mut core::ffi::c_void>, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilPropertyListFromParameterBlock(ppropertytable : *const RESUTIL_PROPERTY_ITEM, poutpropertylist : *mut core::ffi::c_void, pcboutpropertylistsize : *mut u32, pinparams : *const u8, pcbbytesreturned : *mut u32, pcbrequired : *mut u32) -> u32);
    unsafe { ResUtilPropertyListFromParameterBlock(ppropertytable, poutpropertylist.unwrap_or(core::mem::zeroed()) as _, pcboutpropertylistsize as _, pinparams, pcbbytesreturned as _, pcbrequired as _) }
}
#[inline]
pub unsafe fn ResUtilRemoveResourceServiceEnvironment<P0>(pszservicename: P0, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilRemoveResourceServiceEnvironment(pszservicename : windows_core::PCWSTR, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    unsafe { ResUtilRemoveResourceServiceEnvironment(pszservicename.param().abi(), pfnlogevent, hresourcehandle) }
}
#[inline]
pub unsafe fn ResUtilResourceDepEnum(hself: HRESOURCE, enumtype: u32, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilResourceDepEnum(hself : HRESOURCE, enumtype : u32, prescallback : LPRESOURCE_CALLBACK_EX, pparameter : *mut core::ffi::c_void) -> u32);
    unsafe { ResUtilResourceDepEnum(hself, enumtype, prescallback, pparameter as _) }
}
#[inline]
pub unsafe fn ResUtilResourceTypesEqual<P0>(lpszresourcetypename: P0, hresource: HRESOURCE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilResourceTypesEqual(lpszresourcetypename : windows_core::PCWSTR, hresource : HRESOURCE) -> windows_core::BOOL);
    unsafe { ResUtilResourceTypesEqual(lpszresourcetypename.param().abi(), hresource) }
}
#[inline]
pub unsafe fn ResUtilResourcesEqual(hself: HRESOURCE, hresource: HRESOURCE) -> windows_core::BOOL {
    windows_core::link!("resutils.dll" "system" fn ResUtilResourcesEqual(hself : HRESOURCE, hresource : HRESOURCE) -> windows_core::BOOL);
    unsafe { ResUtilResourcesEqual(hself, hresource) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetBinaryValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, pbnewvalue: &[u8], ppboutvalue: Option<*mut *mut u8>, pcboutvaluesize: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetBinaryValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, pbnewvalue : *const u8, cbnewvaluesize : u32, ppboutvalue : *mut *mut u8, pcboutvaluesize : *mut u32) -> u32);
    unsafe { ResUtilSetBinaryValue(hkeyclusterkey, pszvaluename.param().abi(), core::mem::transmute(pbnewvalue.as_ptr()), pbnewvalue.len().try_into().unwrap(), ppboutvalue.unwrap_or(core::mem::zeroed()) as _, pcboutvaluesize as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetDwordValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetDwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, dwnewvalue : u32, pdwoutvalue : *mut u32) -> u32);
    unsafe { ResUtilSetDwordValue(hkeyclusterkey, pszvaluename.param().abi(), dwnewvalue, pdwoutvalue as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetExpandSzValue<P1, P2>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, psznewvalue: P2, ppszoutstring: Option<*mut windows_core::PWSTR>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetExpandSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, ppszoutstring : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilSetExpandSzValue(hkeyclusterkey, pszvaluename.param().abi(), psznewvalue.param().abi(), ppszoutstring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetMultiSzValue<P1, P2>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, psznewvalue: P2, cbnewvaluesize: u32, ppszoutvalue: Option<*mut windows_core::PWSTR>, pcboutvaluesize: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetMultiSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, cbnewvaluesize : u32, ppszoutvalue : *mut windows_core::PWSTR, pcboutvaluesize : *mut u32) -> u32);
    unsafe { ResUtilSetMultiSzValue(hkeyclusterkey, pszvaluename.param().abi(), psznewvalue.param().abi(), cbnewvaluesize, ppszoutvalue.unwrap_or(core::mem::zeroed()) as _, pcboutvaluesize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPrivatePropertyList(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetPrivatePropertyList(hkeyclusterkey : super::super::System::Registry:: HKEY, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    unsafe { ResUtilSetPrivatePropertyList(hkeyclusterkey, pinpropertylist, cbinpropertylistsize) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetPropertyParameterBlock(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, pinparams : *const u8, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    unsafe { ResUtilSetPropertyParameterBlock(hkeyclusterkey, ppropertytable, reserved as _, pinparams, pinpropertylist, cbinpropertylistsize, poutparams as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyParameterBlockEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: bool, poutparams: *mut u8) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetPropertyParameterBlockEx(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, pinparams : *const u8, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, bforcewrite : windows_core::BOOL, poutparams : *mut u8) -> u32);
    unsafe { ResUtilSetPropertyParameterBlockEx(hkeyclusterkey, ppropertytable, reserved as _, pinparams, pinpropertylist, cbinpropertylistsize, bforcewrite.into(), poutparams as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyTable(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: Option<*const core::ffi::c_void>, ballowunknownproperties: bool, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: Option<*mut u8>) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetPropertyTable(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *const core::ffi::c_void, ballowunknownproperties : windows_core::BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    unsafe { ResUtilSetPropertyTable(hkeyclusterkey, ppropertytable, reserved.unwrap_or(core::mem::zeroed()) as _, ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, poutparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetPropertyTableEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, ballowunknownproperties: bool, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: bool, poutparams: *mut u8) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetPropertyTableEx(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *mut core::ffi::c_void, ballowunknownproperties : windows_core::BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, bforcewrite : windows_core::BOOL, poutparams : *mut u8) -> u32);
    unsafe { ResUtilSetPropertyTableEx(hkeyclusterkey, ppropertytable, reserved as _, ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, bforcewrite.into(), poutparams as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetQwordValue<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, qwnewvalue: u64, pqwoutvalue: Option<*mut u64>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetQwordValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, qwnewvalue : u64, pqwoutvalue : *mut u64) -> u32);
    unsafe { ResUtilSetQwordValue(hkeyclusterkey, pszvaluename.param().abi(), qwnewvalue, pqwoutvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilSetResourceServiceEnvironment<P0>(pszservicename: P0, hresource: HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetResourceServiceEnvironment(pszservicename : windows_core::PCWSTR, hresource : HRESOURCE, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    unsafe { ResUtilSetResourceServiceEnvironment(pszservicename.param().abi(), hresource, pfnlogevent, hresourcehandle) }
}
#[cfg(feature = "Win32_System_Services")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParameters<P0>(pszservicename: P0, schscmhandle: super::super::System::Services::SC_HANDLE, phservice: *mut super::super::System::Services::SC_HANDLE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetResourceServiceStartParameters(pszservicename : windows_core::PCWSTR, schscmhandle : super::super::System::Services:: SC_HANDLE, phservice : *mut super::super::System::Services:: SC_HANDLE, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    unsafe { ResUtilSetResourceServiceStartParameters(pszservicename.param().abi(), schscmhandle, phservice as _, pfnlogevent, hresourcehandle) }
}
#[cfg(feature = "Win32_System_Services")]
#[inline]
pub unsafe fn ResUtilSetResourceServiceStartParametersEx<P0>(pszservicename: P0, schscmhandle: super::super::System::Services::SC_HANDLE, phservice: *mut super::super::System::Services::SC_HANDLE, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetResourceServiceStartParametersEx(pszservicename : windows_core::PCWSTR, schscmhandle : super::super::System::Services:: SC_HANDLE, phservice : *mut super::super::System::Services:: SC_HANDLE, dwdesiredaccess : u32, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    unsafe { ResUtilSetResourceServiceStartParametersEx(pszservicename.param().abi(), schscmhandle, phservice as _, dwdesiredaccess, pfnlogevent, hresourcehandle) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetSzValue<P1, P2>(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: P1, psznewvalue: P2, ppszoutstring: Option<*mut windows_core::PWSTR>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetSzValue(hkeyclusterkey : super::super::System::Registry:: HKEY, pszvaluename : windows_core::PCWSTR, psznewvalue : windows_core::PCWSTR, ppszoutstring : *mut windows_core::PWSTR) -> u32);
    unsafe { ResUtilSetSzValue(hkeyclusterkey, pszvaluename.param().abi(), psznewvalue.param().abi(), ppszoutstring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilSetUnknownProperties(hkeyclusterkey : super::super::System::Registry:: HKEY, ppropertytable : *const RESUTIL_PROPERTY_ITEM, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    unsafe { ResUtilSetUnknownProperties(hkeyclusterkey, ppropertytable, pinpropertylist, cbinpropertylistsize) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilSetValueEx<P1>(hkeyclusterkey: super::super::System::Registry::HKEY, valuename: P1, valuetype: u32, valuedata: &[u8], flags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilSetValueEx(hkeyclusterkey : super::super::System::Registry:: HKEY, valuename : windows_core::PCWSTR, valuetype : u32, valuedata : *const u8, valuesize : u32, flags : u32) -> u32);
    unsafe { ResUtilSetValueEx(hkeyclusterkey, valuename.param().abi(), valuetype, core::mem::transmute(valuedata.as_ptr()), valuedata.len().try_into().unwrap(), flags) }
}
#[cfg(feature = "Win32_System_Services")]
#[inline]
pub unsafe fn ResUtilStartResourceService<P0>(pszservicename: P0, phservicehandle: *mut super::super::System::Services::SC_HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilStartResourceService(pszservicename : windows_core::PCWSTR, phservicehandle : *mut super::super::System::Services:: SC_HANDLE) -> u32);
    unsafe { ResUtilStartResourceService(pszservicename.param().abi(), phservicehandle as _) }
}
#[inline]
pub unsafe fn ResUtilStopResourceService<P0>(pszservicename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilStopResourceService(pszservicename : windows_core::PCWSTR) -> u32);
    unsafe { ResUtilStopResourceService(pszservicename.param().abi()) }
}
#[cfg(feature = "Win32_System_Services")]
#[inline]
pub unsafe fn ResUtilStopService(hservicehandle: super::super::System::Services::SC_HANDLE) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilStopService(hservicehandle : super::super::System::Services:: SC_HANDLE) -> u32);
    unsafe { ResUtilStopService(hservicehandle) }
}
#[inline]
pub unsafe fn ResUtilTerminateServiceProcessFromResDll(dwservicepid: u32, boffline: bool, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilTerminateServiceProcessFromResDll(dwservicepid : u32, boffline : windows_core::BOOL, pdwresourcestate : *mut u32, pfnlogevent : PLOG_EVENT_ROUTINE, hresourcehandle : isize) -> u32);
    unsafe { ResUtilTerminateServiceProcessFromResDll(dwservicepid, boffline.into(), pdwresourcestate as _, pfnlogevent, hresourcehandle) }
}
#[inline]
pub unsafe fn ResUtilVerifyPrivatePropertyList(pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilVerifyPrivatePropertyList(pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32) -> u32);
    unsafe { ResUtilVerifyPrivatePropertyList(pinpropertylist, cbinpropertylistsize) }
}
#[inline]
pub unsafe fn ResUtilVerifyPropertyTable(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: Option<*const core::ffi::c_void>, ballowunknownproperties: bool, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: Option<*mut u8>) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilVerifyPropertyTable(ppropertytable : *const RESUTIL_PROPERTY_ITEM, reserved : *const core::ffi::c_void, ballowunknownproperties : windows_core::BOOL, pinpropertylist : *const core::ffi::c_void, cbinpropertylistsize : u32, poutparams : *mut u8) -> u32);
    unsafe { ResUtilVerifyPropertyTable(ppropertytable, reserved.unwrap_or(core::mem::zeroed()) as _, ballowunknownproperties.into(), pinpropertylist, cbinpropertylistsize, poutparams.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ResUtilVerifyResourceService<P0>(pszservicename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilVerifyResourceService(pszservicename : windows_core::PCWSTR) -> u32);
    unsafe { ResUtilVerifyResourceService(pszservicename.param().abi()) }
}
#[cfg(feature = "Win32_System_Services")]
#[inline]
pub unsafe fn ResUtilVerifyService(hservicehandle: super::super::System::Services::SC_HANDLE) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilVerifyService(hservicehandle : super::super::System::Services:: SC_HANDLE) -> u32);
    unsafe { ResUtilVerifyService(hservicehandle) }
}
#[inline]
pub unsafe fn ResUtilVerifyShutdownSafe(flags: u32, reason: u32, presult: *mut u32) -> u32 {
    windows_core::link!("resutils.dll" "system" fn ResUtilVerifyShutdownSafe(flags : u32, reason : u32, presult : *mut u32) -> u32);
    unsafe { ResUtilVerifyShutdownSafe(flags, reason, presult as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn ResUtilsDeleteKeyTree<P1>(key: super::super::System::Registry::HKEY, keyname: P1, treatnokeyaserror: bool) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("resutils.dll" "system" fn ResUtilsDeleteKeyTree(key : super::super::System::Registry:: HKEY, keyname : windows_core::PCWSTR, treatnokeyaserror : windows_core::BOOL) -> u32);
    unsafe { ResUtilsDeleteKeyTree(key, keyname.param().abi(), treatnokeyaserror.into()) }
}
#[inline]
pub unsafe fn ResetAllAppInstanceVersions() -> u32 {
    windows_core::link!("ntlanman.dll" "system" fn ResetAllAppInstanceVersions() -> u32);
    unsafe { ResetAllAppInstanceVersions() }
}
#[inline]
pub unsafe fn RestartClusterResource(hresource: HRESOURCE, dwflags: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RestartClusterResource(hresource : HRESOURCE, dwflags : u32) -> u32);
    unsafe { RestartClusterResource(hresource, dwflags) }
}
#[inline]
pub unsafe fn RestartClusterResourceEx<P2>(hresource: HRESOURCE, dwflags: u32, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RestartClusterResourceEx(hresource : HRESOURCE, dwflags : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RestartClusterResourceEx(hresource, dwflags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RestoreClusterDatabase<P0, P2>(lpszpathname: P0, bforce: bool, lpszquorumdriveletter: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RestoreClusterDatabase(lpszpathname : windows_core::PCWSTR, bforce : windows_core::BOOL, lpszquorumdriveletter : windows_core::PCWSTR) -> u32);
    unsafe { RestoreClusterDatabase(lpszpathname.param().abi(), bforce.into(), lpszquorumdriveletter.param().abi()) }
}
#[inline]
pub unsafe fn ResumeClusterNode(hnode: HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNode(hnode : HNODE) -> u32);
    unsafe { ResumeClusterNode(hnode) }
}
#[inline]
pub unsafe fn ResumeClusterNodeEx(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNodeEx(hnode : HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32) -> u32);
    unsafe { ResumeClusterNodeEx(hnode, eresumefailbacktype, dwresumeflagsreserved) }
}
#[inline]
pub unsafe fn ResumeClusterNodeEx2<P3>(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNodeEx2(hnode : HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ResumeClusterNodeEx2(hnode, eresumefailbacktype, dwresumeflagsreserved, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetAppInstanceCsvFlags(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32 {
    windows_core::link!("ntlanman.dll" "system" fn SetAppInstanceCsvFlags(processhandle : super::super::Foundation:: HANDLE, mask : u32, flags : u32) -> u32);
    unsafe { SetAppInstanceCsvFlags(processhandle, mask, flags) }
}
#[inline]
pub unsafe fn SetClusterGroupName<P1>(hgroup: HGROUP, lpszgroupname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupName(hgroup : HGROUP, lpszgroupname : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupName(hgroup, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupNameEx<P1, P2>(hgroup: HGROUP, lpszgroupname: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNameEx(hgroup : HGROUP, lpszgroupname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupNameEx(hgroup, lpszgroupname.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupNodeList(hgroup: HGROUP, nodelist: Option<&[HNODE]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNodeList(hgroup : HGROUP, nodecount : u32, nodelist : *const HNODE) -> u32);
    unsafe { SetClusterGroupNodeList(hgroup, nodelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(nodelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn SetClusterGroupNodeListEx<P3>(hgroup: HGROUP, nodelist: &[HNODE], lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNodeListEx(hgroup : HGROUP, nodecount : u32, nodelist : *const HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupNodeListEx(hgroup, nodelist.len().try_into().unwrap(), core::mem::transmute(nodelist.as_ptr()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpression<P1>(hgroupset: HGROUPSET, lpszdependencyexprssion: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpression(hgroupset : HGROUPSET, lpszdependencyexprssion : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupSetDependencyExpression(hgroupset, lpszdependencyexprssion.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpressionEx<P1, P2>(hgroupset: HGROUPSET, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpressionEx(hgroupset : HGROUPSET, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupSetDependencyExpressionEx(hgroupset, lpszdependencyexpression.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterName<P1>(hcluster: HCLUSTER, lpsznewclustername: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterName(hcluster : HCLUSTER, lpsznewclustername : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterName(hcluster, lpsznewclustername.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNameEx<P1, P2>(hcluster: HCLUSTER, lpsznewclustername: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNameEx(hcluster : HCLUSTER, lpsznewclustername : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNameEx(hcluster, lpsznewclustername.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkName<P1>(hnetwork: HNETWORK, lpszname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkName(hnetwork : HNETWORK, lpszname : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNetworkName(hnetwork, lpszname.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkNameEx<P1, P2>(hnetwork: HNETWORK, lpszname: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkNameEx(hnetwork : HNETWORK, lpszname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNetworkNameEx(hnetwork, lpszname.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkPriorityOrder(hcluster: HCLUSTER, networklist: &[HNETWORK]) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkPriorityOrder(hcluster : HCLUSTER, networkcount : u32, networklist : *const HNETWORK) -> u32);
    unsafe { SetClusterNetworkPriorityOrder(hcluster, networklist.len().try_into().unwrap(), core::mem::transmute(networklist.as_ptr())) }
}
#[inline]
pub unsafe fn SetClusterQuorumResource<P1>(hresource: HRESOURCE, lpszdevicename: P1, dwmaxquologsize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterQuorumResource(hresource : HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquologsize : u32) -> u32);
    unsafe { SetClusterQuorumResource(hresource, lpszdevicename.param().abi(), dwmaxquologsize) }
}
#[inline]
pub unsafe fn SetClusterQuorumResourceEx<P1, P3>(hresource: HRESOURCE, lpszdevicename: P1, dwmaxquorumlogsize: u32, lpszreason: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterQuorumResourceEx(hresource : HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquorumlogsize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterQuorumResourceEx(hresource, lpszdevicename.param().abi(), dwmaxquorumlogsize, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceDependencyExpression<P1>(hresource: HRESOURCE, lpszdependencyexpression: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceDependencyExpression(hresource : HRESOURCE, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceDependencyExpression(hresource, lpszdependencyexpression.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceName<P1>(hresource: HRESOURCE, lpszresourcename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceName(hresource : HRESOURCE, lpszresourcename : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceName(hresource, lpszresourcename.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceNameEx<P1, P2>(hresource: HRESOURCE, lpszresourcename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceNameEx(hresource : HRESOURCE, lpszresourcename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceNameEx(hresource, lpszresourcename.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterServiceAccountPassword<P0, P1>(lpszclustername: P0, lpsznewpassword: P1, dwflags: u32, lpreturnstatusbuffer: Option<*mut CLUSTER_SET_PASSWORD_STATUS>, lpcbreturnstatusbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterServiceAccountPassword(lpszclustername : windows_core::PCWSTR, lpsznewpassword : windows_core::PCWSTR, dwflags : u32, lpreturnstatusbuffer : *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize : *mut u32) -> u32);
    unsafe { SetClusterServiceAccountPassword(lpszclustername.param().abi(), lpsznewpassword.param().abi(), dwflags, lpreturnstatusbuffer.unwrap_or(core::mem::zeroed()) as _, lpcbreturnstatusbuffersize as _) }
}
#[inline]
pub unsafe fn SetGroupDependencyExpression<P1>(hgroup: HGROUP, lpszdependencyexpression: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetGroupDependencyExpression(hgroup : HGROUP, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    unsafe { SetGroupDependencyExpression(hgroup, lpszdependencyexpression.param().abi()) }
}
#[inline]
pub unsafe fn SetGroupDependencyExpressionEx<P1, P2>(hgroup: HGROUP, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetGroupDependencyExpressionEx(hgroup : HGROUP, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetGroupDependencyExpressionEx(hgroup, lpszdependencyexpression.param().abi(), lpszreason.param().abi()) }
}
pub const BitLockerDecrypted: i32 = 4i32;
pub const BitLockerDecrypting: i32 = 16i32;
pub const BitLockerEnabled: i32 = 1i32;
pub const BitLockerPaused: i32 = 64i32;
pub const BitLockerStopped: i32 = 128i32;
pub const BitlockerEncrypted: i32 = 8i32;
pub const BitlockerEncrypting: i32 = 32i32;
pub const CA_UPGRADE_VERSION: u32 = 1u32;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194478i32);
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: CLCTL_CODES = CLCTL_CODES(4195030i32);
pub const CLCTL_ADD_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242898i32);
pub const CLCTL_ADD_OWNER: CLCTL_CODES = CLCTL_CODES(5242906i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194466i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: CLCTL_CODES = CLCTL_CODES(4194498i32);
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: CLCTL_CODES = CLCTL_CODES(4194494i32);
pub const CLCTL_BATCH_BLOCK_KEY: CLCTL_CODES = CLCTL_CODES(574i32);
pub const CLCTL_BATCH_UNBLOCK_KEY: CLCTL_CODES = CLCTL_CODES(577i32);
pub const CLCTL_BLOCK_GEM_SEND_RECV: CLCTL_CODES = CLCTL_CODES(717i32);
pub const CLCTL_CHECK_DRAIN_VETO: CLCTL_CODES = CLCTL_CODES(1057069i32);
pub const CLCTL_CHECK_VOTER_DOWN: CLCTL_CODES = CLCTL_CODES(73i32);
pub const CLCTL_CHECK_VOTER_DOWN_WITNESS: CLCTL_CODES = CLCTL_CODES(113i32);
pub const CLCTL_CHECK_VOTER_EVICT: CLCTL_CODES = CLCTL_CODES(69i32);
pub const CLCTL_CHECK_VOTER_EVICT_WITNESS: CLCTL_CODES = CLCTL_CODES(109i32);
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: CLCTL_CODES = CLCTL_CODES(4195078i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLCTL_CODES = CLCTL_CODES(8417i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLCTL_CODES = CLCTL_CODES(8433i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLCTL_CODES = CLCTL_CODES(4202742i32);
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLCTL_CODES = CLCTL_CODES(4202726i32);
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
pub const CLCTL_CLUSTER_NAME_CHANGED: CLCTL_CODES = CLCTL_CODES(5242922i32);
pub const CLCTL_CLUSTER_VERSION_CHANGED: CLCTL_CODES = CLCTL_CODES(5242926i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLCTL_CODES(pub i32);
pub const CLCTL_DELETE: CLCTL_CODES = CLCTL_CODES(5242886i32);
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194482i32);
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: CLCTL_CODES = CLCTL_CODES(4194470i32);
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194958i32);
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = CLCTL_CODES(4194954i32);
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: CLCTL_CODES = CLCTL_CODES(11741i32);
pub const CLCTL_ENUM_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(81i32);
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(121i32);
pub const CLCTL_EVICT_NODE: CLCTL_CODES = CLCTL_CODES(5242894i32);
pub const CLCTL_FILESERVER_SHARE_ADD: CLCTL_CODES = CLCTL_CODES(4194886i32);
pub const CLCTL_FILESERVER_SHARE_DEL: CLCTL_CODES = CLCTL_CODES(4194890i32);
pub const CLCTL_FILESERVER_SHARE_MODIFY: CLCTL_CODES = CLCTL_CODES(4194894i32);
pub const CLCTL_FILESERVER_SHARE_REPORT: CLCTL_CODES = CLCTL_CODES(593i32);
pub const CLCTL_FIXUP_ON_UPGRADE: CLCTL_CODES = CLCTL_CODES(5242930i32);
pub const CLCTL_FORCE_DB_FLUSH: CLCTL_CODES = CLCTL_CODES(4206054i32);
pub const CLCTL_FORCE_QUORUM: CLCTL_CODES = CLCTL_CODES(5242950i32);
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(1048669i32);
pub const CLCTL_FSWITNESS_RELEASE_LOCK: CLCTL_CODES = CLCTL_CODES(5242982i32);
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: CLCTL_CODES = CLCTL_CODES(5242978i32);
pub const CLCTL_GET_ARB_TIMEOUT: CLCTL_CODES = CLCTL_CODES(21i32);
pub const CLCTL_GET_CHARACTERISTICS: CLCTL_CODES = CLCTL_CODES(5i32);
pub const CLCTL_GET_CLASS_INFO: CLCTL_CODES = CLCTL_CODES(13i32);
pub const CLCTL_GET_CLUSDB_TIMESTAMP: CLCTL_CODES = CLCTL_CODES(681i32);
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLCTL_CODES = CLCTL_CODES(65i32);
pub const CLCTL_GET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(89i32);
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(101i32);
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(105i32);
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(181i32);
pub const CLCTL_GET_DNS_NAME: CLCTL_CODES = CLCTL_CODES(373i32);
pub const CLCTL_GET_FAILURE_INFO: CLCTL_CODES = CLCTL_CODES(25i32);
pub const CLCTL_GET_FLAGS: CLCTL_CODES = CLCTL_CODES(9i32);
pub const CLCTL_GET_FQDN: CLCTL_CODES = CLCTL_CODES(61i32);
pub const CLCTL_GET_GEMID_VECTOR: CLCTL_CODES = CLCTL_CODES(721i32);
pub const CLCTL_GET_GUM_LOCK_OWNER: CLCTL_CODES = CLCTL_CODES(697i32);
pub const CLCTL_GET_ID: CLCTL_CODES = CLCTL_CODES(57i32);
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(11657i32);
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: CLCTL_CODES = CLCTL_CODES(201i32);
pub const CLCTL_GET_NAME: CLCTL_CODES = CLCTL_CODES(41i32);
pub const CLCTL_GET_NETWORK: CLCTL_CODES = CLCTL_CODES(53i32);
pub const CLCTL_GET_NETWORK_NAME: CLCTL_CODES = CLCTL_CODES(361i32);
pub const CLCTL_GET_NODE: CLCTL_CODES = CLCTL_CODES(49i32);
pub const CLCTL_GET_NODES_IN_FD: CLCTL_CODES = CLCTL_CODES(11745i32);
pub const CLCTL_GET_OPERATION_CONTEXT: CLCTL_CODES = CLCTL_CODES(1057001i32);
pub const CLCTL_GET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(129i32);
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(141i32);
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = CLCTL_CODES(145i32);
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: CLCTL_CODES = CLCTL_CODES(169i32);
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: CLCTL_CODES = CLCTL_CODES(17i32);
pub const CLCTL_GET_RESOURCE_TYPE: CLCTL_CODES = CLCTL_CODES(45i32);
pub const CLCTL_GET_RO_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(85i32);
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(125i32);
pub const CLCTL_GET_SHARED_VOLUME_ID: CLCTL_CODES = CLCTL_CODES(657i32);
pub const CLCTL_GET_STATE_CHANGE_TIME: CLCTL_CODES = CLCTL_CODES(11613i32);
pub const CLCTL_GET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(741i32);
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: CLCTL_CODES = CLCTL_CODES(745i32);
pub const CLCTL_GET_STUCK_NODES: CLCTL_CODES = CLCTL_CODES(701i32);
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
pub const CLCTL_GROUPSET_GET_GROUPS: CLCTL_CODES = CLCTL_CODES(11633i32);
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11637i32);
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11641i32);
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: CLCTL_CODES = CLCTL_CODES(729i32);
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: CLCTL_CODES = CLCTL_CODES(11645i32);
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: CLCTL_CODES = CLCTL_CODES(11649i32);
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: CLCTL_CODES = CLCTL_CODES(4205958i32);
pub const CLCTL_HOLD_IO: CLCTL_CODES = CLCTL_CODES(5242942i32);
pub const CLCTL_INITIALIZE: CLCTL_CODES = CLCTL_CODES(5242954i32);
pub const CLCTL_INJECT_GEM_FAULT: CLCTL_CODES = CLCTL_CODES(705i32);
pub const CLCTL_INSTALL_NODE: CLCTL_CODES = CLCTL_CODES(5242890i32);
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: CLCTL_CODES = CLCTL_CODES(709i32);
pub const CLCTL_IPADDRESS_RELEASE_LEASE: CLCTL_CODES = CLCTL_CODES(4194754i32);
pub const CLCTL_IPADDRESS_RENEW_LEASE: CLCTL_CODES = CLCTL_CODES(4194750i32);
pub const CLCTL_IS_FEATURE_INSTALLED: CLCTL_CODES = CLCTL_CODES(753i32);
pub const CLCTL_IS_QUORUM_BLOCKED: CLCTL_CODES = CLCTL_CODES(689i32);
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: CLCTL_CODES = CLCTL_CODES(757i32);
pub const CLCTL_JOINING_GROUP: CLCTL_CODES = CLCTL_CODES(5242970i32);
pub const CLCTL_LEAVING_GROUP: CLCTL_CODES = CLCTL_CODES(5242966i32);
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: CLCTL_CODES = CLCTL_CODES(5242986i32);
pub const CLCTL_NETNAME_DELETE_CO: CLCTL_CODES = CLCTL_CODES(382i32);
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: CLCTL_CODES = CLCTL_CODES(4194926i32);
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLCTL_CODES = CLCTL_CODES(365i32);
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: CLCTL_CODES = CLCTL_CODES(370i32);
pub const CLCTL_NETNAME_REPAIR_VCO: CLCTL_CODES = CLCTL_CODES(397i32);
pub const CLCTL_NETNAME_RESET_VCO: CLCTL_CODES = CLCTL_CODES(389i32);
pub const CLCTL_NETNAME_SET_PWD_INFO: CLCTL_CODES = CLCTL_CODES(378i32);
pub const CLCTL_NETNAME_SET_PWD_INFOEX: CLCTL_CODES = CLCTL_CODES(794i32);
pub const CLCTL_NETNAME_VALIDATE_VCO: CLCTL_CODES = CLCTL_CODES(385i32);
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: CLCTL_CODES = CLCTL_CODES(1057073i32);
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: CLCTL_CODES = CLCTL_CODES(4205970i32);
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: CLCTL_CODES = CLCTL_CODES(1048705i32);
pub const CLCTL_NOTIFY_OWNER_CHANGE: CLCTL_CODES = CLCTL_CODES(5251362i32);
pub const CLCTL_NOTIFY_QUORUM_STATUS: CLCTL_CODES = CLCTL_CODES(5243006i32);
pub const CLCTL_POOL_GET_DRIVE_INFO: CLCTL_CODES = CLCTL_CODES(693i32);
pub const CLCTL_PROVIDER_STATE_CHANGE: CLCTL_CODES = CLCTL_CODES(5242962i32);
pub const CLCTL_QUERY_DELETE: CLCTL_CODES = CLCTL_CODES(441i32);
pub const CLCTL_QUERY_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(481i32);
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: CLCTL_CODES = CLCTL_CODES(11730i32);
pub const CLCTL_REMOVE_DEPENDENCY: CLCTL_CODES = CLCTL_CODES(5242902i32);
pub const CLCTL_REMOVE_NODE: CLCTL_CODES = CLCTL_CODES(4195054i32);
pub const CLCTL_REMOVE_OWNER: CLCTL_CODES = CLCTL_CODES(5242910i32);
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: CLCTL_CODES = CLCTL_CODES(8514i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLCTL_CODES = CLCTL_CODES(8521i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLCTL_CODES = CLCTL_CODES(8529i32);
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLCTL_CODES = CLCTL_CODES(8525i32);
pub const CLCTL_REPLICATION_GET_LOG_INFO: CLCTL_CODES = CLCTL_CODES(8517i32);
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: CLCTL_CODES = CLCTL_CODES(8541i32);
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: CLCTL_CODES = CLCTL_CODES(8533i32);
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLCTL_CODES = CLCTL_CODES(8549i32);
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: CLCTL_CODES = CLCTL_CODES(8537i32);
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: CLCTL_CODES = CLCTL_CODES(8545i32);
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: CLCTL_CODES = CLCTL_CODES(4202730i32);
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: CLCTL_CODES = CLCTL_CODES(4202734i32);
pub const CLCTL_RESOURCE_UPGRADE_DLL: CLCTL_CODES = CLCTL_CODES(4194490i32);
pub const CLCTL_RESUME_IO: CLCTL_CODES = CLCTL_CODES(5242946i32);
pub const CLCTL_RW_MODIFY_NOOP: CLCTL_CODES = CLCTL_CODES(4194990i32);
pub const CLCTL_SCALEOUT_COMMAND: CLCTL_CODES = CLCTL_CODES(4205974i32);
pub const CLCTL_SCALEOUT_CONTROL: CLCTL_CODES = CLCTL_CODES(4205978i32);
pub const CLCTL_SCALEOUT_GET_CLUSTERS: CLCTL_CODES = CLCTL_CODES(4205981i32);
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: CLCTL_CODES = CLCTL_CODES(713i32);
pub const CLCTL_SET_ACCOUNT_ACCESS: CLCTL_CODES = CLCTL_CODES(4194546i32);
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLCTL_CODES = CLCTL_CODES(4205934i32);
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: CLCTL_CODES = CLCTL_CODES(4205922i32);
pub const CLCTL_SET_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194398i32);
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194966i32);
pub const CLCTL_SET_DNS_DOMAIN: CLCTL_CODES = CLCTL_CODES(4195082i32);
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = CLCTL_CODES(4205966i32);
pub const CLCTL_SET_MAINTENANCE_MODE: CLCTL_CODES = CLCTL_CODES(4194790i32);
pub const CLCTL_SET_NAME: CLCTL_CODES = CLCTL_CODES(5242918i32);
pub const CLCTL_SET_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(4194438i32);
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: CLCTL_CODES = CLCTL_CODES(4194970i32);
pub const CLCTL_SET_STORAGE_CONFIGURATION: CLCTL_CODES = CLCTL_CODES(4195042i32);
pub const CLCTL_SHUTDOWN: CLCTL_CODES = CLCTL_CODES(77i32);
pub const CLCTL_STARTING_PHASE1: CLCTL_CODES = CLCTL_CODES(5242934i32);
pub const CLCTL_STARTING_PHASE2: CLCTL_CODES = CLCTL_CODES(5242938i32);
pub const CLCTL_STATE_CHANGE_REASON: CLCTL_CODES = CLCTL_CODES(5242958i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: CLCTL_CODES = CLCTL_CODES(405i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: CLCTL_CODES = CLCTL_CODES(501i32);
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLCTL_CODES = CLCTL_CODES(8161i32);
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: CLCTL_CODES = CLCTL_CODES(769i32);
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: CLCTL_CODES = CLCTL_CODES(765i32);
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: CLCTL_CODES = CLCTL_CODES(509i32);
pub const CLCTL_STORAGE_GET_DIRTY: CLCTL_CODES = CLCTL_CODES(537i32);
pub const CLCTL_STORAGE_GET_DISKID: CLCTL_CODES = CLCTL_CODES(517i32);
pub const CLCTL_STORAGE_GET_DISK_INFO: CLCTL_CODES = CLCTL_CODES(401i32);
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: CLCTL_CODES = CLCTL_CODES(497i32);
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: CLCTL_CODES = CLCTL_CODES(505i32);
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: CLCTL_CODES = CLCTL_CODES(417i32);
pub const CLCTL_STORAGE_GET_DRIVELETTERS: CLCTL_CODES = CLCTL_CODES(493i32);
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: CLCTL_CODES = CLCTL_CODES(529i32);
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: CLCTL_CODES = CLCTL_CODES(761i32);
pub const CLCTL_STORAGE_GET_RESOURCEID: CLCTL_CODES = CLCTL_CODES(557i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: CLCTL_CODES = CLCTL_CODES(549i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLCTL_CODES = CLCTL_CODES(669i32);
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: CLCTL_CODES = CLCTL_CODES(4194978i32);
pub const CLCTL_STORAGE_IS_CLUSTERABLE: CLCTL_CODES = CLCTL_CODES(521i32);
pub const CLCTL_STORAGE_IS_CSV_FILE: CLCTL_CODES = CLCTL_CODES(553i32);
pub const CLCTL_STORAGE_IS_PATH_VALID: CLCTL_CODES = CLCTL_CODES(409i32);
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(677i32);
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(513i32);
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: CLCTL_CODES = CLCTL_CODES(4194830i32);
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: CLCTL_CODES = CLCTL_CODES(11734i32);
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: CLCTL_CODES = CLCTL_CODES(11738i32);
pub const CLCTL_STORAGE_SET_DRIVELETTER: CLCTL_CODES = CLCTL_CODES(4194794i32);
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: CLCTL_CODES = CLCTL_CODES(4194718i32);
pub const CLCTL_UNDELETE: CLCTL_CODES = CLCTL_CODES(5243014i32);
pub const CLCTL_UNKNOWN: CLCTL_CODES = CLCTL_CODES(0i32);
pub const CLCTL_USER_SHIFT: u32 = 21u32;
pub const CLCTL_VALIDATE_CHANGE_GROUP: CLCTL_CODES = CLCTL_CODES(1057061i32);
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: CLCTL_CODES = CLCTL_CODES(97i32);
pub const CLCTL_VALIDATE_DIRECTORY: CLCTL_CODES = CLCTL_CODES(569i32);
pub const CLCTL_VALIDATE_NETNAME: CLCTL_CODES = CLCTL_CODES(565i32);
pub const CLCTL_VALIDATE_PATH: CLCTL_CODES = CLCTL_CODES(561i32);
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: CLCTL_CODES = CLCTL_CODES(137i32);
pub const CLOUD_WITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("msft-cloud-witness");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CLRES_CALLBACK_FUNCTION_TABLE {
    pub LogEvent: PLOG_EVENT_ROUTINE,
    pub SetResourceStatusEx: PSET_RESOURCE_STATUS_ROUTINE_EX,
    pub SetResourceLockedMode: PSET_RESOURCE_LOCKED_MODE_ROUTINE,
    pub SignalFailure: PSIGNAL_FAILURE_ROUTINE,
    pub SetResourceInMemoryNodeLocalProperties: PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE,
    pub EndControlCall: PEND_CONTROL_CALL,
    pub EndTypeControlCall: PEND_TYPE_CONTROL_CALL,
    pub ExtendControlCall: PEXTEND_RES_CONTROL_CALL,
    pub ExtendTypeControlCall: PEXTEND_RES_TYPE_CONTROL_CALL,
    pub RaiseResTypeNotification: PRAISE_RES_TYPE_NOTIFICATION,
    pub ChangeResourceProcessForDumps: PCHANGE_RESOURCE_PROCESS_FOR_DUMPS,
    pub ChangeResTypeProcessForDumps: PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS,
    pub SetInternalState: PSET_INTERNAL_STATE,
    pub SetResourceLockedModeEx: PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE,
    pub RequestDump: PREQUEST_DUMP_ROUTINE,
    pub SetResourceWprPolicy: PSET_RESOURCE_WPR_POLICY_ROUTINE,
    pub ArmWprWatchdogForCurrentResourceCall: PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub struct CLRES_FUNCTION_TABLE {
    pub TableSize: u32,
    pub Version: u32,
    pub Anonymous: CLRES_FUNCTION_TABLE_0,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy)]
pub union CLRES_FUNCTION_TABLE_0 {
    pub V1Functions: CLRES_V1_FUNCTIONS,
    pub V2Functions: CLRES_V2_FUNCTIONS,
    pub V3Functions: CLRES_V3_FUNCTIONS,
    pub V4Functions: CLRES_V4_FUNCTIONS,
}
#[cfg(feature = "Win32_System_Registry")]
impl Default for CLRES_FUNCTION_TABLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CLRES_V1_FUNCTIONS {
    pub Open: POPEN_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_ROUTINE,
    pub Offline: POFFLINE_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CLRES_V2_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CLRES_V3_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CLRES_V4_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
    pub BeginResourceControlAsUser: PBEGIN_RESCALL_AS_USER_ROUTINE,
    pub BeginResourceTypeControlAsUser: PBEGIN_RESTYPECALL_AS_USER_ROUTINE,
}
pub const CLRES_VERSION_V1_00: u32 = 256u32;
pub const CLRES_VERSION_V2_00: u32 = 512u32;
pub const CLRES_VERSION_V3_00: u32 = 768u32;
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUADMEX_OBJECT_TYPE(pub i32);
pub const CLUADMEX_OT_CLUSTER: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(1i32);
pub const CLUADMEX_OT_GROUP: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(3i32);
pub const CLUADMEX_OT_NETINTERFACE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(7i32);
pub const CLUADMEX_OT_NETWORK: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(6i32);
pub const CLUADMEX_OT_NODE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(2i32);
pub const CLUADMEX_OT_NONE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(0i32);
pub const CLUADMEX_OT_RESOURCE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(4i32);
pub const CLUADMEX_OT_RESOURCETYPE: CLUADMEX_OBJECT_TYPE = CLUADMEX_OBJECT_TYPE(5i32);
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_PINNED_VMS_ONLY: u32 = 4u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_STORAGE: u32 = 1u32;
pub const CLUSAPI_NODE_RESUME_FAILBACK_VMS: u32 = 2u32;
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CLUSAPI_REASON_HANDLER {
    pub lpParameter: *mut core::ffi::c_void,
    pub pfnHandler: PCLUSAPI_PFN_REASON_HANDLER,
}
impl Default for CLUSAPI_REASON_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
pub const CLUSAPI_VERSION: u32 = 2572u32;
pub const CLUSAPI_VERSION_NI: u32 = 2572u32;
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_AFFINITYRULE_CODES(pub i32);
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995033i32);
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(151006577i32);
pub const CLUSCTL_AFFINITYRULE_GET_ID: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995001i32);
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(150995029i32);
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = CLUSCTL_AFFINITYRULE_CODES(155189342i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562849i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562865i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979958i32);
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979942i32);
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441086i32);
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441089i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440585i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN_WITNESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440625i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440581i32);
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT_WITNESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440621i32);
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635590i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_CLUSTER_CODES(pub i32);
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452253i32);
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440593i32);
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440633i32);
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646566i32);
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440877i32);
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441193i32);
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440601i32);
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440613i32);
pub const CLUSCTL_CLUSTER_GET_FQDN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440573i32);
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441209i32);
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452257i32);
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440641i32);
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440653i32);
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440597i32);
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440637i32);
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441169i32);
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441253i32);
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117441257i32);
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452242i32);
pub const CLUSCTL_CLUSTER_REMOVE_NODE: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635566i32);
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635058i32);
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646446i32);
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121646434i32);
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634910i32);
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635594i32);
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121634950i32);
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(121635554i32);
pub const CLUSCTL_CLUSTER_SHUTDOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440589i32);
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452246i32);
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117452250i32);
pub const CLUSCTL_CLUSTER_UNKNOWN: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440512i32);
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440609i32);
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = CLUSCTL_CLUSTER_CODES(117440649i32);
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_GROUPSET_CODES(pub i32);
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217817i32);
pub const CLUSCTL_GROUPSET_GET_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229361i32);
pub const CLUSCTL_GROUPSET_GET_ID: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217785i32);
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229365i32);
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229369i32);
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134217813i32);
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(138412126i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_GROUP_CODES(pub i32);
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331729i32);
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331769i32);
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331653i32);
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331737i32);
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331749i32);
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331673i32);
pub const CLUSCTL_GROUP_GET_FLAGS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331657i32);
pub const CLUSCTL_GROUP_GET_ID: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331705i32);
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332377i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::super::Foundation::SYSTEMTIME,
    pub NodeId: u32,
}
pub const CLUSCTL_GROUP_GET_NAME: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331689i32);
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331777i32);
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331789i32);
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229373i32);
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = CLUSCTL_GROUPSET_CODES(134229377i32);
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331733i32);
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331773i32);
pub const CLUSCTL_GROUP_QUERY_DELETE: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50332089i32);
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54537606i32);
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526046i32);
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(54526086i32);
pub const CLUSCTL_GROUP_UNKNOWN: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331648i32);
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331745i32);
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = CLUSCTL_GROUP_CODES(50331785i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_NETINTERFACE_CODES(pub i32);
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663377i32);
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663417i32);
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663301i32);
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663385i32);
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663397i32);
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663305i32);
pub const CLUSCTL_NETINTERFACE_GET_ID: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663353i32);
pub const CLUSCTL_NETINTERFACE_GET_NAME: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663337i32);
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663349i32);
pub const CLUSCTL_NETINTERFACE_GET_NODE: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663345i32);
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663425i32);
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663437i32);
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663381i32);
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663421i32);
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857694i32);
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(104857734i32);
pub const CLUSCTL_NETINTERFACE_UNKNOWN: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663296i32);
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663393i32);
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = CLUSCTL_NETINTERFACE_CODES(100663433i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_NETWORK_CODES(pub i32);
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886161i32);
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886201i32);
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886085i32);
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886169i32);
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886181i32);
pub const CLUSCTL_NETWORK_GET_FLAGS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886089i32);
pub const CLUSCTL_NETWORK_GET_ID: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886137i32);
pub const CLUSCTL_NETWORK_GET_NAME: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886121i32);
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886209i32);
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886221i32);
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886165i32);
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886205i32);
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080478i32);
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(88080518i32);
pub const CLUSCTL_NETWORK_UNKNOWN: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886080i32);
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886177i32);
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = CLUSCTL_NETWORK_CODES(83886217i32);
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109581i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_NODE_CODES(pub i32);
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108945i32);
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108985i32);
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108869i32);
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108929i32);
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108953i32);
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108965i32);
pub const CLUSCTL_NODE_GET_FLAGS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108873i32);
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109585i32);
pub const CLUSCTL_NODE_GET_ID: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108921i32);
pub const CLUSCTL_NODE_GET_NAME: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108905i32);
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108993i32);
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109005i32);
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108949i32);
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108989i32);
pub const CLUSCTL_NODE_GET_STUCK_NODES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109565i32);
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109569i32);
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109573i32);
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109577i32);
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303262i32);
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(71303302i32);
pub const CLUSCTL_NODE_UNKNOWN: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108864i32);
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67108961i32);
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = CLUSCTL_NODE_CODES(67109001i32);
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971694i32);
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972246i32);
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020114i32);
pub const CLUSCTL_RESOURCE_ADD_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020122i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971682i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971714i32);
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971710i32);
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834285i32);
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020138i32);
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020142i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_RESOURCE_CODES(pub i32);
pub const CLUSCTL_RESOURCE_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020102i32);
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971698i32);
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971686i32);
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972174i32);
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972170i32);
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777297i32);
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777337i32);
pub const CLUSCTL_RESOURCE_EVICT_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020110i32);
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020166i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17825885i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020198i32);
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020194i32);
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777221i32);
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777229i32);
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777305i32);
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777317i32);
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777397i32);
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777589i32);
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777241i32);
pub const CLUSCTL_RESOURCE_GET_FLAGS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777225i32);
pub const CLUSCTL_RESOURCE_GET_ID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777273i32);
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788873i32);
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777417i32);
pub const CLUSCTL_RESOURCE_GET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777257i32);
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777577i32);
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788961i32);
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834217i32);
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777345i32);
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777357i32);
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777385i32);
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777233i32);
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777261i32);
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777301i32);
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777341i32);
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788829i32);
pub const CLUSCTL_RESOURCE_INITIALIZE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020170i32);
pub const CLUSCTL_RESOURCE_INSTALL_NODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020106i32);
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971970i32);
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971966i32);
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777905i32);
pub const CLUSCTL_RESOURCE_JOINING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020186i32);
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020182i32);
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020202i32);
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777598i32);
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777586i32);
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777613i32);
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777605i32);
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777601i32);
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834289i32);
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22028578i32);
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020222i32);
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777909i32);
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979946i32);
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020178i32);
pub const CLUSCTL_RESOURCE_QUERY_DELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777657i32);
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777697i32);
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020118i32);
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020126i32);
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777581i32);
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777594i32);
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16778010i32);
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972206i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983190i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983194i32);
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983197i32);
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971614i32);
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972182i32);
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20983182i32);
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972006i32);
pub const CLUSCTL_RESOURCE_SET_NAME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020134i32);
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971654i32);
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972186i32);
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020174i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777753i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777733i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777617i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777713i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777721i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777633i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777745i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777765i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777885i32);
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972194i32);
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777625i32);
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777893i32);
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788950i32);
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16788954i32);
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20972010i32);
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611501i32);
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797358i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSCTL_RESOURCE_TYPE_CODES(pub i32);
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554513i32);
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554553i32);
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797326i32);
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797362i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33555001i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554453i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554437i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554445i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554521i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554533i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554537i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554613i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554441i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554561i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554573i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554577i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554601i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554449i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554517i32);
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554557i32);
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797374i32);
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797322i32);
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749358i32);
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554997i32);
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34611505i32);
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(34603137i32);
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757162i32);
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554873i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562946i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562953i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562961i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562957i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562949i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562973i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562965i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562981i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562969i32);
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562977i32);
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797378i32);
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748830i32);
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37748870i32);
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797366i32);
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(38797370i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554837i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554933i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: windows_core::GUID,
}
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33562593i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554949i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554925i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554989i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554953i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(16777769i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554945i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749262i32);
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37749150i32);
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554432i32);
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(37757166i32);
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554529i32);
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554569i32);
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = CLUSCTL_RESOURCE_TYPE_CODES(33554993i32);
pub const CLUSCTL_RESOURCE_UNDELETE: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(22020230i32);
pub const CLUSCTL_RESOURCE_UNKNOWN: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777216i32);
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20979950i32);
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(20971706i32);
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(17834277i32);
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777313i32);
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = CLUSCTL_RESOURCE_CODES(16777353i32);
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSGROUP_TYPE(pub i32);
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_BINARY {
    pub Base: CLUSPROP_VALUE,
    pub rgb: [u8; 1],
}
impl Default for CLUSPROP_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: windows_core::PWSTR,
    pub pList: *mut CLUSPROP_LIST,
    pub pSyntax: *mut CLUSPROP_SYNTAX,
    pub pName: *mut CLUSPROP_SZ,
    pub pValue: *mut CLUSPROP_VALUE,
    pub pBinaryValue: *mut CLUSPROP_BINARY,
    pub pWordValue: *mut CLUSPROP_WORD,
    pub pDwordValue: *mut CLUSPROP_DWORD,
    pub pLongValue: *mut CLUSPROP_LONG,
    pub pULargeIntegerValue: *mut CLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: *mut CLUSPROP_LARGE_INTEGER,
    pub pStringValue: *mut CLUSPROP_SZ,
    pub pMultiSzValue: *mut CLUSPROP_SZ,
    pub pSecurityDescriptor: *mut CLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: *mut CLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: *mut CLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: *mut CLUSPROP_DWORD,
    pub pScsiAddressValue: *mut CLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: *mut CLUSPROP_DWORD,
    pub pPartitionInfoValue: *mut CLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: *mut CLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: *mut CLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: *mut CLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: *mut CLUSPROP_FILETIME,
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_BUFFER_HELPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_DWORD {
    pub Base: CLUSPROP_VALUE,
    pub dw: u32,
}
impl Default for CLUSPROP_DWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FILETIME {
    pub Base: CLUSPROP_VALUE,
    pub ft: super::super::Foundation::FILETIME,
}
impl Default for CLUSPROP_FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSPROP_FORMAT_BINARY: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(1i32);
pub const CLUSPROP_FORMAT_DWORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(2i32);
pub const CLUSPROP_FORMAT_EXPANDED_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(8i32);
pub const CLUSPROP_FORMAT_EXPAND_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(4i32);
pub const CLUSPROP_FORMAT_FILETIME: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(12i32);
pub const CLUSPROP_FORMAT_LARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(10i32);
pub const CLUSPROP_FORMAT_LONG: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(7i32);
pub const CLUSPROP_FORMAT_MULTI_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(5i32);
pub const CLUSPROP_FORMAT_PROPERTY_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(14i32);
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(9i32);
pub const CLUSPROP_FORMAT_SZ: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(3i32);
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(6i32);
pub const CLUSPROP_FORMAT_UNKNOWN: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(0i32);
pub const CLUSPROP_FORMAT_USER: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(32768i32);
pub const CLUSPROP_FORMAT_VALUE_LIST: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(13i32);
pub const CLUSPROP_FORMAT_WORD: CLUSTER_PROPERTY_FORMAT = CLUSTER_PROPERTY_FORMAT(11i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FTSET_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_FTSET_INFO,
}
impl Default for CLUSPROP_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSPROP_IPADDR_ENABLENETBIOS(pub i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(0i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(1i32);
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: CLUSPROP_IPADDR_ENABLENETBIOS = CLUSPROP_IPADDR_ENABLENETBIOS(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: i64,
}
impl Default for CLUSPROP_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_SZ,
}
impl Default for CLUSPROP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LONG {
    pub Base: CLUSPROP_VALUE,
    pub l: i32,
}
impl Default for CLUSPROP_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO,
}
impl Default for CLUSPROP_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO_EX,
}
impl Default for CLUSPROP_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub Base: CLUSPROP_PARTITION_INFO_EX,
    pub Base2: CLUS_PARTITION_INFO_EX2,
}
impl Default for CLUSPROP_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSPROP_PIFLAGS(pub i32);
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(8i32);
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(32i32);
pub const CLUSPROP_PIFLAG_RAW: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(64i32);
pub const CLUSPROP_PIFLAG_REMOVABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(2i32);
pub const CLUSPROP_PIFLAG_STICKY: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(1i32);
pub const CLUSPROP_PIFLAG_UNKNOWN: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(-2147483648i32);
pub const CLUSPROP_PIFLAG_USABLE: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(4i32);
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: CLUSPROP_PIFLAGS = CLUSPROP_PIFLAGS(16i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl Default for CLUSPROP_REQUIRED_DEPENDENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub Base: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl Default for CLUSPROP_RESOURCE_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_RESOURCE_CLASS_INFO,
}
impl Default for CLUSPROP_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_SCSI_ADDRESS,
}
impl Default for CLUSPROP_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub Base: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::super::Security::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 1],
}
#[cfg(feature = "Win32_Security")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl Default for CLUSPROP_SYNTAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
pub const CLUSPROP_SYNTAX_DISK_GUID: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(720899u32);
pub const CLUSPROP_SYNTAX_DISK_NUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(458754u32);
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(655363u32);
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(327682u32);
pub const CLUSPROP_SYNTAX_DISK_SIZE: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(786438u32);
pub const CLUSPROP_SYNTAX_ENDMARK: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(0u32);
pub const CLUSPROP_SYNTAX_FTSET_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(589825u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65537u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65538u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65544u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65540u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65548u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65546u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65543u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65541u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65550u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65545u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65539u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65542u32);
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(65547u32);
pub const CLUSPROP_SYNTAX_NAME: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(262147u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(524289u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(851969u32);
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(917505u32);
pub const CLUSPROP_SYNTAX_RESCLASS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(131074u32);
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(393218u32);
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = CLUSTER_PROPERTY_SYNTAX(983041u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SZ {
    pub Base: CLUSPROP_VALUE,
    pub sz: [u16; 1],
}
impl Default for CLUSPROP_SZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSPROP_TYPE_DISK_GUID: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(11i32);
pub const CLUSPROP_TYPE_DISK_NUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(7i32);
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(10i32);
pub const CLUSPROP_TYPE_DISK_SIZE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(12i32);
pub const CLUSPROP_TYPE_ENDMARK: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(0i32);
pub const CLUSPROP_TYPE_FTSET_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(9i32);
pub const CLUSPROP_TYPE_LIST_VALUE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(1i32);
pub const CLUSPROP_TYPE_NAME: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(4i32);
pub const CLUSPROP_TYPE_PARTITION_INFO: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(8i32);
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(13i32);
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(14i32);
pub const CLUSPROP_TYPE_RESCLASS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(2i32);
pub const CLUSPROP_TYPE_RESERVED1: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(3i32);
pub const CLUSPROP_TYPE_SCSI_ADDRESS: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(6i32);
pub const CLUSPROP_TYPE_SIGNATURE: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(5i32);
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(15i32);
pub const CLUSPROP_TYPE_UNKNOWN: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(-1i32);
pub const CLUSPROP_TYPE_USER: CLUSTER_PROPERTY_TYPE = CLUSTER_PROPERTY_TYPE(32768i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: u64,
}
impl Default for CLUSPROP_ULARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl Default for CLUSPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_WORD {
    pub Base: CLUSPROP_VALUE,
    pub w: u16,
}
impl Default for CLUSPROP_WORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSREG_COMMAND_NONE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(0i32);
pub const CLUSREG_CONDITION_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(11i32);
pub const CLUSREG_CONDITION_IS_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(13i32);
pub const CLUSREG_CONDITION_IS_GREATER_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(15i32);
pub const CLUSREG_CONDITION_IS_LESS_THAN: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(16i32);
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(14i32);
pub const CLUSREG_CONDITION_KEY_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(17i32);
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(18i32);
pub const CLUSREG_CONDITION_NOT_EXISTS: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(12i32);
pub const CLUSREG_CONTROL_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(10i32);
pub const CLUSREG_CREATE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(2i32);
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
pub const CLUSREG_DELETE_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(3i32);
pub const CLUSREG_DELETE_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(4i32);
pub const CLUSREG_KEYNAME_OBJECTGUIDS: windows_core::PCWSTR = windows_core::w!("ObjectGUIDs");
pub const CLUSREG_LAST_COMMAND: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(19i32);
pub const CLUSREG_NAME_AFFINITYRULE_ENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const CLUSREG_NAME_AFFINITYRULE_GROUPS: windows_core::PCWSTR = windows_core::w!("Groups");
pub const CLUSREG_NAME_AFFINITYRULE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_AFFINITYRULE_TYPE: windows_core::PCWSTR = windows_core::w!("RuleType");
pub const CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME: windows_core::PCWSTR = windows_core::w!("AccountName");
pub const CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("ContainerName");
pub const CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO: windows_core::PCWSTR = windows_core::w!("EndpointInfo");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY: windows_core::PCWSTR = windows_core::w!("PrimaryKey");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN: windows_core::PCWSTR = windows_core::w!("PrimaryToken");
pub const CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE: windows_core::PCWSTR = windows_core::w!("DefaultNetworkRole");
pub const CLUSREG_NAME_CLUS_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_CLUS_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_CROSS_SITE_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSiteDelay");
pub const CLUSREG_NAME_CROSS_SITE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSiteThreshold");
pub const CLUSREG_NAME_CROSS_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSubnetDelay");
pub const CLUSREG_NAME_CROSS_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSubnetThreshold");
pub const CLUSREG_NAME_CSV_BLOCK_CACHE: windows_core::PCWSTR = windows_core::w!("BlockCacheSize");
pub const CLUSREG_NAME_CSV_MDS_SD: windows_core::PCWSTR = windows_core::w!("SharedVolumeSecurityDescriptor");
pub const CLUSREG_NAME_DATABASE_READ_WRITE_MODE: windows_core::PCWSTR = windows_core::w!("DatabaseReadWriteMode");
pub const CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("DdaDeviceAllocations");
pub const CLUSREG_NAME_DHCP_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_DHCP_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_DRAIN_ON_SHUTDOWN: windows_core::PCWSTR = windows_core::w!("DrainOnShutdown");
pub const CLUSREG_NAME_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE: windows_core::PCWSTR = windows_core::w!("FailoverMoveMigrationType");
pub const CLUSREG_NAME_FILESHR_CA_TIMEOUT: windows_core::PCWSTR = windows_core::w!("CATimeout");
pub const CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES: windows_core::PCWSTR = windows_core::w!("HideSubDirShares");
pub const CLUSREG_NAME_FILESHR_IS_DFS_ROOT: windows_core::PCWSTR = windows_core::w!("IsDfsRoot");
pub const CLUSREG_NAME_FILESHR_MAX_USERS: windows_core::PCWSTR = windows_core::w!("MaxUsers");
pub const CLUSREG_NAME_FILESHR_PATH: windows_core::PCWSTR = windows_core::w!("Path");
pub const CLUSREG_NAME_FILESHR_QOS_FLOWSCOPE: windows_core::PCWSTR = windows_core::w!("QosFlowScope");
pub const CLUSREG_NAME_FILESHR_QOS_POLICYID: windows_core::PCWSTR = windows_core::w!("QosPolicyId");
pub const CLUSREG_NAME_FILESHR_REMARK: windows_core::PCWSTR = windows_core::w!("Remark");
pub const CLUSREG_NAME_FILESHR_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_FILESHR_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("ServerName");
pub const CLUSREG_NAME_FILESHR_SHARE_FLAGS: windows_core::PCWSTR = windows_core::w!("ShareFlags");
pub const CLUSREG_NAME_FILESHR_SHARE_NAME: windows_core::PCWSTR = windows_core::w!("ShareName");
pub const CLUSREG_NAME_FILESHR_SHARE_SUBDIRS: windows_core::PCWSTR = windows_core::w!("ShareSubDirs");
pub const CLUSREG_NAME_FIXQUORUM: windows_core::PCWSTR = windows_core::w!("FixQuorum");
pub const CLUSREG_NAME_FSWITNESS_ARB_DELAY: windows_core::PCWSTR = windows_core::w!("ArbitrationDelay");
pub const CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO: windows_core::PCWSTR = windows_core::w!("ImpersonateCNO");
pub const CLUSREG_NAME_FSWITNESS_SHARE_PATH: windows_core::PCWSTR = windows_core::w!("SharePath");
pub const CLUSREG_NAME_FUNCTIONAL_LEVEL: windows_core::PCWSTR = windows_core::w!("ClusterFunctionalLevel");
pub const CLUSREG_NAME_GENAPP_COMMAND_LINE: windows_core::PCWSTR = windows_core::w!("CommandLine");
pub const CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_GENAPP_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH: windows_core::PCWSTR = windows_core::w!("ScriptFilepath");
pub const CLUSREG_NAME_GENSVC_SERVICE_NAME: windows_core::PCWSTR = windows_core::w!("ServiceName");
pub const CLUSREG_NAME_GENSVC_STARTUP_PARAMS: windows_core::PCWSTR = windows_core::w!("StartupParameters");
pub const CLUSREG_NAME_GENSVC_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("GpupDeviceAllocations");
pub const CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING: windows_core::PCWSTR = windows_core::w!("NodeDomainInfo");
pub const CLUSREG_NAME_GROUPSET_FAULT_DOMAINS: windows_core::PCWSTR = windows_core::w!("FaultDomains");
pub const CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET: windows_core::PCWSTR = windows_core::w!("IsAvailabilitySet");
pub const CLUSREG_NAME_GROUPSET_IS_GLOBAL: windows_core::PCWSTR = windows_core::w!("IsGlobal");
pub const CLUSREG_NAME_GROUPSET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GROUPSET_RESERVE_NODE: windows_core::PCWSTR = windows_core::w!("ReserveSpareNode");
pub const CLUSREG_NAME_GROUPSET_STARTUP_COUNT: windows_core::PCWSTR = windows_core::w!("StartupCount");
pub const CLUSREG_NAME_GROUPSET_STARTUP_DELAY: windows_core::PCWSTR = windows_core::w!("StartupDelay");
pub const CLUSREG_NAME_GROUPSET_STARTUP_SETTING: windows_core::PCWSTR = windows_core::w!("StartupSetting");
pub const CLUSREG_NAME_GROUPSET_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS: windows_core::PCWSTR = windows_core::w!("UpdateDomains");
pub const CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("GroupDependencyTimeout");
pub const CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME: windows_core::PCWSTR = windows_core::w!("AntiAffinityClassNames");
pub const CLUSREG_NAME_GRP_CCF_EPOCH: windows_core::PCWSTR = windows_core::w!("CCFEpoch");
pub const CLUSREG_NAME_GRP_CCF_EPOCH_HIGH: windows_core::PCWSTR = windows_core::w!("CCFEpochHigh");
pub const CLUSREG_NAME_GRP_COLD_START_SETTING: windows_core::PCWSTR = windows_core::w!("ColdStartSetting");
pub const CLUSREG_NAME_GRP_DEFAULT_OWNER: windows_core::PCWSTR = windows_core::w!("DefaultOwner");
pub const CLUSREG_NAME_GRP_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_GRP_FAILBACK_TYPE: windows_core::PCWSTR = windows_core::w!("AutoFailbackType");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_END: windows_core::PCWSTR = windows_core::w!("FailbackWindowEnd");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_START: windows_core::PCWSTR = windows_core::w!("FailbackWindowStart");
pub const CLUSREG_NAME_GRP_FAILOVER_PERIOD: windows_core::PCWSTR = windows_core::w!("FailoverPeriod");
pub const CLUSREG_NAME_GRP_FAILOVER_THRESHOLD: windows_core::PCWSTR = windows_core::w!("FailoverThreshold");
pub const CLUSREG_NAME_GRP_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_GRP_LOCK_MOVE: windows_core::PCWSTR = windows_core::w!("LockedFromMoving");
pub const CLUSREG_NAME_GRP_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GRP_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_GRP_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_GRP_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
pub const CLUSREG_NAME_GRP_PRIORITY: windows_core::PCWSTR = windows_core::w!("Priority");
pub const CLUSREG_NAME_GRP_RESILIENCY_PERIOD: windows_core::PCWSTR = windows_core::w!("ResiliencyPeriod");
pub const CLUSREG_NAME_GRP_START_DELAY: windows_core::PCWSTR = windows_core::w!("GroupStartDelay");
pub const CLUSREG_NAME_GRP_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GRP_TYPE: windows_core::PCWSTR = windows_core::w!("GroupType");
pub const CLUSREG_NAME_GRP_UPDATE_DOMAIN: windows_core::PCWSTR = windows_core::w!("UpdateDomain");
pub const CLUSREG_NAME_IGNORE_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("IgnorePersistentStateOnStartup");
pub const CLUSREG_NAME_IPADDR_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPADDR_DHCP_ADDRESS: windows_core::PCWSTR = windows_core::w!("DhcpAddress");
pub const CLUSREG_NAME_IPADDR_DHCP_SERVER: windows_core::PCWSTR = windows_core::w!("DhcpServer");
pub const CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("DhcpSubnetMask");
pub const CLUSREG_NAME_IPADDR_ENABLE_DHCP: windows_core::PCWSTR = windows_core::w!("EnableDhcp");
pub const CLUSREG_NAME_IPADDR_ENABLE_NETBIOS: windows_core::PCWSTR = windows_core::w!("EnableNetBIOS");
pub const CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME: windows_core::PCWSTR = windows_core::w!("LeaseObtainedTime");
pub const CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME: windows_core::PCWSTR = windows_core::w!("LeaseExpiresTime");
pub const CLUSREG_NAME_IPADDR_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH: windows_core::PCWSTR = windows_core::w!("OverrideAddressMatch");
pub const CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("ProbeFailureThreshold");
pub const CLUSREG_NAME_IPADDR_PROBE_PORT: windows_core::PCWSTR = windows_core::w!("ProbePort");
pub const CLUSREG_NAME_IPADDR_SHARED_NETNAME: windows_core::PCWSTR = windows_core::w!("SharedNetname");
pub const CLUSREG_NAME_IPADDR_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("SubnetMask");
pub const CLUSREG_NAME_IPADDR_T1: windows_core::PCWSTR = windows_core::w!("T1");
pub const CLUSREG_NAME_IPADDR_T2: windows_core::PCWSTR = windows_core::w!("T2");
pub const CLUSREG_NAME_IPV6_NATIVE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_NATIVE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_IPV6_TUNNEL_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE: windows_core::PCWSTR = windows_core::w!("TunnelType");
pub const CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME: windows_core::PCWSTR = windows_core::w!("RecentEventsResetTime");
pub const CLUSREG_NAME_LOG_FILE_PATH: windows_core::PCWSTR = windows_core::w!("LogFilePath");
pub const CLUSREG_NAME_MESSAGE_BUFFER_LENGTH: windows_core::PCWSTR = windows_core::w!("MessageBufferLength");
pub const CLUSREG_NAME_MIXED_MODE: windows_core::PCWSTR = windows_core::w!("MixedMode");
pub const CLUSREG_NAME_NETFT_IPSEC_ENABLED: windows_core::PCWSTR = windows_core::w!("NetftIPSecEnabled");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_ID: windows_core::PCWSTR = windows_core::w!("AdapterId");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("Adapter");
pub const CLUSREG_NAME_NETIFACE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NETIFACE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NETIFACE_DHCP_ENABLED: windows_core::PCWSTR = windows_core::w!("DhcpEnabled");
pub const CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NETIFACE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETIFACE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_NETIFACE_NODE: windows_core::PCWSTR = windows_core::w!("Node");
pub const CLUSREG_NAME_NETNAME_AD_AWARE: windows_core::PCWSTR = windows_core::w!("ADAware");
pub const CLUSREG_NAME_NETNAME_ALIASES: windows_core::PCWSTR = windows_core::w!("Aliases");
pub const CLUSREG_NAME_NETNAME_CONTAINERGUID: windows_core::PCWSTR = windows_core::w!("CryptoContainerGUID");
pub const CLUSREG_NAME_NETNAME_CREATING_DC: windows_core::PCWSTR = windows_core::w!("CreatingDC");
pub const CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES: windows_core::PCWSTR = windows_core::w!("DisableClones");
pub const CLUSREG_NAME_NETNAME_DNS_NAME: windows_core::PCWSTR = windows_core::w!("DnsName");
pub const CLUSREG_NAME_NETNAME_DNS_SUFFIX: windows_core::PCWSTR = windows_core::w!("DnsSuffix");
pub const CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS: windows_core::PCWSTR = windows_core::w!("ExcludeNetworks");
pub const CLUSREG_NAME_NETNAME_HOST_TTL: windows_core::PCWSTR = windows_core::w!("HostRecordTTL");
pub const CLUSREG_NAME_NETNAME_IN_USE_NETWORKS: windows_core::PCWSTR = windows_core::w!("InUseNetworks");
pub const CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE: windows_core::PCWSTR = windows_core::w!("LastDNSUpdateTime");
pub const CLUSREG_NAME_NETNAME_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETNAME_OBJECT_ID: windows_core::PCWSTR = windows_core::w!("ObjectGUID");
pub const CLUSREG_NAME_NETNAME_PUBLISH_PTR: windows_core::PCWSTR = windows_core::w!("PublishPTRRecords");
pub const CLUSREG_NAME_NETNAME_REGISTER_ALL_IP: windows_core::PCWSTR = windows_core::w!("RegisterAllProvidersIP");
pub const CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES: windows_core::PCWSTR = windows_core::w!("RemapPipeNames");
pub const CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE: windows_core::PCWSTR = windows_core::w!("DeleteVcoOnResCleanup");
pub const CLUSREG_NAME_NETNAME_RESOURCE_DATA: windows_core::PCWSTR = windows_core::w!("ResourceData");
pub const CLUSREG_NAME_NETNAME_STATUS_DNS: windows_core::PCWSTR = windows_core::w!("StatusDNS");
pub const CLUSREG_NAME_NETNAME_STATUS_KERBEROS: windows_core::PCWSTR = windows_core::w!("StatusKerberos");
pub const CLUSREG_NAME_NETNAME_STATUS_NETBIOS: windows_core::PCWSTR = windows_core::w!("StatusNetBIOS");
pub const CLUSREG_NAME_NETNAME_VCO_CONTAINER: windows_core::PCWSTR = windows_core::w!("VcoContainer");
pub const CLUSREG_NAME_NET_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NET_ADDRESS_MASK: windows_core::PCWSTR = windows_core::w!("AddressMask");
pub const CLUSREG_NAME_NET_AUTOMETRIC: windows_core::PCWSTR = windows_core::w!("AutoMetric");
pub const CLUSREG_NAME_NET_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NET_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv4PrefixLengths");
pub const CLUSREG_NAME_NET_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv6PrefixLengths");
pub const CLUSREG_NAME_NET_METRIC: windows_core::PCWSTR = windows_core::w!("Metric");
pub const CLUSREG_NAME_NET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NET_RDMA_CAPABLE: windows_core::PCWSTR = windows_core::w!("RdmaCapable");
pub const CLUSREG_NAME_NET_ROLE: windows_core::PCWSTR = windows_core::w!("Role");
pub const CLUSREG_NAME_NET_RSS_CAPABLE: windows_core::PCWSTR = windows_core::w!("RssCapable");
pub const CLUSREG_NAME_NET_SPEED: windows_core::PCWSTR = windows_core::w!("LinkSpeed");
pub const CLUSREG_NAME_NODE_BUILD_NUMBER: windows_core::PCWSTR = windows_core::w!("BuildNumber");
pub const CLUSREG_NAME_NODE_CSDVERSION: windows_core::PCWSTR = windows_core::w!("CSDVersion");
pub const CLUSREG_NAME_NODE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NODE_DRAIN_STATUS: windows_core::PCWSTR = windows_core::w!("NodeDrainStatus");
pub const CLUSREG_NAME_NODE_DRAIN_TARGET: windows_core::PCWSTR = windows_core::w!("NodeDrainTarget");
pub const CLUSREG_NAME_NODE_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("DynamicWeight");
pub const CLUSREG_NAME_NODE_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_NODE_FDID: windows_core::PCWSTR = windows_core::w!("FaultDomainId");
pub const CLUSREG_NAME_NODE_HIGHEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeHighestVersion");
pub const CLUSREG_NAME_NODE_IS_PRIMARY: windows_core::PCWSTR = windows_core::w!("IsPrimary");
pub const CLUSREG_NAME_NODE_LOWEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeLowestVersion");
pub const CLUSREG_NAME_NODE_MAJOR_VERSION: windows_core::PCWSTR = windows_core::w!("MajorVersion");
pub const CLUSREG_NAME_NODE_MANUFACTURER: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const CLUSREG_NAME_NODE_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("MinorVersion");
pub const CLUSREG_NAME_NODE_MODEL: windows_core::PCWSTR = windows_core::w!("Model");
pub const CLUSREG_NAME_NODE_NAME: windows_core::PCWSTR = windows_core::w!("NodeName");
pub const CLUSREG_NAME_NODE_NEEDS_PQ: windows_core::PCWSTR = windows_core::w!("NeedsPreventQuorum");
pub const CLUSREG_NAME_NODE_SERIALNUMBER: windows_core::PCWSTR = windows_core::w!("SerialNumber");
pub const CLUSREG_NAME_NODE_STATUS_INFO: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_NODE_UNIQUEID: windows_core::PCWSTR = windows_core::w!("UniqueID");
pub const CLUSREG_NAME_NODE_WEIGHT: windows_core::PCWSTR = windows_core::w!("NodeWeight");
pub const CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE: windows_core::PCWSTR = windows_core::w!("EnableBlockCache");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT: windows_core::PCWSTR = windows_core::w!("SnapshotAgeLimit");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE: windows_core::PCWSTR = windows_core::w!("SnapshotDiffSize");
pub const CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH: windows_core::PCWSTR = windows_core::w!("CsvEnforceWriteThrough");
pub const CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL: windows_core::PCWSTR = windows_core::w!("DiskArbInterval");
pub const CLUSREG_NAME_PHYSDISK_DISKARBTYPE: windows_core::PCWSTR = windows_core::w!("DiskArbType");
pub const CLUSREG_NAME_PHYSDISK_DISKGUID: windows_core::PCWSTR = windows_core::w!("DiskGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDGUID: windows_core::PCWSTR = windows_core::w!("DiskIdGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDTYPE: windows_core::PCWSTR = windows_core::w!("DiskIdType");
pub const CLUSREG_NAME_PHYSDISK_DISKIODELAY: windows_core::PCWSTR = windows_core::w!("MaxIoLatency");
pub const CLUSREG_NAME_PHYSDISK_DISKPATH: windows_core::PCWSTR = windows_core::w!("DiskPath");
pub const CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION: windows_core::PCWSTR = windows_core::w!("DiskRecoveryAction");
pub const CLUSREG_NAME_PHYSDISK_DISKRELOAD: windows_core::PCWSTR = windows_core::w!("DiskReload");
pub const CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK: windows_core::PCWSTR = windows_core::w!("DiskRunChkDsk");
pub const CLUSREG_NAME_PHYSDISK_DISKSIGNATURE: windows_core::PCWSTR = windows_core::w!("DiskSignature");
pub const CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS: windows_core::PCWSTR = windows_core::w!("DiskUniqueIds");
pub const CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO: windows_core::PCWSTR = windows_core::w!("DiskVolumeInfo");
pub const CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE: windows_core::PCWSTR = windows_core::w!("FastOnlineArbitrate");
pub const CLUSREG_NAME_PHYSDISK_MAINTMODE: windows_core::PCWSTR = windows_core::w!("MaintenanceMode");
pub const CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP: windows_core::PCWSTR = windows_core::w!("MigrateDriveLetters");
pub const CLUSREG_NAME_PHYSDISK_SPACEIDGUID: windows_core::PCWSTR = windows_core::w!("VirtualDiskId");
pub const CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT: windows_core::PCWSTR = windows_core::w!("VolsnapActivateTimeout");
pub const CLUSREG_NAME_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES: windows_core::PCWSTR = windows_core::w!("PlumbAllCrossSubnetRoutes");
pub const CLUSREG_NAME_PREVENTQUORUM: windows_core::PCWSTR = windows_core::w!("PreventQuorum");
pub const CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR: windows_core::PCWSTR = windows_core::w!("DefaultSpoolDirectory");
pub const CLUSREG_NAME_PRTSPOOL_TIMEOUT: windows_core::PCWSTR = windows_core::w!("JobCompletionTimeout");
pub const CLUSREG_NAME_QUARANTINE_DURATION: windows_core::PCWSTR = windows_core::w!("QuarantineDuration");
pub const CLUSREG_NAME_QUARANTINE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("QuarantineThreshold");
pub const CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("QuorumArbitrationTimeMax");
pub const CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS: windows_core::PCWSTR = windows_core::w!("ResiliencyDefaultPeriod");
pub const CLUSREG_NAME_RESILIENCY_LEVEL: windows_core::PCWSTR = windows_core::w!("ResiliencyLevel");
pub const CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS: windows_core::PCWSTR = windows_core::w!("AdminExtensions");
pub const CLUSREG_NAME_RESTYPE_DEADLOCK_TIMEOUT: windows_core::PCWSTR = windows_core::w!("DeadlockTimeout");
pub const CLUSREG_NAME_RESTYPE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RESTYPE_DLL_NAME: windows_core::PCWSTR = windows_core::w!("DllName");
pub const CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY: windows_core::PCWSTR = windows_core::w!("DumpLogQuery");
pub const CLUSREG_NAME_RESTYPE_DUMP_POLICY: windows_core::PCWSTR = windows_core::w!("DumpPolicy");
pub const CLUSREG_NAME_RESTYPE_DUMP_SERVICES: windows_core::PCWSTR = windows_core::w!("DumpServices");
pub const CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_RESTYPE_IS_ALIVE: windows_core::PCWSTR = windows_core::w!("IsAlivePollInterval");
pub const CLUSREG_NAME_RESTYPE_LOOKS_ALIVE: windows_core::PCWSTR = windows_core::w!("LooksAlivePollInterval");
pub const CLUSREG_NAME_RESTYPE_MAX_MONITORS: windows_core::PCWSTR = windows_core::w!("MaximumMonitors");
pub const CLUSREG_NAME_RESTYPE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RESTYPE_PENDING_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PendingTimeout");
pub const CLUSREG_NAME_RESTYPE_WPR_PROFILES: windows_core::PCWSTR = windows_core::w!("WprProfiles");
pub const CLUSREG_NAME_RESTYPE_WPR_START_AFTER: windows_core::PCWSTR = windows_core::w!("WprStartAfter");
pub const CLUSREG_NAME_RES_DATA1: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData1");
pub const CLUSREG_NAME_RES_DATA2: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData2");
pub const CLUSREG_NAME_RES_DEADLOCK_TIMEOUT: windows_core::PCWSTR = windows_core::w!("DeadlockTimeout");
pub const CLUSREG_NAME_RES_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION: windows_core::PCWSTR = windows_core::w!("EmbeddedFailureAction");
pub const CLUSREG_NAME_RES_IS_ALIVE: windows_core::PCWSTR = windows_core::w!("IsAlivePollInterval");
pub const CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE: windows_core::PCWSTR = windows_core::w!("LastOperationStatusCode");
pub const CLUSREG_NAME_RES_LOOKS_ALIVE: windows_core::PCWSTR = windows_core::w!("LooksAlivePollInterval");
pub const CLUSREG_NAME_RES_MONITOR_PID: windows_core::PCWSTR = windows_core::w!("MonitorProcessId");
pub const CLUSREG_NAME_RES_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RES_PENDING_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PendingTimeout");
pub const CLUSREG_NAME_RES_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_RES_RESTART_ACTION: windows_core::PCWSTR = windows_core::w!("RestartAction");
pub const CLUSREG_NAME_RES_RESTART_DELAY: windows_core::PCWSTR = windows_core::w!("RestartDelay");
pub const CLUSREG_NAME_RES_RESTART_PERIOD: windows_core::PCWSTR = windows_core::w!("RestartPeriod");
pub const CLUSREG_NAME_RES_RESTART_THRESHOLD: windows_core::PCWSTR = windows_core::w!("RestartThreshold");
pub const CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE: windows_core::PCWSTR = windows_core::w!("RetryPeriodOnFailure");
pub const CLUSREG_NAME_RES_SEPARATE_MONITOR: windows_core::PCWSTR = windows_core::w!("SeparateMonitor");
pub const CLUSREG_NAME_RES_STATUS: windows_core::PCWSTR = windows_core::w!("ResourceSpecificStatus");
pub const CLUSREG_NAME_RES_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_RES_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const CLUSREG_NAME_ROUTE_HISTORY_LENGTH: windows_core::PCWSTR = windows_core::w!("RouteHistoryLength");
pub const CLUSREG_NAME_SAME_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("SameSubnetDelay");
pub const CLUSREG_NAME_SAME_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("SameSubnetThreshold");
pub const CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES: windows_core::PCWSTR = windows_core::w!("ShutdownTimeoutInMinutes");
pub const CLUSREG_NAME_SOFS_SMBASYMMETRYMODE: windows_core::PCWSTR = windows_core::w!("SmbAsymmetryMode");
pub const CLUSREG_NAME_START_MEMORY: windows_core::PCWSTR = windows_core::w!("StartMemory");
pub const CLUSREG_NAME_STORAGESPACE_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("VirtualDiskDescription");
pub const CLUSREG_NAME_STORAGESPACE_HEALTH: windows_core::PCWSTR = windows_core::w!("VirtualDiskHealth");
pub const CLUSREG_NAME_STORAGESPACE_NAME: windows_core::PCWSTR = windows_core::w!("VirtualDiskName");
pub const CLUSREG_NAME_STORAGESPACE_POOLARBITRATE: windows_core::PCWSTR = windows_core::w!("Arbitrate");
pub const CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY: windows_core::PCWSTR = windows_core::w!("ConsumedCapacity");
pub const CLUSREG_NAME_STORAGESPACE_POOLDESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS: windows_core::PCWSTR = windows_core::w!("DriveIds");
pub const CLUSREG_NAME_STORAGESPACE_POOLHEALTH: windows_core::PCWSTR = windows_core::w!("Health");
pub const CLUSREG_NAME_STORAGESPACE_POOLIDGUID: windows_core::PCWSTR = windows_core::w!("PoolId");
pub const CLUSREG_NAME_STORAGESPACE_POOLNAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE: windows_core::PCWSTR = windows_core::w!("PoolQuorumShare");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT: windows_core::PCWSTR = windows_core::w!("PoolQuorumUserAccount");
pub const CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT: windows_core::PCWSTR = windows_core::w!("ReEvaluatePlacementTimeout");
pub const CLUSREG_NAME_STORAGESPACE_POOLSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY: windows_core::PCWSTR = windows_core::w!("TotalCapacity");
pub const CLUSREG_NAME_STORAGESPACE_PROVISIONING: windows_core::PCWSTR = windows_core::w!("VirtualDiskProvisioning");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyColumns");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyInterleave");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyType");
pub const CLUSREG_NAME_STORAGESPACE_STATE: windows_core::PCWSTR = windows_core::w!("VirtualDiskState");
pub const CLUSREG_NAME_UPGRADE_VERSION: windows_core::PCWSTR = windows_core::w!("ClusterUpgradeVersion");
pub const CLUSREG_NAME_VIP_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("AdapterName");
pub const CLUSREG_NAME_VIP_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_VIP_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_VIP_RDID: windows_core::PCWSTR = windows_core::w!("RDID");
pub const CLUSREG_NAME_VIP_VSID: windows_core::PCWSTR = windows_core::w!("VSID");
pub const CLUSREG_NAME_VIRTUAL_NUMA_COUNT: windows_core::PCWSTR = windows_core::w!("VirtualNumaCount");
pub const CLUSREG_NAME_VSSTASK_APPNAME: windows_core::PCWSTR = windows_core::w!("ApplicationName");
pub const CLUSREG_NAME_VSSTASK_APPPARAMS: windows_core::PCWSTR = windows_core::w!("ApplicationParams");
pub const CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_VSSTASK_TRIGGERARRAY: windows_core::PCWSTR = windows_core::w!("TriggerArray");
pub const CLUSREG_NAME_WINS_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_WINS_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("WitnessDynamicWeight");
pub const CLUSREG_READ_ERROR: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(9i32);
pub const CLUSREG_READ_KEY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(7i32);
pub const CLUSREG_READ_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(8i32);
pub const CLUSREG_SET_KEY_SECURITY: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(5i32);
pub const CLUSREG_SET_VALUE: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(1i32);
pub const CLUSREG_VALUE_DELETED: CLUSTER_REG_COMMAND = CLUSTER_REG_COMMAND(6i32);
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
pub const CLUSRES_DISABLE_WPR_WATCHDOG_FOR_OFFLINE_CALLS: u32 = 2u32;
pub const CLUSRES_DISABLE_WPR_WATCHDOG_FOR_ONLINE_CALLS: u32 = 1u32;
pub const CLUSRES_NAME_GET_OPERATION_CONTEXT_FLAGS: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTERSET_OBJECT_TYPE(pub i32);
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(3i32);
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(1i32);
pub const CLUSTERSET_OBJECT_TYPE_NONE: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(0i32);
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: CLUSTERSET_OBJECT_TYPE = CLUSTERSET_OBJECT_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTERVERSIONINFO {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
    pub dwClusterHighestVersion: u32,
    pub dwClusterLowestVersion: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl Default for CLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl Default for CLUSTERVERSIONINFO_NT4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_ADD_EVICT_DELAY: windows_core::PCWSTR = windows_core::w!("AddEvictDelay");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: windows_core::BOOL,
}
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl Default for CLUSTER_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE(pub i32);
pub const CLUSTER_CHANGE_ALL: CLUSTER_CHANGE = CLUSTER_CHANGE(-1i32);
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8191i32);
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(128i32);
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4i32);
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(8i32);
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(512i32);
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2048i32);
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(16i32);
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(32i32);
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(256i32);
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(1073741824i32);
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: CLUSTER_CHANGE = CLUSTER_CHANGE(524288i32);
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1i32);
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(1024i32);
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(64i32);
pub const CLUSTER_CHANGE_CLUSTER_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(536870912i32);
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(2i32);
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: CLUSTER_CHANGE_CLUSTER_V2 = CLUSTER_CHANGE_CLUSTER_V2(4096i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_CLUSTER_V2(pub i32);
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(511i32);
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(2i32);
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(1i32);
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(64i32);
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(128i32);
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(16i32);
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(32i32);
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(256i32);
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(4i32);
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: CLUSTER_CHANGE_GROUPSET_V2 = CLUSTER_CHANGE_GROUPSET_V2(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_GROUPSET_V2(pub i32);
pub const CLUSTER_CHANGE_GROUP_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(16384i32);
pub const CLUSTER_CHANGE_GROUP_ALL_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1023i32);
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(2i32);
pub const CLUSTER_CHANGE_GROUP_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(8192i32);
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(1i32);
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(512i32);
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(16i32);
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(32i32);
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(4i32);
pub const CLUSTER_CHANGE_GROUP_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(32768i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(64i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(128i32);
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(256i32);
pub const CLUSTER_CHANGE_GROUP_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(4096i32);
pub const CLUSTER_CHANGE_GROUP_STATE_V2: CLUSTER_CHANGE_GROUP_V2 = CLUSTER_CHANGE_GROUP_V2(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_GROUP_V2(pub i32);
pub const CLUSTER_CHANGE_HANDLE_CLOSE: CLUSTER_CHANGE = CLUSTER_CHANGE(-2147483648i32);
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(67108864i32);
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(31i32);
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(2i32);
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(33554432i32);
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(1i32);
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(16i32);
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(4i32);
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(134217728i32);
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(16777216i32);
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = CLUSTER_CHANGE_NETINTERFACE_V2(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_NETINTERFACE_V2(pub i32);
pub const CLUSTER_CHANGE_NETWORK_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4194304i32);
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(31i32);
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(2i32);
pub const CLUSTER_CHANGE_NETWORK_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2097152i32);
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(1i32);
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(16i32);
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(4i32);
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8388608i32);
pub const CLUSTER_CHANGE_NETWORK_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1048576i32);
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: CLUSTER_CHANGE_NETWORK_V2 = CLUSTER_CHANGE_NETWORK_V2(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_NETWORK_V2(pub i32);
pub const CLUSTER_CHANGE_NODE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(4i32);
pub const CLUSTER_CHANGE_NODE_ALL_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(255i32);
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(4i32);
pub const CLUSTER_CHANGE_NODE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(2i32);
pub const CLUSTER_CHANGE_NODE_DELETED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(2i32);
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(32i32);
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(64i32);
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(128i32);
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(1i32);
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(8i32);
pub const CLUSTER_CHANGE_NODE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(8i32);
pub const CLUSTER_CHANGE_NODE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(1i32);
pub const CLUSTER_CHANGE_NODE_STATE_V2: CLUSTER_CHANGE_NODE_V2 = CLUSTER_CHANGE_NODE_V2(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_NODE_V2(pub i32);
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
pub const CLUSTER_CHANGE_QUORUM_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(268435456i32);
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: CLUSTER_CHANGE_QUORUM_V2 = CLUSTER_CHANGE_QUORUM_V2(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_QUORUM_V2(pub i32);
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(31i32);
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: CLUSTER_CHANGE = CLUSTER_CHANGE(32i32);
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(1i32);
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(16i32);
pub const CLUSTER_CHANGE_REGISTRY_NAME: CLUSTER_CHANGE = CLUSTER_CHANGE(16i32);
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(2i32);
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: CLUSTER_CHANGE = CLUSTER_CHANGE(128i32);
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_REGISTRY_V2(pub i32);
pub const CLUSTER_CHANGE_REGISTRY_VALUE: CLUSTER_CHANGE = CLUSTER_CHANGE(64i32);
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: CLUSTER_CHANGE_REGISTRY_V2 = CLUSTER_CHANGE_REGISTRY_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(1024i32);
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2047i32);
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1i32);
pub const CLUSTER_CHANGE_RESOURCE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(512i32);
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(128i32);
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(16i32);
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(32i32);
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(256i32);
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(512i32);
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(64i32);
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(2i32);
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(2048i32);
pub const CLUSTER_CHANGE_RESOURCE_STATE: CLUSTER_CHANGE = CLUSTER_CHANGE(256i32);
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(4i32);
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = CLUSTER_CHANGE_RESOURCE_V2(1024i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: CLUSTER_CHANGE = CLUSTER_CHANGE(131072i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(63i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(2i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: CLUSTER_CHANGE = CLUSTER_CHANGE(65536i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(1i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(16i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(8i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(4i32);
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: CLUSTER_CHANGE = CLUSTER_CHANGE(262144i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_RESOURCE_TYPE_V2(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_RESOURCE_V2(pub i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(2i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(7i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(4i32);
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = CLUSTER_CHANGE_SHARED_VOLUME_V2(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_SHARED_VOLUME_V2(pub i32);
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: CLUSTER_CHANGE_SPACEPORT_V2 = CLUSTER_CHANGE_SPACEPORT_V2(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CHANGE_SPACEPORT_V2(pub i32);
pub const CLUSTER_CHANGE_UPGRADE_ALL: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(7i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(2i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(4i32);
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CLOUD_TYPE(pub i32);
pub const CLUSTER_CLOUD_TYPE_AZURE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(1i32);
pub const CLUSTER_CLOUD_TYPE_MIXED: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(128i32);
pub const CLUSTER_CLOUD_TYPE_NONE: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(0i32);
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: CLUSTER_CLOUD_TYPE = CLUSTER_CLOUD_TYPE(-1i32);
pub const CLUSTER_CONFIGURED: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CONTROL_OBJECT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
pub const CLUSTER_CSA_VSS_STATE: windows_core::PCWSTR = windows_core::w!("BackupInProgress");
pub const CLUSTER_CSV_COMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeCompatibleFilters");
pub const CLUSTER_CSV_INCOMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeIncompatibleFilters");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_CSV_VOLUME_FAULT_STATE(pub i32);
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
pub const CLUSTER_ENFORCED_ANTIAFFINITY: windows_core::PCWSTR = windows_core::w!("ClusterEnforcedAntiaffinity");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_ENUM(pub i32);
pub const CLUSTER_ENUM_ALL: CLUSTER_ENUM = CLUSTER_ENUM(63i32);
pub const CLUSTER_ENUM_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(8i32);
pub const CLUSTER_ENUM_INTERNAL_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(-2147483648i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
}
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_ENUM_NETINTERFACE: CLUSTER_ENUM = CLUSTER_ENUM(32i32);
pub const CLUSTER_ENUM_NETWORK: CLUSTER_ENUM = CLUSTER_ENUM(16i32);
pub const CLUSTER_ENUM_NODE: CLUSTER_ENUM = CLUSTER_ENUM(1i32);
pub const CLUSTER_ENUM_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(4i32);
pub const CLUSTER_ENUM_RESTYPE: CLUSTER_ENUM = CLUSTER_ENUM(2i32);
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: CLUSTER_ENUM = CLUSTER_ENUM(536870912i32);
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: CLUSTER_ENUM = CLUSTER_ENUM(1073741824i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_GROUP_AUTOFAILBACK_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_GROUP_ENUM(pub i32);
pub const CLUSTER_GROUP_ENUM_ALL: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(3i32);
pub const CLUSTER_GROUP_ENUM_CONTAINS: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: windows_core::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl Default for CLUSTER_GROUP_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_GROUP_ENUM_NODES: CLUSTER_GROUP_ENUM = CLUSTER_GROUP_ENUM(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_GROUP_PRIORITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_GROUP_STATE(pub i32);
pub const CLUSTER_GROUP_WAIT_DELAY: windows_core::PCWSTR = windows_core::w!("ClusterGroupWaitDelay");
pub const CLUSTER_HANG_RECOVERY_ACTION_KEYNAME: windows_core::PCWSTR = windows_core::w!("HangRecoveryAction");
pub const CLUSTER_HANG_TIMEOUT_KEYNAME: windows_core::PCWSTR = windows_core::w!("ClusSvcHangTimeout");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_HEALTH_FAULT {
    pub Id: windows_core::PWSTR,
    pub ErrorType: u32,
    pub ErrorCode: u32,
    pub Description: windows_core::PWSTR,
    pub Provider: windows_core::PWSTR,
    pub Flags: u32,
    pub Reserved: u32,
}
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_HEALTH_FAULT_ARRAY {
    pub numFaults: u32,
    pub faults: *mut CLUSTER_HEALTH_FAULT,
}
impl Default for CLUSTER_HEALTH_FAULT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION_LABEL: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
pub const CLUSTER_HEALTH_FAULT_ERRORCODE_LABEL: windows_core::PCWSTR = windows_core::w!("ErrorCode");
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE_LABEL: windows_core::PCWSTR = windows_core::w!("ErrorType");
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
pub const CLUSTER_HEALTH_FAULT_FLAGS_LABEL: windows_core::PCWSTR = windows_core::w!("Flags");
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
pub const CLUSTER_HEALTH_FAULT_ID_LABEL: windows_core::PCWSTR = windows_core::w!("Id");
pub const CLUSTER_HEALTH_FAULT_PROPERTY_NAME: windows_core::PCWSTR = windows_core::w!("ClusterHealth");
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
pub const CLUSTER_HEALTH_FAULT_PROVIDER_LABEL: windows_core::PCWSTR = windows_core::w!("Provider");
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
pub const CLUSTER_HEALTH_FAULT_RESERVED_LABEL: windows_core::PCWSTR = windows_core::w!("Reserved");
pub const CLUSTER_INSTALLED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: windows_core::PCWSTR,
    pub dwPrefixLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: windows_core::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
impl Default for CLUSTER_MEMBERSHIP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_MGMT_POINT_RESTYPE(pub i32);
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(0i32);
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(2i32);
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: CLUSTER_MGMT_POINT_RESTYPE = CLUSTER_MGMT_POINT_RESTYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_MGMT_POINT_TYPE(pub i32);
pub const CLUSTER_MGMT_POINT_TYPE_CNO: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(1i32);
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(3i32);
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(2i32);
pub const CLUSTER_MGMT_POINT_TYPE_NONE: CLUSTER_MGMT_POINT_TYPE = CLUSTER_MGMT_POINT_TYPE(0i32);
pub const CLUSTER_NAME_AUTO_BALANCER_LEVEL: windows_core::PCWSTR = windows_core::w!("AutoBalancerLevel");
pub const CLUSTER_NAME_AUTO_BALANCER_MODE: windows_core::PCWSTR = windows_core::w!("AutoBalancerMode");
pub const CLUSTER_NAME_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NETINTERFACE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NETWORK_ENUM(pub i32);
pub const CLUSTER_NETWORK_ENUM_ALL: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: CLUSTER_NETWORK_ENUM = CLUSTER_NETWORK_ENUM(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NETWORK_ROLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NETWORK_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NODE_DRAIN_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NODE_ENUM(pub i32);
pub const CLUSTER_NODE_ENUM_ALL: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(3i32);
pub const CLUSTER_NODE_ENUM_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(2i32);
pub const CLUSTER_NODE_ENUM_NETINTERFACES: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(1i32);
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: CLUSTER_NODE_ENUM = CLUSTER_NODE_ENUM(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NODE_RESUME_FAILBACK_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NODE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NODE_STATUS(pub i32);
pub const CLUSTER_NOTIFICATIONS_V1: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(1i32);
pub const CLUSTER_NOTIFICATIONS_V2: CLUSTER_NOTIFICATIONS_VERSION = CLUSTER_NOTIFICATIONS_VERSION(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_NOTIFICATIONS_VERSION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_OBJECT_TYPE(pub i32);
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(16i32);
pub const CLUSTER_OBJECT_TYPE_CLUSTER: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(1i32);
pub const CLUSTER_OBJECT_TYPE_FAULTDOMAIN: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(17i32);
pub const CLUSTER_OBJECT_TYPE_GROUP: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(2i32);
pub const CLUSTER_OBJECT_TYPE_GROUPSET: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(13i32);
pub const CLUSTER_OBJECT_TYPE_NETWORK: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(6i32);
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(5i32);
pub const CLUSTER_OBJECT_TYPE_NODE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(7i32);
pub const CLUSTER_OBJECT_TYPE_NONE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(0i32);
pub const CLUSTER_OBJECT_TYPE_QUORUM: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(9i32);
pub const CLUSTER_OBJECT_TYPE_REGISTRY: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(8i32);
pub const CLUSTER_OBJECT_TYPE_RESOURCE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(3i32);
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(4i32);
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: CLUSTER_OBJECT_TYPE = CLUSTER_OBJECT_TYPE(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_PROPERTY_FORMAT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_PROPERTY_SYNTAX(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_PROPERTY_TYPE(pub i32);
pub const CLUSTER_QUORUM_LOST: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(1i32);
pub const CLUSTER_QUORUM_MAINTAINED: CLUSTER_QUORUM_VALUE = CLUSTER_QUORUM_VALUE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_QUORUM_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_QUORUM_VALUE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: windows_core::PCWSTR,
    pub wzValueName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl Default for CLUSTER_READ_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_REG_COMMAND(pub i32);
pub const CLUSTER_REQUEST_REPLY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("RequestReplyTimeout");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_APPLICATION_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_CLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_CREATE_FLAGS(pub i32);
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_ENUM(pub i32);
pub const CLUSTER_RESOURCE_ENUM_ALL: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(7i32);
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: windows_core::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: windows_core::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl Default for CLUSTER_RESOURCE_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const CLUSTER_RESOURCE_ENUM_NODES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(4i32);
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: CLUSTER_RESOURCE_ENUM = CLUSTER_RESOURCE_ENUM(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_RESTART_ACTION(pub i32);
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_STATE_CHANGE_REASON(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_RESOURCE_TYPE_ENUM(pub i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(3i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(1i32);
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: CLUSTER_RESOURCE_TYPE_ENUM = CLUSTER_RESOURCE_TYPE_ENUM(2i32);
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = CLUSTER_CHANGE_RESOURCE_TYPE_V2(32i32);
pub const CLUSTER_RESOURCE_VALID_FLAGS: CLUSTER_RESOURCE_CREATE_FLAGS = CLUSTER_RESOURCE_CREATE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_ROLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_ROLE_STATE(pub i32);
pub const CLUSTER_RUNNING: u32 = 16u32;
pub const CLUSTER_S2D_BUS_TYPES: windows_core::PCWSTR = windows_core::w!("S2DBusTypes");
pub const CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS: windows_core::PCWSTR = windows_core::w!("S2DCacheBehavior");
pub const CLUSTER_S2D_CACHE_DESIRED_STATE: windows_core::PCWSTR = windows_core::w!("S2DCacheDesiredState");
pub const CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT: windows_core::PCWSTR = windows_core::w!("S2DCacheFlashReservePercent");
pub const CLUSTER_S2D_CACHE_METADATA_RESERVE: windows_core::PCWSTR = windows_core::w!("S2DCacheMetadataReserveBytes");
pub const CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES: windows_core::PCWSTR = windows_core::w!("S2DCachePageSizeKBytes");
pub const CLUSTER_S2D_ENABLED: windows_core::PCWSTR = windows_core::w!("S2DEnabled");
pub const CLUSTER_S2D_IO_LATENCY_THRESHOLD: windows_core::PCWSTR = windows_core::w!("S2DIOLatencyThreshold");
pub const CLUSTER_S2D_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("S2DOptimizations");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SETUP_PHASE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SETUP_PHASE_SEVERITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SETUP_PHASE_TYPE(pub i32);
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: bool,
    pub ReturnStatus: u32,
}
pub const CLUSTER_SHARED_VOLUMES_ROOT: windows_core::PCWSTR = windows_core::w!("SharedVolumesRoot");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_BACKUP_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("SharedVolumeVssWriterOperationTimeout");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_STORAGENODE_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUSTER_UPGRADE_PHASE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 1],
}
impl Default for CLUSTER_VALIDATE_CSV_FILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 1],
}
impl Default for CLUSTER_VALIDATE_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 1],
}
impl Default for CLUSTER_VALIDATE_NETNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 1],
}
impl Default for CLUSTER_VALIDATE_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
pub const CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("WitnessDatabaseWriteTimeout");
pub const CLUSTER_WITNESS_FAILED_RESTART_INTERVAL: windows_core::PCWSTR = windows_core::w!("WitnessRestartInterval");
pub const CLUS_ACCESS_ANY: u32 = 0u32;
pub const CLUS_ACCESS_READ: u32 = 1u32;
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(3i32);
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
pub const CLUS_AFFINITY_RULE_MAX: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(4i32);
pub const CLUS_AFFINITY_RULE_MIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
pub const CLUS_AFFINITY_RULE_NONE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(0i32);
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(1i32);
pub const CLUS_AFFINITY_RULE_SAME_NODE: CLUS_AFFINITY_RULE_TYPE = CLUS_AFFINITY_RULE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_AFFINITY_RULE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_CHARACTERISTICS(pub i32);
pub const CLUS_CHAR_BROADCAST_DELETE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32i32);
pub const CLUS_CHAR_CLONES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8192i32);
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(256i32);
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2i32);
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(524288i32);
pub const CLUS_CHAR_INFRASTRUCTURE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(131072i32);
pub const CLUS_CHAR_LOCAL_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4i32);
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(8i32);
pub const CLUS_CHAR_MONITOR_DETACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1024i32);
pub const CLUS_CHAR_MONITOR_REATTACH: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(2048i32);
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(32768i32);
pub const CLUS_CHAR_NOT_PREEMPTABLE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16384i32);
pub const CLUS_CHAR_OPERATION_CONTEXT: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(4096i32);
pub const CLUS_CHAR_PLACEMENT_DATA: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(512i32);
pub const CLUS_CHAR_QUORUM: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(1i32);
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(16i32);
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(64i32);
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(128i32);
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(65536i32);
pub const CLUS_CHAR_UNKNOWN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(0i32);
pub const CLUS_CHAR_VETO_DRAIN: CLUS_CHARACTERISTICS = CLUS_CHARACTERISTICS(262144i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl Default for CLUS_CHKDSK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: windows_core::BOOL,
    pub VolumeName: [u16; 260],
}
impl Default for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl Default for CLUS_CSV_VOLUME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl Default for CLUS_CSV_VOLUME_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: windows_core::BOOL,
    pub IsFileServerPresent: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_FLAGS(pub i32);
pub const CLUS_FLAG_CORE: CLUS_FLAGS = CLUS_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl Default for CLUS_FORCE_QUORUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
pub const CLUS_GLOBAL: u32 = 1u32;
pub const CLUS_GROUP_DO_NOT_START: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(1i32);
pub const CLUS_GROUP_START_ALLOWED: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(2i32);
pub const CLUS_GROUP_START_ALWAYS: CLUS_GROUP_START_SETTING = CLUS_GROUP_START_SETTING(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_GROUP_START_SETTING(pub i32);
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: windows_core::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
pub const CLUS_MODIFY: u32 = 1u32;
pub const CLUS_NAME_RES_TYPE_CLUSTER_GROUPID: windows_core::PCWSTR = windows_core::w!("ClusterGroupId");
pub const CLUS_NAME_RES_TYPE_DATA_RESID: windows_core::PCWSTR = windows_core::w!("DataResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_MULTIPLE: windows_core::PCWSTR = windows_core::w!("LogSizeMultiple");
pub const CLUS_NAME_RES_TYPE_LOG_RESID: windows_core::PCWSTR = windows_core::w!("LogResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_VOLUME: windows_core::PCWSTR = windows_core::w!("LogVolume");
pub const CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE: windows_core::PCWSTR = windows_core::w!("MinimumLogSizeInBytes");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUPID: windows_core::PCWSTR = windows_core::w!("ReplicationGroupId");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE: windows_core::PCWSTR = windows_core::w!("ReplicationClusterGroupType");
pub const CLUS_NAME_RES_TYPE_SOURCE_RESID: windows_core::PCWSTR = windows_core::w!("SourceResourceId");
pub const CLUS_NAME_RES_TYPE_SOURCE_VOLUMES: windows_core::PCWSTR = windows_core::w!("SourceVolumes");
pub const CLUS_NAME_RES_TYPE_TARGET_RESID: windows_core::PCWSTR = windows_core::w!("TargetResourceId");
pub const CLUS_NAME_RES_TYPE_TARGET_VOLUMES: windows_core::PCWSTR = windows_core::w!("TargetVolumes");
pub const CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE: windows_core::PCWSTR = windows_core::w!("UnitOfLogSizeChangeInBytes");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl Default for CLUS_NETNAME_IP_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl Default for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl Default for CLUS_NETNAME_PWD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl Default for CLUS_NETNAME_PWD_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_NETNAME_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: windows_core::BOOL,
}
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
pub const CLUS_NO_MODIFY: u32 = 0u32;
pub const CLUS_OBJECT_AFFINITYRULE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(9i32);
pub const CLUS_OBJECT_CLUSTER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(7i32);
pub const CLUS_OBJECT_GROUP: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(3i32);
pub const CLUS_OBJECT_GROUPSET: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(8i32);
pub const CLUS_OBJECT_INVALID: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(0i32);
pub const CLUS_OBJECT_NETINTERFACE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(6i32);
pub const CLUS_OBJECT_NETWORK: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(5i32);
pub const CLUS_OBJECT_NODE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(4i32);
pub const CLUS_OBJECT_RESOURCE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(1i32);
pub const CLUS_OBJECT_RESOURCE_TYPE: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(2i32);
pub const CLUS_OBJECT_USER: CLUSTER_CONTROL_OBJECT = CLUSTER_CONTROL_OBJECT(128i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl Default for CLUS_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PARTITION_INFO_EX {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
    pub TotalSizeInBytes: u64,
    pub FreeSizeInBytes: u64,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
    pub VolumeGuid: windows_core::GUID,
}
impl Default for CLUS_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: windows_core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl Default for CLUS_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl Default for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUS_RESCLASS_NETWORK: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(2i32);
pub const CLUS_RESCLASS_STORAGE: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(1i32);
pub const CLUS_RESCLASS_UNKNOWN: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(0i32);
pub const CLUS_RESCLASS_USER: CLUSTER_RESOURCE_CLASS = CLUSTER_RESOURCE_CLASS(32768i32);
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl Default for CLUS_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_RESSUBCLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_RESSUBCLASS_NETWORK(pub i32);
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: CLUS_RESSUBCLASS_NETWORK = CLUS_RESSUBCLASS_NETWORK(-2147483648i32);
pub const CLUS_RESSUBCLASS_SHARED: CLUS_RESSUBCLASS = CLUS_RESSUBCLASS(-2147483648i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLUS_RESSUBCLASS_STORAGE(pub i32);
pub const CLUS_RESSUBCLASS_STORAGE_DISK: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(1073741824i32);
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(268435456i32);
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: CLUS_RESSUBCLASS_STORAGE = CLUS_RESSUBCLASS_STORAGE(-2147483648i32);
pub const CLUS_RESTYPE_NAME_CAU: windows_core::PCWSTR = windows_core::w!("ClusterAwareUpdatingResource");
pub const CLUS_RESTYPE_NAME_CLOUD_WITNESS: windows_core::PCWSTR = windows_core::w!("Cloud Witness");
pub const CLUS_RESTYPE_NAME_CONTAINER: windows_core::PCWSTR = windows_core::w!("Container");
pub const CLUS_RESTYPE_NAME_CROSS_CLUSTER: windows_core::PCWSTR = windows_core::w!("Cross Cluster Dependency Orchestrator");
pub const CLUS_RESTYPE_NAME_DFS: windows_core::PCWSTR = windows_core::w!("Distributed File System");
pub const CLUS_RESTYPE_NAME_DFSR: windows_core::PCWSTR = windows_core::w!("DFS Replicated Folder");
pub const CLUS_RESTYPE_NAME_DHCP: windows_core::PCWSTR = windows_core::w!("DHCP Service");
pub const CLUS_RESTYPE_NAME_DNN: windows_core::PCWSTR = windows_core::w!("Distributed Network Name");
pub const CLUS_RESTYPE_NAME_FILESERVER: windows_core::PCWSTR = windows_core::w!("File Server");
pub const CLUS_RESTYPE_NAME_FILESHR: windows_core::PCWSTR = windows_core::w!("File Share");
pub const CLUS_RESTYPE_NAME_FSWITNESS: windows_core::PCWSTR = windows_core::w!("File Share Witness");
pub const CLUS_RESTYPE_NAME_GENAPP: windows_core::PCWSTR = windows_core::w!("Generic Application");
pub const CLUS_RESTYPE_NAME_GENSCRIPT: windows_core::PCWSTR = windows_core::w!("Generic Script");
pub const CLUS_RESTYPE_NAME_GENSVC: windows_core::PCWSTR = windows_core::w!("Generic Service");
pub const CLUS_RESTYPE_NAME_HARDDISK: windows_core::PCWSTR = windows_core::w!("Physical Disk");
pub const CLUS_RESTYPE_NAME_HCSVM: windows_core::PCWSTR = windows_core::w!("HCS Virtual Machine");
pub const CLUS_RESTYPE_NAME_HEALTH_SERVICE: windows_core::PCWSTR = windows_core::w!("Health Service");
pub const CLUS_RESTYPE_NAME_IPADDR: windows_core::PCWSTR = windows_core::w!("IP Address");
pub const CLUS_RESTYPE_NAME_IPV6_NATIVE: windows_core::PCWSTR = windows_core::w!("IPv6 Address");
pub const CLUS_RESTYPE_NAME_IPV6_TUNNEL: windows_core::PCWSTR = windows_core::w!("IPv6 Tunnel Address");
pub const CLUS_RESTYPE_NAME_ISCSITARGET: windows_core::PCWSTR = windows_core::w!("iSCSI Target Server");
pub const CLUS_RESTYPE_NAME_ISNS: windows_core::PCWSTR = windows_core::w!("Microsoft iSNS");
pub const CLUS_RESTYPE_NAME_MSDTC: windows_core::PCWSTR = windows_core::w!("Distributed Transaction Coordinator");
pub const CLUS_RESTYPE_NAME_MSMQ: windows_core::PCWSTR = windows_core::w!("Microsoft Message Queue Server");
pub const CLUS_RESTYPE_NAME_MSMQ_TRIGGER: windows_core::PCWSTR = windows_core::w!("MSMQTriggers");
pub const CLUS_RESTYPE_NAME_NAT: windows_core::PCWSTR = windows_core::w!("Nat");
pub const CLUS_RESTYPE_NAME_NETNAME: windows_core::PCWSTR = windows_core::w!("Network Name");
pub const CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NEW_MSMQ: windows_core::PCWSTR = windows_core::w!("MSMQ");
pub const CLUS_RESTYPE_NAME_NFS: windows_core::PCWSTR = windows_core::w!("NFS Share");
pub const CLUS_RESTYPE_NAME_NFS_MSNS: windows_core::PCWSTR = windows_core::w!("NFS Multi Server Namespace");
pub const CLUS_RESTYPE_NAME_NFS_V2: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS: windows_core::PCWSTR = windows_core::w!("Provider Address");
pub const CLUS_RESTYPE_NAME_PHYS_DISK: windows_core::PCWSTR = windows_core::w!("Physical Disk");
pub const CLUS_RESTYPE_NAME_PRTSPLR: windows_core::PCWSTR = windows_core::w!("Print Spooler");
pub const CLUS_RESTYPE_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RESTYPE_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
pub const CLUS_RESTYPE_NAME_SDDC_MANAGEMENT: windows_core::PCWSTR = windows_core::w!("SDDC Management");
pub const CLUS_RESTYPE_NAME_SODAFILESERVER: windows_core::PCWSTR = windows_core::w!("Scale Out File Server");
pub const CLUS_RESTYPE_NAME_STORAGE_POLICIES: windows_core::PCWSTR = windows_core::w!("Storage Policies");
pub const CLUS_RESTYPE_NAME_STORAGE_POOL: windows_core::PCWSTR = windows_core::w!("Storage Pool");
pub const CLUS_RESTYPE_NAME_STORAGE_REPLICA: windows_core::PCWSTR = windows_core::w!("Storage Replica");
pub const CLUS_RESTYPE_NAME_STORQOS: windows_core::PCWSTR = windows_core::w!("Storage QoS Policy Manager");
pub const CLUS_RESTYPE_NAME_TASKSCHEDULER: windows_core::PCWSTR = windows_core::w!("Task Scheduler");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV4: windows_core::PCWSTR = windows_core::w!("Disjoint IPv4 Address");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV6: windows_core::PCWSTR = windows_core::w!("Disjoint IPv6 Address");
pub const CLUS_RESTYPE_NAME_VM: windows_core::PCWSTR = windows_core::w!("Virtual Machine");
pub const CLUS_RESTYPE_NAME_VMREPLICA_BROKER: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Broker");
pub const CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Coordinator");
pub const CLUS_RESTYPE_NAME_VM_CONFIG: windows_core::PCWSTR = windows_core::w!("Virtual Machine Configuration");
pub const CLUS_RESTYPE_NAME_VM_WMI: windows_core::PCWSTR = windows_core::w!("Virtual Machine Cluster WMI");
pub const CLUS_RESTYPE_NAME_VSSTASK: windows_core::PCWSTR = windows_core::w!("Volume Shadow Copy Service Task");
pub const CLUS_RESTYPE_NAME_WINS: windows_core::PCWSTR = windows_core::w!("WINS Service");
pub const CLUS_RES_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RES_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl Default for CLUS_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl Default for CLUS_SCSI_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: windows_core::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
impl Default for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl Default for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: windows_core::BOOL,
    pub bFirst: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_WORKER {
    pub hThread: super::super::Foundation::HANDLE,
    pub Terminate: windows_core::BOOL,
}
pub const CREATEDC_PRESENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *const windows_core::PCWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: *mut CLUSTER_IP_ENTRY,
    pub fEmptyCluster: bool,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
}
impl Default for CREATE_CLUSTER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub pszUserName: windows_core::PCWSTR,
    pub pszPassword: windows_core::PCWSTR,
    pub pszDomain: windows_core::PCWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: bool,
}
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
pub const CTCTL_GET_FAULT_DOMAIN_STATE: CLCTL_CODES = CLCTL_CODES(789i32);
pub const CTCTL_GET_ROUTESTATUS_BASIC: CLCTL_CODES = CLCTL_CODES(781i32);
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: CLCTL_CODES = CLCTL_CODES(785i32);
pub const ClusApplication: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e5_2631_11d1_89f1_00a0c90d061e);
pub const ClusCryptoKeys: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6072b_2631_11d1_89f1_00a0c90d061e);
pub const ClusDisk: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60723_2631_11d1_89f1_00a0c90d061e);
pub const ClusDisks: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60725_2631_11d1_89f1_00a0c90d061e);
pub const ClusGroupTypeAvailableStorage: CLUSGROUP_TYPE = CLUSGROUP_TYPE(2i32);
pub const ClusGroupTypeClusterUpdateAgent: CLUSGROUP_TYPE = CLUSGROUP_TYPE(117i32);
pub const ClusGroupTypeCoreCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(1i32);
pub const ClusGroupTypeCoreSddc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(123i32);
pub const ClusGroupTypeCrossClusterOrchestrator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(121i32);
pub const ClusGroupTypeDhcpServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(102i32);
pub const ClusGroupTypeDtc: CLUSGROUP_TYPE = CLUSGROUP_TYPE(103i32);
pub const ClusGroupTypeFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(100i32);
pub const ClusGroupTypeGenericApplication: CLUSGROUP_TYPE = CLUSGROUP_TYPE(107i32);
pub const ClusGroupTypeGenericScript: CLUSGROUP_TYPE = CLUSGROUP_TYPE(109i32);
pub const ClusGroupTypeGenericService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(108i32);
pub const ClusGroupTypeIScsiNameService: CLUSGROUP_TYPE = CLUSGROUP_TYPE(110i32);
pub const ClusGroupTypeIScsiTarget: CLUSGROUP_TYPE = CLUSGROUP_TYPE(113i32);
pub const ClusGroupTypeInfrastructureFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(122i32);
pub const ClusGroupTypeMsmq: CLUSGROUP_TYPE = CLUSGROUP_TYPE(104i32);
pub const ClusGroupTypePrintServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(101i32);
pub const ClusGroupTypeScaleoutCluster: CLUSGROUP_TYPE = CLUSGROUP_TYPE(118i32);
pub const ClusGroupTypeScaleoutFileServer: CLUSGROUP_TYPE = CLUSGROUP_TYPE(114i32);
pub const ClusGroupTypeSharedVolume: CLUSGROUP_TYPE = CLUSGROUP_TYPE(4i32);
pub const ClusGroupTypeStandAloneDfs: CLUSGROUP_TYPE = CLUSGROUP_TYPE(106i32);
pub const ClusGroupTypeStoragePool: CLUSGROUP_TYPE = CLUSGROUP_TYPE(5i32);
pub const ClusGroupTypeStorageReplica: CLUSGROUP_TYPE = CLUSGROUP_TYPE(119i32);
pub const ClusGroupTypeTaskScheduler: CLUSGROUP_TYPE = CLUSGROUP_TYPE(116i32);
pub const ClusGroupTypeTemporary: CLUSGROUP_TYPE = CLUSGROUP_TYPE(3i32);
pub const ClusGroupTypeTsSessionBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(112i32);
pub const ClusGroupTypeUnknown: CLUSGROUP_TYPE = CLUSGROUP_TYPE(9999i32);
pub const ClusGroupTypeVMReplicaBroker: CLUSGROUP_TYPE = CLUSGROUP_TYPE(115i32);
pub const ClusGroupTypeVMReplicaCoordinator: CLUSGROUP_TYPE = CLUSGROUP_TYPE(120i32);
pub const ClusGroupTypeVirtualMachine: CLUSGROUP_TYPE = CLUSGROUP_TYPE(111i32);
pub const ClusGroupTypeWins: CLUSGROUP_TYPE = CLUSGROUP_TYPE(105i32);
pub const ClusNetInterface: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ed_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ef_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetwork: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f1_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetworkNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f5_2631_11d1_89f1_00a0c90d061e);
pub const ClusNetworks: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f3_2631_11d1_89f1_00a0c90d061e);
pub const ClusNode: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f7_2631_11d1_89f1_00a0c90d061e);
pub const ClusNodeNetInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606fb_2631_11d1_89f1_00a0c90d061e);
pub const ClusNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606f9_2631_11d1_89f1_00a0c90d061e);
pub const ClusPartition: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071f_2631_11d1_89f1_00a0c90d061e);
pub const ClusPartitionEx: windows_core::GUID = windows_core::GUID::from_u128(0x53d51d26_b51b_4a79_b2c3_5048d93a98fc);
pub const ClusPartitions: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60721_2631_11d1_89f1_00a0c90d061e);
pub const ClusProperties: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606ff_2631_11d1_89f1_00a0c90d061e);
pub const ClusProperty: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606fd_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValue: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60719_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValueData: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071d_2631_11d1_89f1_00a0c90d061e);
pub const ClusPropertyValues: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6071b_2631_11d1_89f1_00a0c90d061e);
pub const ClusRefObject: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60701_2631_11d1_89f1_00a0c90d061e);
pub const ClusRegistryKeys: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60729_2631_11d1_89f1_00a0c90d061e);
pub const ClusResDependencies: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60703_2631_11d1_89f1_00a0c90d061e);
pub const ClusResDependents: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6072d_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroup: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60705_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroupPreferredOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e7_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroupResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e9_2631_11d1_89f1_00a0c90d061e);
pub const ClusResGroups: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60707_2631_11d1_89f1_00a0c90d061e);
pub const ClusResPossibleOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070d_2631_11d1_89f1_00a0c90d061e);
pub const ClusResType: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070f_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypePossibleOwnerNodes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60717_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypeResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60713_2631_11d1_89f1_00a0c90d061e);
pub const ClusResTypes: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60711_2631_11d1_89f1_00a0c90d061e);
pub const ClusResource: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60709_2631_11d1_89f1_00a0c90d061e);
pub const ClusResources: windows_core::GUID = windows_core::GUID::from_u128(0xf2e6070b_2631_11d1_89f1_00a0c90d061e);
pub const ClusScsiAddress: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60727_2631_11d1_89f1_00a0c90d061e);
pub const ClusVersion: windows_core::GUID = windows_core::GUID::from_u128(0xf2e60715_2631_11d1_89f1_00a0c90d061e);
pub const Cluster: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e3_2631_11d1_89f1_00a0c90d061e);
pub const ClusterGroupAllowFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(1i32);
pub const ClusterGroupFailbackTypeCount: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(2i32);
pub const ClusterGroupFailed: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(2i32);
pub const ClusterGroupOffline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(1i32);
pub const ClusterGroupOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(0i32);
pub const ClusterGroupPartialOnline: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(3i32);
pub const ClusterGroupPending: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(4i32);
pub const ClusterGroupPreventFailback: CLUSTER_GROUP_AUTOFAILBACK_TYPE = CLUSTER_GROUP_AUTOFAILBACK_TYPE(0i32);
pub const ClusterGroupStateUnknown: CLUSTER_GROUP_STATE = CLUSTER_GROUP_STATE(-1i32);
pub const ClusterNames: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606eb_2631_11d1_89f1_00a0c90d061e);
pub const ClusterNetInterfaceFailed: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(1i32);
pub const ClusterNetInterfaceStateUnknown: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(-1i32);
pub const ClusterNetInterfaceUnavailable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(0i32);
pub const ClusterNetInterfaceUnreachable: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(2i32);
pub const ClusterNetInterfaceUp: CLUSTER_NETINTERFACE_STATE = CLUSTER_NETINTERFACE_STATE(3i32);
pub const ClusterNetworkDown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(1i32);
pub const ClusterNetworkPartitioned: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(2i32);
pub const ClusterNetworkRoleClientAccess: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(2i32);
pub const ClusterNetworkRoleInternalAndClient: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(3i32);
pub const ClusterNetworkRoleInternalUse: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(1i32);
pub const ClusterNetworkRoleNone: CLUSTER_NETWORK_ROLE = CLUSTER_NETWORK_ROLE(0i32);
pub const ClusterNetworkStateUnknown: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(-1i32);
pub const ClusterNetworkUnavailable: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(0i32);
pub const ClusterNetworkUp: CLUSTER_NETWORK_STATE = CLUSTER_NETWORK_STATE(3i32);
pub const ClusterNodeDown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(1i32);
pub const ClusterNodeDrainStatusCount: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(4i32);
pub const ClusterNodeJoining: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(3i32);
pub const ClusterNodePaused: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(2i32);
pub const ClusterNodeResumeFailbackTypeCount: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(3i32);
pub const ClusterNodeStateUnknown: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(-1i32);
pub const ClusterNodeUp: CLUSTER_NODE_STATE = CLUSTER_NODE_STATE(0i32);
pub const ClusterResourceApplicationOSHeartBeat: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(2i32);
pub const ClusterResourceApplicationReady: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(3i32);
pub const ClusterResourceApplicationStateUnknown: CLUSTER_RESOURCE_APPLICATION_STATE = CLUSTER_RESOURCE_APPLICATION_STATE(1i32);
pub const ClusterResourceDontRestart: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(0i32);
pub const ClusterResourceEmbeddedFailureActionLogOnly: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(1i32);
pub const ClusterResourceEmbeddedFailureActionNone: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(0i32);
pub const ClusterResourceEmbeddedFailureActionRecover: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(2i32);
pub const ClusterResourceFailed: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(4i32);
pub const ClusterResourceInherited: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(0i32);
pub const ClusterResourceInitializing: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(1i32);
pub const ClusterResourceOffline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(3i32);
pub const ClusterResourceOfflinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(130i32);
pub const ClusterResourceOnline: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(2i32);
pub const ClusterResourceOnlinePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(129i32);
pub const ClusterResourcePending: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(128i32);
pub const ClusterResourceRestartActionCount: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(3i32);
pub const ClusterResourceRestartNoNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(1i32);
pub const ClusterResourceRestartNotify: CLUSTER_RESOURCE_RESTART_ACTION = CLUSTER_RESOURCE_RESTART_ACTION(2i32);
pub const ClusterResourceStateUnknown: CLUSTER_RESOURCE_STATE = CLUSTER_RESOURCE_STATE(-1i32);
pub const ClusterRoleClustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(0i32);
pub const ClusterRoleDFSReplicatedFolder: CLUSTER_ROLE = CLUSTER_ROLE(15i32);
pub const ClusterRoleDHCP: CLUSTER_ROLE = CLUSTER_ROLE(0i32);
pub const ClusterRoleDTC: CLUSTER_ROLE = CLUSTER_ROLE(1i32);
pub const ClusterRoleDistributedFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(16i32);
pub const ClusterRoleDistributedNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(17i32);
pub const ClusterRoleFileServer: CLUSTER_ROLE = CLUSTER_ROLE(2i32);
pub const ClusterRoleFileShare: CLUSTER_ROLE = CLUSTER_ROLE(18i32);
pub const ClusterRoleFileShareWitness: CLUSTER_ROLE = CLUSTER_ROLE(19i32);
pub const ClusterRoleGenericApplication: CLUSTER_ROLE = CLUSTER_ROLE(3i32);
pub const ClusterRoleGenericScript: CLUSTER_ROLE = CLUSTER_ROLE(4i32);
pub const ClusterRoleGenericService: CLUSTER_ROLE = CLUSTER_ROLE(5i32);
pub const ClusterRoleHardDisk: CLUSTER_ROLE = CLUSTER_ROLE(20i32);
pub const ClusterRoleIPAddress: CLUSTER_ROLE = CLUSTER_ROLE(21i32);
pub const ClusterRoleIPV6Address: CLUSTER_ROLE = CLUSTER_ROLE(22i32);
pub const ClusterRoleIPV6TunnelAddress: CLUSTER_ROLE = CLUSTER_ROLE(23i32);
pub const ClusterRoleISCSINameServer: CLUSTER_ROLE = CLUSTER_ROLE(6i32);
pub const ClusterRoleISCSITargetServer: CLUSTER_ROLE = CLUSTER_ROLE(24i32);
pub const ClusterRoleMSMQ: CLUSTER_ROLE = CLUSTER_ROLE(7i32);
pub const ClusterRoleNFS: CLUSTER_ROLE = CLUSTER_ROLE(8i32);
pub const ClusterRoleNetworkFileSystem: CLUSTER_ROLE = CLUSTER_ROLE(14i32);
pub const ClusterRoleNetworkName: CLUSTER_ROLE = CLUSTER_ROLE(25i32);
pub const ClusterRolePhysicalDisk: CLUSTER_ROLE = CLUSTER_ROLE(26i32);
pub const ClusterRolePrintServer: CLUSTER_ROLE = CLUSTER_ROLE(9i32);
pub const ClusterRoleSODAFileServer: CLUSTER_ROLE = CLUSTER_ROLE(27i32);
pub const ClusterRoleStandAloneNamespaceServer: CLUSTER_ROLE = CLUSTER_ROLE(10i32);
pub const ClusterRoleStoragePool: CLUSTER_ROLE = CLUSTER_ROLE(28i32);
pub const ClusterRoleTaskScheduler: CLUSTER_ROLE = CLUSTER_ROLE(13i32);
pub const ClusterRoleUnclustered: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(1i32);
pub const ClusterRoleUnknown: CLUSTER_ROLE_STATE = CLUSTER_ROLE_STATE(-1i32);
pub const ClusterRoleVirtualMachine: CLUSTER_ROLE = CLUSTER_ROLE(29i32);
pub const ClusterRoleVirtualMachineConfiguration: CLUSTER_ROLE = CLUSTER_ROLE(30i32);
pub const ClusterRoleVirtualMachineReplicaBroker: CLUSTER_ROLE = CLUSTER_ROLE(31i32);
pub const ClusterRoleVolumeShadowCopyServiceTask: CLUSTER_ROLE = CLUSTER_ROLE(11i32);
pub const ClusterRoleWINS: CLUSTER_ROLE = CLUSTER_ROLE(12i32);
pub const ClusterSetupPhaseAddClusterProperties: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(201i32);
pub const ClusterSetupPhaseAddNodeToCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(301i32);
pub const ClusterSetupPhaseCleanupCOs: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(402i32);
pub const ClusterSetupPhaseCleanupNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(405i32);
pub const ClusterSetupPhaseClusterGroupOnline: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(206i32);
pub const ClusterSetupPhaseConfigureClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(104i32);
pub const ClusterSetupPhaseConfigureClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(109i32);
pub const ClusterSetupPhaseContinue: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(2i32);
pub const ClusterSetupPhaseCoreGroupCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(406i32);
pub const ClusterSetupPhaseCreateClusterAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(108i32);
pub const ClusterSetupPhaseCreateGroups: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(203i32);
pub const ClusterSetupPhaseCreateIPAddressResources: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(204i32);
pub const ClusterSetupPhaseCreateNetworkName: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(205i32);
pub const ClusterSetupPhaseCreateResourceTypes: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(202i32);
pub const ClusterSetupPhaseDeleteGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(401i32);
pub const ClusterSetupPhaseEnd: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(3i32);
pub const ClusterSetupPhaseEvictNode: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(404i32);
pub const ClusterSetupPhaseFailureCleanup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(999i32);
pub const ClusterSetupPhaseFatal: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(3i32);
pub const ClusterSetupPhaseFormingCluster: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(200i32);
pub const ClusterSetupPhaseGettingCurrentMembership: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(300i32);
pub const ClusterSetupPhaseInformational: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(1i32);
pub const ClusterSetupPhaseInitialize: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(1i32);
pub const ClusterSetupPhaseMoveGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(400i32);
pub const ClusterSetupPhaseNodeUp: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(302i32);
pub const ClusterSetupPhaseOfflineGroup: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(403i32);
pub const ClusterSetupPhaseQueryClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(106i32);
pub const ClusterSetupPhaseReport: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(4i32);
pub const ClusterSetupPhaseStart: CLUSTER_SETUP_PHASE_TYPE = CLUSTER_SETUP_PHASE_TYPE(1i32);
pub const ClusterSetupPhaseStartingClusSvc: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(105i32);
pub const ClusterSetupPhaseValidateClusDisk: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(103i32);
pub const ClusterSetupPhaseValidateClusterNameAccount: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(107i32);
pub const ClusterSetupPhaseValidateNetft: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(102i32);
pub const ClusterSetupPhaseValidateNodeState: CLUSTER_SETUP_PHASE = CLUSTER_SETUP_PHASE(100i32);
pub const ClusterSetupPhaseWarning: CLUSTER_SETUP_PHASE_SEVERITY = CLUSTER_SETUP_PHASE_SEVERITY(2i32);
pub const ClusterSharedVolumeHWSnapshotCompleted: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(2i32);
pub const ClusterSharedVolumePrepareForFreeze: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(3i32);
pub const ClusterSharedVolumePrepareForHWSnapshot: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(1i32);
pub const ClusterSharedVolumeRenameInputTypeNone: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(0i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(4i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeId: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(2i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeName: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(3i32);
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(1i32);
pub const ClusterSharedVolumeSnapshotStateUnknown: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(0i32);
pub const ClusterStateNotConfigured: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(1i32);
pub const ClusterStateNotInstalled: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(0i32);
pub const ClusterStateNotRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(3i32);
pub const ClusterStateRunning: NODE_CLUSTER_STATE = NODE_CLUSTER_STATE(19i32);
pub const ClusterStorageNodeDown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(2i32);
pub const ClusterStorageNodePaused: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(3i32);
pub const ClusterStorageNodeStarting: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(4i32);
pub const ClusterStorageNodeStateUnknown: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(0i32);
pub const ClusterStorageNodeStopping: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(5i32);
pub const ClusterStorageNodeUp: CLUSTER_STORAGENODE_STATE = CLUSTER_STORAGENODE_STATE(1i32);
pub const ClusterUpgradePhaseInitialize: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(1i32);
pub const ClusterUpgradePhaseInstallingNewComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(4i32);
pub const ClusterUpgradePhaseUpgradeComplete: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(5i32);
pub const ClusterUpgradePhaseUpgradingComponents: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(3i32);
pub const ClusterUpgradePhaseValidatingUpgrade: CLUSTER_UPGRADE_PHASE = CLUSTER_UPGRADE_PHASE(2i32);
pub const DNS_LENGTH: u32 = 64u32;
pub const DoNotFailbackGroups: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(0i32);
pub const DomainNames: windows_core::GUID = windows_core::GUID::from_u128(0xf2e606e1_2631_11d1_89f1_00a0c90d061e);
pub const ENABLE_CLUSTER_SHARED_VOLUMES: windows_core::PCWSTR = windows_core::w!("EnableSharedVolumes");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FAILURE_TYPE(pub i32);
pub const FAILURE_TYPE_EMBEDDED: FAILURE_TYPE = FAILURE_TYPE(1i32);
pub const FAILURE_TYPE_GENERAL: FAILURE_TYPE = FAILURE_TYPE(0i32);
pub const FAILURE_TYPE_NETWORK_LOSS: FAILURE_TYPE = FAILURE_TYPE(2i32);
pub const FE_UPGRADE_VERSION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl Default for FILESHARE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILESHARE_CHANGE_ADD: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(1i32);
pub const FILESHARE_CHANGE_DEL: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILESHARE_CHANGE_ENUM(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 1],
}
impl Default for FILESHARE_CHANGE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILESHARE_CHANGE_MODIFY: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(3i32);
pub const FILESHARE_CHANGE_NONE: FILESHARE_CHANGE_ENUM = FILESHARE_CHANGE_ENUM(0i32);
pub const FailbackGroupsImmediately: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(1i32);
pub const FailbackGroupsPerPolicy: CLUSTER_NODE_RESUME_FAILBACK_TYPE = CLUSTER_NODE_RESUME_FAILBACK_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GET_OPERATION_CONTEXT_PARAMS {
    pub Size: u32,
    pub Version: u32,
    pub Type: RESDLL_CONTEXT_OPERATION_TYPE,
    pub Priority: u32,
}
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRP_PLACEMENT_OPTIONS(pub i32);
pub const GRP_PLACEMENT_OPTIONS_ALL: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(1i32);
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: GRP_PLACEMENT_OPTIONS = GRP_PLACEMENT_OPTIONS(0i32);
pub const GUID_PRESENT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCHANGE(pub isize);
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCLUSCRYPTPROVIDER(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCLUSENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCLUSENUMEX(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCLUSTER(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGROUP(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGROUPENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGROUPENUMEX(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGROUPSET(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HGROUPSETENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNETINTERFACE(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNETINTERFACEENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNETWORK(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNETWORKENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNODE(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNODEENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HNODEENUMEX(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREGBATCH(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREGBATCHNOTIFICATION(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREGBATCHPORT(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREGREADBATCH(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HREGREADBATCHREPLY(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HRESENUM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HRESENUMEX(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HRESOURCE(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HRESTYPEENUM(pub isize);
windows_core::imp::define_interface!(IGetClusterDataInfo, IGetClusterDataInfo_Vtbl, 0x97dede51_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterDataInfo, windows_core::IUnknown);
impl IGetClusterDataInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClusterName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszname), pcchname as _).ok() }
    }
    pub unsafe fn GetClusterHandle(&self) -> HCLUSTER {
        unsafe { (windows_core::Interface::vtable(self).GetClusterHandle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterDataInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetClusterHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> HCLUSTER,
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
}
pub trait IGetClusterDataInfo_Impl: windows_core::IUnknownImpl {
    fn GetClusterName(&self, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetClusterHandle(&self) -> HCLUSTER;
    fn GetObjectCount(&self) -> i32;
}
impl IGetClusterDataInfo_Vtbl {
    pub const fn new<Identity: IGetClusterDataInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClusterName<Identity: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: *mut core::ffi::c_void, pcchname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterDataInfo_Impl::GetClusterName(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetClusterHandle<Identity: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> HCLUSTER {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterDataInfo_Impl::GetClusterHandle(this)
            }
        }
        unsafe extern "system" fn GetObjectCount<Identity: IGetClusterDataInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterDataInfo_Impl::GetObjectCount(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, OFFSET>,
            GetClusterHandle: GetClusterHandle::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterDataInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterDataInfo {}
windows_core::imp::define_interface!(IGetClusterGroupInfo, IGetClusterGroupInfo_Vtbl, 0x97dede54_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterGroupInfo, windows_core::IUnknown);
impl IGetClusterGroupInfo {
    pub unsafe fn GetGroupHandle(&self, lobjindex: i32) -> HGROUP {
        unsafe { (windows_core::Interface::vtable(self).GetGroupHandle)(windows_core::Interface::as_raw(self), lobjindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterGroupInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGroupHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HGROUP,
}
pub trait IGetClusterGroupInfo_Impl: windows_core::IUnknownImpl {
    fn GetGroupHandle(&self, lobjindex: i32) -> HGROUP;
}
impl IGetClusterGroupInfo_Vtbl {
    pub const fn new<Identity: IGetClusterGroupInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGroupHandle<Identity: IGetClusterGroupInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HGROUP {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterGroupInfo_Impl::GetGroupHandle(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetGroupHandle: GetGroupHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterGroupInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterGroupInfo {}
windows_core::imp::define_interface!(IGetClusterNetInterfaceInfo, IGetClusterNetInterfaceInfo_Vtbl, 0x97dede57_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterNetInterfaceInfo, windows_core::IUnknown);
impl IGetClusterNetInterfaceInfo {
    pub unsafe fn GetNetInterfaceHandle(&self, lobjindex: i32) -> HNETINTERFACE {
        unsafe { (windows_core::Interface::vtable(self).GetNetInterfaceHandle)(windows_core::Interface::as_raw(self), lobjindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNetInterfaceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNetInterfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNETINTERFACE,
}
pub trait IGetClusterNetInterfaceInfo_Impl: windows_core::IUnknownImpl {
    fn GetNetInterfaceHandle(&self, lobjindex: i32) -> HNETINTERFACE;
}
impl IGetClusterNetInterfaceInfo_Vtbl {
    pub const fn new<Identity: IGetClusterNetInterfaceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetInterfaceHandle<Identity: IGetClusterNetInterfaceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNETINTERFACE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterNetInterfaceInfo_Impl::GetNetInterfaceHandle(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNetInterfaceHandle: GetNetInterfaceHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNetInterfaceInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterNetInterfaceInfo {}
windows_core::imp::define_interface!(IGetClusterNetworkInfo, IGetClusterNetworkInfo_Vtbl, 0x97dede56_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterNetworkInfo, windows_core::IUnknown);
impl IGetClusterNetworkInfo {
    pub unsafe fn GetNetworkHandle(&self, lobjindex: i32) -> HNETWORK {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkHandle)(windows_core::Interface::as_raw(self), lobjindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNetworkInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNetworkHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNETWORK,
}
pub trait IGetClusterNetworkInfo_Impl: windows_core::IUnknownImpl {
    fn GetNetworkHandle(&self, lobjindex: i32) -> HNETWORK;
}
impl IGetClusterNetworkInfo_Vtbl {
    pub const fn new<Identity: IGetClusterNetworkInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNetworkHandle<Identity: IGetClusterNetworkInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNETWORK {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterNetworkInfo_Impl::GetNetworkHandle(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNetworkHandle: GetNetworkHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNetworkInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterNetworkInfo {}
windows_core::imp::define_interface!(IGetClusterNodeInfo, IGetClusterNodeInfo_Vtbl, 0x97dede53_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterNodeInfo, windows_core::IUnknown);
impl IGetClusterNodeInfo {
    pub unsafe fn GetNodeHandle(&self, lobjindex: i32) -> HNODE {
        unsafe { (windows_core::Interface::vtable(self).GetNodeHandle)(windows_core::Interface::as_raw(self), lobjindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterNodeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNodeHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HNODE,
}
pub trait IGetClusterNodeInfo_Impl: windows_core::IUnknownImpl {
    fn GetNodeHandle(&self, lobjindex: i32) -> HNODE;
}
impl IGetClusterNodeInfo_Vtbl {
    pub const fn new<Identity: IGetClusterNodeInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeHandle<Identity: IGetClusterNodeInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterNodeInfo_Impl::GetNodeHandle(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNodeHandle: GetNodeHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNodeInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterNodeInfo {}
windows_core::imp::define_interface!(IGetClusterObjectInfo, IGetClusterObjectInfo_Vtbl, 0x97dede52_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterObjectInfo, windows_core::IUnknown);
impl IGetClusterObjectInfo {
    pub unsafe fn GetObjectName(&self, lobjindex: i32, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpszname), pcchname as _).ok() }
    }
    pub unsafe fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetObjectType)(windows_core::Interface::as_raw(self), lobjindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterObjectInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> CLUADMEX_OBJECT_TYPE,
}
pub trait IGetClusterObjectInfo_Impl: windows_core::IUnknownImpl {
    fn GetObjectName(&self, lobjindex: i32, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE;
}
impl IGetClusterObjectInfo_Vtbl {
    pub const fn new<Identity: IGetClusterObjectInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectName<Identity: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpszname: *mut core::ffi::c_void, pcchname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterObjectInfo_Impl::GetObjectName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetObjectType<Identity: IGetClusterObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterObjectInfo_Impl::GetObjectType(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectName: GetObjectName::<Identity, OFFSET>,
            GetObjectType: GetObjectType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterObjectInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterObjectInfo {}
windows_core::imp::define_interface!(IGetClusterResourceInfo, IGetClusterResourceInfo_Vtbl, 0x97dede55_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterResourceInfo, windows_core::IUnknown);
impl IGetClusterResourceInfo {
    pub unsafe fn GetResourceHandle(&self, lobjindex: i32) -> HRESOURCE {
        unsafe { (windows_core::Interface::vtable(self).GetResourceHandle)(windows_core::Interface::as_raw(self), lobjindex) }
    }
    pub unsafe fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: &windows_core::BSTR, pcchrestypename: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResourceTypeName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpszrestypename), pcchrestypename as _).ok() }
    }
    pub unsafe fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: &windows_core::BSTR, pcchnetname: *mut u32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetResourceNetworkName)(windows_core::Interface::as_raw(self), lobjindex, core::mem::transmute_copy(lpsznetname), pcchnetname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterResourceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> HRESOURCE,
    pub GetResourceTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetResourceNetworkName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut u32) -> windows_core::BOOL,
}
pub trait IGetClusterResourceInfo_Impl: windows_core::IUnknownImpl {
    fn GetResourceHandle(&self, lobjindex: i32) -> HRESOURCE;
    fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: windows_core::BSTR, pcchrestypename: *mut i32) -> windows_core::Result<()>;
    fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: windows_core::BSTR, pcchnetname: *mut u32) -> windows_core::BOOL;
}
impl IGetClusterResourceInfo_Vtbl {
    pub const fn new<Identity: IGetClusterResourceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResourceHandle<Identity: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HRESOURCE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterResourceInfo_Impl::GetResourceHandle(this, core::mem::transmute_copy(&lobjindex))
            }
        }
        unsafe extern "system" fn GetResourceTypeName<Identity: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpszrestypename: *mut core::ffi::c_void, pcchrestypename: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterResourceInfo_Impl::GetResourceTypeName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpszrestypename), core::mem::transmute_copy(&pcchrestypename)).into()
            }
        }
        unsafe extern "system" fn GetResourceNetworkName<Identity: IGetClusterResourceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpsznetname: *mut core::ffi::c_void, pcchnetname: *mut u32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterResourceInfo_Impl::GetResourceNetworkName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpsznetname), core::mem::transmute_copy(&pcchnetname))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceHandle: GetResourceHandle::<Identity, OFFSET>,
            GetResourceTypeName: GetResourceTypeName::<Identity, OFFSET>,
            GetResourceNetworkName: GetResourceNetworkName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterResourceInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGetClusterResourceInfo {}
windows_core::imp::define_interface!(IGetClusterUIInfo, IGetClusterUIInfo_Vtbl, 0x97dede50_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IGetClusterUIInfo, windows_core::IUnknown);
impl IGetClusterUIInfo {
    pub unsafe fn GetClusterName(&self, lpszname: &windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClusterName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszname), pcchname as _).ok() }
    }
    pub unsafe fn GetLocale(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLocale)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT {
        unsafe { (windows_core::Interface::vtable(self).GetFont)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON {
        unsafe { (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetClusterUIInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClusterName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetLocale: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetFont: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetFont: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IGetClusterUIInfo_Impl: windows_core::IUnknownImpl {
    fn GetClusterName(&self, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetLocale(&self) -> u32;
    fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT;
    fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IGetClusterUIInfo_Vtbl {
    pub const fn new<Identity: IGetClusterUIInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClusterName<Identity: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: *mut core::ffi::c_void, pcchname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterUIInfo_Impl::GetClusterName(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
            }
        }
        unsafe extern "system" fn GetLocale<Identity: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterUIInfo_Impl::GetLocale(this)
            }
        }
        unsafe extern "system" fn GetFont<Identity: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterUIInfo_Impl::GetFont(this)
            }
        }
        unsafe extern "system" fn GetIcon<Identity: IGetClusterUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetClusterUIInfo_Impl::GetIcon(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterUIInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IGetClusterUIInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusApplication, ISClusApplication_Vtbl, 0xf2e606e6_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusApplication {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusApplication, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusApplication {
    pub unsafe fn DomainNames(&self) -> windows_core::Result<ISDomainNames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainNames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ClusterNames(&self, bstrdomainname: &windows_core::BSTR) -> windows_core::Result<ISClusterNames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClusterNames)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdomainname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenCluster(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenCluster)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclustername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DomainNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClusterNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenCluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusApplication_Impl: super::super::System::Com::IDispatch_Impl {
    fn DomainNames(&self) -> windows_core::Result<ISDomainNames>;
    fn get_ClusterNames(&self, bstrdomainname: &windows_core::BSTR) -> windows_core::Result<ISClusterNames>;
    fn OpenCluster(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusApplication_Vtbl {
    pub const fn new<Identity: ISClusApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DomainNames<Identity: ISClusApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomains: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusApplication_Impl::DomainNames(this) {
                    Ok(ok__) => {
                        ppdomains.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ClusterNames<Identity: ISClusApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomainname: *mut core::ffi::c_void, ppclusters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusApplication_Impl::get_ClusterNames(this, core::mem::transmute(&bstrdomainname)) {
                    Ok(ok__) => {
                        ppclusters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenCluster<Identity: ISClusApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: *mut core::ffi::c_void, pcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusApplication_Impl::OpenCluster(this, core::mem::transmute(&bstrclustername)) {
                    Ok(ok__) => {
                        pcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DomainNames: DomainNames::<Identity, OFFSET>,
            get_ClusterNames: get_ClusterNames::<Identity, OFFSET>,
            OpenCluster: OpenCluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusApplication {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusCryptoKeys, ISClusCryptoKeys_Vtbl, 0xf2e6072c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusCryptoKeys {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusCryptoKeys, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusCryptoKeys {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddItem(&self, bstrcryptokey: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcryptokey)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusCryptoKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusCryptoKeys_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn AddItem(&self, bstrcryptokey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusCryptoKeys_Vtbl {
    pub const fn new<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusCryptoKeys_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusCryptoKeys_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusCryptoKeys_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrcyrptokey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusCryptoKeys_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pbstrcyrptokey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcryptokey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusCryptoKeys_Impl::AddItem(this, core::mem::transmute(&bstrcryptokey)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusCryptoKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusCryptoKeys_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusCryptoKeys as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusCryptoKeys {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusDisk, ISClusDisk_Vtbl, 0xf2e60724_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusDisk {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusDisk, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisk {
    pub unsafe fn Signature(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Signature)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScsiAddress(&self) -> windows_core::Result<ISClusScsiAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScsiAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DiskNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiskNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Partitions(&self) -> windows_core::Result<ISClusPartitions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Partitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusDisk_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Signature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScsiAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiskNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Partitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusDisk_Impl: super::super::System::Com::IDispatch_Impl {
    fn Signature(&self) -> windows_core::Result<i32>;
    fn ScsiAddress(&self) -> windows_core::Result<ISClusScsiAddress>;
    fn DiskNumber(&self) -> windows_core::Result<i32>;
    fn Partitions(&self) -> windows_core::Result<ISClusPartitions>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusDisk_Vtbl {
    pub const fn new<Identity: ISClusDisk_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Signature<Identity: ISClusDisk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsignature: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisk_Impl::Signature(this) {
                    Ok(ok__) => {
                        plsignature.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScsiAddress<Identity: ISClusDisk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscsiaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisk_Impl::ScsiAddress(this) {
                    Ok(ok__) => {
                        ppscsiaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DiskNumber<Identity: ISClusDisk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldisknumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisk_Impl::DiskNumber(this) {
                    Ok(ok__) => {
                        pldisknumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Partitions<Identity: ISClusDisk_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppartitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisk_Impl::Partitions(this) {
                    Ok(ok__) => {
                        pppartitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Signature: Signature::<Identity, OFFSET>,
            ScsiAddress: ScsiAddress::<Identity, OFFSET>,
            DiskNumber: DiskNumber::<Identity, OFFSET>,
            Partitions: Partitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusDisk as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusDisk {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusDisks, ISClusDisks_Vtbl, 0xf2e60726_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusDisks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusDisks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisks {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusDisk> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusDisks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusDisks_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusDisk>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusDisks_Vtbl {
    pub const fn new<Identity: ISClusDisks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusDisks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisks_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusDisks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisks_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusDisks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusDisks_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppdisk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusDisks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusDisks {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetInterface, ISClusNetInterface_Vtbl, 0xf2e606ee_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetInterface {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetInterface, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterface {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NETINTERFACE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetInterface_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NETINTERFACE_STATE) -> windows_core::HRESULT,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetInterface_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn State(&self) -> windows_core::Result<CLUSTER_NETINTERFACE_STATE>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNetInterface_Vtbl {
    pub const fn new<Identity: ISClusNetInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::State(this) {
                    Ok(ok__) => {
                        dwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusNetInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterface_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetInterface as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNetInterface {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetInterfaces, ISClusNetInterfaces_Vtbl, 0xf2e606f0_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetInterfaces_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNetInterfaces_Vtbl {
    pub const fn new<Identity: ISClusNetInterfaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterfaces_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterfaces_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNetInterfaces_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusnetinterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetwork, ISClusNetwork_Vtbl, 0xf2e606f2_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetwork {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetwork, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetwork {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrnetworkname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnetworkname)).ok() }
    }
    pub unsafe fn NetworkID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NETWORK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNetworkNetInterfaces> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetwork_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NETWORK_STATE) -> windows_core::HRESULT,
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetwork_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrnetworkname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NetworkID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<CLUSTER_NETWORK_STATE>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNetworkNetInterfaces>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNetwork_Vtbl {
    pub const fn new<Identity: ISClusNetwork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnetworkname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNetwork_Impl::SetName(this, core::mem::transmute(&bstrnetworkname)).into()
            }
        }
        unsafe extern "system" fn NetworkID<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnetworkid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::NetworkID(this) {
                    Ok(ok__) => {
                        pbstrnetworkid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::State(this) {
                    Ok(ok__) => {
                        dwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::NetInterfaces(this) {
                    Ok(ok__) => {
                        ppclusnetinterfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetwork_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            NetworkID: NetworkID::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetwork as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNetwork {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetworkNetInterfaces, ISClusNetworkNetInterfaces_Vtbl, 0xf2e606f6_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetworkNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetworkNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetworkNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetworkNetInterfaces_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNetworkNetInterfaces_Vtbl {
    pub const fn new<Identity: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworkNetInterfaces_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworkNetInterfaces_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNetworkNetInterfaces_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusNetworkNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworkNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusnetinterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetworkNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNetworkNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNetworks, ISClusNetworks_Vtbl, 0xf2e606f4_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNetworks {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNetworks, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworks {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetwork> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNetworks_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNetworks_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetwork>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNetworks_Vtbl {
    pub const fn new<Identity: ISClusNetworks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworks_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworks_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNetworks_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusNetworks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNetworks_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusnetwork.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetworks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNetworks {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNode, ISClusNode_Vtbl, 0xf2e606f8_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNode {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNode, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNode {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NodeID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NodeID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_NODE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Evict(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Evict)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNodeNetInterfaces> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNode_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub NodeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_NODE_STATE) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Evict: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNode_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn NodeID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<CLUSTER_NODE_STATE>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Evict(&self) -> windows_core::Result<()>;
    fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNodeNetInterfaces>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNode_Vtbl {
    pub const fn new<Identity: ISClusNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NodeID<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnodeid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::NodeID(this) {
                    Ok(ok__) => {
                        pbstrnodeid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::State(this) {
                    Ok(ok__) => {
                        dwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNode_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNode_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Evict<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNode_Impl::Evict(this).into()
            }
        }
        unsafe extern "system" fn ResourceGroups<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::ResourceGroups(this) {
                    Ok(ok__) => {
                        ppresourcegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ISClusNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNode_Impl::NetInterfaces(this) {
                    Ok(ok__) => {
                        ppclusnetinterfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            NodeID: NodeID::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Evict: Evict::<Identity, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNode as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNode {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNodeNetInterfaces, ISClusNodeNetInterfaces_Vtbl, 0xf2e606fc_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNodeNetInterfaces {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNodeNetInterfaces, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeNetInterfaces {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNodeNetInterfaces_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNodeNetInterfaces_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNodeNetInterfaces_Vtbl {
    pub const fn new<Identity: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodeNetInterfaces_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodeNetInterfaces_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNodeNetInterfaces_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusNodeNetInterfaces_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodeNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusnetinterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNodeNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNodeNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusNodes, ISClusNodes_Vtbl, 0xf2e606fa_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusNodes_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusNodes_Vtbl {
    pub const fn new<Identity: ISClusNodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusNodes_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusNodes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartition, ISClusPartition_Vtbl, 0xf2e60720_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartition {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartition, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartition {
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeLabel(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeLabel)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MaximumComponentLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumComponentLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileSystemFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileSystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaximumComponentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FileSystemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartition_Impl: super::super::System::Com::IDispatch_Impl {
    fn Flags(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SerialNumber(&self) -> windows_core::Result<i32>;
    fn MaximumComponentLength(&self) -> windows_core::Result<i32>;
    fn FileSystemFlags(&self) -> windows_core::Result<i32>;
    fn FileSystem(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPartition_Vtbl {
    pub const fn new<Identity: ISClusPartition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Flags<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::Flags(this) {
                    Ok(ok__) => {
                        plflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::DeviceName(this) {
                    Ok(ok__) => {
                        pbstrdevicename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeLabel<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvolumelabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::VolumeLabel(this) {
                    Ok(ok__) => {
                        pbstrvolumelabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plserialnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::SerialNumber(this) {
                    Ok(ok__) => {
                        plserialnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaximumComponentLength<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::MaximumComponentLength(this) {
                    Ok(ok__) => {
                        plmaximumcomponentlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileSystemFlags<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfilesystemflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::FileSystemFlags(this) {
                    Ok(ok__) => {
                        plfilesystemflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileSystem<Identity: ISClusPartition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilesystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartition_Impl::FileSystem(this) {
                    Ok(ok__) => {
                        pbstrfilesystem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Flags: Flags::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            VolumeLabel: VolumeLabel::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            MaximumComponentLength: MaximumComponentLength::<Identity, OFFSET>,
            FileSystemFlags: FileSystemFlags::<Identity, OFFSET>,
            FileSystem: FileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPartition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartitionEx, ISClusPartitionEx_Vtbl, 0x8802d4fe_b32e_4ad1_9dbd_64f18e1166ce);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartitionEx {
    type Target = ISClusPartition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartitionEx, windows_core::IUnknown, super::super::System::Com::IDispatch, ISClusPartition);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionEx {
    pub unsafe fn TotalSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSpace(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PartitionNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PartitionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VolumeGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartitionEx_Vtbl {
    pub base__: ISClusPartition_Vtbl,
    pub TotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PartitionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VolumeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartitionEx_Impl: ISClusPartition_Impl {
    fn TotalSize(&self) -> windows_core::Result<i32>;
    fn FreeSpace(&self) -> windows_core::Result<i32>;
    fn DeviceNumber(&self) -> windows_core::Result<i32>;
    fn PartitionNumber(&self) -> windows_core::Result<i32>;
    fn VolumeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPartitionEx_Vtbl {
    pub const fn new<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TotalSize<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltotalsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitionEx_Impl::TotalSize(this) {
                    Ok(ok__) => {
                        pltotalsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSpace<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfreespace: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitionEx_Impl::FreeSpace(this) {
                    Ok(ok__) => {
                        plfreespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceNumber<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldevicenumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitionEx_Impl::DeviceNumber(this) {
                    Ok(ok__) => {
                        pldevicenumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PartitionNumber<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpartitionnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitionEx_Impl::PartitionNumber(this) {
                    Ok(ok__) => {
                        plpartitionnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeGuid<Identity: ISClusPartitionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvolumeguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitionEx_Impl::VolumeGuid(this) {
                    Ok(ok__) => {
                        pbstrvolumeguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISClusPartition_Vtbl::new::<Identity, OFFSET>(),
            TotalSize: TotalSize::<Identity, OFFSET>,
            FreeSpace: FreeSpace::<Identity, OFFSET>,
            DeviceNumber: DeviceNumber::<Identity, OFFSET>,
            PartitionNumber: PartitionNumber::<Identity, OFFSET>,
            VolumeGuid: VolumeGuid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartitionEx as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISClusPartition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPartitionEx {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPartitions, ISClusPartitions_Vtbl, 0xf2e60722_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPartitions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPartitions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitions {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPartition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPartitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPartitions_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPartition>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPartitions_Vtbl {
    pub const fn new<Identity: ISClusPartitions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitions_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitions_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusPartitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pppartition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPartitions_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pppartition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartitions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPartitions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusProperties, ISClusProperties_Vtbl, 0xf2e60700_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusProperties {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusProperties, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperties {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(varvalue), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UseDefaultValue(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UseDefaultValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Private(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Private)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Common(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Common)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UseDefaultValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UseDefaultValue: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SaveChanges: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReadOnly: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Private: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Private: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Common: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Common: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Modified: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusProperties_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusProperty>;
    fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusProperty>;
    fn UseDefaultValue(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SaveChanges(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Private(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Common(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusProperties_Vtbl {
    pub const fn new<Identity: ISClusProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperties_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT, pproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::CreateItem(this, core::mem::transmute(&bstrname), core::mem::transmute(&varvalue)) {
                    Ok(ok__) => {
                        pproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperties_Impl::UseDefaultValue(this, core::mem::transmute(&varindex)).into()
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarstatuscode: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::SaveChanges(this) {
                    Ok(ok__) => {
                        pvarstatuscode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreadonly: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        pvarreadonly.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Private<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprivate: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::Private(this) {
                    Ok(ok__) => {
                        pvarprivate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Common<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcommon: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::Common(this) {
                    Ok(ok__) => {
                        pvarcommon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Modified<Identity: ISClusProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperties_Impl::Modified(this) {
                    Ok(ok__) => {
                        pvarmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Private: Private::<Identity, OFFSET>,
            Common: Common::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusProperties {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusProperty, ISClusProperty_Vtbl, 0xf2e606fe_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ValueCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Values(&self) -> windows_core::Result<ISClusPropertyValues> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Values)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varvalue)).ok() }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Private(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Private)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Common(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Common)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UseDefaultValue(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UseDefaultValue)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ValueCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReadOnly: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Private: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Private: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Common: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Common: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Modified: usize,
    pub UseDefaultValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusProperty_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn ValueCount(&self) -> windows_core::Result<i32>;
    fn Values(&self) -> windows_core::Result<ISClusPropertyValues>;
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()>;
    fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Private(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Common(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn UseDefaultValue(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusProperty_Vtbl {
    pub const fn new<Identity: ISClusProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Length(this) {
                    Ok(ok__) => {
                        plength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValueCount<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::ValueCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Values<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterpropertyvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Values(this) {
                    Ok(ok__) => {
                        ppclusterpropertyvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperty_Impl::SetValue(this, core::mem::transmute(&varvalue)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Type(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperty_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Format<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Format(this) {
                    Ok(ok__) => {
                        pformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperty_Impl::SetFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreadonly: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::ReadOnly(this) {
                    Ok(ok__) => {
                        pvarreadonly.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Private<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprivate: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Private(this) {
                    Ok(ok__) => {
                        pvarprivate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Common<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcommon: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Common(this) {
                    Ok(ok__) => {
                        pvarcommon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Modified<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusProperty_Impl::Modified(this) {
                    Ok(ok__) => {
                        pvarmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: ISClusProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusProperty_Impl::UseDefaultValue(this).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            ValueCount: ValueCount::<Identity, OFFSET>,
            Values: Values::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Private: Private::<Identity, OFFSET>,
            Common: Common::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusProperty {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValue, ISClusPropertyValue_Vtbl, 0xf2e6071a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValue {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varvalue)).ok() }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
    pub unsafe fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Data(&self) -> windows_core::Result<ISClusPropertyValueData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValue_Impl: super::super::System::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetValue(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()>;
    fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn DataCount(&self) -> windows_core::Result<i32>;
    fn Data(&self) -> windows_core::Result<ISClusPropertyValueData>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPropertyValue_Vtbl {
    pub const fn new<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::Value(this) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusPropertyValue_Impl::SetValue(this, core::mem::transmute(&varvalue)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::Type(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusPropertyValue_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Format<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::Format(this) {
                    Ok(ok__) => {
                        pformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusPropertyValue_Impl::SetFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn Length<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::Length(this) {
                    Ok(ok__) => {
                        plength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataCount<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::DataCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: ISClusPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterpropertyvaluedata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValue_Impl::Data(this) {
                    Ok(ok__) => {
                        ppclusterpropertyvaluedata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            DataCount: DataCount::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPropertyValue {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValueData, ISClusPropertyValueData_Vtbl, 0xf2e6071e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValueData {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValueData, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueData {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateItem(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varvalue), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValueData_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValueData_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CreateItem(&self, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPropertyValueData_Vtbl {
    pub const fn new<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValueData_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValueData_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pvarvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValueData_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT, pvardata: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValueData_Impl::CreateItem(this, core::mem::transmute(&varvalue)) {
                    Ok(ok__) => {
                        pvardata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusPropertyValueData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusPropertyValueData_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValueData as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPropertyValueData {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusPropertyValues, ISClusPropertyValues_Vtbl, 0xf2e6071c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusPropertyValues {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusPropertyValues, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValues {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPropertyValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPropertyValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(varvalue), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusPropertyValues_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusPropertyValues_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPropertyValue>;
    fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusPropertyValue>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusPropertyValues_Vtbl {
    pub const fn new<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValues_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValues_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pppropertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValues_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pppropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, varvalue: super::super::System::Variant::VARIANT, pppropertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusPropertyValues_Impl::CreateItem(this, core::mem::transmute(&bstrname), core::mem::transmute(&varvalue)) {
                    Ok(ok__) => {
                        pppropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusPropertyValues_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusPropertyValues_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValues as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusPropertyValues {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusRefObject, ISClusRefObject_Vtbl, 0xf2e60702_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusRefObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusRefObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRefObject {
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusRefObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusRefObject_Impl: super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<usize>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusRefObject_Vtbl {
    pub const fn new<Identity: ISClusRefObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handle<Identity: ISClusRefObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusRefObject_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Handle: Handle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusRefObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusRefObject {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusRegistryKeys, ISClusRegistryKeys_Vtbl, 0xf2e6072a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusRegistryKeys {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusRegistryKeys, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusRegistryKeys {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddItem(&self, bstrregistrykey: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrregistrykey)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusRegistryKeys_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusRegistryKeys_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn AddItem(&self, bstrregistrykey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusRegistryKeys_Vtbl {
    pub const fn new<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusRegistryKeys_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusRegistryKeys_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusRegistryKeys_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrregistrykey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusRegistryKeys_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pbstrregistrykey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrregistrykey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusRegistryKeys_Impl::AddItem(this, core::mem::transmute(&bstrregistrykey)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusRegistryKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusRegistryKeys_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusRegistryKeys as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusRegistryKeys {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResDependencies, ISClusResDependencies_Vtbl, 0xf2e60704_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResDependencies {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResDependencies, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependencies {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename), core::mem::transmute_copy(bstrresourcetype), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), presource.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResDependencies_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResDependencies_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddItem(&self, presource: windows_core::Ref<ISClusResource>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResDependencies_Vtbl {
    pub const fn new<Identity: ISClusResDependencies_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependencies_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependencies_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependencies_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependencies_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void, bstrresourcetype: *mut core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependencies_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependencies_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependencies_Impl::AddItem(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusResDependencies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependencies_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResDependencies as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResDependencies {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResDependents, ISClusResDependents_Vtbl, 0xf2e6072e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResDependents {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResDependents, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependents {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename), core::mem::transmute_copy(bstrresourcetype), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
    pub unsafe fn AddItem<P0>(&self, presource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), presource.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResDependents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResDependents_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn AddItem(&self, presource: windows_core::Ref<ISClusResource>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResDependents_Vtbl {
    pub const fn new<Identity: ISClusResDependents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependents_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependents_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependents_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependents_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void, bstrresourcetype: *mut core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResDependents_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependents_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependents_Impl::AddItem(this, core::mem::transmute_copy(&presource)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusResDependents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResDependents_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResDependents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResDependents {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroup, ISClusResGroup_Vtbl, 0xf2e60706_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroup {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroup, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroup {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupname)).ok() }
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_GROUP_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OwnerNode(&self) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResGroupResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PreferredOwnerNodes(&self) -> windows_core::Result<ISClusResGroupPreferredOwnerNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Online(&self, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vartimeout), core::mem::transmute_copy(varnode), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Move(&self, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vartimeout), core::mem::transmute_copy(varnode), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Offline(&self, vartimeout: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vartimeout), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_GROUP_STATE) -> windows_core::HRESULT,
    pub OwnerNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreferredOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Online: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Move: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Offline: usize,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroup_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<CLUSTER_GROUP_STATE>;
    fn OwnerNode(&self) -> windows_core::Result<ISClusNode>;
    fn Resources(&self) -> windows_core::Result<ISClusResGroupResources>;
    fn PreferredOwnerNodes(&self) -> windows_core::Result<ISClusResGroupPreferredOwnerNodes>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Online(&self, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Move(&self, vartimeout: &super::super::System::Variant::VARIANT, varnode: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Offline(&self, vartimeout: &super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResGroup_Vtbl {
    pub const fn new<Identity: ISClusResGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroup_Impl::SetName(this, core::mem::transmute(&bstrgroupname)).into()
            }
        }
        unsafe extern "system" fn State<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::State(this) {
                    Ok(ok__) => {
                        dwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::OwnerNode(this) {
                    Ok(ok__) => {
                        ppownernode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resources<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclustergroupresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Resources(this) {
                    Ok(ok__) => {
                        ppclustergroupresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreferredOwnerNodes<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::PreferredOwnerNodes(this) {
                    Ok(ok__) => {
                        ppownernodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroup_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Online<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, varnode: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Online(this, core::mem::transmute(&vartimeout), core::mem::transmute(&varnode)) {
                    Ok(ok__) => {
                        pvarpending.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, varnode: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Move(this, core::mem::transmute(&vartimeout), core::mem::transmute(&varnode)) {
                    Ok(ok__) => {
                        pvarpending.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Offline<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: super::super::System::Variant::VARIANT, pvarpending: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Offline(this, core::mem::transmute(&vartimeout)) {
                    Ok(ok__) => {
                        pvarpending.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusResGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroup_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            OwnerNode: OwnerNode::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            PreferredOwnerNodes: PreferredOwnerNodes::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResGroup {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroupPreferredOwnerNodes, ISClusResGroupPreferredOwnerNodes_Vtbl, 0xf2e606e8_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroupPreferredOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroupPreferredOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupPreferredOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertItem<P0>(&self, pnode: P0, nposition: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertItem)(windows_core::Interface::as_raw(self), pnode.param().abi(), nposition).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SaveChanges(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveChanges)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub InsertItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Modified: usize,
    pub SaveChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroupPreferredOwnerNodes_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode>;
    fn InsertItem(&self, pnode: windows_core::Ref<ISClusNode>, nposition: i32) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SaveChanges(&self) -> windows_core::Result<()>;
    fn AddItem(&self, pnode: windows_core::Ref<ISClusNode>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub const fn new<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupPreferredOwnerNodes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupPreferredOwnerNodes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupPreferredOwnerNodes_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupPreferredOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertItem<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, nposition: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupPreferredOwnerNodes_Impl::InsertItem(this, core::mem::transmute_copy(&pnode), core::mem::transmute_copy(&nposition)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupPreferredOwnerNodes_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        unsafe extern "system" fn Modified<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupPreferredOwnerNodes_Impl::Modified(this) {
                    Ok(ok__) => {
                        pvarmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupPreferredOwnerNodes_Impl::SaveChanges(this).into()
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusResGroupPreferredOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupPreferredOwnerNodes_Impl::AddItem(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            InsertItem: InsertItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroupPreferredOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResGroupPreferredOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroupResources, ISClusResGroupResources_Vtbl, 0xf2e606ea_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroupResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroupResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename), core::mem::transmute_copy(bstrresourcetype), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroupResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroupResources_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResGroupResources_Vtbl {
    pub const fn new<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupResources_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupResources_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupResources_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void, bstrresourcetype: *mut core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroupResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResGroupResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroupResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroupResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResGroupResources {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResGroups, ISClusResGroups_Vtbl, 0xf2e60708_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResGroups {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResGroups, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroups {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcegroupname: &windows_core::BSTR) -> windows_core::Result<ISClusResGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcegroupname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResGroups_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResGroups_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResGroup>;
    fn CreateItem(&self, bstrresourcegroupname: &windows_core::BSTR) -> windows_core::Result<ISClusResGroup>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResGroups_Vtbl {
    pub const fn new<Identity: ISClusResGroups_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroups_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroups_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroups_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroups_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcegroupname: *mut core::ffi::c_void, ppresourcegroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResGroups_Impl::CreateItem(this, core::mem::transmute(&bstrresourcegroupname)) {
                    Ok(ok__) => {
                        ppresourcegroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResGroups_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResGroups_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroups as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResGroups {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResPossibleOwnerNodes, ISClusResPossibleOwnerNodes_Vtbl, 0xf2e6070e_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResPossibleOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResPossibleOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResPossibleOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddItem<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResPossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RemoveItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RemoveItem: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Modified: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResPossibleOwnerNodes_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode>;
    fn AddItem(&self, pnode: windows_core::Ref<ISClusNode>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResPossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResPossibleOwnerNodes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResPossibleOwnerNodes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResPossibleOwnerNodes_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResPossibleOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddItem<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResPossibleOwnerNodes_Impl::AddItem(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResPossibleOwnerNodes_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        unsafe extern "system" fn Modified<Identity: ISClusResPossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResPossibleOwnerNodes_Impl::Modified(this) {
                    Ok(ok__) => {
                        pvarmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResPossibleOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResPossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResType, ISClusResType_Vtbl, 0xf2e60710_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResType {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResType, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResType {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResTypeResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResTypePossibleOwnerNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PossibleOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AvailableDisks(&self) -> windows_core::Result<ISClusDisks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AvailableDisks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResType_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PossibleOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AvailableDisks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResType_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn Resources(&self) -> windows_core::Result<ISClusResTypeResources>;
    fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResTypePossibleOwnerNodes>;
    fn AvailableDisks(&self) -> windows_core::Result<ISClusDisks>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResType_Vtbl {
    pub const fn new<Identity: ISClusResType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResType_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resources<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterrestyperesources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::Resources(this) {
                    Ok(ok__) => {
                        ppclusterrestyperesources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::PossibleOwnerNodes(this) {
                    Ok(ok__) => {
                        ppownernodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AvailableDisks<Identity: ISClusResType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppavailabledisks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResType_Impl::AvailableDisks(this) {
                    Ok(ok__) => {
                        ppavailabledisks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, OFFSET>,
            AvailableDisks: AvailableDisks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResType as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResType {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypePossibleOwnerNodes, ISClusResTypePossibleOwnerNodes_Vtbl, 0xf2e60718_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypePossibleOwnerNodes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypePossibleOwnerNodes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypePossibleOwnerNodes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypePossibleOwnerNodes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypePossibleOwnerNodes_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusNode>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResTypePossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypePossibleOwnerNodes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypePossibleOwnerNodes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResTypePossibleOwnerNodes_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResTypePossibleOwnerNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypePossibleOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypePossibleOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResTypePossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypeResources, ISClusResTypeResources_Vtbl, 0xf2e60714_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypeResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypeResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename), core::mem::transmute_copy(bstrgroupname), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypeResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypeResources_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResTypeResources_Vtbl {
    pub const fn new<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypeResources_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypeResources_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResTypeResources_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypeResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypeResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResTypeResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResTypeResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypeResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResTypeResources {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResTypes, ISClusResTypes_Vtbl, 0xf2e60712_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResTypes {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResTypes, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypes {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcetypename: &windows_core::BSTR, bstrdisplayname: &windows_core::BSTR, bstrresourcetypedll: &windows_core::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> windows_core::Result<ISClusResType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcetypename), core::mem::transmute_copy(bstrdisplayname), core::mem::transmute_copy(bstrresourcetypedll), dwlooksalivepollinterval, dwisalivepollinterval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResTypes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResTypes_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResType>;
    fn CreateItem(&self, bstrresourcetypename: &windows_core::BSTR, bstrdisplayname: &windows_core::BSTR, bstrresourcetypedll: &windows_core::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> windows_core::Result<ISClusResType>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResTypes_Vtbl {
    pub const fn new<Identity: ISClusResTypes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypes_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypes_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResTypes_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusrestype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusrestype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcetypename: *mut core::ffi::c_void, bstrdisplayname: *mut core::ffi::c_void, bstrresourcetypedll: *mut core::ffi::c_void, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResTypes_Impl::CreateItem(this, core::mem::transmute(&bstrresourcetypename), core::mem::transmute(&bstrdisplayname), core::mem::transmute(&bstrresourcetypedll), core::mem::transmute_copy(&dwlooksalivepollinterval), core::mem::transmute_copy(&dwisalivepollinterval)) {
                    Ok(ok__) => {
                        ppresourcetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResTypes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResTypes_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResTypes {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResource, ISClusResource_Vtbl, 0xf2e6070a_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResource {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResource, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResource {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrresourcename: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename)).ok() }
    }
    pub unsafe fn State(&self) -> windows_core::Result<CLUSTER_RESOURCE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CoreFlag(&self) -> windows_core::Result<CLUS_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoreFlag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BecomeQuorumResource(&self, bstrdevicepath: &windows_core::BSTR, lmaxlogsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BecomeQuorumResource)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdevicepath), lmaxlogsize).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Fail(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Fail)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Online(&self, ntimeout: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), ntimeout, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Offline(&self, ntimeout: i32) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Offline)(windows_core::Interface::as_raw(self), ntimeout, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ChangeResourceGroup<P0>(&self, presourcegroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResGroup>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChangeResourceGroup)(windows_core::Interface::as_raw(self), presourcegroup.param().abi()).ok() }
    }
    pub unsafe fn AddResourceNode<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddResourceNode)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok() }
    }
    pub unsafe fn RemoveResourceNode<P0>(&self, pnode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveResourceNode)(windows_core::Interface::as_raw(self), pnode.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanResourceBeDependent<P0>(&self, presource: P0) -> windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanResourceBeDependent)(windows_core::Interface::as_raw(self), presource.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResPossibleOwnerNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PossibleOwnerNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Dependencies(&self) -> windows_core::Result<ISClusResDependencies> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Dependents(&self) -> windows_core::Result<ISClusResDependents> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dependents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Group(&self) -> windows_core::Result<ISClusResGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OwnerNode(&self) -> windows_core::Result<ISClusNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Cluster(&self) -> windows_core::Result<ISCluster> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cluster)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ClassInfo(&self) -> windows_core::Result<CLUSTER_RESOURCE_CLASS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Disk(&self) -> windows_core::Result<ISClusDisk> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Disk)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegistryKeys(&self) -> windows_core::Result<ISClusRegistryKeys> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CryptoKeys(&self) -> windows_core::Result<ISClusCryptoKeys> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CryptoKeys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<ISClusResType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MaintenanceMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaintenanceMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaintenanceMode(&self, bmaintenancemode: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaintenanceMode)(windows_core::Interface::as_raw(self), bmaintenancemode.into()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResource_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_RESOURCE_STATE) -> windows_core::HRESULT,
    pub CoreFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUS_FLAGS) -> windows_core::HRESULT,
    pub BecomeQuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Fail: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Online: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Offline: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Offline: usize,
    pub ChangeResourceGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddResourceNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveResourceNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CanResourceBeDependent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CanResourceBeDependent: usize,
    pub PossibleOwnerNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dependents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OwnerNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cluster: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLUSTER_RESOURCE_CLASS) -> windows_core::HRESULT,
    pub Disk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CryptoKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaintenanceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMaintenanceMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResource_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrresourcename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<CLUSTER_RESOURCE_STATE>;
    fn CoreFlag(&self) -> windows_core::Result<CLUS_FLAGS>;
    fn BecomeQuorumResource(&self, bstrdevicepath: &windows_core::BSTR, lmaxlogsize: i32) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Fail(&self) -> windows_core::Result<()>;
    fn Online(&self, ntimeout: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Offline(&self, ntimeout: i32) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn ChangeResourceGroup(&self, presourcegroup: windows_core::Ref<ISClusResGroup>) -> windows_core::Result<()>;
    fn AddResourceNode(&self, pnode: windows_core::Ref<ISClusNode>) -> windows_core::Result<()>;
    fn RemoveResourceNode(&self, pnode: windows_core::Ref<ISClusNode>) -> windows_core::Result<()>;
    fn CanResourceBeDependent(&self, presource: windows_core::Ref<ISClusResource>) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResPossibleOwnerNodes>;
    fn Dependencies(&self) -> windows_core::Result<ISClusResDependencies>;
    fn Dependents(&self) -> windows_core::Result<ISClusResDependents>;
    fn Group(&self) -> windows_core::Result<ISClusResGroup>;
    fn OwnerNode(&self) -> windows_core::Result<ISClusNode>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn ClassInfo(&self) -> windows_core::Result<CLUSTER_RESOURCE_CLASS>;
    fn Disk(&self) -> windows_core::Result<ISClusDisk>;
    fn RegistryKeys(&self) -> windows_core::Result<ISClusRegistryKeys>;
    fn CryptoKeys(&self) -> windows_core::Result<ISClusCryptoKeys>;
    fn TypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<ISClusResType>;
    fn MaintenanceMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetMaintenanceMode(&self, bmaintenancemode: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResource_Vtbl {
    pub const fn new<Identity: ISClusResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::SetName(this, core::mem::transmute(&bstrresourcename)).into()
            }
        }
        unsafe extern "system" fn State<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::State(this) {
                    Ok(ok__) => {
                        dwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CoreFlag<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::CoreFlag(this) {
                    Ok(ok__) => {
                        dwcoreflag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BecomeQuorumResource<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicepath: *mut core::ffi::c_void, lmaxlogsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::BecomeQuorumResource(this, core::mem::transmute(&bstrdevicepath), core::mem::transmute_copy(&lmaxlogsize)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Fail<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::Fail(this).into()
            }
        }
        unsafe extern "system" fn Online<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Online(this, core::mem::transmute_copy(&ntimeout)) {
                    Ok(ok__) => {
                        pvarpending.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Offline<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntimeout: i32, pvarpending: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Offline(this, core::mem::transmute_copy(&ntimeout)) {
                    Ok(ok__) => {
                        pvarpending.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChangeResourceGroup<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcegroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::ChangeResourceGroup(this, core::mem::transmute_copy(&presourcegroup)).into()
            }
        }
        unsafe extern "system" fn AddResourceNode<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::AddResourceNode(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn RemoveResourceNode<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::RemoveResourceNode(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn CanResourceBeDependent<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pvardependent: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::CanResourceBeDependent(this, core::mem::transmute_copy(&presource)) {
                    Ok(ok__) => {
                        pvardependent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::PossibleOwnerNodes(this) {
                    Ok(ok__) => {
                        ppownernodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Dependencies<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Dependencies(this) {
                    Ok(ok__) => {
                        ppresdependencies.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Dependents<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresdependents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Dependents(this) {
                    Ok(ok__) => {
                        ppresdependents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Group<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Group(this) {
                    Ok(ok__) => {
                        ppresgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::OwnerNode(this) {
                    Ok(ok__) => {
                        ppownernode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cluster<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Cluster(this) {
                    Ok(ok__) => {
                        ppcluster.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassInfo<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::ClassInfo(this) {
                    Ok(ok__) => {
                        prcclassinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disk<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Disk(this) {
                    Ok(ok__) => {
                        ppdisk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegistryKeys<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrykeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::RegistryKeys(this) {
                    Ok(ok__) => {
                        ppregistrykeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CryptoKeys<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcryptokeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::CryptoKeys(this) {
                    Ok(ok__) => {
                        ppcryptokeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TypeName<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtypename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::TypeName(this) {
                    Ok(ok__) => {
                        pbstrtypename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::Type(this) {
                    Ok(ok__) => {
                        ppresourcetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaintenanceMode<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmaintenancemode: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResource_Impl::MaintenanceMode(this) {
                    Ok(ok__) => {
                        pbmaintenancemode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaintenanceMode<Identity: ISClusResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmaintenancemode: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResource_Impl::SetMaintenanceMode(this, core::mem::transmute_copy(&bmaintenancemode)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            CoreFlag: CoreFlag::<Identity, OFFSET>,
            BecomeQuorumResource: BecomeQuorumResource::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Fail: Fail::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            ChangeResourceGroup: ChangeResourceGroup::<Identity, OFFSET>,
            AddResourceNode: AddResourceNode::<Identity, OFFSET>,
            RemoveResourceNode: RemoveResourceNode::<Identity, OFFSET>,
            CanResourceBeDependent: CanResourceBeDependent::<Identity, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, OFFSET>,
            Dependencies: Dependencies::<Identity, OFFSET>,
            Dependents: Dependents::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            OwnerNode: OwnerNode::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            ClassInfo: ClassInfo::<Identity, OFFSET>,
            Disk: Disk::<Identity, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, OFFSET>,
            CryptoKeys: CryptoKeys::<Identity, OFFSET>,
            TypeName: TypeName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            MaintenanceMode: MaintenanceMode::<Identity, OFFSET>,
            SetMaintenanceMode: SetMaintenanceMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResource as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResource {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusResources, ISClusResources_Vtbl, 0xf2e6070c_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusResources {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusResources, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusResources {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrresourcename), core::mem::transmute_copy(bstrresourcetype), core::mem::transmute_copy(bstrgroupname), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusResources_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub CreateItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CLUSTER_RESOURCE_CREATE_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DeleteItem: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusResources_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusResources_Vtbl {
    pub const fn new<Identity: ISClusResources_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResources_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResources_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResources_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        ppclusresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateItem<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: *mut core::ffi::c_void, bstrresourcetype: *mut core::ffi::c_void, bstrgroupname: *mut core::ffi::c_void, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: ISClusResources_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusResources {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusScsiAddress, ISClusScsiAddress_Vtbl, 0xf2e60728_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusScsiAddress {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusScsiAddress, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusScsiAddress {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PortNumber(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PortNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PathId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PathId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TargetId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Lun(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lun)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusScsiAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PortNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PortNumber: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PathId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PathId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TargetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TargetId: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Lun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Lun: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusScsiAddress_Impl: super::super::System::Com::IDispatch_Impl {
    fn PortNumber(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PathId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn TargetId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Lun(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusScsiAddress_Vtbl {
    pub const fn new<Identity: ISClusScsiAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PortNumber<Identity: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarportnumber: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusScsiAddress_Impl::PortNumber(this) {
                    Ok(ok__) => {
                        pvarportnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PathId<Identity: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarpathid: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusScsiAddress_Impl::PathId(this) {
                    Ok(ok__) => {
                        pvarpathid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetId<Identity: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvartargetid: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusScsiAddress_Impl::TargetId(this) {
                    Ok(ok__) => {
                        pvartargetid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Lun<Identity: ISClusScsiAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlun: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusScsiAddress_Impl::Lun(this) {
                    Ok(ok__) => {
                        pvarlun.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PortNumber: PortNumber::<Identity, OFFSET>,
            PathId: PathId::<Identity, OFFSET>,
            TargetId: TargetId::<Identity, OFFSET>,
            Lun: Lun::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusScsiAddress as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusScsiAddress {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusVersion, ISClusVersion_Vtbl, 0xf2e60716_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusVersion {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusVersion, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusVersion {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    pub unsafe fn BuildNumber(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VendorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CSDVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CSDVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClusterHighestVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClusterHighestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClusterLowestVersion(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClusterLowestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MixedVersion(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MixedVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusVersion_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BuildNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CSDVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClusterHighestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ClusterLowestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MixedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MixedVersion: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusVersion_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn BuildNumber(&self) -> windows_core::Result<i16>;
    fn VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CSDVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClusterHighestVersion(&self) -> windows_core::Result<i32>;
    fn ClusterLowestVersion(&self) -> windows_core::Result<i32>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn MixedVersion(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusVersion_Vtbl {
    pub const fn new<Identity: ISClusVersion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrclustername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrclustername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnmajorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::MajorVersion(this) {
                    Ok(ok__) => {
                        pnmajorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::MinorVersion(this) {
                    Ok(ok__) => {
                        pnminorversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BuildNumber<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnbuildnumber: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::BuildNumber(this) {
                    Ok(ok__) => {
                        pnbuildnumber.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VendorId<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvendorid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::VendorId(this) {
                    Ok(ok__) => {
                        pbstrvendorid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CSDVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsdversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::CSDVersion(this) {
                    Ok(ok__) => {
                        pbstrcsdversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClusterHighestVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnclusterhighestversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::ClusterHighestVersion(this) {
                    Ok(ok__) => {
                        pnclusterhighestversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClusterLowestVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnclusterlowestversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::ClusterLowestVersion(this) {
                    Ok(ok__) => {
                        pnclusterlowestversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Flags<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::Flags(this) {
                    Ok(ok__) => {
                        pnflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MixedVersion<Identity: ISClusVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmixedversion: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusVersion_Impl::MixedVersion(this) {
                    Ok(ok__) => {
                        pvarmixedversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            BuildNumber: BuildNumber::<Identity, OFFSET>,
            VendorId: VendorId::<Identity, OFFSET>,
            CSDVersion: CSDVersion::<Identity, OFFSET>,
            ClusterHighestVersion: ClusterHighestVersion::<Identity, OFFSET>,
            ClusterLowestVersion: ClusterLowestVersion::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            MixedVersion: MixedVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusVersion as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusVersion {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISCluster, ISCluster_Vtbl, 0xf2e606e4_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISCluster {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISCluster, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISCluster {
    pub unsafe fn CommonProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrivateROProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Open(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclustername)).ok() }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclustername)).ok() }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<ISClusVersion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetQuorumResource<P0>(&self, pclusterresource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISClusResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetQuorumResource)(windows_core::Interface::as_raw(self), pclusterresource.param().abi()).ok() }
    }
    pub unsafe fn QuorumResource(&self) -> windows_core::Result<ISClusResource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuorumResource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QuorumLogSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuorumLogSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuorumLogSize(&self, nlogsize: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuorumLogSize)(windows_core::Interface::as_raw(self), nlogsize).ok() }
    }
    pub unsafe fn QuorumPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuorumPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetQuorumPath(&self, ppath: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetQuorumPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(ppath)).ok() }
    }
    pub unsafe fn Nodes(&self) -> windows_core::Result<ISClusNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Nodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Resources(&self) -> windows_core::Result<ISClusResources> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ResourceTypes(&self) -> windows_core::Result<ISClusResTypes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Networks(&self) -> windows_core::Result<ISClusNetworks> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Networks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NetInterfaces(&self) -> windows_core::Result<ISClusNetInterfaces> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetInterfaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISCluster_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CommonProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrivateROProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuorumResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuorumLogSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetQuorumLogSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub QuorumPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuorumPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Nodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Networks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISCluster_Impl: super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Open(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<ISClusVersion>;
    fn SetQuorumResource(&self, pclusterresource: windows_core::Ref<ISClusResource>) -> windows_core::Result<()>;
    fn QuorumResource(&self) -> windows_core::Result<ISClusResource>;
    fn QuorumLogSize(&self) -> windows_core::Result<i32>;
    fn SetQuorumLogSize(&self, nlogsize: i32) -> windows_core::Result<()>;
    fn QuorumPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetQuorumPath(&self, ppath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Nodes(&self) -> windows_core::Result<ISClusNodes>;
    fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups>;
    fn Resources(&self) -> windows_core::Result<ISClusResources>;
    fn ResourceTypes(&self) -> windows_core::Result<ISClusResTypes>;
    fn Networks(&self) -> windows_core::Result<ISClusNetworks>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNetInterfaces>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISCluster_Vtbl {
    pub const fn new<Identity: ISCluster_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommonProperties<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::CommonProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::PrivateProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::CommonROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::PrivateROProperties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handle<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Handle(this) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Open<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCluster_Impl::Open(this, core::mem::transmute(&bstrclustername)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCluster_Impl::SetName(this, core::mem::transmute(&bstrclustername)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Version(this) {
                    Ok(ok__) => {
                        ppclusversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuorumResource<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclusterresource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCluster_Impl::SetQuorumResource(this, core::mem::transmute_copy(&pclusterresource)).into()
            }
        }
        unsafe extern "system" fn QuorumResource<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::QuorumResource(this) {
                    Ok(ok__) => {
                        pclusterresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuorumLogSize<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlogsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::QuorumLogSize(this) {
                    Ok(ok__) => {
                        pnlogsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuorumLogSize<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlogsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCluster_Impl::SetQuorumLogSize(this, core::mem::transmute_copy(&nlogsize)).into()
            }
        }
        unsafe extern "system" fn QuorumPath<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::QuorumPath(this) {
                    Ok(ok__) => {
                        pppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuorumPath<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISCluster_Impl::SetQuorumPath(this, core::mem::transmute(&ppath)).into()
            }
        }
        unsafe extern "system" fn Nodes<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Nodes(this) {
                    Ok(ok__) => {
                        ppnodes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResourceGroups<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterresourcegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::ResourceGroups(this) {
                    Ok(ok__) => {
                        ppclusterresourcegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resources<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Resources(this) {
                    Ok(ok__) => {
                        ppclusterresources.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResourceTypes<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::ResourceTypes(this) {
                    Ok(ok__) => {
                        ppresourcetypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Networks<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::Networks(this) {
                    Ok(ok__) => {
                        ppnetworks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: ISCluster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISCluster_Impl::NetInterfaces(this) {
                    Ok(ok__) => {
                        ppnetinterfaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetQuorumResource: SetQuorumResource::<Identity, OFFSET>,
            QuorumResource: QuorumResource::<Identity, OFFSET>,
            QuorumLogSize: QuorumLogSize::<Identity, OFFSET>,
            SetQuorumLogSize: SetQuorumLogSize::<Identity, OFFSET>,
            QuorumPath: QuorumPath::<Identity, OFFSET>,
            SetQuorumPath: SetQuorumPath::<Identity, OFFSET>,
            Nodes: Nodes::<Identity, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            ResourceTypes: ResourceTypes::<Identity, OFFSET>,
            Networks: Networks::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCluster as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISCluster {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISClusterNames, ISClusterNames_Vtbl, 0xf2e606ec_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISClusterNames {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISClusterNames, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISClusterNames {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DomainName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISClusterNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISClusterNames_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISClusterNames_Vtbl {
    pub const fn new<Identity: ISClusterNames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISClusterNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusterNames_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISClusterNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusterNames_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISClusterNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISClusterNames_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISClusterNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrclustername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusterNames_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pbstrclustername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainName<Identity: ISClusterNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomainname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISClusterNames_Impl::DomainName(this) {
                    Ok(ok__) => {
                        pbstrdomainname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            DomainName: DomainName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusterNames as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISClusterNames {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISDomainNames, ISDomainNames_Vtbl, 0xf2e606e2_2631_11d1_89f1_00a0c90d061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISDomainNames {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISDomainNames, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISDomainNames {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISDomainNames_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISDomainNames_Impl: super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISDomainNames_Vtbl {
    pub const fn new<Identity: ISDomainNames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ISDomainNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISDomainNames_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISDomainNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISDomainNames_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISDomainNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISDomainNames_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ISDomainNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pbstrdomainname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISDomainNames_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                    Ok(ok__) => {
                        pbstrdomainname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISDomainNames as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISDomainNames {}
windows_core::imp::define_interface!(IWCContextMenuCallback, IWCContextMenuCallback_Vtbl, 0x97dede64_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWCContextMenuCallback, windows_core::IUnknown);
impl IWCContextMenuCallback {
    pub unsafe fn AddExtensionMenuItem(&self, lpszname: &windows_core::BSTR, lpszstatusbartext: &windows_core::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddExtensionMenuItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(lpszname), core::mem::transmute_copy(lpszstatusbartext), ncommandid, nsubmenucommandid, uflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCContextMenuCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddExtensionMenuItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
}
pub trait IWCContextMenuCallback_Impl: windows_core::IUnknownImpl {
    fn AddExtensionMenuItem(&self, lpszname: &windows_core::BSTR, lpszstatusbartext: &windows_core::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::Result<()>;
}
impl IWCContextMenuCallback_Vtbl {
    pub const fn new<Identity: IWCContextMenuCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddExtensionMenuItem<Identity: IWCContextMenuCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: *mut core::ffi::c_void, lpszstatusbartext: *mut core::ffi::c_void, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCContextMenuCallback_Impl::AddExtensionMenuItem(this, core::mem::transmute(&lpszname), core::mem::transmute(&lpszstatusbartext), core::mem::transmute_copy(&ncommandid), core::mem::transmute_copy(&nsubmenucommandid), core::mem::transmute_copy(&uflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExtensionMenuItem: AddExtensionMenuItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCContextMenuCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWCContextMenuCallback {}
windows_core::imp::define_interface!(IWCPropertySheetCallback, IWCPropertySheetCallback_Vtbl, 0x97dede60_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWCPropertySheetCallback, windows_core::IUnknown);
impl IWCPropertySheetCallback {
    pub unsafe fn AddPropertySheetPage(&self, hpage: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddPropertySheetPage)(windows_core::Interface::as_raw(self), hpage).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCPropertySheetCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddPropertySheetPage: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
}
pub trait IWCPropertySheetCallback_Impl: windows_core::IUnknownImpl {
    fn AddPropertySheetPage(&self, hpage: *const i32) -> windows_core::Result<()>;
}
impl IWCPropertySheetCallback_Vtbl {
    pub const fn new<Identity: IWCPropertySheetCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddPropertySheetPage<Identity: IWCPropertySheetCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCPropertySheetCallback_Impl::AddPropertySheetPage(this, core::mem::transmute_copy(&hpage)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPropertySheetPage: AddPropertySheetPage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCPropertySheetCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWCPropertySheetCallback {}
windows_core::imp::define_interface!(IWCWizard97Callback, IWCWizard97Callback_Vtbl, 0x97dede67_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWCWizard97Callback, windows_core::IUnknown);
impl IWCWizard97Callback {
    pub unsafe fn AddWizard97Page(&self, hpage: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddWizard97Page)(windows_core::Interface::as_raw(self), hpage).ok() }
    }
    pub unsafe fn EnableNext(&self, hpage: *const i32, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableNext)(windows_core::Interface::as_raw(self), hpage, benable.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCWizard97Callback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddWizard97Page: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
    pub EnableNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWCWizard97Callback_Impl: windows_core::IUnknownImpl {
    fn AddWizard97Page(&self, hpage: *const i32) -> windows_core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IWCWizard97Callback_Vtbl {
    pub const fn new<Identity: IWCWizard97Callback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddWizard97Page<Identity: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCWizard97Callback_Impl::AddWizard97Page(this, core::mem::transmute_copy(&hpage)).into()
            }
        }
        unsafe extern "system" fn EnableNext<Identity: IWCWizard97Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCWizard97Callback_Impl::EnableNext(this, core::mem::transmute_copy(&hpage), core::mem::transmute_copy(&benable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddWizard97Page: AddWizard97Page::<Identity, OFFSET>,
            EnableNext: EnableNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCWizard97Callback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWCWizard97Callback {}
windows_core::imp::define_interface!(IWCWizardCallback, IWCWizardCallback_Vtbl, 0x97dede62_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWCWizardCallback, windows_core::IUnknown);
impl IWCWizardCallback {
    pub unsafe fn AddWizardPage(&self, hpage: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddWizardPage)(windows_core::Interface::as_raw(self), hpage).ok() }
    }
    pub unsafe fn EnableNext(&self, hpage: *const i32, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableNext)(windows_core::Interface::as_raw(self), hpage, benable.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCWizardCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddWizardPage: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32) -> windows_core::HRESULT,
    pub EnableNext: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWCWizardCallback_Impl: windows_core::IUnknownImpl {
    fn AddWizardPage(&self, hpage: *const i32) -> windows_core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IWCWizardCallback_Vtbl {
    pub const fn new<Identity: IWCWizardCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddWizardPage<Identity: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCWizardCallback_Impl::AddWizardPage(this, core::mem::transmute_copy(&hpage)).into()
            }
        }
        unsafe extern "system" fn EnableNext<Identity: IWCWizardCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWCWizardCallback_Impl::EnableNext(this, core::mem::transmute_copy(&hpage), core::mem::transmute_copy(&benable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddWizardPage: AddWizardPage::<Identity, OFFSET>,
            EnableNext: EnableNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCWizardCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWCWizardCallback {}
windows_core::imp::define_interface!(IWEExtendContextMenu, IWEExtendContextMenu_Vtbl, 0x97dede65_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWEExtendContextMenu, windows_core::IUnknown);
impl IWEExtendContextMenu {
    pub unsafe fn AddContextMenuItems<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCContextMenuCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddContextMenuItems)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendContextMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddContextMenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWEExtendContextMenu_Impl: windows_core::IUnknownImpl {
    fn AddContextMenuItems(&self, pidata: windows_core::Ref<windows_core::IUnknown>, picallback: windows_core::Ref<IWCContextMenuCallback>) -> windows_core::Result<()>;
}
impl IWEExtendContextMenu_Vtbl {
    pub const fn new<Identity: IWEExtendContextMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddContextMenuItems<Identity: IWEExtendContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWEExtendContextMenu_Impl::AddContextMenuItems(this, core::mem::transmute_copy(&pidata), core::mem::transmute_copy(&picallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddContextMenuItems: AddContextMenuItems::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendContextMenu as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWEExtendContextMenu {}
windows_core::imp::define_interface!(IWEExtendPropertySheet, IWEExtendPropertySheet_Vtbl, 0x97dede61_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWEExtendPropertySheet, windows_core::IUnknown);
impl IWEExtendPropertySheet {
    pub unsafe fn CreatePropertySheetPages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCPropertySheetCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePropertySheetPages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendPropertySheet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePropertySheetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWEExtendPropertySheet_Impl: windows_core::IUnknownImpl {
    fn CreatePropertySheetPages(&self, pidata: windows_core::Ref<windows_core::IUnknown>, picallback: windows_core::Ref<IWCPropertySheetCallback>) -> windows_core::Result<()>;
}
impl IWEExtendPropertySheet_Vtbl {
    pub const fn new<Identity: IWEExtendPropertySheet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertySheetPages<Identity: IWEExtendPropertySheet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWEExtendPropertySheet_Impl::CreatePropertySheetPages(this, core::mem::transmute_copy(&pidata), core::mem::transmute_copy(&picallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePropertySheetPages: CreatePropertySheetPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendPropertySheet as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWEExtendPropertySheet {}
windows_core::imp::define_interface!(IWEExtendWizard, IWEExtendWizard_Vtbl, 0x97dede63_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWEExtendWizard, windows_core::IUnknown);
impl IWEExtendWizard {
    pub unsafe fn CreateWizardPages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCWizardCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateWizardPages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendWizard_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateWizardPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWEExtendWizard_Impl: windows_core::IUnknownImpl {
    fn CreateWizardPages(&self, pidata: windows_core::Ref<windows_core::IUnknown>, picallback: windows_core::Ref<IWCWizardCallback>) -> windows_core::Result<()>;
}
impl IWEExtendWizard_Vtbl {
    pub const fn new<Identity: IWEExtendWizard_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateWizardPages<Identity: IWEExtendWizard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWEExtendWizard_Impl::CreateWizardPages(this, core::mem::transmute_copy(&pidata), core::mem::transmute_copy(&picallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateWizardPages: CreateWizardPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendWizard as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWEExtendWizard {}
windows_core::imp::define_interface!(IWEExtendWizard97, IWEExtendWizard97_Vtbl, 0x97dede68_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWEExtendWizard97, windows_core::IUnknown);
impl IWEExtendWizard97 {
    pub unsafe fn CreateWizard97Pages<P0, P1>(&self, pidata: P0, picallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IWCWizard97Callback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateWizard97Pages)(windows_core::Interface::as_raw(self), pidata.param().abi(), picallback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEExtendWizard97_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateWizard97Pages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWEExtendWizard97_Impl: windows_core::IUnknownImpl {
    fn CreateWizard97Pages(&self, pidata: windows_core::Ref<windows_core::IUnknown>, picallback: windows_core::Ref<IWCWizard97Callback>) -> windows_core::Result<()>;
}
impl IWEExtendWizard97_Vtbl {
    pub const fn new<Identity: IWEExtendWizard97_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateWizard97Pages<Identity: IWEExtendWizard97_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWEExtendWizard97_Impl::CreateWizard97Pages(this, core::mem::transmute_copy(&pidata), core::mem::transmute_copy(&picallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateWizard97Pages: CreateWizard97Pages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendWizard97 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWEExtendWizard97 {}
windows_core::imp::define_interface!(IWEInvokeCommand, IWEInvokeCommand_Vtbl, 0x97dede66_fc6b_11cf_b5f5_00a0c90ab505);
windows_core::imp::interface_hierarchy!(IWEInvokeCommand, windows_core::IUnknown);
impl IWEInvokeCommand {
    pub unsafe fn InvokeCommand<P1>(&self, ncommandid: u32, pidata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeCommand)(windows_core::Interface::as_raw(self), ncommandid, pidata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWEInvokeCommand_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InvokeCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWEInvokeCommand_Impl: windows_core::IUnknownImpl {
    fn InvokeCommand(&self, ncommandid: u32, pidata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IWEInvokeCommand_Vtbl {
    pub const fn new<Identity: IWEInvokeCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeCommand<Identity: IWEInvokeCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncommandid: u32, pidata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWEInvokeCommand_Impl::InvokeCommand(this, core::mem::transmute_copy(&ncommandid), core::mem::transmute_copy(&pidata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvokeCommand: InvokeCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEInvokeCommand as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWEInvokeCommand {}
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
pub const LOG_ERROR: LOG_LEVEL = LOG_LEVEL(2i32);
pub const LOG_INFORMATION: LOG_LEVEL = LOG_LEVEL(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOG_LEVEL(pub i32);
pub const LOG_SEVERE: LOG_LEVEL = LOG_LEVEL(3i32);
pub const LOG_WARNING: LOG_LEVEL = LOG_LEVEL(1i32);
pub type LPGROUP_CALLBACK_EX = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HGROUP, param2: HGROUP, param3: *mut core::ffi::c_void) -> u32>;
pub type LPNODE_CALLBACK = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HNODE, param2: CLUSTER_NODE_STATE, param3: *mut core::ffi::c_void) -> u32>;
pub type LPRESOURCE_CALLBACK = Option<unsafe extern "system" fn(param0: HRESOURCE, param1: HRESOURCE, param2: *mut core::ffi::c_void) -> u32>;
pub type LPRESOURCE_CALLBACK_EX = Option<unsafe extern "system" fn(param0: HCLUSTER, param1: HRESOURCE, param2: HRESOURCE, param3: *mut core::ffi::c_void) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MAINTENANCE_MODE_TYPE_ENUM(pub i32);
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
pub const MAX_OBJECTID: u32 = 64u32;
pub const MINIMUM_NEVER_PREEMPT_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumNeverPreemptPriority");
pub const MINIMUM_PREEMPTOR_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumPreemptorPriority");
pub const MN_UPGRADE_VERSION: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONITOR_STATE {
    pub LastUpdate: i64,
    pub State: RESOURCE_MONITOR_STATE,
    pub ActiveResource: super::super::Foundation::HANDLE,
    pub ResmonStop: windows_core::BOOL,
}
pub const MaintenanceModeTypeDisableIsAliveCheck: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(1i32);
pub const MaintenanceModeTypeOfflineResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(2i32);
pub const MaintenanceModeTypeUnclusterResource: MAINTENANCE_MODE_TYPE_ENUM = MAINTENANCE_MODE_TYPE_ENUM(3i32);
pub const ModifyQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(1i32);
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
pub const NI_UPGRADE_VERSION: u32 = 2u32;
pub const NNLEN: u32 = 80u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NODE_CLUSTER_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
pub const NT10_MAJOR_VERSION: u32 = 9u32;
pub const NT11_MAJOR_VERSION: u32 = 10u32;
pub const NT12_MAJOR_VERSION: u32 = 11u32;
pub const NT13_MAJOR_VERSION: u32 = 12u32;
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
pub const NT4_MAJOR_VERSION: u32 = 1u32;
pub const NT51_MAJOR_VERSION: u32 = 4u32;
pub const NT5_MAJOR_VERSION: u32 = 3u32;
pub const NT6_MAJOR_VERSION: u32 = 5u32;
pub const NT7_MAJOR_VERSION: u32 = 6u32;
pub const NT8_MAJOR_VERSION: u32 = 7u32;
pub const NT9_MAJOR_VERSION: u32 = 8u32;
pub const NodeDrainStatusCompleted: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(2i32);
pub const NodeDrainStatusFailed: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(3i32);
pub const NodeDrainStatusInProgress: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(1i32);
pub const NodeDrainStatusNotInitiated: CLUSTER_NODE_DRAIN_STATUS = CLUSTER_NODE_DRAIN_STATUS(0i32);
pub const NodeStatusAvoidPlacement: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(32i32);
pub const NodeStatusDrainCompleted: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(8i32);
pub const NodeStatusDrainFailed: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(16i32);
pub const NodeStatusDrainInProgress: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(4i32);
pub const NodeStatusIsolated: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(1i32);
pub const NodeStatusMax: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(51i32);
pub const NodeStatusNormal: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(0i32);
pub const NodeStatusQuarantined: CLUSTER_NODE_STATUS = CLUSTER_NODE_STATUS(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeUtilizationInfoElement {
    pub Id: u64,
    pub AvailableMemory: u64,
    pub AvailableMemoryAfterReclamation: u64,
}
pub const OperationalQuorum: CLUSTER_QUORUM_TYPE = CLUSTER_QUORUM_TYPE(0i32);
pub type PARBITRATE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, lostquorumresource: PQUORUM_RESOURCE_LOST) -> u32>;
pub type PARM_WPR_WATCHDOG_FOR_CURRENT_RESOURCE_CALL_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, timeoutinms: u64) -> u32>;
pub type PBEGIN_RESCALL_AS_USER_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut windows_core::BOOL) -> u32>;
pub type PBEGIN_RESCALL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut windows_core::BOOL) -> u32>;
pub type PBEGIN_RESTYPECALL_AS_USER_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut windows_core::BOOL) -> u32>;
pub type PBEGIN_RESTYPECALL_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut windows_core::BOOL) -> u32>;
pub type PCANCEL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, cancelflags_reserved: u32) -> u32>;
pub type PCHANGE_RESOURCE_PROCESS_FOR_DUMPS = Option<unsafe extern "system" fn(resource: isize, processname: windows_core::PCWSTR, processid: u32, isadd: windows_core::BOOL) -> u32>;
pub type PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, processname: windows_core::PCWSTR, processid: u32, isadd: windows_core::BOOL) -> u32>;
pub type PCLOSE_CLUSTER_CRYPT_PROVIDER = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER) -> u32>;
pub type PCLOSE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void)>;
pub type PCLUSAPIClusWorkerCheckTerminate = Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER) -> windows_core::BOOL>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroup: HGROUP) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, hprovidergroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: HGROUP, hprovidergroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszpathname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = Option<unsafe extern "system" fn(hresource: HRESOURCE, hresourcedependent: HRESOURCE) -> windows_core::BOOL>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP, flags: u64) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX2 = Option<unsafe extern "system" fn(hresource: HRESOURCE, hgroup: HGROUP, flags: u64, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLOSE_CLUSTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: HCHANGE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUPSET_WITH_DOMAINS_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hgroup: HGROUP, faultdomain: u32, updatedomain: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, affinityrulename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = Option<unsafe extern "system" fn(henum: HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM = Option<unsafe extern "system" fn(henum: HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = Option<unsafe extern "system" fn(henum: HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hclusterenum: HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = Option<unsafe extern "system" fn(hgroup: HGROUP, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hgroupenum: HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hgroupenumex: HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = Option<unsafe extern "system" fn(hgroup: HGROUP, dwtype: u32) -> HGROUPENUM>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX>;
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = Option<unsafe extern "system" fn(hnetwork: HNETWORK, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL_EX = Option<unsafe extern "system" fn(hnetwork: HNETWORK, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnetworkenum: HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = Option<unsafe extern "system" fn(hnetwork: HNETWORK, dwtype: u32) -> HNETWORKENUM>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL_EX = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = Option<unsafe extern "system" fn(hnode: HNODE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL_EX = Option<unsafe extern "system" fn(hnode: HNODE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnodeenum: HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hnodeenum: HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = Option<unsafe extern "system" fn(hnode: HNODE, dwtype: u32) -> HNODEENUM>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hnode: HNODE, dwtype: u32, poptions: *const core::ffi::c_void) -> HNODEENUMEX>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: HCLUSTER, dwtype: u32) -> HCLUSENUM>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, dwtype: u32, poptions: *const core::ffi::c_void) -> HCLUSENUMEX>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregbatch: *mut HREGBATCH) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpsubkey: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: windows_core::PCWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> i32>;
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, flags: u32) -> i32>;
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, rulename: windows_core::PCWSTR, hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = Option<unsafe extern "system" fn(hresenum: HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = Option<unsafe extern "system" fn(hresenum: HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hresenum: HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hresourceenumex: HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwtype: u32) -> HRESENUM>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HRESENUMEX>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hrestypeenum: HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, dwtype: u32) -> HRESTYPEENUM>;
pub type PCLUSAPI_CLUSTER_UPGRADE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, perform: windows_core::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CLUS_WORKER_CREATE = Option<unsafe extern "system" fn(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CLUS_WORKER_TERMINATE = Option<unsafe extern "system" fn(lpworker: *const CLUS_WORKER)>;
pub type PCLUSAPI_CREATE_CLUSTER = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpavailabilitysetname: windows_core::PCWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: HCHANGE, hcluster: HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, hcluster: HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32, lpszreason: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcetypename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsztypename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void, fdeletevirtualcomputerobjects: windows_core::BOOL) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: HGROUPSET) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> HCLUSTER>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = Option<unsafe extern "system" fn(hgroup: HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = Option<unsafe extern "system" fn(hgroup: HGROUP, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE>;
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_KEY = Option<unsafe extern "system" fn(hcluster: HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = Option<unsafe extern "system" fn(hnetwork: HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = Option<unsafe extern "system" fn(hnetwork: HNETWORK) -> CLUSTER_NETWORK_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, lpsznetworkname: windows_core::PCWSTR, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = Option<unsafe extern "system" fn(hnetinterface: HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = Option<unsafe extern "system" fn(hnode: HNODE, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = Option<unsafe extern "system" fn(hnode: HNODE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = Option<unsafe extern "system" fn(hnode: HNODE) -> CLUSTER_NODE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: windows_core::PWSTR, lpcchobjectid: *mut u32, lpszparentid: windows_core::PWSTR, lpcchparentid: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpsztype: windows_core::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdependencyexpression: windows_core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = Option<unsafe extern "system" fn(hresource: HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> windows_core::BOOL>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32, lpszgroupname: windows_core::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsztypename: windows_core::PCWSTR, samdesired: u32) -> super::super::System::Registry::HKEY>;
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = Option<unsafe extern "system" fn(lpsznodename: windows_core::PCWSTR, pdwclusterstate: *mut u32) -> u32>;
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, pbfileisonsharedvolume: *mut windows_core::BOOL) -> u32>;
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP, hdestinationnode: HNODE) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: HGROUP, hdestinationnode: HNODE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_OPEN_CLUSTER = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_EX = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetinterfacename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetworkname: windows_core::PCWSTR) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznetworkname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszinterfacename: windows_core::PCWSTR) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznodename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpszresourcename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_NODE_BY_ID = Option<unsafe extern "system" fn(hcluster: HCLUSTER, nodeid: u32) -> HNODE>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, bdrainnode: windows_core::BOOL, dwpauseflags: u32, hnodedraintarget: HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, bdrainnode: windows_core::BOOL, dwpauseflags: u32, hnodedraintarget: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_PFN_REASON_HANDLER = Option<unsafe extern "system" fn(lpparameter: *const core::ffi::c_void, hcluster: HCLUSTER, szreason: windows_core::PWSTR, lpsize: *mut u32) -> windows_core::BOOL>;
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUP) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hdependson: HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, hdependson: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, hdependson: HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: HCLUSTER) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hdependson: HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, hnode: HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, bforce: windows_core::BOOL, lpszquorumdriveletter: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: HNODE) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroupset: HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszgroupname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszgroupname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = Option<unsafe extern "system" fn(hgroup: HGROUP, nodecount: u32, nodelist: *const HNODE) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, nodecount: u32, nodelist: *const HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NAME_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznewclustername: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpszname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME_EX = Option<unsafe extern "system" fn(hnetwork: HNETWORK, lpszname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, networkcount: u32, networklist: *const HNETWORK) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquologsize: u32) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquorumlogsize: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszresourcename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME_EX = Option<unsafe extern "system" fn(hresource: HRESOURCE, lpszresourcename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, lpsznewpassword: windows_core::PCWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: HGROUP, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroup: HGROUP, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_REASON_HANDLER = Option<unsafe extern "system" fn(lphandler: *const CLUSAPI_REASON_HANDLER) -> *mut CLUSAPI_REASON_HANDLER>;
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = Option<unsafe extern "system" fn(guidsnapshotset: windows_core::GUID, lpszvolumename: windows_core::PCWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32>;
pub type PCLUSAPI_SetClusterName = Option<unsafe extern "system" fn(hcluster: HCLUSTER, lpsznewclustername: windows_core::PCWSTR) -> u32>;
pub type PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszvolumepathname: windows_core::PCWSTR) -> u32>;
pub type PCLUSTER_DECRYPT = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32>;
pub type PCLUSTER_ENCRYPT = Option<unsafe extern "system" fn(hcluscryptprovider: HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32>;
pub type PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT = Option<unsafe extern "system" fn(lpszvolumemountpoint: windows_core::PCWSTR, lpszvolumename: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::BOOL>;
pub type PCLUSTER_GET_VOLUME_PATH_NAME = Option<unsafe extern "system" fn(lpszfilename: windows_core::PCWSTR, lpszvolumepathname: windows_core::PWSTR, cchbufferlength: u32) -> windows_core::BOOL>;
pub type PCLUSTER_IS_PATH_ON_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP = Option<unsafe extern "system" fn(lpszfilename: windows_core::PCWSTR, lpszvolumepathname: windows_core::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: windows_core::PWSTR, lpcchvolumename: *mut u32) -> u32>;
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregbatch: HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: windows_core::PCWSTR, dwoptions: u32, lpdata: *const core::ffi::c_void, cbdata: u32) -> i32>;
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotification: HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_BATCH_READ_COMMAND = Option<unsafe extern "system" fn(hbatchnotification: HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH = Option<unsafe extern "system" fn(hregbatch: HREGBATCH, bcommit: windows_core::BOOL, failedcommandnumber: *mut i32) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hbatchnotifyport: HREGBATCHPORT) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = Option<unsafe extern "system" fn(hregreadbatchreply: HREGREADBATCHREPLY) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut HREGBATCHPORT) -> i32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = Option<unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut HREGREADBATCH) -> i32>;
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotify: HREGBATCHPORT, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregreadbatch: HREGREADBATCH, wzsubkeyname: windows_core::PCWSTR, wzvaluename: windows_core::PCWSTR) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = Option<unsafe extern "system" fn(hregreadbatchreply: HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32>;
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: windows_core::PCWSTR, dwstatus: u32) -> windows_core::BOOL>;
pub type PCLUSTER_SET_ACCOUNT_ACCESS = Option<unsafe extern "system" fn(hcluster: HCLUSTER, szaccountsid: windows_core::PCWSTR, dwaccess: u32, dwcontroltype: u32) -> u32>;
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> windows_core::BOOL>;
pub type PEND_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
pub type PEND_TYPE_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, status: u32) -> u32>;
pub type PEXTEND_RES_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
pub type PEXTEND_RES_TYPE_CONTROL_CALL = Option<unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32>;
pub type PFREE_CLUSTER_CRYPT = Option<unsafe extern "system" fn(pcryptinfo: *const core::ffi::c_void) -> u32>;
pub type PIS_ALIVE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PLACEMENT_OPTIONS(pub i32);
pub const PLACEMENT_OPTIONS_ALL: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1023i32);
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(512i32);
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(2i32);
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(1i32);
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(128i32);
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(32i32);
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(8i32);
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(16i32);
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(4i32);
pub const PLACEMENT_OPTIONS_MIN_VALUE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(0i32);
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(256i32);
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = PLACEMENT_OPTIONS(64i32);
pub type PLOG_EVENT_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, loglevel: LOG_LEVEL, formatstring: windows_core::PCWSTR)>;
pub type PLOOKS_ALIVE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type POFFLINE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> u32>;
pub type POFFLINE_V2_ROUTINE = Option<unsafe extern "system" fn(resource: *const core::ffi::c_void, destinationnodename: windows_core::PCWSTR, offlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
pub type PONLINE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> u32>;
pub type PONLINE_V2_ROUTINE = Option<unsafe extern "system" fn(resource: *const core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE, onlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32>;
pub type POPEN_CLUSTER_CRYPT_PROVIDER = Option<unsafe extern "system" fn(lpszresource: windows_core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER>;
pub type POPEN_CLUSTER_CRYPT_PROVIDEREX = Option<unsafe extern "system" fn(lpszresource: windows_core::PCWSTR, lpszkeyname: windows_core::PCWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> HCLUSCRYPTPROVIDER>;
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_ROUTINE = Option<unsafe extern "system" fn(resourcename: windows_core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Registry")]
pub type POPEN_V2_ROUTINE = Option<unsafe extern "system" fn(resourcename: windows_core::PCWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize, openflags: u32) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POST_UPGRADE_VERSION_INFO {
    pub newMajorVersion: u32,
    pub newUpgradeVersion: u32,
    pub oldMajorVersion: u32,
    pub oldUpgradeVersion: u32,
    pub reserved: u32,
}
pub type PQUERY_APPINSTANCE_VERSION = Option<unsafe extern "system" fn(appinstanceid: *const windows_core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32>;
pub type PQUORUM_RESOURCE_LOST = Option<unsafe extern "system" fn(resource: isize)>;
pub type PRAISE_RES_TYPE_NOTIFICATION = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, ppayload: *const u8, payloadsize: u32) -> u32>;
pub type PREGISTER_APPINSTANCE = Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const windows_core::GUID, childreninheritappinstance: windows_core::BOOL) -> u32>;
pub type PREGISTER_APPINSTANCE_VERSION = Option<unsafe extern "system" fn(appinstanceid: *const windows_core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32>;
pub type PRELEASE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void) -> u32>;
pub type PREQUEST_DUMP_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, dumpduetocallinprogress: windows_core::BOOL, dumpdelayinms: u32) -> u32>;
pub type PRESET_ALL_APPINSTANCE_VERSIONS = Option<unsafe extern "system" fn() -> u32>;
pub type PRESOURCE_CONTROL_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
pub type PRESOURCE_TYPE_CONTROL_ROUTINE = Option<unsafe extern "system" fn(resourcetypename: windows_core::PCWSTR, controlcode: u32, inbuffer: *mut core::ffi::c_void, inbuffersize: u32, outbuffer: *mut core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ADD_UNKNOWN_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_CREATE_DIRECTORY_TREE = Option<unsafe extern "system" fn(pszpath: windows_core::PCWSTR) -> u32>;
pub type PRESUTIL_DUP_PARAMETER_BLOCK = Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32>;
pub type PRESUTIL_DUP_STRING = Option<unsafe extern "system" fn(pszinstring: windows_core::PCWSTR) -> windows_core::PWSTR>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_ENUM_PRIVATE_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_ENUM_PROPERTIES = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: windows_core::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES = Option<unsafe extern "system" fn(hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_ENUM_RESOURCES_EX2 = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: HRESOURCE, lpszrestypename: windows_core::PCWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut core::ffi::c_void, dwdesiredaccess: u32) -> u32>;
pub type PRESUTIL_EXPAND_ENVIRONMENT_STRINGS = Option<unsafe extern "system" fn(pszsrc: windows_core::PCWSTR) -> windows_core::PWSTR>;
pub type PRESUTIL_FIND_BINARY_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hresource: HRESOURCE, pszdriveletter: windows_core::PWSTR, pcchdriveletter: *mut u32) -> u32>;
pub type PRESUTIL_FIND_DWORD_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pdwpropertyvalue: *mut u32) -> u32>;
pub type PRESUTIL_FIND_EXPANDED_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_EXPAND_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_FILETIME_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32>;
pub type PRESUTIL_FIND_LONG_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, plpropertyvalue: *mut i32) -> u32>;
pub type PRESUTIL_FIND_MULTI_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_FIND_SZ_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, pszpropertyvalue: *mut windows_core::PWSTR) -> u32>;
pub type PRESUTIL_FIND_ULARGEINTEGER_PROPERTY = Option<unsafe extern "system" fn(ppropertylist: *const core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: windows_core::PCWSTR, plpropertyvalue: *mut u64) -> u32>;
pub type PRESUTIL_FREE_ENVIRONMENT = Option<unsafe extern "system" fn(lpenvironment: *mut core::ffi::c_void) -> u32>;
pub type PRESUTIL_FREE_PARAMETER_BLOCK = Option<unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM)>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_ALL_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_GET_BINARY_PROPERTY = Option<unsafe extern "system" fn(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_BINARY_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES = Option<unsafe extern "system" fn(hcluster: HCLUSTER, phclusternameresource: *mut HRESOURCE, phclusteripaddressresource: *mut HRESOURCE, phclusterquorumresource: *mut HRESOURCE) -> u32>;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX = Option<unsafe extern "system" fn(hclusterin: HCLUSTER, phclusternameresourceout: *mut HRESOURCE, phclusteripaddressresourceout: *mut HRESOURCE, phclusterquorumresourceout: *mut HRESOURCE, dwdesiredaccess: u32) -> u32>;
pub type PRESUTIL_GET_DWORD_PROPERTY = Option<unsafe extern "system" fn(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_DWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32>;
pub type PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE) -> *mut core::ffi::c_void>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_EXPAND_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, bexpand: windows_core::BOOL) -> windows_core::PWSTR>;
pub type PRESUTIL_GET_FILETIME_PROPERTY = Option<unsafe extern "system" fn(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_LONG_PROPERTY = Option<unsafe extern "system" fn(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_MULTI_SZ_PROPERTY = Option<unsafe extern "system" fn(ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: windows_core::PCWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PRIVATE_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: windows_core::BOOL, psznameofpropinerror: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTY = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32>;
pub type PRESUTIL_GET_PROPERTY_FORMATS = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PROPERTY_SIZE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_QWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: windows_core::BOOL) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: windows_core::BOOL, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, brecurse: windows_core::BOOL) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX = Option<unsafe extern "system" fn(hcluster: HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, brecurse: windows_core::BOOL, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: windows_core::PCWSTR, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS = Option<unsafe extern "system" fn(hresource: HRESOURCE, pszaddress: windows_core::PWSTR, pcchaddress: *mut u32, pszsubnetmask: windows_core::PWSTR, pcchsubnetmask: *mut u32, psznetwork: windows_core::PWSTR, pcchnetwork: *mut u32) -> u32>;
pub type PRESUTIL_GET_RESOURCE_NAME = Option<unsafe extern "system" fn(hresource: HRESOURCE, pszresourcename: windows_core::PWSTR, pcchresourcenameinout: *mut u32) -> u32>;
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY = Option<unsafe extern "system" fn(lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR) -> HRESOURCE>;
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX = Option<unsafe extern "system" fn(lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwdesiredaccess: u32) -> HRESOURCE>;
pub type PRESUTIL_GET_SZ_PROPERTY = Option<unsafe extern "system" fn(ppszoutvalue: *mut windows_core::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: windows_core::PCWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR) -> windows_core::PWSTR>;
pub type PRESUTIL_IS_PATH_VALID = Option<unsafe extern "system" fn(pszpath: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PRESUTIL_IS_RESOURCE_CLASS_EQUAL = Option<unsafe extern "system" fn(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: HRESOURCE) -> windows_core::BOOL>;
pub type PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32>;
pub type PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
pub type PRESUTIL_RESOURCES_EQUAL = Option<unsafe extern "system" fn(hself: HRESOURCE, hresource: HRESOURCE) -> windows_core::BOOL>;
pub type PRESUTIL_RESOURCE_TYPES_EQUAL = Option<unsafe extern "system" fn(lpszresourcetypename: windows_core::PCWSTR, hresource: HRESOURCE) -> windows_core::BOOL>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_BINARY_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_DWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_EXPAND_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, ppszoutstring: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_MULTI_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut windows_core::PWSTR, pcboutvaluesize: *mut u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PRIVATE_PROPERTY_LIST = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: windows_core::BOOL, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_TABLE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *const core::ffi::c_void, ballowunknownproperties: windows_core::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PROPERTY_TABLE_EX = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut core::ffi::c_void, ballowunknownproperties: windows_core::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: windows_core::BOOL, poutparams: *mut u8) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_QWORD_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32>;
pub type PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, hresource: HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_System_Services")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, schscmhandle: super::super::System::Services::SC_HANDLE, phservice: *mut super::super::System::Services::SC_HANDLE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_System_Services")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, schscmhandle: super::super::System::Services::SC_HANDLE, phservice: *mut super::super::System::Services::SC_HANDLE, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_SZ_VALUE = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: windows_core::PCWSTR, psznewvalue: windows_core::PCWSTR, ppszoutstring: *mut windows_core::PWSTR) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_UNKNOWN_PROPERTIES = Option<unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
#[cfg(feature = "Win32_System_Services")]
pub type PRESUTIL_START_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR, phservicehandle: *mut super::super::System::Services::SC_HANDLE) -> u32>;
pub type PRESUTIL_STOP_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Services")]
pub type PRESUTIL_STOP_SERVICE = Option<unsafe extern "system" fn(hservicehandle: super::super::System::Services::SC_HANDLE) -> u32>;
pub type PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL = Option<unsafe extern "system" fn(dwservicepid: u32, boffline: windows_core::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32>;
pub type PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST = Option<unsafe extern "system" fn(pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32) -> u32>;
pub type PRESUTIL_VERIFY_PROPERTY_TABLE = Option<unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *const core::ffi::c_void, ballowunknownproperties: windows_core::BOOL, pinpropertylist: *const core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32>;
pub type PRESUTIL_VERIFY_RESOURCE_SERVICE = Option<unsafe extern "system" fn(pszservicename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_System_Services")]
pub type PRESUTIL_VERIFY_SERVICE = Option<unsafe extern "system" fn(hservicehandle: super::super::System::Services::SC_HANDLE) -> u32>;
pub type PRES_UTIL_VERIFY_SHUTDOWN_SAFE = Option<unsafe extern "system" fn(flags: u32, reason: u32, presult: *mut u32) -> u32>;
pub type PSET_INTERNAL_STATE = Option<unsafe extern "system" fn(param0: isize, statetype: CLUSTER_RESOURCE_APPLICATION_STATE, active: windows_core::BOOL) -> u32>;
pub type PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, propertylistbuffer: *const u8, propertylistbuffersize: u32) -> u32>;
pub type PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: windows_core::BOOL, lockedmodereason: u32, lockedmodeflags: u32) -> u32>;
pub type PSET_RESOURCE_LOCKED_MODE_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: windows_core::BOOL, lockedmodereason: u32) -> u32>;
pub type PSET_RESOURCE_STATUS_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS) -> u32>;
pub type PSET_RESOURCE_STATUS_ROUTINE_EX = Option<unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS_EX) -> u32>;
pub type PSET_RESOURCE_WPR_POLICY_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, wprpolicyflags: u32) -> u32>;
pub type PSIGNAL_FAILURE_ROUTINE = Option<unsafe extern "system" fn(resourcehandle: isize, failuretype: FAILURE_TYPE, applicationspecificerrorcode: u32) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PSTARTUP_EX_ROUTINE = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, monitorcallbackfunctions: *mut CLRES_CALLBACK_FUNCTION_TABLE, resourcedllinterfacefunctions: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
#[cfg(feature = "Win32_System_Registry")]
pub type PSTARTUP_ROUTINE = Option<unsafe extern "system" fn(resourcetype: windows_core::PCWSTR, minversionsupported: u32, maxversionsupported: u32, setresourcestatus: PSET_RESOURCE_STATUS_ROUTINE, logevent: PLOG_EVENT_ROUTINE, functiontable: *mut *mut CLRES_FUNCTION_TABLE) -> u32>;
pub type PTERMINATE_ROUTINE = Option<unsafe extern "system" fn(resource: *mut core::ffi::c_void)>;
pub type PWORKER_START_ROUTINE = Option<unsafe extern "system" fn(pworker: *mut CLUS_WORKER, lpthreadparameter: *mut core::ffi::c_void) -> u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PaxosTagCStruct {
    pub __padding__PaxosTagVtable: u64,
    pub __padding__NextEpochVtable: u64,
    pub __padding__NextEpoch_DateTimeVtable: u64,
    pub NextEpoch_DateTime_ticks: u64,
    pub NextEpoch_Value: i32,
    pub __padding__BoundryNextEpoch: u32,
    pub __padding__EpochVtable: u64,
    pub __padding__Epoch_DateTimeVtable: u64,
    pub Epoch_DateTime_ticks: u64,
    pub Epoch_Value: i32,
    pub __padding__BoundryEpoch: u32,
    pub Sequence: i32,
    pub __padding__BoundrySequence: u32,
}
pub const PriorityDisabled: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(0i32);
pub const PriorityHigh: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(3000i32);
pub const PriorityLow: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(1000i32);
pub const PriorityMedium: CLUSTER_GROUP_PRIORITY = CLUSTER_GROUP_PRIORITY(2000i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESDLL_CONTEXT_OPERATION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESOURCE_EXIT_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESOURCE_MONITOR_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_STATUS {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub WaitHint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_STATUS_EX {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub ApplicationSpecificErrorCode: u32,
    pub Flags: u32,
    pub WaitHint: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: windows_core::BOOL,
    pub restartPeriodRemaining: u32,
}
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESUTIL_FILETIME_DATA {
    pub Default: super::super::Foundation::FILETIME,
    pub Minimum: super::super::Foundation::FILETIME,
    pub Maximum: super::super::Foundation::FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESUTIL_LARGEINT_DATA {
    pub Default: i64,
    pub Minimum: i64,
    pub Maximum: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESUTIL_PROPERTY_ITEM {
    pub Name: windows_core::PWSTR,
    pub KeyName: windows_core::PWSTR,
    pub Format: u32,
    pub Anonymous: RESUTIL_PROPERTY_ITEM_0,
    pub Minimum: u32,
    pub Maximum: u32,
    pub Flags: u32,
    pub Offset: u32,
}
impl Default for RESUTIL_PROPERTY_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RESUTIL_PROPERTY_ITEM_0 {
    pub DefaultPtr: usize,
    pub Default: u32,
    pub lpDefault: *mut core::ffi::c_void,
    pub LargeIntData: *mut RESUTIL_LARGEINT_DATA,
    pub ULargeIntData: *mut RESUTIL_ULARGEINT_DATA,
    pub FileTimeData: *mut RESUTIL_FILETIME_DATA,
}
impl Default for RESUTIL_PROPERTY_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESUTIL_ULARGEINT_DATA {
    pub Default: u64,
    pub Minimum: u64,
    pub Maximum: u64,
}
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const RedirectedIOReasonReFs: u64 = 32u64;
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
pub const ResdllContextOperationTypeDrain: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(1i32);
pub const ResdllContextOperationTypeDrainFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(2i32);
pub const ResdllContextOperationTypeEmbeddedFailure: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(3i32);
pub const ResdllContextOperationTypeFailback: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(0i32);
pub const ResdllContextOperationTypeNetworkDisconnect: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(5i32);
pub const ResdllContextOperationTypeNetworkDisconnectMoveRetry: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(6i32);
pub const ResdllContextOperationTypePreemption: RESDLL_CONTEXT_OPERATION_TYPE = RESDLL_CONTEXT_OPERATION_TYPE(4i32);
pub const ResourceExitStateContinue: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(0i32);
pub const ResourceExitStateMax: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(2i32);
pub const ResourceExitStateTerminate: RESOURCE_EXIT_STATE = RESOURCE_EXIT_STATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ResourceUtilizationInfoElement {
    pub PhysicalNumaId: u64,
    pub CurrentMemory: u64,
}
pub const RmonArbitrateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(10i32);
pub const RmonDeadlocked: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(15i32);
pub const RmonDeletingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(7i32);
pub const RmonIdle: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(1i32);
pub const RmonInitializing: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(0i32);
pub const RmonInitializingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(3i32);
pub const RmonIsAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(8i32);
pub const RmonLooksAlivePoll: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(9i32);
pub const RmonOfflineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(5i32);
pub const RmonOnlineResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(4i32);
pub const RmonReleaseResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(11i32);
pub const RmonResourceControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(12i32);
pub const RmonResourceTypeControl: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(13i32);
pub const RmonShutdownResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(6i32);
pub const RmonStartingResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(2i32);
pub const RmonTerminateResource: RESOURCE_MONITOR_STATE = RESOURCE_MONITOR_STATE(14i32);
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
pub type SET_APP_INSTANCE_CSV_FLAGS = Option<unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SR_DISK_REPLICATION_ELIGIBLE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SR_REPLICATED_DISK_TYPE(pub i32);
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: bool,
    pub EnableEncryption: bool,
    pub EnableCompression: bool,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [u16; 260],
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl Default for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeOfflineDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeAvailableStoargeDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: windows_core::GUID,
    pub TargetReplicationGroupGuid: windows_core::GUID,
    pub SkipConnectivityCheck: bool,
    pub IncludeOfflineDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: windows_core::GUID,
    pub ReplicationGroupId: windows_core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
pub const STARTUP_EX_ROUTINE: windows_core::PCSTR = windows_core::s!("StartupEx");
pub const STARTUP_ROUTINE: windows_core::PCSTR = windows_core::s!("Startup");
pub const SharedVolumeStateActive: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(2i32);
pub const SharedVolumeStateActiveRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(3i32);
pub const SharedVolumeStateActiveVolumeRedirected: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(4i32);
pub const SharedVolumeStatePaused: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(1i32);
pub const SharedVolumeStateUnavailable: CLUSTER_SHARED_VOLUME_STATE = CLUSTER_SHARED_VOLUME_STATE(0i32);
pub const SrDiskReplicationEligibleAlreadyInReplication: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9i32);
pub const SrDiskReplicationEligibleFileSystemNotSupported: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(8i32);
pub const SrDiskReplicationEligibleInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(7i32);
pub const SrDiskReplicationEligibleInsufficientFreeSpace: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(5i32);
pub const SrDiskReplicationEligibleNone: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(0i32);
pub const SrDiskReplicationEligibleNotGpt: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(3i32);
pub const SrDiskReplicationEligibleNotInSameSite: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(6i32);
pub const SrDiskReplicationEligibleOffline: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(2i32);
pub const SrDiskReplicationEligibleOther: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(9999i32);
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(4i32);
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(10i32);
pub const SrDiskReplicationEligibleYes: SR_DISK_REPLICATION_ELIGIBLE = SR_DISK_REPLICATION_ELIGIBLE(1i32);
pub const SrReplicatedDiskTypeDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(3i32);
pub const SrReplicatedDiskTypeLogDestination: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(4i32);
pub const SrReplicatedDiskTypeLogNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(6i32);
pub const SrReplicatedDiskTypeLogSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(2i32);
pub const SrReplicatedDiskTypeNone: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(0i32);
pub const SrReplicatedDiskTypeNotInParthership: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(5i32);
pub const SrReplicatedDiskTypeOther: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(7i32);
pub const SrReplicatedDiskTypeSource: SR_REPLICATED_DISK_TYPE = SR_REPLICATED_DISK_TYPE(1i32);
pub const USE_CLIENT_ACCESS_NETWORKS_FOR_CSV: windows_core::PCWSTR = windows_core::w!("UseClientAccessNetworksForSharedVolumes");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VM_RESDLL_CONTEXT(pub i32);
pub const VmResdllContextLiveMigration: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(4i32);
pub const VmResdllContextSave: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(1i32);
pub const VmResdllContextShutdown: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(2i32);
pub const VmResdllContextShutdownForce: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(3i32);
pub const VmResdllContextTurnOff: VM_RESDLL_CONTEXT = VM_RESDLL_CONTEXT(0i32);
pub const VolumeBackupInProgress: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(1i32);
pub const VolumeBackupNone: CLUSTER_SHARED_VOLUME_BACKUP_STATE = CLUSTER_SHARED_VOLUME_BACKUP_STATE(0i32);
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
pub const VolumeStateDismounted: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(8i32);
pub const VolumeStateInMaintenance: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(4i32);
pub const VolumeStateNoAccess: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(2i32);
pub const VolumeStateNoDirectIO: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(1i32);
pub const VolumeStateNoFaults: CLUSTER_CSV_VOLUME_FAULT_STATE = CLUSTER_CSV_VOLUME_FAULT_STATE(0i32);
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WitnessTagHelper {
    pub Version: i32,
    pub paxosToValidate: PaxosTagCStruct,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WitnessTagUpdateHelper {
    pub Version: i32,
    pub paxosToSet: PaxosTagCStruct,
    pub paxosToValidate: PaxosTagCStruct,
}
pub const eResourceStateChangeReasonFailedMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(3i32);
pub const eResourceStateChangeReasonFailover: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(2i32);
pub const eResourceStateChangeReasonMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(1i32);
pub const eResourceStateChangeReasonRundown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(5i32);
pub const eResourceStateChangeReasonShutdown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(4i32);
pub const eResourceStateChangeReasonUnknown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = CLUSTER_RESOURCE_STATE_CHANGE_REASON(0i32);
