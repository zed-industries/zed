#[inline]
pub unsafe fn WebSocketAbortHandle<P0>(hwebsocket: P0)
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketAbortHandle(hwebsocket : WEB_SOCKET_HANDLE));
    WebSocketAbortHandle(hwebsocket.param().abi())
}
#[inline]
pub unsafe fn WebSocketBeginClientHandshake<P0>(hwebsocket: P0, pszsubprotocols: Option<&[windows_core::PCSTR]>, pszextensions: Option<&[windows_core::PCSTR]>, pinitialheaders: Option<&[WEB_SOCKET_HTTP_HEADER]>, padditionalheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketBeginClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocols : *const windows_core::PCSTR, ulsubprotocolcount : u32, pszextensions : *const windows_core::PCSTR, ulextensioncount : u32, pinitialheaders : *const WEB_SOCKET_HTTP_HEADER, ulinitialheadercount : u32, padditionalheaders : *mut *mut WEB_SOCKET_HTTP_HEADER, puladditionalheadercount : *mut u32) -> windows_core::HRESULT);
    WebSocketBeginClientHandshake(
        hwebsocket.param().abi(),
        core::mem::transmute(pszsubprotocols.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pszsubprotocols.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pszextensions.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pszextensions.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        core::mem::transmute(pinitialheaders.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pinitialheaders.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        padditionalheaders,
        puladditionalheadercount,
    )
    .ok()
}
#[inline]
pub unsafe fn WebSocketBeginServerHandshake<P0, P1>(hwebsocket: P0, pszsubprotocolselected: P1, pszextensionselected: Option<&[windows_core::PCSTR]>, prequestheaders: &[WEB_SOCKET_HTTP_HEADER], presponseheaders: *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketBeginServerHandshake(hwebsocket : WEB_SOCKET_HANDLE, pszsubprotocolselected : windows_core::PCSTR, pszextensionselected : *const windows_core::PCSTR, ulextensionselectedcount : u32, prequestheaders : *const WEB_SOCKET_HTTP_HEADER, ulrequestheadercount : u32, presponseheaders : *mut *mut WEB_SOCKET_HTTP_HEADER, pulresponseheadercount : *mut u32) -> windows_core::HRESULT);
    WebSocketBeginServerHandshake(hwebsocket.param().abi(), pszsubprotocolselected.param().abi(), core::mem::transmute(pszextensionselected.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszextensionselected.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestheaders.as_ptr()), prequestheaders.len().try_into().unwrap(), presponseheaders, pulresponseheadercount).ok()
}
#[inline]
pub unsafe fn WebSocketCompleteAction<P0>(hwebsocket: P0, pvactioncontext: *const core::ffi::c_void, ulbytestransferred: u32)
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketCompleteAction(hwebsocket : WEB_SOCKET_HANDLE, pvactioncontext : *const core::ffi::c_void, ulbytestransferred : u32));
    WebSocketCompleteAction(hwebsocket.param().abi(), pvactioncontext, ulbytestransferred)
}
#[inline]
pub unsafe fn WebSocketCreateClientHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_targets::link!("websocket.dll" "system" fn WebSocketCreateClientHandle(pproperties : *const WEB_SOCKET_PROPERTY, ulpropertycount : u32, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WebSocketCreateClientHandle(core::mem::transmute(pproperties.as_ptr()), pproperties.len().try_into().unwrap(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WebSocketCreateServerHandle(pproperties: &[WEB_SOCKET_PROPERTY]) -> windows_core::Result<WEB_SOCKET_HANDLE> {
    windows_targets::link!("websocket.dll" "system" fn WebSocketCreateServerHandle(pproperties : *const WEB_SOCKET_PROPERTY, ulpropertycount : u32, phwebsocket : *mut WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    WebSocketCreateServerHandle(core::mem::transmute(pproperties.as_ptr()), pproperties.len().try_into().unwrap(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn WebSocketDeleteHandle<P0>(hwebsocket: P0)
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketDeleteHandle(hwebsocket : WEB_SOCKET_HANDLE));
    WebSocketDeleteHandle(hwebsocket.param().abi())
}
#[inline]
pub unsafe fn WebSocketEndClientHandshake<P0>(hwebsocket: P0, presponseheaders: &[WEB_SOCKET_HTTP_HEADER], pulselectedextensions: Option<*mut u32>, pulselectedextensioncount: Option<*mut u32>, pulselectedsubprotocol: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketEndClientHandshake(hwebsocket : WEB_SOCKET_HANDLE, presponseheaders : *const WEB_SOCKET_HTTP_HEADER, ulreponseheadercount : u32, pulselectedextensions : *mut u32, pulselectedextensioncount : *mut u32, pulselectedsubprotocol : *mut u32) -> windows_core::HRESULT);
    WebSocketEndClientHandshake(hwebsocket.param().abi(), core::mem::transmute(presponseheaders.as_ptr()), presponseheaders.len().try_into().unwrap(), core::mem::transmute(pulselectedextensions.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pulselectedextensioncount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pulselectedsubprotocol.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn WebSocketEndServerHandshake<P0>(hwebsocket: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketEndServerHandshake(hwebsocket : WEB_SOCKET_HANDLE) -> windows_core::HRESULT);
    WebSocketEndServerHandshake(hwebsocket.param().abi()).ok()
}
#[inline]
pub unsafe fn WebSocketGetAction<P0>(hwebsocket: P0, eactionqueue: WEB_SOCKET_ACTION_QUEUE, pdatabuffers: *mut WEB_SOCKET_BUFFER, puldatabuffercount: *mut u32, paction: *mut WEB_SOCKET_ACTION, pbuffertype: *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext: Option<*mut *mut core::ffi::c_void>, pvactioncontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketGetAction(hwebsocket : WEB_SOCKET_HANDLE, eactionqueue : WEB_SOCKET_ACTION_QUEUE, pdatabuffers : *mut WEB_SOCKET_BUFFER, puldatabuffercount : *mut u32, paction : *mut WEB_SOCKET_ACTION, pbuffertype : *mut WEB_SOCKET_BUFFER_TYPE, pvapplicationcontext : *mut *mut core::ffi::c_void, pvactioncontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    WebSocketGetAction(hwebsocket.param().abi(), eactionqueue, pdatabuffers, puldatabuffercount, paction, pbuffertype, core::mem::transmute(pvapplicationcontext.unwrap_or(std::ptr::null_mut())), pvactioncontext).ok()
}
#[inline]
pub unsafe fn WebSocketGetGlobalProperty(etype: WEB_SOCKET_PROPERTY_TYPE, pvvalue: *mut core::ffi::c_void, ulsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("websocket.dll" "system" fn WebSocketGetGlobalProperty(etype : WEB_SOCKET_PROPERTY_TYPE, pvvalue : *mut core::ffi::c_void, ulsize : *mut u32) -> windows_core::HRESULT);
    WebSocketGetGlobalProperty(etype, pvvalue, ulsize).ok()
}
#[inline]
pub unsafe fn WebSocketReceive<P0>(hwebsocket: P0, pbuffer: Option<*const WEB_SOCKET_BUFFER>, pvcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketReceive(hwebsocket : WEB_SOCKET_HANDLE, pbuffer : *const WEB_SOCKET_BUFFER, pvcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    WebSocketReceive(hwebsocket.param().abi(), core::mem::transmute(pbuffer.unwrap_or(std::ptr::null())), core::mem::transmute(pvcontext.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WebSocketSend<P0>(hwebsocket: P0, buffertype: WEB_SOCKET_BUFFER_TYPE, pbuffer: Option<*const WEB_SOCKET_BUFFER>, context: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<WEB_SOCKET_HANDLE>,
{
    windows_targets::link!("websocket.dll" "system" fn WebSocketSend(hwebsocket : WEB_SOCKET_HANDLE, buffertype : WEB_SOCKET_BUFFER_TYPE, pbuffer : *const WEB_SOCKET_BUFFER, context : *const core::ffi::c_void) -> windows_core::HRESULT);
    WebSocketSend(hwebsocket.param().abi(), buffertype, core::mem::transmute(pbuffer.unwrap_or(std::ptr::null())), core::mem::transmute(context.unwrap_or(std::ptr::null()))).ok()
}
pub const WEB_SOCKET_ABORTED_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1006i32);
pub const WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(3i32);
pub const WEB_SOCKET_ALL_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(3i32);
pub const WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483645i32);
pub const WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483646i32);
pub const WEB_SOCKET_CLOSE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483644i32);
pub const WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(2i32);
pub const WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(4i32);
pub const WEB_SOCKET_EMPTY_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1001i32);
pub const WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(4i32);
pub const WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(2i32);
pub const WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(5i32);
pub const WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
pub const WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const WEB_SOCKET_NO_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(0i32);
pub const WEB_SOCKET_PING_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483643i32);
pub const WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1008i32);
pub const WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const WEB_SOCKET_RECEIVE_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(2i32);
pub const WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(0i32);
pub const WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(3i32);
pub const WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1015i32);
pub const WEB_SOCKET_SEND_ACTION_QUEUE: WEB_SOCKET_ACTION_QUEUE = WEB_SOCKET_ACTION_QUEUE(1i32);
pub const WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(1i32);
pub const WEB_SOCKET_SEND_TO_NETWORK_ACTION: WEB_SOCKET_ACTION = WEB_SOCKET_ACTION(1i32);
pub const WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const WEB_SOCKET_SUCCESS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE: WEB_SOCKET_PROPERTY_TYPE = WEB_SOCKET_PROPERTY_TYPE(6i32);
pub const WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483642i32);
pub const WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: WEB_SOCKET_CLOSE_STATUS = WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483647i32);
pub const WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE: WEB_SOCKET_BUFFER_TYPE = WEB_SOCKET_BUFFER_TYPE(-2147483648i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEB_SOCKET_ACTION(pub i32);
impl windows_core::TypeKind for WEB_SOCKET_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEB_SOCKET_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEB_SOCKET_ACTION_QUEUE(pub i32);
impl windows_core::TypeKind for WEB_SOCKET_ACTION_QUEUE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEB_SOCKET_ACTION_QUEUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_ACTION_QUEUE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEB_SOCKET_BUFFER_TYPE(pub i32);
impl windows_core::TypeKind for WEB_SOCKET_BUFFER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEB_SOCKET_BUFFER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEB_SOCKET_CLOSE_STATUS(pub i32);
impl windows_core::TypeKind for WEB_SOCKET_CLOSE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEB_SOCKET_CLOSE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_CLOSE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WEB_SOCKET_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for WEB_SOCKET_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WEB_SOCKET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WEB_SOCKET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WEB_SOCKET_BUFFER {
    pub Data: WEB_SOCKET_BUFFER_1,
    pub CloseStatus: WEB_SOCKET_BUFFER_0,
}
impl windows_core::TypeKind for WEB_SOCKET_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEB_SOCKET_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WEB_SOCKET_BUFFER_0 {
    pub pbReason: *mut u8,
    pub ulReasonLength: u32,
    pub usStatus: u16,
}
impl windows_core::TypeKind for WEB_SOCKET_BUFFER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEB_SOCKET_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WEB_SOCKET_BUFFER_1 {
    pub pbBuffer: *mut u8,
    pub ulBufferLength: u32,
}
impl windows_core::TypeKind for WEB_SOCKET_BUFFER_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEB_SOCKET_BUFFER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WEB_SOCKET_HANDLE(pub *mut core::ffi::c_void);
impl WEB_SOCKET_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for WEB_SOCKET_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            WebSocketDeleteHandle(*self);
        }
    }
}
impl Default for WEB_SOCKET_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WEB_SOCKET_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WEB_SOCKET_HTTP_HEADER {
    pub pcName: windows_core::PSTR,
    pub ulNameLength: u32,
    pub pcValue: windows_core::PSTR,
    pub ulValueLength: u32,
}
impl windows_core::TypeKind for WEB_SOCKET_HTTP_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEB_SOCKET_HTTP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WEB_SOCKET_PROPERTY {
    pub Type: WEB_SOCKET_PROPERTY_TYPE,
    pub pvValue: *mut core::ffi::c_void,
    pub ulValueSize: u32,
}
impl windows_core::TypeKind for WEB_SOCKET_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEB_SOCKET_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
