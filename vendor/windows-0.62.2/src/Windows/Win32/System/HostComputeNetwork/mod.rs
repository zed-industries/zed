#[inline]
pub unsafe fn HcnCloseEndpoint(endpoint: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnCloseEndpoint(endpoint : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnCloseEndpoint(endpoint).ok() }
}
#[inline]
pub unsafe fn HcnCloseGuestNetworkService(guestnetworkservice: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnCloseGuestNetworkService(guestnetworkservice : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnCloseGuestNetworkService(guestnetworkservice).ok() }
}
#[inline]
pub unsafe fn HcnCloseLoadBalancer(loadbalancer: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnCloseLoadBalancer(loadbalancer : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnCloseLoadBalancer(loadbalancer).ok() }
}
#[inline]
pub unsafe fn HcnCloseNamespace(namespace: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnCloseNamespace(namespace : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnCloseNamespace(namespace).ok() }
}
#[inline]
pub unsafe fn HcnCloseNetwork(network: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnCloseNetwork(network : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnCloseNetwork(network).ok() }
}
#[inline]
pub unsafe fn HcnCreateEndpoint<P2>(network: *const core::ffi::c_void, id: *const windows_core::GUID, settings: P2, endpoint: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnCreateEndpoint(network : *const core::ffi::c_void, id : *const windows_core::GUID, settings : windows_core::PCWSTR, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnCreateEndpoint(network, id, settings.param().abi(), endpoint as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnCreateGuestNetworkService<P1>(id: *const windows_core::GUID, settings: P1, guestnetworkservice: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnCreateGuestNetworkService(id : *const windows_core::GUID, settings : windows_core::PCWSTR, guestnetworkservice : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnCreateGuestNetworkService(id, settings.param().abi(), guestnetworkservice as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnCreateLoadBalancer<P1>(id: *const windows_core::GUID, settings: P1, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnCreateLoadBalancer(id : *const windows_core::GUID, settings : windows_core::PCWSTR, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnCreateLoadBalancer(id, settings.param().abi(), loadbalancer as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnCreateNamespace<P1>(id: *const windows_core::GUID, settings: P1, namespace: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnCreateNamespace(id : *const windows_core::GUID, settings : windows_core::PCWSTR, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnCreateNamespace(id, settings.param().abi(), namespace as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnCreateNetwork<P1>(id: *const windows_core::GUID, settings: P1, network: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnCreateNetwork(id : *const windows_core::GUID, settings : windows_core::PCWSTR, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnCreateNetwork(id, settings.param().abi(), network as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnDeleteEndpoint(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnDeleteEndpoint(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnDeleteEndpoint(id, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnDeleteGuestNetworkService(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnDeleteGuestNetworkService(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnDeleteGuestNetworkService(id, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnDeleteLoadBalancer(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnDeleteLoadBalancer(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnDeleteLoadBalancer(id, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnDeleteNamespace(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnDeleteNamespace(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnDeleteNamespace(id, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnDeleteNetwork(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnDeleteNetwork(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnDeleteNetwork(id, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnEnumerateEndpoints<P0>(query: P0, endpoints: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnEnumerateEndpoints(query : windows_core::PCWSTR, endpoints : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnEnumerateEndpoints(query.param().abi(), endpoints as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnEnumerateGuestNetworkPortReservations(returncount : *mut u32, portentries : *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_core::HRESULT);
    unsafe { HcnEnumerateGuestNetworkPortReservations(returncount as _, portentries as _).ok() }
}
#[inline]
pub unsafe fn HcnEnumerateLoadBalancers<P0>(query: P0, loadbalancer: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnEnumerateLoadBalancers(query : windows_core::PCWSTR, loadbalancer : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnEnumerateLoadBalancers(query.param().abi(), loadbalancer as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnEnumerateNamespaces<P0>(query: P0, namespaces: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnEnumerateNamespaces(query : windows_core::PCWSTR, namespaces : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnEnumerateNamespaces(query.param().abi(), namespaces as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnEnumerateNetworks<P0>(query: P0, networks: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnEnumerateNetworks(query : windows_core::PCWSTR, networks : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnEnumerateNetworks(query.param().abi(), networks as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: Option<*mut HCN_PORT_RANGE_ENTRY>) {
    windows_core::link!("computenetwork.dll" "system" fn HcnFreeGuestNetworkPortReservations(portentries : *mut HCN_PORT_RANGE_ENTRY));
    unsafe { HcnFreeGuestNetworkPortReservations(portentries.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HcnModifyEndpoint<P1>(endpoint: *const core::ffi::c_void, settings: P1, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnModifyEndpoint(endpoint : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnModifyEndpoint(endpoint, settings.param().abi(), errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnModifyGuestNetworkService<P1>(guestnetworkservice: *const core::ffi::c_void, settings: P1, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnModifyGuestNetworkService(guestnetworkservice : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnModifyGuestNetworkService(guestnetworkservice, settings.param().abi(), errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnModifyLoadBalancer<P1>(loadbalancer: *const core::ffi::c_void, settings: P1, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnModifyLoadBalancer(loadbalancer : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnModifyLoadBalancer(loadbalancer, settings.param().abi(), errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnModifyNamespace<P1>(namespace: *const core::ffi::c_void, settings: P1, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnModifyNamespace(namespace : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnModifyNamespace(namespace, settings.param().abi(), errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnModifyNetwork<P1>(network: *const core::ffi::c_void, settings: P1, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnModifyNetwork(network : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnModifyNetwork(network, settings.param().abi(), errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnOpenEndpoint(id: *const windows_core::GUID, endpoint: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnOpenEndpoint(id : *const windows_core::GUID, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnOpenEndpoint(id, endpoint as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnOpenLoadBalancer(id: *const windows_core::GUID, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnOpenLoadBalancer(id : *const windows_core::GUID, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnOpenLoadBalancer(id, loadbalancer as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnOpenNamespace(id: *const windows_core::GUID, namespace: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnOpenNamespace(id : *const windows_core::GUID, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnOpenNamespace(id, namespace as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnOpenNetwork(id: *const windows_core::GUID, network: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnOpenNetwork(id : *const windows_core::GUID, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnOpenNetwork(id, network as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryEndpointAddresses<P1>(endpoint: *const core::ffi::c_void, query: P1, addresses: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryEndpointAddresses(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, addresses : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryEndpointAddresses(endpoint, query.param().abi(), addresses as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryEndpointProperties<P1>(endpoint: *const core::ffi::c_void, query: P1, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryEndpointProperties(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryEndpointProperties(endpoint, query.param().abi(), properties as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryEndpointStats<P1>(endpoint: *const core::ffi::c_void, query: P1, stats: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryEndpointStats(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, stats : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryEndpointStats(endpoint, query.param().abi(), stats as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryLoadBalancerProperties<P1>(loadbalancer: *const core::ffi::c_void, query: P1, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryLoadBalancerProperties(loadbalancer : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryLoadBalancerProperties(loadbalancer, query.param().abi(), properties as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryNamespaceProperties<P1>(namespace: *const core::ffi::c_void, query: P1, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryNamespaceProperties(namespace : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryNamespaceProperties(namespace, query.param().abi(), properties as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnQueryNetworkProperties<P1>(network: *const core::ffi::c_void, query: P1, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("computenetwork.dll" "system" fn HcnQueryNetworkProperties(network : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HcnQueryNetworkProperties(network, query.param().abi(), properties as _, errorrecord.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice : *const core::ffi::c_void, callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnRegisterGuestNetworkServiceCallback(guestnetworkservice, callback, context, callbackhandle as _).ok() }
}
#[inline]
pub unsafe fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnRegisterServiceCallback(callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnRegisterServiceCallback(callback, context, callbackhandle as _).ok() }
}
#[inline]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle).ok() }
}
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePort(guestnetworkservice : *const core::ffi::c_void, protocol : HCN_PORT_PROTOCOL, access : HCN_PORT_ACCESS, port : u16, portreservationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        HcnReserveGuestNetworkServicePort(guestnetworkservice, protocol, access, port, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice : *const core::ffi::c_void, portcount : u16, portrangereservation : *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { HcnReserveGuestNetworkServicePortRange(guestnetworkservice, portcount, portrangereservation as _, portreservationhandle as _).ok() }
}
#[inline]
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnUnregisterGuestNetworkServiceCallback(callbackhandle).ok() }
}
#[inline]
pub unsafe fn HcnUnregisterServiceCallback(callbackhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("computenetwork.dll" "system" fn HcnUnregisterServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HcnUnregisterServiceCallback(callbackhandle).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCN_NOTIFICATIONS(pub i32);
pub type HCN_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: u32, context: *const core::ffi::c_void, notificationstatus: windows_core::HRESULT, notificationdata: windows_core::PCWSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCN_PORT_ACCESS(pub i32);
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = HCN_PORT_ACCESS(1i32);
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = HCN_PORT_ACCESS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCN_PORT_PROTOCOL(pub i32);
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(3i32);
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(1i32);
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HCN_PORT_RANGE_ENTRY {
    pub OwningPartitionId: windows_core::GUID,
    pub TargetPartitionId: windows_core::GUID,
    pub Protocol: HCN_PORT_PROTOCOL,
    pub Priority: u64,
    pub ReservationType: u32,
    pub SharingFlags: u32,
    pub DeliveryMode: u32,
    pub StartingPort: u16,
    pub EndingPort: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
pub const HcnNotificationFlagsReserved: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(-268435456i32);
pub const HcnNotificationGuestNetworkServiceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(7i32);
pub const HcnNotificationGuestNetworkServiceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(8i32);
pub const HcnNotificationGuestNetworkServiceInterfaceStateChanged: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(18i32);
pub const HcnNotificationGuestNetworkServiceStateChanged: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(17i32);
pub const HcnNotificationInvalid: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(0i32);
pub const HcnNotificationNamespaceCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(5i32);
pub const HcnNotificationNamespaceDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(6i32);
pub const HcnNotificationNetworkCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(2i32);
pub const HcnNotificationNetworkDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(4i32);
pub const HcnNotificationNetworkEndpointAttached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(9i32);
pub const HcnNotificationNetworkEndpointDetached: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16i32);
pub const HcnNotificationNetworkPreCreate: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(1i32);
pub const HcnNotificationNetworkPreDelete: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(3i32);
pub const HcnNotificationServiceDisconnect: HCN_NOTIFICATIONS = HCN_NOTIFICATIONS(16777216i32);
