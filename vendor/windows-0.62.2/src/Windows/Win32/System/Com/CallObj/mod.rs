#[inline]
pub unsafe fn CoGetInterceptor<P1>(iidintercepted: *const windows_core::GUID, punkouter: P1, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInterceptor(iidintercepted : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetInterceptor(iidintercepted, punkouter.param().abi(), iid, ppv as _).ok() }
}
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<P1, P2>(iidintercepted: *const windows_core::GUID, punkouter: P1, typeinfo: P2, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<super::ITypeInfo>,
{
    windows_core::link!("ole32.dll" "system" fn CoGetInterceptorFromTypeInfo(iidintercepted : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, typeinfo : * mut core::ffi::c_void, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoGetInterceptorFromTypeInfo(iidintercepted, punkouter.param().abi(), typeinfo.param().abi(), iid, ppv as _).ok() }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: windows_core::BOOL,
    pub fHasInOutValues: windows_core::BOOL,
    pub fHasOutValues: windows_core::BOOL,
    pub fDerivesFromIDispatch: windows_core::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: windows_core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: bool,
    pub fOut: bool,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLFRAME_COPY(pub i32);
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLFRAME_FREE(pub i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: bool,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub punkReserved: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub guidTransferSyntax: windows_core::GUID,
}
impl Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLFRAME_NULL(pub i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALLFRAME_WALK(pub i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
windows_core::imp::define_interface!(ICallFrame, ICallFrame_Vtbl, 0xd573b4b0_894e_11d2_b8b6_00c04fb9618a);
windows_core::imp::interface_hierarchy!(ICallFrame, windows_core::IUnknown);
impl ICallFrame {
    pub unsafe fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), pinfo as _).ok() }
    }
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIIDAndMethod)(windows_core::Interface::as_raw(self), piid as _, pimethod as _).ok() }
    }
    pub unsafe fn GetNames(&self, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), pwszinterface as _, pwszmethod as _).ok() }
    }
    pub unsafe fn GetStackLocation(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).GetStackLocation)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStackLocation(&self, pvstack: *const core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).SetStackLocation)(windows_core::Interface::as_raw(self), pvstack) }
    }
    pub unsafe fn SetReturnValue(&self, hr: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).SetReturnValue)(windows_core::Interface::as_raw(self), hr) }
    }
    pub unsafe fn GetReturnValue(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetReturnValue)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> windows_core::Result<CALLFRAMEPARAMINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParamInfo)(windows_core::Interface::as_raw(self), iparam, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const super::super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParam)(windows_core::Interface::as_raw(self), iparam, core::mem::transmute(pvar)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetParam(&self, iparam: u32) -> windows_core::Result<super::super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParam)(windows_core::Interface::as_raw(self), iparam, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Copy<P1>(&self, copycontrol: CALLFRAME_COPY, pwalker: P1) -> windows_core::Result<ICallFrame>
    where
        P1: windows_core::Param<ICallFrameWalker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), copycontrol, pwalker.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Free<P0, P1, P2, P4>(&self, pframeargsdest: P0, pwalkerdestfree: P1, pwalkercopy: P2, freeflags: u32, pwalkerfree: P4, nullflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrame>,
        P1: windows_core::Param<ICallFrameWalker>,
        P2: windows_core::Param<ICallFrameWalker>,
        P4: windows_core::Param<ICallFrameWalker>,
    {
        unsafe { (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pframeargsdest.param().abi(), pwalkerdestfree.param().abi(), pwalkercopy.param().abi(), freeflags, pwalkerfree.param().abi(), nullflags).ok() }
    }
    pub unsafe fn FreeParam<P2>(&self, iparam: u32, freeflags: u32, pwalkerfree: P2, nullflags: u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ICallFrameWalker>,
    {
        unsafe { (windows_core::Interface::vtable(self).FreeParam)(windows_core::Interface::as_raw(self), iparam, freeflags, pwalkerfree.param().abi(), nullflags).ok() }
    }
    pub unsafe fn WalkFrame<P1>(&self, walkwhat: u32, pwalker: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ICallFrameWalker>,
    {
        unsafe { (windows_core::Interface::vtable(self).WalkFrame)(windows_core::Interface::as_raw(self), walkwhat, pwalker.param().abi()).ok() }
    }
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMarshalSizeMax)(windows_core::Interface::as_raw(self), core::mem::transmute(pmshlcontext), mshlflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: &[u8], pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Marshal)(windows_core::Interface::as_raw(self), core::mem::transmute(pmshlcontext), mshlflags, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pcbbufferused as _, pdatarep as _, prpcflags as _).ok() }
    }
    pub unsafe fn Unmarshal(&self, pbuffer: &[u8], datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), datarep, core::mem::transmute(pcontext), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), ibfirstrelease, datarep, core::mem::transmute(pcontext)).ok() }
    }
    pub unsafe fn Invoke(&self, pvreceiver: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvreceiver).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetParam: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetParam: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetParam: usize,
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
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICallFrame_Impl: windows_core::IUnknownImpl {
    fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> windows_core::Result<()>;
    fn GetIIDAndMethod(&self, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::Result<()>;
    fn GetNames(&self, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetStackLocation(&self) -> *mut core::ffi::c_void;
    fn SetStackLocation(&self, pvstack: *const core::ffi::c_void);
    fn SetReturnValue(&self, hr: windows_core::HRESULT);
    fn GetReturnValue(&self) -> windows_core::Result<()>;
    fn GetParamInfo(&self, iparam: u32) -> windows_core::Result<CALLFRAMEPARAMINFO>;
    fn SetParam(&self, iparam: u32, pvar: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetParam(&self, iparam: u32) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn Copy(&self, copycontrol: CALLFRAME_COPY, pwalker: windows_core::Ref<ICallFrameWalker>) -> windows_core::Result<ICallFrame>;
    fn Free(&self, pframeargsdest: windows_core::Ref<ICallFrame>, pwalkerdestfree: windows_core::Ref<ICallFrameWalker>, pwalkercopy: windows_core::Ref<ICallFrameWalker>, freeflags: u32, pwalkerfree: windows_core::Ref<ICallFrameWalker>, nullflags: u32) -> windows_core::Result<()>;
    fn FreeParam(&self, iparam: u32, freeflags: u32, pwalkerfree: windows_core::Ref<ICallFrameWalker>, nullflags: u32) -> windows_core::Result<()>;
    fn WalkFrame(&self, walkwhat: u32, pwalker: windows_core::Ref<ICallFrameWalker>) -> windows_core::Result<()>;
    fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> windows_core::Result<u32>;
    fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::Result<()>;
    fn Unmarshal(&self, pbuffer: *const core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<u32>;
    fn ReleaseMarshalData(&self, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()>;
    fn Invoke(&self, pvreceiver: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICallFrame_Vtbl {
    pub const fn new<Identity: ICallFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInfo<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn GetIIDAndMethod<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::GetIIDAndMethod(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pimethod)).into()
            }
        }
        unsafe extern "system" fn GetNames<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::GetNames(this, core::mem::transmute_copy(&pwszinterface), core::mem::transmute_copy(&pwszmethod)).into()
            }
        }
        unsafe extern "system" fn GetStackLocation<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::GetStackLocation(this)
            }
        }
        unsafe extern "system" fn SetStackLocation<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvstack: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::SetStackLocation(this, core::mem::transmute_copy(&pvstack))
            }
        }
        unsafe extern "system" fn SetReturnValue<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::SetReturnValue(this, core::mem::transmute_copy(&hr))
            }
        }
        unsafe extern "system" fn GetReturnValue<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::GetReturnValue(this).into()
            }
        }
        unsafe extern "system" fn GetParamInfo<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFrame_Impl::GetParamInfo(this, core::mem::transmute_copy(&iparam)) {
                    Ok(ok__) => {
                        pinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParam<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pvar: *const super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::SetParam(this, core::mem::transmute_copy(&iparam), core::mem::transmute_copy(&pvar)).into()
            }
        }
        unsafe extern "system" fn GetParam<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pvar: *mut super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFrame_Impl::GetParam(this, core::mem::transmute_copy(&iparam)) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Copy<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: *mut core::ffi::c_void, ppframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFrame_Impl::Copy(this, core::mem::transmute_copy(&copycontrol), core::mem::transmute_copy(&pwalker)) {
                    Ok(ok__) => {
                        ppframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Free<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframeargsdest: *mut core::ffi::c_void, pwalkerdestfree: *mut core::ffi::c_void, pwalkercopy: *mut core::ffi::c_void, freeflags: u32, pwalkerfree: *mut core::ffi::c_void, nullflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::Free(this, core::mem::transmute_copy(&pframeargsdest), core::mem::transmute_copy(&pwalkerdestfree), core::mem::transmute_copy(&pwalkercopy), core::mem::transmute_copy(&freeflags), core::mem::transmute_copy(&pwalkerfree), core::mem::transmute_copy(&nullflags)).into()
            }
        }
        unsafe extern "system" fn FreeParam<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: *mut core::ffi::c_void, nullflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::FreeParam(this, core::mem::transmute_copy(&iparam), core::mem::transmute_copy(&freeflags), core::mem::transmute_copy(&pwalkerfree), core::mem::transmute_copy(&nullflags)).into()
            }
        }
        unsafe extern "system" fn WalkFrame<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walkwhat: u32, pwalker: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::WalkFrame(this, core::mem::transmute_copy(&walkwhat), core::mem::transmute_copy(&pwalker)).into()
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFrame_Impl::GetMarshalSizeMax(this, core::mem::transmute_copy(&pmshlcontext), core::mem::transmute_copy(&mshlflags)) {
                    Ok(ok__) => {
                        pcbbufferneeded.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Marshal<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::Marshal(this, core::mem::transmute_copy(&pmshlcontext), core::mem::transmute_copy(&mshlflags), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcbbufferused), core::mem::transmute_copy(&pdatarep), core::mem::transmute_copy(&prpcflags)).into()
            }
        }
        unsafe extern "system" fn Unmarshal<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFrame_Impl::Unmarshal(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)) {
                    Ok(ok__) => {
                        pcbunmarshalled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::ReleaseMarshalData(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ibfirstrelease), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: ICallFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvreceiver: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrame_Impl::Invoke(this, core::mem::transmute_copy(&pvreceiver)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetIIDAndMethod: GetIIDAndMethod::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetStackLocation: GetStackLocation::<Identity, OFFSET>,
            SetStackLocation: SetStackLocation::<Identity, OFFSET>,
            SetReturnValue: SetReturnValue::<Identity, OFFSET>,
            GetReturnValue: GetReturnValue::<Identity, OFFSET>,
            GetParamInfo: GetParamInfo::<Identity, OFFSET>,
            SetParam: SetParam::<Identity, OFFSET>,
            GetParam: GetParam::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            FreeParam: FreeParam::<Identity, OFFSET>,
            WalkFrame: WalkFrame::<Identity, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, OFFSET>,
            Marshal: Marshal::<Identity, OFFSET>,
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrame as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICallFrame {}
windows_core::imp::define_interface!(ICallFrameEvents, ICallFrameEvents_Vtbl, 0xfd5e0843_fc91_11d0_97d7_00c04fb9618a);
windows_core::imp::interface_hierarchy!(ICallFrameEvents, windows_core::IUnknown);
impl ICallFrameEvents {
    pub unsafe fn OnCall<P0>(&self, pframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICallFrame>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCall)(windows_core::Interface::as_raw(self), pframe.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICallFrameEvents_Impl: windows_core::IUnknownImpl {
    fn OnCall(&self, pframe: windows_core::Ref<ICallFrame>) -> windows_core::Result<()>;
}
impl ICallFrameEvents_Vtbl {
    pub const fn new<Identity: ICallFrameEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCall<Identity: ICallFrameEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrameEvents_Impl::OnCall(this, core::mem::transmute_copy(&pframe)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrameEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallFrameEvents {}
windows_core::imp::define_interface!(ICallFrameWalker, ICallFrameWalker_Vtbl, 0x08b23919_392d_11d2_b8a4_00c04fb9618a);
windows_core::imp::interface_hierarchy!(ICallFrameWalker, windows_core::IUnknown);
impl ICallFrameWalker {
    pub unsafe fn OnWalkInterface(&self, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: bool, fout: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnWalkInterface)(windows_core::Interface::as_raw(self), iid, ppvinterface, fin.into(), fout.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnWalkInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ICallFrameWalker_Impl: windows_core::IUnknownImpl {
    fn OnWalkInterface(&self, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: windows_core::BOOL, fout: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ICallFrameWalker_Vtbl {
    pub const fn new<Identity: ICallFrameWalker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnWalkInterface<Identity: ICallFrameWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: windows_core::BOOL, fout: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFrameWalker_Impl::OnWalkInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvinterface), core::mem::transmute_copy(&fin), core::mem::transmute_copy(&fout)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWalkInterface: OnWalkInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrameWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallFrameWalker {}
windows_core::imp::define_interface!(ICallIndirect, ICallIndirect_Vtbl, 0xd573b4b1_894e_11d2_b8b6_00c04fb9618a);
windows_core::imp::interface_hierarchy!(ICallIndirect, windows_core::IUnknown);
impl ICallIndirect {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CallIndirect)(windows_core::Interface::as_raw(self), phrreturn as _, imethod, pvargs, cbargs as _).ok() }
    }
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMethodInfo)(windows_core::Interface::as_raw(self), imethod, pinfo as _, pwszmethod as _).ok() }
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStackSize)(windows_core::Interface::as_raw(self), imethod, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIID(&self, piid: Option<*mut windows_core::GUID>, pfderivesfromidispatch: Option<*mut windows_core::BOOL>, pcmethod: Option<*mut u32>, pwszinterface: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), piid.unwrap_or(core::mem::zeroed()) as _, pfderivesfromidispatch.unwrap_or(core::mem::zeroed()) as _, pcmethod.unwrap_or(core::mem::zeroed()) as _, pwszinterface as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallIndirect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CallIndirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, u32, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMethodInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CALLFRAMEINFO, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut windows_core::BOOL, *mut u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ICallIndirect_Impl: windows_core::IUnknownImpl {
    fn CallIndirect(&self, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::Result<()>;
    fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetStackSize(&self, imethod: u32) -> windows_core::Result<u32>;
    fn GetIID(&self, piid: *mut windows_core::GUID, pfderivesfromidispatch: *mut windows_core::BOOL, pcmethod: *mut u32, pwszinterface: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl ICallIndirect_Vtbl {
    pub const fn new<Identity: ICallIndirect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CallIndirect<Identity: ICallIndirect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallIndirect_Impl::CallIndirect(this, core::mem::transmute_copy(&phrreturn), core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pvargs), core::mem::transmute_copy(&cbargs)).into()
            }
        }
        unsafe extern "system" fn GetMethodInfo<Identity: ICallIndirect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallIndirect_Impl::GetMethodInfo(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pwszmethod)).into()
            }
        }
        unsafe extern "system" fn GetStackSize<Identity: ICallIndirect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallIndirect_Impl::GetStackSize(this, core::mem::transmute_copy(&imethod)) {
                    Ok(ok__) => {
                        cbargs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIID<Identity: ICallIndirect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pfderivesfromidispatch: *mut windows_core::BOOL, pcmethod: *mut u32, pwszinterface: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallIndirect_Impl::GetIID(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pfderivesfromidispatch), core::mem::transmute_copy(&pcmethod), core::mem::transmute_copy(&pwszinterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallIndirect: CallIndirect::<Identity, OFFSET>,
            GetMethodInfo: GetMethodInfo::<Identity, OFFSET>,
            GetStackSize: GetStackSize::<Identity, OFFSET>,
            GetIID: GetIID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallIndirect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallIndirect {}
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
        unsafe { (windows_core::Interface::vtable(self).RegisterSink)(windows_core::Interface::as_raw(self), psink.param().abi()).ok() }
    }
    pub unsafe fn GetRegisteredSink(&self) -> windows_core::Result<ICallFrameEvents> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredSink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallInterceptor_Vtbl {
    pub base__: ICallIndirect_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICallInterceptor_Impl: ICallIndirect_Impl {
    fn RegisterSink(&self, psink: windows_core::Ref<ICallFrameEvents>) -> windows_core::Result<()>;
    fn GetRegisteredSink(&self) -> windows_core::Result<ICallFrameEvents>;
}
impl ICallInterceptor_Vtbl {
    pub const fn new<Identity: ICallInterceptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterSink<Identity: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallInterceptor_Impl::RegisterSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn GetRegisteredSink<Identity: ICallInterceptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallInterceptor_Impl::GetRegisteredSink(this) {
                    Ok(ok__) => {
                        ppsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ICallIndirect_Vtbl::new::<Identity, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, OFFSET>,
            GetRegisteredSink: GetRegisteredSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallInterceptor as windows_core::Interface>::IID || iid == &<ICallIndirect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallInterceptor {}
windows_core::imp::define_interface!(ICallUnmarshal, ICallUnmarshal_Vtbl, 0x5333b003_2e42_11d2_b89d_00c04fb9618a);
windows_core::imp::interface_hierarchy!(ICallUnmarshal, windows_core::IUnknown);
impl ICallUnmarshal {
    pub unsafe fn Unmarshal(&self, imethod: u32, pbuffer: &[u8], fforcebuffercopy: bool, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut Option<ICallFrame>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), imethod, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), fforcebuffercopy.into(), datarep, core::mem::transmute(pcontext), pcbunmarshalled as _, core::mem::transmute(ppframe)).ok() }
    }
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), imethod, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), ibfirstrelease, datarep, core::mem::transmute(pcontext)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallUnmarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, windows_core::BOOL, u32, *const CALLFRAME_MARSHALCONTEXT, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32, u32, *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT,
}
pub trait ICallUnmarshal_Impl: windows_core::IUnknownImpl {
    fn Unmarshal(&self, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: windows_core::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: windows_core::OutRef<ICallFrame>) -> windows_core::Result<()>;
    fn ReleaseMarshalData(&self, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()>;
}
impl ICallUnmarshal_Vtbl {
    pub const fn new<Identity: ICallUnmarshal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unmarshal<Identity: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: windows_core::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallUnmarshal_Impl::Unmarshal(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&fforcebuffercopy), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcbunmarshalled), core::mem::transmute_copy(&ppframe)).into()
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: ICallUnmarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallUnmarshal_Impl::ReleaseMarshalData(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ibfirstrelease), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallUnmarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallUnmarshal {}
windows_core::imp::define_interface!(IInterfaceRelated, IInterfaceRelated_Vtbl, 0xd1fb5a79_7706_11d1_adba_00c04fc2adc0);
windows_core::imp::interface_hierarchy!(IInterfaceRelated, windows_core::IUnknown);
impl IInterfaceRelated {
    pub unsafe fn SetIID(&self, iid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIID)(windows_core::Interface::as_raw(self), iid).ok() }
    }
    pub unsafe fn GetIID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterfaceRelated_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IInterfaceRelated_Impl: windows_core::IUnknownImpl {
    fn SetIID(&self, iid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetIID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IInterfaceRelated_Vtbl {
    pub const fn new<Identity: IInterfaceRelated_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetIID<Identity: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInterfaceRelated_Impl::SetIID(this, core::mem::transmute_copy(&iid)).into()
            }
        }
        unsafe extern "system" fn GetIID<Identity: IInterfaceRelated_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInterfaceRelated_Impl::GetIID(this) {
                    Ok(ok__) => {
                        piid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIID: SetIID::<Identity, OFFSET>, GetIID: GetIID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterfaceRelated as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInterfaceRelated {}
