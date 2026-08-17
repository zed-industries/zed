#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplication, IRDPSRAPIApplication_Vtbl, 0x41e7a09d_eb7a_436e_935d_780ca2628324);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplication {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplication, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplication {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Windows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShared<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShared)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Shared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShared: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplicationFilter, IRDPSRAPIApplicationFilter_Vtbl, 0xd20f10ca_6637_4f06_b1d5_277ea7e5160d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplicationFilter {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplicationFilter, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> windows_core::Result<IRDPSRAPIApplicationList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Applications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> windows_core::Result<IRDPSRAPIWindowList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Windows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplicationFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIApplicationList, IRDPSRAPIApplicationList_Vtbl, 0xd4b4aeb3_22dc_4837_b3b6_42ea2517849a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIApplicationList {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIApplicationList, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationList {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIApplication> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIApplicationList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendee, IRDPSRAPIAttendee_Vtbl, 0xec0671b3_1b78_4b80_a464_9132247543e3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendee {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendee, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendee {
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoteName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ControlLevel(&self) -> windows_core::Result<CTRL_LEVEL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControlLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControlLevel)(windows_core::Interface::as_raw(self), pnewval).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitation(&self) -> windows_core::Result<IRDPSRAPIInvitation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Invitation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TerminateConnection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TerminateConnection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ConnectivityInfo(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectivityInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ControlLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CTRL_LEVEL) -> windows_core::HRESULT,
    pub SetControlLevel: unsafe extern "system" fn(*mut core::ffi::c_void, CTRL_LEVEL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitation: usize,
    pub TerminateConnection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ConnectivityInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendeeDisconnectInfo, IRDPSRAPIAttendeeDisconnectInfo_Vtbl, 0xc187689f_447c_44a1_9c14_fffbb3b7ec17);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendeeDisconnectInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendeeDisconnectInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeDisconnectInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendee(&self) -> windows_core::Result<IRDPSRAPIAttendee> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attendee)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reason(&self) -> windows_core::Result<ATTENDEE_DISCONNECT_REASON> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Code(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Code)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendee: usize,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ATTENDEE_DISCONNECT_REASON) -> windows_core::HRESULT,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIAttendeeManager, IRDPSRAPIAttendeeManager_Vtbl, 0xba3a37e8_33da_4749_8da0_07fa34da7944);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIAttendeeManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIAttendeeManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, id: i32) -> windows_core::Result<IRDPSRAPIAttendee> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIAttendeeManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
windows_core::imp::define_interface!(IRDPSRAPIAudioStream, IRDPSRAPIAudioStream_Vtbl, 0xe3e30ef9_89c6_4541_ba3b_19336ac6d31c);
impl core::ops::Deref for IRDPSRAPIAudioStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPIAudioStream, windows_core::IUnknown);
impl IRDPSRAPIAudioStream {
    pub unsafe fn Initialize(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppbdata, pcbdata, ptimestamp).ok()
    }
    pub unsafe fn FreeBuffer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRDPSRAPIAudioStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u64) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRDPSRAPIClipboardUseEvents, IRDPSRAPIClipboardUseEvents_Vtbl, 0xd559f59a_7a27_4138_8763_247ce5f659a8);
impl core::ops::Deref for IRDPSRAPIClipboardUseEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPIClipboardUseEvents, windows_core::IUnknown);
impl IRDPSRAPIClipboardUseEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnPasteFromClipboard<P0>(&self, clipboardformat: u32, pattendee: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<super::Com::IDispatch>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnPasteFromClipboard)(windows_core::Interface::as_raw(self), clipboardformat, pattendee.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRDPSRAPIClipboardUseEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnPasteFromClipboard: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnPasteFromClipboard: usize,
}
windows_core::imp::define_interface!(IRDPSRAPIDebug, IRDPSRAPIDebug_Vtbl, 0xaa1e42b5_496d_4ca4_a690_348dcb2ec4ad);
impl core::ops::Deref for IRDPSRAPIDebug {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPIDebug, windows_core::IUnknown);
impl IRDPSRAPIDebug {
    pub unsafe fn SetCLXCmdLine<P0>(&self, clxcmdline: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCLXCmdLine)(windows_core::Interface::as_raw(self), clxcmdline.param().abi()).ok()
    }
    pub unsafe fn CLXCmdLine(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CLXCmdLine)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRDPSRAPIDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCLXCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CLXCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIFrameBuffer, IRDPSRAPIFrameBuffer_Vtbl, 0x3d67e7d2_b27b_448e_81b3_c6110ed8b4be);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIFrameBuffer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIFrameBuffer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIFrameBuffer {
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Bpp(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Bpp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameBufferBits)(windows_core::Interface::as_raw(self), x, y, width, heigth, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIFrameBuffer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Bpp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFrameBufferBits: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFrameBufferBits: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIInvitation, IRDPSRAPIInvitation_Vtbl, 0x4fac1d43_fc51_45bb_b1b4_2b53aa562fa3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIInvitation {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIInvitation, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitation {
    pub unsafe fn ConnectionString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectionString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GroupName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GroupName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Password(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AttendeeLimit(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AttendeeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAttendeeLimit(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttendeeLimit)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Revoked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Revoked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRevoked<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRevoked)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIInvitation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ConnectionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AttendeeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAttendeeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Revoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIInvitationManager, IRDPSRAPIInvitationManager_Vtbl, 0x4722b049_92c3_4c2d_8a65_f7348f644dcf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIInvitationManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIInvitationManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, item: P0) -> windows_core::Result<IRDPSRAPIInvitation>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInvitation<P0, P1, P2>(&self, bstrauthstring: P0, bstrgroupname: P1, bstrpassword: P2, attendeelimit: i32) -> windows_core::Result<IRDPSRAPIInvitation>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInvitation)(windows_core::Interface::as_raw(self), bstrauthstring.param().abi(), bstrgroupname.param().abi(), bstrpassword.param().abi(), attendeelimit, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIInvitationManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateInvitation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateInvitation: usize,
}
windows_core::imp::define_interface!(IRDPSRAPIPerfCounterLogger, IRDPSRAPIPerfCounterLogger_Vtbl, 0x071c2533_0fa4_4e8f_ae83_9c10b4305ab5);
impl core::ops::Deref for IRDPSRAPIPerfCounterLogger {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPIPerfCounterLogger, windows_core::IUnknown);
impl IRDPSRAPIPerfCounterLogger {
    pub unsafe fn LogValue(&self, lvalue: i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LogValue)(windows_core::Interface::as_raw(self), lvalue).ok()
    }
}
#[repr(C)]
pub struct IRDPSRAPIPerfCounterLogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LogValue: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRDPSRAPIPerfCounterLoggingManager, IRDPSRAPIPerfCounterLoggingManager_Vtbl, 0x9a512c86_ac6e_4a8e_b1a4_fcef363f6e64);
impl core::ops::Deref for IRDPSRAPIPerfCounterLoggingManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPIPerfCounterLoggingManager, windows_core::IUnknown);
impl IRDPSRAPIPerfCounterLoggingManager {
    pub unsafe fn CreateLogger<P0>(&self, bstrcountername: P0) -> windows_core::Result<IRDPSRAPIPerfCounterLogger>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLogger)(windows_core::Interface::as_raw(self), bstrcountername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateLogger: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISessionProperties, IRDPSRAPISessionProperties_Vtbl, 0x339b24f2_9bc0_4f16_9aac_f165433d13d4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISessionProperties {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISessionProperties, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISessionProperties {
    pub unsafe fn get_Property<P0>(&self, propertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Property)(windows_core::Interface::as_raw(self), propertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Property<P0, P1>(&self, propertyname: P0, newval: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).put_Property)(windows_core::Interface::as_raw(self), propertyname.param().abi(), newval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISessionProperties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Property: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub put_Property: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISharingSession, IRDPSRAPISharingSession_Vtbl, 0xeeb20886_e470_4cf6_842b_2739c0ec5cfb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISharingSession {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISharingSession, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorDepth)(windows_core::Interface::as_raw(self), colordepth).ok()
    }
    pub unsafe fn ColorDepth(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ColorDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attendees)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Invitations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VirtualChannelManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ConnectToClient<P0>(&self, bstrconnectionstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConnectToClient)(windows_core::Interface::as_raw(self), bstrconnectionstring.param().abi()).ok()
    }
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDesktopSharedRect)(windows_core::Interface::as_raw(self), left, top, right, bottom).ok()
    }
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDesktopSharedRect)(windows_core::Interface::as_raw(self), pleft, ptop, pright, pbottom).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISharingSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorDepth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ColorDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectToClient: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDesktopSharedRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    pub GetDesktopSharedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPISharingSession2, IRDPSRAPISharingSession2_Vtbl, 0xfee4ee57_e3e8_4205_8fb0_8fd1d0675c21);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPISharingSession2 {
    type Target = IRDPSRAPISharingSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPISharingSession2, windows_core::IUnknown, super::Com::IDispatch, IRDPSRAPISharingSession);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession2 {
    pub unsafe fn ConnectUsingTransportStream<P0, P1, P2>(&self, pstream: P0, bstrgroup: P1, bstrauthenticatedattendeename: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStream>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ConnectUsingTransportStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), bstrgroup.param().abi(), bstrauthenticatedattendeename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FrameBuffer(&self) -> windows_core::Result<IRDPSRAPIFrameBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FrameBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendControlLevelChangeResponse<P0>(&self, pattendee: P0, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPIAttendee>,
    {
        (windows_core::Interface::vtable(self).SendControlLevelChangeResponse)(windows_core::Interface::as_raw(self), pattendee.param().abi(), requestedlevel, reasoncode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPISharingSession2_Vtbl {
    pub base__: IRDPSRAPISharingSession_Vtbl,
    pub ConnectUsingTransportStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FrameBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FrameBuffer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendControlLevelChangeResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, CTRL_LEVEL, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendControlLevelChangeResponse: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPITcpConnectionInfo, IRDPSRAPITcpConnectionInfo_Vtbl, 0xf74049a4_3d06_4028_8193_0a8c29bc2452);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPITcpConnectionInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPITcpConnectionInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPITcpConnectionInfo {
    pub unsafe fn Protocol(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Protocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LocalIP(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalIP)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PeerPort(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeerPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PeerIP(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PeerIP)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPITcpConnectionInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LocalIP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PeerPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PeerIP: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRDPSRAPITransportStream, IRDPSRAPITransportStream_Vtbl, 0x36cfa065_43bb_4ef7_aed7_9b88a5053036);
impl core::ops::Deref for IRDPSRAPITransportStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStream, windows_core::IUnknown);
impl IRDPSRAPITransportStream {
    pub unsafe fn AllocBuffer(&self, maxpayload: i32) -> windows_core::Result<IRDPSRAPITransportStreamBuffer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllocBuffer)(windows_core::Interface::as_raw(self), maxpayload, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FreeBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
    pub unsafe fn WriteBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        (windows_core::Interface::vtable(self).WriteBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
    pub unsafe fn ReadBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        (windows_core::Interface::vtable(self).ReadBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
    pub unsafe fn Open<P0>(&self, pcallbacks: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamEvents>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pcallbacks.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRDPSRAPITransportStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllocBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRDPSRAPITransportStreamBuffer, IRDPSRAPITransportStreamBuffer_Vtbl, 0x81c80290_5085_44b0_b460_f865c39cb4a9);
impl core::ops::Deref for IRDPSRAPITransportStreamBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStreamBuffer, windows_core::IUnknown);
impl IRDPSRAPITransportStreamBuffer {
    pub unsafe fn Storage(&self) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Storage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn StorageSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StorageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PayloadSize(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PayloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPayloadSize(&self, lval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPayloadSize)(windows_core::Interface::as_raw(self), lval).ok()
    }
    pub unsafe fn PayloadOffset(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PayloadOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPayloadOffset(&self, lretval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPayloadOffset)(windows_core::Interface::as_raw(self), lretval).ok()
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, lflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Context(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRDPSRAPITransportStreamBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Storage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub StorageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PayloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPayloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PayloadOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPayloadOffset: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRDPSRAPITransportStreamEvents, IRDPSRAPITransportStreamEvents_Vtbl, 0xea81c254_f5af_4e40_982e_3e63bb595276);
impl core::ops::Deref for IRDPSRAPITransportStreamEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPSRAPITransportStreamEvents, windows_core::IUnknown);
impl IRDPSRAPITransportStreamEvents {
    pub unsafe fn OnWriteCompleted<P0>(&self, pbuffer: P0)
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        (windows_core::Interface::vtable(self).OnWriteCompleted)(windows_core::Interface::as_raw(self), pbuffer.param().abi())
    }
    pub unsafe fn OnReadCompleted<P0>(&self, pbuffer: P0)
    where
        P0: windows_core::Param<IRDPSRAPITransportStreamBuffer>,
    {
        (windows_core::Interface::vtable(self).OnReadCompleted)(windows_core::Interface::as_raw(self), pbuffer.param().abi())
    }
    pub unsafe fn OnStreamClosed(&self, hrreason: windows_core::HRESULT) {
        (windows_core::Interface::vtable(self).OnStreamClosed)(windows_core::Interface::as_raw(self), hrreason)
    }
}
#[repr(C)]
pub struct IRDPSRAPITransportStreamEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnWriteCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnReadCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnStreamClosed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIViewer, IRDPSRAPIViewer_Vtbl, 0xc6bfcd38_8ce9_404d_8ae8_f31d00c65cb5);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIViewer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIViewer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIViewer {
    pub unsafe fn Connect<P0, P1, P2>(&self, bstrconnectionstring: P0, bstrname: P1, bstrpassword: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), bstrconnectionstring.param().abi(), bstrname.param().abi(), bstrpassword.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendees(&self) -> windows_core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attendees)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitations(&self) -> windows_core::Result<IRDPSRAPIInvitationManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Invitations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationFilter(&self) -> windows_core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ApplicationFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VirtualChannelManager(&self) -> windows_core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VirtualChannelManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSmartSizing<P0>(&self, vbsmartsizing: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSmartSizing)(windows_core::Interface::as_raw(self), vbsmartsizing.param().abi()).ok()
    }
    pub unsafe fn SmartSizing(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SmartSizing)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestControl)(windows_core::Interface::as_raw(self), ctrllevel).ok()
    }
    pub unsafe fn SetDisconnectedText<P0>(&self, bstrdisconnectedtext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisconnectedText)(windows_core::Interface::as_raw(self), bstrdisconnectedtext.param().abi()).ok()
    }
    pub unsafe fn DisconnectedText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisconnectedText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestColorDepthChange(&self, bpp: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestColorDepthChange)(windows_core::Interface::as_raw(self), bpp).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> windows_core::Result<IRDPSRAPISessionProperties> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StartReverseConnectListener<P0, P1, P2>(&self, bstrconnectionstring: P0, bstrusername: P1, bstrpassword: P2) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartReverseConnectListener)(windows_core::Interface::as_raw(self), bstrconnectionstring.param().abi(), bstrusername.param().abi(), bstrpassword.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIViewer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub SetSmartSizing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SmartSizing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RequestControl: unsafe extern "system" fn(*mut core::ffi::c_void, CTRL_LEVEL) -> windows_core::HRESULT,
    pub SetDisconnectedText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisconnectedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RequestColorDepthChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub StartReverseConnectListener: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIVirtualChannel, IRDPSRAPIVirtualChannel_Vtbl, 0x05e12f95_28b3_4c9a_8780_d0248574a1e0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIVirtualChannel {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIVirtualChannel, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannel {
    pub unsafe fn SendData<P0>(&self, bstrdata: P0, lattendeeid: i32, channelsendflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SendData)(windows_core::Interface::as_raw(self), bstrdata.param().abi(), lattendeeid, channelsendflags).ok()
    }
    pub unsafe fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAccess)(windows_core::Interface::as_raw(self), lattendeeid, accesstype).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<CHANNEL_PRIORITY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIVirtualChannel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SendData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, u32) -> windows_core::HRESULT,
    pub SetAccess: unsafe extern "system" fn(*mut core::ffi::c_void, i32, CHANNEL_ACCESS_ENUM) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CHANNEL_PRIORITY) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIVirtualChannelManager, IRDPSRAPIVirtualChannelManager_Vtbl, 0x0d11c661_5d0d_4ee4_89df_2166ae1fdfed);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIVirtualChannelManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIVirtualChannelManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelManager {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, item: P0) -> windows_core::Result<IRDPSRAPIVirtualChannel>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateVirtualChannel<P0>(&self, bstrchannelname: P0, priority: CHANNEL_PRIORITY, channelflags: u32) -> windows_core::Result<IRDPSRAPIVirtualChannel>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), bstrchannelname.param().abi(), priority, channelflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIVirtualChannelManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, CHANNEL_PRIORITY, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateVirtualChannel: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIWindow, IRDPSRAPIWindow_Vtbl, 0xbeafe0f9_c77b_4933_ba9f_a24cddcc27cf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIWindow {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIWindow, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindow {
    pub unsafe fn Id(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> windows_core::Result<IRDPSRAPIApplication> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Shared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Shared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetShared<P0>(&self, newval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetShared)(windows_core::Interface::as_raw(self), newval.param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Show(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIWindow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
    pub Shared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetShared: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRDPSRAPIWindowList, IRDPSRAPIWindowList_Vtbl, 0x8a05ce44_715a_4116_a189_a118f30a07bd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRDPSRAPIWindowList {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRDPSRAPIWindowList, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowList {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> windows_core::Result<IRDPSRAPIWindow> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), item, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRDPSRAPIWindowList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
windows_core::imp::define_interface!(IRDPViewerInputSink, IRDPViewerInputSink_Vtbl, 0xbb590853_a6c5_4a7b_8dd4_76b69eea12d5);
impl core::ops::Deref for IRDPViewerInputSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRDPViewerInputSink, windows_core::IUnknown);
impl IRDPViewerInputSink {
    pub unsafe fn SendMouseButtonEvent<P0>(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: P0, xpos: u32, ypos: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SendMouseButtonEvent)(windows_core::Interface::as_raw(self), buttontype, vbbuttondown.param().abi(), xpos, ypos).ok()
    }
    pub unsafe fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendMouseMoveEvent)(windows_core::Interface::as_raw(self), xpos, ypos).ok()
    }
    pub unsafe fn SendMouseWheelEvent(&self, wheelrotation: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendMouseWheelEvent)(windows_core::Interface::as_raw(self), wheelrotation).ok()
    }
    pub unsafe fn SendKeyboardEvent<P0, P1, P2>(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: P0, vbrepeat: P1, vbextended: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P2: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SendKeyboardEvent)(windows_core::Interface::as_raw(self), codetype, keycode, vbkeyup.param().abi(), vbrepeat.param().abi(), vbextended.param().abi()).ok()
    }
    pub unsafe fn SendSyncEvent(&self, syncflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendSyncEvent)(windows_core::Interface::as_raw(self), syncflags).ok()
    }
    pub unsafe fn BeginTouchFrame(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginTouchFrame)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddTouchInput)(windows_core::Interface::as_raw(self), contactid, event, x, y).ok()
    }
    pub unsafe fn EndTouchFrame(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndTouchFrame)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRDPViewerInputSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendMouseButtonEvent: unsafe extern "system" fn(*mut core::ffi::c_void, RDPSRAPI_MOUSE_BUTTON_TYPE, super::super::Foundation::VARIANT_BOOL, u32, u32) -> windows_core::HRESULT,
    pub SendMouseMoveEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SendMouseWheelEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SendKeyboardEvent: unsafe extern "system" fn(*mut core::ffi::c_void, RDPSRAPI_KBD_CODE_TYPE, u16, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SendSyncEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BeginTouchFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTouchInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32, i32) -> windows_core::HRESULT,
    pub EndTouchFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_IRDPSessionEvents, _IRDPSessionEvents_Vtbl, 0x98a97042_6698_40e9_8efd_b3200990004b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _IRDPSessionEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_IRDPSessionEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl _IRDPSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _IRDPSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = RDPSRAPI_APP_FLAGS(1i32);
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(1i32);
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = RDPENCOMAPI_ATTENDEE_FLAGS(1i32);
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(0i32);
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(1i32);
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = CHANNEL_FLAGS(4i32);
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = CHANNEL_FLAGS(1i32);
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = CHANNEL_FLAGS(2i32);
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = CHANNEL_PRIORITY(2i32);
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = CHANNEL_PRIORITY(0i32);
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = CHANNEL_PRIORITY(1i32);
pub const CONST_ATTENDEE_ID_DEFAULT: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
pub const CONST_ATTENDEE_ID_EVERYONE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
pub const CONST_ATTENDEE_ID_HOST: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(0i32);
pub const CONST_CONN_INTERVAL: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(50i32);
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(1024i32);
pub const CONST_MAX_CHANNEL_NAME_LEN: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(8i32);
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(409600i32);
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(3i32);
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = CTRL_LEVEL(1i32);
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = CTRL_LEVEL(4i32);
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = CTRL_LEVEL(2i32);
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(0i32);
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(1i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(4i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(8i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(2i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(0i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(2i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(3i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(4i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(5i32);
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = RDPSRAPI_WND_FLAGS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ATTENDEE_DISCONNECT_REASON(pub i32);
impl windows_core::TypeKind for ATTENDEE_DISCONNECT_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ATTENDEE_DISCONNECT_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ATTENDEE_DISCONNECT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANNEL_ACCESS_ENUM(pub i32);
impl windows_core::TypeKind for CHANNEL_ACCESS_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANNEL_ACCESS_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANNEL_ACCESS_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANNEL_FLAGS(pub i32);
impl windows_core::TypeKind for CHANNEL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANNEL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANNEL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANNEL_PRIORITY(pub i32);
impl windows_core::TypeKind for CHANNEL_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANNEL_PRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANNEL_PRIORITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CTRL_LEVEL(pub i32);
impl windows_core::TypeKind for CTRL_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CTRL_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CTRL_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(pub i32);
impl windows_core::TypeKind for RDPENCOMAPI_ATTENDEE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPENCOMAPI_ATTENDEE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPENCOMAPI_CONSTANTS(pub i32);
impl windows_core::TypeKind for RDPENCOMAPI_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPENCOMAPI_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPENCOMAPI_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPSRAPI_APP_FLAGS(pub i32);
impl windows_core::TypeKind for RDPSRAPI_APP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPSRAPI_APP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPSRAPI_APP_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPSRAPI_KBD_CODE_TYPE(pub i32);
impl windows_core::TypeKind for RDPSRAPI_KBD_CODE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPSRAPI_KBD_CODE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPSRAPI_KBD_CODE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(pub i32);
impl windows_core::TypeKind for RDPSRAPI_KBD_SYNC_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPSRAPI_KBD_SYNC_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPSRAPI_KBD_SYNC_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(pub i32);
impl windows_core::TypeKind for RDPSRAPI_MOUSE_BUTTON_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPSRAPI_MOUSE_BUTTON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDPSRAPI_WND_FLAGS(pub i32);
impl windows_core::TypeKind for RDPSRAPI_WND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDPSRAPI_WND_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDPSRAPI_WND_FLAGS").field(&self.0).finish()
    }
}
pub const RDPSRAPIApplication: windows_core::GUID = windows_core::GUID::from_u128(0xc116a484_4b25_4b9f_8a54_b934b06e57fa);
pub const RDPSRAPIApplicationFilter: windows_core::GUID = windows_core::GUID::from_u128(0xe35ace89_c7e8_427e_a4f9_b9da072826bd);
pub const RDPSRAPIApplicationList: windows_core::GUID = windows_core::GUID::from_u128(0x9e31c815_7433_4876_97fb_ed59fe2baa22);
pub const RDPSRAPIAttendee: windows_core::GUID = windows_core::GUID::from_u128(0x74f93bb5_755f_488e_8a29_2390108aef55);
pub const RDPSRAPIAttendeeDisconnectInfo: windows_core::GUID = windows_core::GUID::from_u128(0xb47d7250_5bdb_405d_b487_caad9c56f4f8);
pub const RDPSRAPIAttendeeManager: windows_core::GUID = windows_core::GUID::from_u128(0xd7b13a01_f7d4_42a6_8595_12fc8c24e851);
pub const RDPSRAPIFrameBuffer: windows_core::GUID = windows_core::GUID::from_u128(0xa4f66bcc_538e_4101_951d_30847adb5101);
pub const RDPSRAPIInvitation: windows_core::GUID = windows_core::GUID::from_u128(0x49174dc6_0731_4b5e_8ee1_83a63d3868fa);
pub const RDPSRAPIInvitationManager: windows_core::GUID = windows_core::GUID::from_u128(0x53d9c9db_75ab_4271_948a_4c4eb36a8f2b);
pub const RDPSRAPISessionProperties: windows_core::GUID = windows_core::GUID::from_u128(0xdd7594ff_ea2a_4c06_8fdf_132de48b6510);
pub const RDPSRAPITcpConnectionInfo: windows_core::GUID = windows_core::GUID::from_u128(0xbe49db3f_ebb6_4278_8ce0_d5455833eaee);
pub const RDPSRAPIWindow: windows_core::GUID = windows_core::GUID::from_u128(0x03cf46db_ce45_4d36_86ed_ed28b74398bf);
pub const RDPSRAPIWindowList: windows_core::GUID = windows_core::GUID::from_u128(0x9c21e2b8_5dd4_42cc_81ba_1c099852e6fa);
pub const RDPSession: windows_core::GUID = windows_core::GUID::from_u128(0x9b78f0e6_3e05_4a5b_b2e8_e743a8956b65);
pub const RDPTransportStreamBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x8d4a1c69_f17f_4549_a699_761c6e6b5c0a);
pub const RDPTransportStreamEvents: windows_core::GUID = windows_core::GUID::from_u128(0x31e3ab20_5350_483f_9dc6_6748665efdeb);
pub const RDPViewer: windows_core::GUID = windows_core::GUID::from_u128(0x32be5ed2_5c86_480f_a914_0ff8885a1b3f);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
impl windows_core::TypeKind for __ReferenceRemainingTypes__ {
    type TypeKind = windows_core::CopyType;
}
impl Default for __ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
