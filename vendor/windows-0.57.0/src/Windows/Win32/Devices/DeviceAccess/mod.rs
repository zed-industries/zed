#[inline]
pub unsafe fn CreateDeviceAccessInstance<P0>(deviceinterfacepath: P0, desiredaccess: u32) -> windows_core::Result<ICreateDeviceAccessAsync>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("deviceaccess.dll" "system" fn CreateDeviceAccessInstance(deviceinterfacepath : windows_core::PCWSTR, desiredaccess : u32, createasync : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDeviceAccessInstance(deviceinterfacepath.param().abi(), desiredaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
windows_core::imp::define_interface!(ICreateDeviceAccessAsync, ICreateDeviceAccessAsync_Vtbl, 0x3474628f_683d_42d2_abcb_db018c6503bc);
impl core::ops::Deref for ICreateDeviceAccessAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateDeviceAccessAsync, windows_core::IUnknown);
impl ICreateDeviceAccessAsync {
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), timeout).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetResult<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICreateDeviceAccessAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceIoControl, IDeviceIoControl_Vtbl, 0x9eefe161_23ab_4f18_9b49_991b586ae970);
impl core::ops::Deref for IDeviceIoControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDeviceIoControl, windows_core::IUnknown);
impl IDeviceIoControl {
    pub unsafe fn DeviceIoControlSync(&self, iocontrolcode: u32, inputbuffer: Option<&[u8]>, outputbuffer: Option<&mut [u8]>, bytesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceIoControlSync)(windows_core::Interface::as_raw(self), iocontrolcode, core::mem::transmute(inputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), inputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(outputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), outputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), bytesreturned).ok()
    }
    pub unsafe fn DeviceIoControlAsync<P0>(&self, iocontrolcode: u32, inputbuffer: Option<&[u8]>, outputbuffer: Option<&mut [u8]>, requestcompletioncallback: P0, cancelcontext: Option<*mut usize>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDeviceRequestCompletionCallback>,
    {
        (windows_core::Interface::vtable(self).DeviceIoControlAsync)(
            windows_core::Interface::as_raw(self),
            iocontrolcode,
            core::mem::transmute(inputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            inputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(outputbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
            outputbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            requestcompletioncallback.param().abi(),
            core::mem::transmute(cancelcontext.unwrap_or(std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CancelOperation(&self, cancelcontext: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelOperation)(windows_core::Interface::as_raw(self), cancelcontext).ok()
    }
}
#[repr(C)]
pub struct IDeviceIoControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeviceIoControlSync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub DeviceIoControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub CancelOperation: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceRequestCompletionCallback, IDeviceRequestCompletionCallback_Vtbl, 0x999bad24_9acd_45bb_8669_2a2fc0288b04);
impl core::ops::Deref for IDeviceRequestCompletionCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDeviceRequestCompletionCallback, windows_core::IUnknown);
impl IDeviceRequestCompletionCallback {
    pub unsafe fn Invoke(&self, requestresult: windows_core::HRESULT, bytesreturned: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), requestresult, bytesreturned).ok()
    }
}
#[repr(C)]
pub struct IDeviceRequestCompletionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
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
#[cfg(feature = "implement")]
core::include!("impl.rs");
