#[inline]
pub unsafe fn HcnCloseEndpoint(endpoint: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnCloseEndpoint(endpoint : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnCloseEndpoint(endpoint).ok()
}
#[inline]
pub unsafe fn HcnCloseGuestNetworkService(guestnetworkservice: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnCloseGuestNetworkService(guestnetworkservice : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnCloseGuestNetworkService(guestnetworkservice).ok()
}
#[inline]
pub unsafe fn HcnCloseLoadBalancer(loadbalancer: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnCloseLoadBalancer(loadbalancer : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnCloseLoadBalancer(loadbalancer).ok()
}
#[inline]
pub unsafe fn HcnCloseNamespace(namespace: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnCloseNamespace(namespace : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnCloseNamespace(namespace).ok()
}
#[inline]
pub unsafe fn HcnCloseNetwork(network: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnCloseNetwork(network : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnCloseNetwork(network).ok()
}
#[inline]
pub unsafe fn HcnCreateEndpoint<P0>(network: *const core::ffi::c_void, id: *const windows_core::GUID, settings: P0, endpoint: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnCreateEndpoint(network : *const core::ffi::c_void, id : *const windows_core::GUID, settings : windows_core::PCWSTR, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnCreateEndpoint(network, id, settings.param().abi(), endpoint, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnCreateGuestNetworkService<P0>(id: *const windows_core::GUID, settings: P0, guestnetworkservice: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnCreateGuestNetworkService(id : *const windows_core::GUID, settings : windows_core::PCWSTR, guestnetworkservice : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnCreateGuestNetworkService(id, settings.param().abi(), guestnetworkservice, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnCreateLoadBalancer<P0>(id: *const windows_core::GUID, settings: P0, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnCreateLoadBalancer(id : *const windows_core::GUID, settings : windows_core::PCWSTR, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnCreateLoadBalancer(id, settings.param().abi(), loadbalancer, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnCreateNamespace<P0>(id: *const windows_core::GUID, settings: P0, namespace: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnCreateNamespace(id : *const windows_core::GUID, settings : windows_core::PCWSTR, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnCreateNamespace(id, settings.param().abi(), namespace, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnCreateNetwork<P0>(id: *const windows_core::GUID, settings: P0, network: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnCreateNetwork(id : *const windows_core::GUID, settings : windows_core::PCWSTR, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnCreateNetwork(id, settings.param().abi(), network, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnDeleteEndpoint(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnDeleteEndpoint(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnDeleteEndpoint(id, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnDeleteGuestNetworkService(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnDeleteGuestNetworkService(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnDeleteGuestNetworkService(id, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnDeleteLoadBalancer(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnDeleteLoadBalancer(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnDeleteLoadBalancer(id, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnDeleteNamespace(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnDeleteNamespace(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnDeleteNamespace(id, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnDeleteNetwork(id: *const windows_core::GUID, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnDeleteNetwork(id : *const windows_core::GUID, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnDeleteNetwork(id, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnEnumerateEndpoints<P0>(query: P0, endpoints: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnEnumerateEndpoints(query : windows_core::PCWSTR, endpoints : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnEnumerateEndpoints(query.param().abi(), endpoints, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnEnumerateGuestNetworkPortReservations(returncount: *mut u32, portentries: *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnEnumerateGuestNetworkPortReservations(returncount : *mut u32, portentries : *mut *mut HCN_PORT_RANGE_ENTRY) -> windows_core::HRESULT);
    HcnEnumerateGuestNetworkPortReservations(returncount, portentries).ok()
}
#[inline]
pub unsafe fn HcnEnumerateLoadBalancers<P0>(query: P0, loadbalancer: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnEnumerateLoadBalancers(query : windows_core::PCWSTR, loadbalancer : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnEnumerateLoadBalancers(query.param().abi(), loadbalancer, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnEnumerateNamespaces<P0>(query: P0, namespaces: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnEnumerateNamespaces(query : windows_core::PCWSTR, namespaces : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnEnumerateNamespaces(query.param().abi(), namespaces, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnEnumerateNetworks<P0>(query: P0, networks: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnEnumerateNetworks(query : windows_core::PCWSTR, networks : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnEnumerateNetworks(query.param().abi(), networks, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnFreeGuestNetworkPortReservations(portentries: Option<*mut HCN_PORT_RANGE_ENTRY>) {
    windows_targets::link!("computenetwork.dll" "system" fn HcnFreeGuestNetworkPortReservations(portentries : *mut HCN_PORT_RANGE_ENTRY));
    HcnFreeGuestNetworkPortReservations(core::mem::transmute(portentries.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn HcnModifyEndpoint<P0>(endpoint: *const core::ffi::c_void, settings: P0, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnModifyEndpoint(endpoint : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnModifyEndpoint(endpoint, settings.param().abi(), core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnModifyGuestNetworkService<P0>(guestnetworkservice: *const core::ffi::c_void, settings: P0, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnModifyGuestNetworkService(guestnetworkservice : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnModifyGuestNetworkService(guestnetworkservice, settings.param().abi(), core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnModifyLoadBalancer<P0>(loadbalancer: *const core::ffi::c_void, settings: P0, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnModifyLoadBalancer(loadbalancer : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnModifyLoadBalancer(loadbalancer, settings.param().abi(), core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnModifyNamespace<P0>(namespace: *const core::ffi::c_void, settings: P0, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnModifyNamespace(namespace : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnModifyNamespace(namespace, settings.param().abi(), core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnModifyNetwork<P0>(network: *const core::ffi::c_void, settings: P0, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnModifyNetwork(network : *const core::ffi::c_void, settings : windows_core::PCWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnModifyNetwork(network, settings.param().abi(), core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnOpenEndpoint(id: *const windows_core::GUID, endpoint: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnOpenEndpoint(id : *const windows_core::GUID, endpoint : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnOpenEndpoint(id, endpoint, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnOpenLoadBalancer(id: *const windows_core::GUID, loadbalancer: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnOpenLoadBalancer(id : *const windows_core::GUID, loadbalancer : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnOpenLoadBalancer(id, loadbalancer, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnOpenNamespace(id: *const windows_core::GUID, namespace: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnOpenNamespace(id : *const windows_core::GUID, namespace : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnOpenNamespace(id, namespace, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnOpenNetwork(id: *const windows_core::GUID, network: *mut *mut core::ffi::c_void, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnOpenNetwork(id : *const windows_core::GUID, network : *mut *mut core::ffi::c_void, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnOpenNetwork(id, network, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryEndpointAddresses<P0>(endpoint: *const core::ffi::c_void, query: P0, addresses: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryEndpointAddresses(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, addresses : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryEndpointAddresses(endpoint, query.param().abi(), addresses, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryEndpointProperties<P0>(endpoint: *const core::ffi::c_void, query: P0, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryEndpointProperties(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryEndpointProperties(endpoint, query.param().abi(), properties, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryEndpointStats<P0>(endpoint: *const core::ffi::c_void, query: P0, stats: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryEndpointStats(endpoint : *const core::ffi::c_void, query : windows_core::PCWSTR, stats : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryEndpointStats(endpoint, query.param().abi(), stats, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryLoadBalancerProperties<P0>(loadbalancer: *const core::ffi::c_void, query: P0, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryLoadBalancerProperties(loadbalancer : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryLoadBalancerProperties(loadbalancer, query.param().abi(), properties, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryNamespaceProperties<P0>(namespace: *const core::ffi::c_void, query: P0, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryNamespaceProperties(namespace : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryNamespaceProperties(namespace, query.param().abi(), properties, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnQueryNetworkProperties<P0>(network: *const core::ffi::c_void, query: P0, properties: *mut windows_core::PWSTR, errorrecord: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnQueryNetworkProperties(network : *const core::ffi::c_void, query : windows_core::PCWSTR, properties : *mut windows_core::PWSTR, errorrecord : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    HcnQueryNetworkProperties(network, query.param().abi(), properties, core::mem::transmute(errorrecord.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice: *const core::ffi::c_void, callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnRegisterGuestNetworkServiceCallback(guestnetworkservice : *const core::ffi::c_void, callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    HcnRegisterGuestNetworkServiceCallback(guestnetworkservice, callback, context, callbackhandle).ok()
}
#[inline]
pub unsafe fn HcnRegisterServiceCallback(callback: HCN_NOTIFICATION_CALLBACK, context: *const core::ffi::c_void, callbackhandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnRegisterServiceCallback(callback : HCN_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    HcnRegisterServiceCallback(callback, context, callbackhandle).ok()
}
#[inline]
pub unsafe fn HcnReleaseGuestNetworkServicePortReservationHandle<P0>(portreservationhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("computenetwork.dll" "system" fn HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    HcnReleaseGuestNetworkServicePortReservationHandle(portreservationhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePort(guestnetworkservice: *const core::ffi::c_void, protocol: HCN_PORT_PROTOCOL, access: HCN_PORT_ACCESS, port: u16) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePort(guestnetworkservice : *const core::ffi::c_void, protocol : HCN_PORT_PROTOCOL, access : HCN_PORT_ACCESS, port : u16, portreservationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    HcnReserveGuestNetworkServicePort(guestnetworkservice, protocol, access, port, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice: *const core::ffi::c_void, portcount: u16, portrangereservation: *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnReserveGuestNetworkServicePortRange(guestnetworkservice : *const core::ffi::c_void, portcount : u16, portrangereservation : *mut HCN_PORT_RANGE_RESERVATION, portreservationhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    HcnReserveGuestNetworkServicePortRange(guestnetworkservice, portcount, portrangereservation, portreservationhandle).ok()
}
#[inline]
pub unsafe fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnUnregisterGuestNetworkServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnUnregisterGuestNetworkServiceCallback(callbackhandle).ok()
}
#[inline]
pub unsafe fn HcnUnregisterServiceCallback(callbackhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("computenetwork.dll" "system" fn HcnUnregisterServiceCallback(callbackhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    HcnUnregisterServiceCallback(callbackhandle).ok()
}
pub const HCN_PORT_ACCESS_EXCLUSIVE: HCN_PORT_ACCESS = HCN_PORT_ACCESS(1i32);
pub const HCN_PORT_ACCESS_SHARED: HCN_PORT_ACCESS = HCN_PORT_ACCESS(2i32);
pub const HCN_PORT_PROTOCOL_BOTH: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(3i32);
pub const HCN_PORT_PROTOCOL_TCP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(1i32);
pub const HCN_PORT_PROTOCOL_UDP: HCN_PORT_PROTOCOL = HCN_PORT_PROTOCOL(2i32);
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HCN_NOTIFICATIONS(pub i32);
impl windows_core::TypeKind for HCN_NOTIFICATIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HCN_NOTIFICATIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HCN_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HCN_PORT_ACCESS(pub i32);
impl windows_core::TypeKind for HCN_PORT_ACCESS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HCN_PORT_ACCESS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HCN_PORT_ACCESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HCN_PORT_PROTOCOL(pub i32);
impl windows_core::TypeKind for HCN_PORT_PROTOCOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HCN_PORT_PROTOCOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HCN_PORT_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for HCN_PORT_RANGE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for HCN_PORT_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HCN_PORT_RANGE_RESERVATION {
    pub startingPort: u16,
    pub endingPort: u16,
}
impl windows_core::TypeKind for HCN_PORT_RANGE_RESERVATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for HCN_PORT_RANGE_RESERVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HCN_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(notificationtype: u32, context: *const core::ffi::c_void, notificationstatus: windows_core::HRESULT, notificationdata: windows_core::PCWSTR)>;
