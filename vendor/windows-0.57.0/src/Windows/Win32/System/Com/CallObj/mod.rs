#[inline]
pub unsafe fn CoGetInterceptor<P0>(iidintercepted: *const windows_core::GUID, punkouter: P0, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetInterceptor(iidintercepted : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CoGetInterceptor(iidintercepted, punkouter.param().abi(), iid, ppv).ok()
}
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<P0, P1>(iidintercepted: *const windows_core::GUID, punkouter: P0, typeinfo: P1, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::ITypeInfo>,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetInterceptorFromTypeInfo(iidintercepted : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, typeinfo : * mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CoGetInterceptorFromTypeInfo(iidintercepted, punkouter.param().abi(), typeinfo.param().abi(), iid, ppv).ok()
}
windows_core::imp::define_interface!(ICallFrame, ICallFrame_Vtbl, 0xd573b4b0_894e_11d2_b8b6_00c04fb9618a);
impl core::ops::Deref for ICallFrame {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallFrame, windows_core::IUnknown);
impl ICallFrame {
    pub unsafe fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIIDAndMethod)(windows_core::Interface::as_raw(self), piid, pimethod).ok()
    }
    pub unsafe fn GetNames(&self, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), pwszinterface, pwszmethod).ok()
    }
    pub unsafe fn GetStackLocation(&self) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).GetStackLocation)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetStackLocation(&self, pvstack: *const core::ffi::c_void) {
        (windows_core::Interface::vtable(self).SetStackLocation)(windows_core::Interface::as_raw(self), pvstack)
    }
    pub unsafe fn SetReturnValue(&self, hr: windows_core::HRESULT) {
        (windows_core::Interface::vtable(self).SetReturnValue)(windows_core::Interface::as_raw(self), hr)
    }
    pub unsafe fn GetReturnValue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReturnValue)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> windows_core::Result<CALLFRAMEPARAMINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParamInfo)(windows_core::Interface::as_raw(self), iparam, &mut result__).map(|| result__)
    }
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParam)(windows_core::Interface::as_raw(self), iparam, core::mem::transmute(pvar)).ok()
    }
    pub unsafe fn GetParam(&self, iparam: u32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParam)(windows_core::Interface::as_raw(self), iparam, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Copy<P0>(&self, copycontrol: CALLFRAME_COPY, pwalker: P0) -> windows_core::Result<ICallFrame>
    where
        P0: windows_core::Param<ICallFrameWalker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), copycontrol, pwalker.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Free<P0, P1, P2, P3>(&self, pframeargsdest: P0, pwalkerdestfree: P1, pwalkercopy: P2, freeflags: u32, pwalkerfree: P3, nullflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrame>,
        P1: windows_core::Param<ICallFrameWalker>,
        P2: windows_core::Param<ICallFrameWalker>,
        P3: windows_core::Param<ICallFrameWalker>,
    {
        (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pframeargsdest.param().abi(), pwalkerdestfree.param().abi(), pwalkercopy.param().abi(), freeflags, pwalkerfree.param().abi(), nullflags).ok()
    }
    pub unsafe fn FreeParam<P0>(&self, iparam: u32, freeflags: u32, pwalkerfree: P0, nullflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrameWalker>,
    {
        (windows_core::Interface::vtable(self).FreeParam)(windows_core::Interface::as_raw(self), iparam, freeflags, pwalkerfree.param().abi(), nullflags).ok()
    }
    pub unsafe fn WalkFrame<P0>(&self, walkwhat: u32, pwalker: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrameWalker>,
    {
        (windows_core::Interface::vtable(self).WalkFrame)(windows_core::Interface::as_raw(self), walkwhat, pwalker.param().abi()).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMarshalSizeMax)(windows_core::Interface::as_raw(self), pmshlcontext, mshlflags, &mut result__).map(|| result__)
    }
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: &[u8], pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Marshal)(windows_core::Interface::as_raw(self), pmshlcontext, mshlflags, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pcbbufferused, pdatarep, prpcflags).ok()
    }
    pub unsafe fn Unmarshal(&self, pbuffer: &[u8], datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), datarep, pcontext, &mut result__).map(|| result__)
    }
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), ibfirstrelease, datarep, pcontext).ok()
    }
    pub unsafe fn Invoke(&self, pvreceiver: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvreceiver).ok()
    }
}
#[repr(C)]
pub struct ICallFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CALLFRAMEINFO) -> windows_core::HRESULT,
    pub GetIIDAndMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStackLocation: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub SetStackLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    pub SetReturnValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
    pub GetReturnValue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParamInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CALLFRAMEPARAMINFO) -> windows_core::HRESULT,
    pub SetParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, CALLFRAME_COPY, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FreeParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WalkFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *const CALLFRAME_MARSHALCONTEXT, super::MSHLFLAGS, *mut u32) -> windows_core::HRESULT,
    pub Marshal: unsafe extern "system" fn(*mut core::ffi::c_void, *const CALLFRAME_MARSHALCONTEXT, super::MSHLFLAGS, *const core::ffi::c_void, u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32, *const CALLFRAME_MARSHALCONTEXT, *mut u32) -> windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32, u32, *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallFrameEvents, ICallFrameEvents_Vtbl, 0xfd5e0843_fc91_11d0_97d7_00c04fb9618a);
impl core::ops::Deref for ICallFrameEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallFrameEvents, windows_core::IUnknown);
impl ICallFrameEvents {
    pub unsafe fn OnCall<P0>(&self, pframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrame>,
    {
        (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self), pframe.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICallFrameEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallFrameWalker, ICallFrameWalker_Vtbl, 0x08b23919_392d_11d2_b8a4_00c04fb9618a);
impl core::ops::Deref for ICallFrameWalker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallFrameWalker, windows_core::IUnknown);
impl ICallFrameWalker {
    pub unsafe fn OnWalkInterface<P0, P1>(&self, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: P0, fout: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnWalkInterface)(windows_core::Interface::as_raw(self), iid, ppvinterface, fin.param().abi(), fout.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICallFrameWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnWalkInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const core::ffi::c_void, super::super::super::Foundation::BOOL, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallIndirect, ICallIndirect_Vtbl, 0xd573b4b1_894e_11d2_b8b6_00c04fb9618a);
impl core::ops::Deref for ICallIndirect {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallIndirect, windows_core::IUnknown);
impl ICallIndirect {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CallIndirect)(windows_core::Interface::as_raw(self), phrreturn, imethod, pvargs, cbargs).ok()
    }
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMethodInfo)(windows_core::Interface::as_raw(self), imethod, pinfo, pwszmethod).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStackSize)(windows_core::Interface::as_raw(self), imethod, &mut result__).map(|| result__)
    }
    pub unsafe fn GetIID(&self, piid: Option<*mut windows_core::GUID>, pfderivesfromidispatch: Option<*mut super::super::super::Foundation::BOOL>, pcmethod: Option<*mut u32>, pwszinterface: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), core::mem::transmute(piid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pfderivesfromidispatch.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcmethod.unwrap_or(std::ptr::null_mut())), pwszinterface).ok()
    }
}
#[repr(C)]
pub struct ICallIndirect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CallIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, u32, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMethodInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CALLFRAMEINFO, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut super::super::super::Foundation::BOOL, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallInterceptor, ICallInterceptor_Vtbl, 0x60c7ca75_896d_11d2_b8b6_00c04fb9618a);
impl core::ops::Deref for ICallInterceptor {
    type Target = ICallIndirect;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallInterceptor, windows_core::IUnknown, ICallIndirect);
impl ICallInterceptor {
    pub unsafe fn RegisterSink<P0>(&self, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrameEvents>,
    {
        (windows_core::Interface::vtable(self).RegisterSink)(windows_core::Interface::as_raw(self), psink.param().abi()).ok()
    }
    pub unsafe fn GetRegisteredSink(&self) -> windows_core::Result<ICallFrameEvents> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisteredSink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICallInterceptor_Vtbl {
    pub base__: ICallIndirect_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallUnmarshal, ICallUnmarshal_Vtbl, 0x5333b003_2e42_11d2_b89d_00c04fb9618a);
impl core::ops::Deref for ICallUnmarshal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallUnmarshal, windows_core::IUnknown);
impl ICallUnmarshal {
    pub unsafe fn Unmarshal<P0>(&self, imethod: u32, pbuffer: &[u8], fforcebuffercopy: P0, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut Option<ICallFrame>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), imethod, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), fforcebuffercopy.param().abi(), datarep, pcontext, pcbunmarshalled, core::mem::transmute(ppframe)).ok()
    }
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), imethod, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), ibfirstrelease, datarep, pcontext).ok()
    }
}
#[repr(C)]
pub struct ICallUnmarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, super::super::super::Foundation::BOOL, u32, *const CALLFRAME_MARSHALCONTEXT, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32, u32, *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInterfaceRelated, IInterfaceRelated_Vtbl, 0xd1fb5a79_7706_11d1_adba_00c04fc2adc0);
impl core::ops::Deref for IInterfaceRelated {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInterfaceRelated, windows_core::IUnknown);
impl IInterfaceRelated {
    pub unsafe fn SetIID(&self, iid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIID)(windows_core::Interface::as_raw(self), iid).ok()
    }
    pub unsafe fn GetIID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IInterfaceRelated_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLFRAME_COPY(pub i32);
impl windows_core::TypeKind for CALLFRAME_COPY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLFRAME_COPY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLFRAME_COPY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLFRAME_FREE(pub i32);
impl windows_core::TypeKind for CALLFRAME_FREE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLFRAME_FREE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLFRAME_FREE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLFRAME_NULL(pub i32);
impl windows_core::TypeKind for CALLFRAME_NULL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLFRAME_NULL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLFRAME_NULL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLFRAME_WALK(pub i32);
impl windows_core::TypeKind for CALLFRAME_WALK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLFRAME_WALK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLFRAME_WALK").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: windows_core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
impl windows_core::TypeKind for CALLFRAMEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
impl windows_core::TypeKind for CALLFRAMEPARAMINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub punkReserved: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub guidTransferSyntax: windows_core::GUID,
}
impl Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CALLFRAME_MARSHALCONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
