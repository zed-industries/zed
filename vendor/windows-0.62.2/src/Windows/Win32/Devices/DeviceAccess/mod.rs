#[inline]
pub unsafe fn CreateDeviceAccessInstance<P0>(deviceinterfacepath: P0, desiredaccess: u32) -> windows_core::Result<ICreateDeviceAccessAsync>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("deviceaccess.dll" "system" fn CreateDeviceAccessInstance(deviceinterfacepath : windows_core::PCWSTR, desiredaccess : u32, createasync : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDeviceAccessInstance(deviceinterfacepath.param().abi(), desiredaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const CLSID_DeviceIoControl: windows_core::GUID = windows_core::GUID::from_u128(0x12d3e372_874b_457d_9fdf_73977778686c);
pub const DEV_PORT_1394: u32 = 8u32;
pub const DEV_PORT_ARTI: u32 = 7u32;
pub const DEV_PORT_COM1: u32 = 2u32;
pub const DEV_PORT_COM2: u32 = 3u32;
pub const DEV_PORT_COM3: u32 = 4u32;
pub const DEV_PORT_COM4: u32 = 5u32;
pub const DEV_PORT_DIAQ: u32 = 6u32;
pub const DEV_PORT_MAX: u32 = 9u32;
pub const DEV_PORT_MIN: u32 = 1u32;
pub const DEV_PORT_SIM: u32 = 1u32;
pub const DEV_PORT_USB: u32 = 9u32;
pub const ED_AUDIO_1: i32 = 1i32;
pub const ED_AUDIO_10: i32 = 512i32;
pub const ED_AUDIO_11: i32 = 1024i32;
pub const ED_AUDIO_12: i32 = 2048i32;
pub const ED_AUDIO_13: i32 = 4096i32;
pub const ED_AUDIO_14: i32 = 8192i32;
pub const ED_AUDIO_15: i32 = 16384i32;
pub const ED_AUDIO_16: i32 = 32768i32;
pub const ED_AUDIO_17: i32 = 65536i32;
pub const ED_AUDIO_18: i32 = 131072i32;
pub const ED_AUDIO_19: i32 = 262144i32;
pub const ED_AUDIO_2: i32 = 2i32;
pub const ED_AUDIO_20: i32 = 524288i32;
pub const ED_AUDIO_21: i32 = 1048576i32;
pub const ED_AUDIO_22: i32 = 2097152i32;
pub const ED_AUDIO_23: i32 = 4194304i32;
pub const ED_AUDIO_24: i32 = 8388608i32;
pub const ED_AUDIO_3: i32 = 4i32;
pub const ED_AUDIO_4: i32 = 8i32;
pub const ED_AUDIO_5: i32 = 16i32;
pub const ED_AUDIO_6: i32 = 32i32;
pub const ED_AUDIO_7: i32 = 64i32;
pub const ED_AUDIO_8: i32 = 128i32;
pub const ED_AUDIO_9: i32 = 256i32;
pub const ED_AUDIO_ALL: u32 = 268435456u32;
pub const ED_BASE: i32 = 4096i32;
pub const ED_BOTTOM: u32 = 4u32;
pub const ED_CENTER: u32 = 512u32;
pub const ED_LEFT: u32 = 256u32;
pub const ED_MIDDLE: u32 = 2u32;
pub const ED_RIGHT: u32 = 1024u32;
pub const ED_TOP: u32 = 1u32;
pub const ED_VIDEO: i32 = 33554432i32;
windows_core::imp::define_interface!(ICreateDeviceAccessAsync, ICreateDeviceAccessAsync_Vtbl, 0x3474628f_683d_42d2_abcb_db018c6503bc);
windows_core::imp::interface_hierarchy!(ICreateDeviceAccessAsync, windows_core::IUnknown);
impl ICreateDeviceAccessAsync {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Wait(&self, timeout: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), timeout).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetResult<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateDeviceAccessAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICreateDeviceAccessAsync_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, timeout: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetResult(&self, riid: *const windows_core::GUID, deviceaccess: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICreateDeviceAccessAsync_Vtbl {
    pub const fn new<Identity: ICreateDeviceAccessAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ICreateDeviceAccessAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateDeviceAccessAsync_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Wait<Identity: ICreateDeviceAccessAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateDeviceAccessAsync_Impl::Wait(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ICreateDeviceAccessAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateDeviceAccessAsync_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetResult<Identity: ICreateDeviceAccessAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, deviceaccess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreateDeviceAccessAsync_Impl::GetResult(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&deviceaccess)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateDeviceAccessAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICreateDeviceAccessAsync {}
windows_core::imp::define_interface!(IDeviceIoControl, IDeviceIoControl_Vtbl, 0x9eefe161_23ab_4f18_9b49_991b586ae970);
windows_core::imp::interface_hierarchy!(IDeviceIoControl, windows_core::IUnknown);
impl IDeviceIoControl {
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: Option<&[u8]>, outputbuffer: Option<&mut [u8]>, bytesreturned: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceIoControlSync)(windows_core::Interface::as_raw(self), iocontrolcode, core::mem::transmute(inputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), inputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(outputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), outputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), bytesreturned as _).ok() }
    }
    pub unsafe fn DeviceIoControlAsync<P5>(&self, iocontrolcode: u32, inputbuffer: Option<&[u8]>, outputbuffer: Option<&mut [u8]>, requestcompletioncallback: P5, cancelcontext: Option<*mut usize>) -> windows_core::Result<()>
    where
        P5: windows_core::Param<IDeviceRequestCompletionCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceIoControlAsync)(
                windows_core::Interface::as_raw(self),
                iocontrolcode,
                core::mem::transmute(inputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                inputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(outputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                outputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                requestcompletioncallback.param().abi(),
                cancelcontext.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelOperation)(windows_core::Interface::as_raw(self), cancelcontext).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceIoControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeviceIoControlSync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub DeviceIoControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub CancelOperation: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait IDeviceIoControl_Impl: windows_core::IUnknownImpl {
    fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn DeviceIoControlAsync(&self, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: windows_core::Ref<IDeviceRequestCompletionCallback>, cancelcontext: *mut usize) -> windows_core::Result<()>;
    fn CancelOperation(&self, cancelcontext: usize) -> windows_core::Result<()>;
}
impl IDeviceIoControl_Vtbl {
    pub const fn new<Identity: IDeviceIoControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceIoControlSync<Identity: IDeviceIoControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceIoControl_Impl::DeviceIoControlSync(this, core::mem::transmute_copy(&iocontrolcode), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&inputbuffersize), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        unsafe extern "system" fn DeviceIoControlAsync<Identity: IDeviceIoControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: *mut core::ffi::c_void, cancelcontext: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceIoControl_Impl::DeviceIoControlAsync(this, core::mem::transmute_copy(&iocontrolcode), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&inputbuffersize), core::mem::transmute_copy(&outputbuffer), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&requestcompletioncallback), core::mem::transmute_copy(&cancelcontext)).into()
            }
        }
        unsafe extern "system" fn CancelOperation<Identity: IDeviceIoControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancelcontext: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceIoControl_Impl::CancelOperation(this, core::mem::transmute_copy(&cancelcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceIoControlSync: DeviceIoControlSync::<Identity, OFFSET>,
            DeviceIoControlAsync: DeviceIoControlAsync::<Identity, OFFSET>,
            CancelOperation: CancelOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceIoControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDeviceIoControl {}
windows_core::imp::define_interface!(IDeviceRequestCompletionCallback, IDeviceRequestCompletionCallback_Vtbl, 0x999bad24_9acd_45bb_8669_2a2fc0288b04);
windows_core::imp::interface_hierarchy!(IDeviceRequestCompletionCallback, windows_core::IUnknown);
impl IDeviceRequestCompletionCallback {
    pub unsafe fn Invoke(&self, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), requestresult, bytesreturned).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRequestCompletionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
}
pub trait IDeviceRequestCompletionCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::Result<()>;
}
impl IDeviceRequestCompletionCallback_Vtbl {
    pub const fn new<Identity: IDeviceRequestCompletionCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IDeviceRequestCompletionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceRequestCompletionCallback_Impl::Invoke(this, core::mem::transmute_copy(&requestresult), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceRequestCompletionCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDeviceRequestCompletionCallback {}
