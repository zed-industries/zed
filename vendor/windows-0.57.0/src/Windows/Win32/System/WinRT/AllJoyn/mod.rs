windows_core::imp::define_interface!(IWindowsDevicesAllJoynBusAttachmentFactoryInterop, IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl, 0x4b8f7505_b239_4e7b_88af_f6682575d861);
impl core::ops::Deref for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentFactoryInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64, enableaboutdata: u8) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromWin32Handle)(windows_core::Interface::as_raw(self), win32handle, enableaboutdata, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u8, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsDevicesAllJoynBusAttachmentInterop, IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl, 0xfd89c65b_b50e_4a19_9d0c_b42b783281cd);
impl core::ops::Deref for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusAttachmentInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    pub unsafe fn Win32Handle(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Win32Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Win32Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsDevicesAllJoynBusObjectFactoryInterop, IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl, 0x6174e506_8b95_4e36_95c0_b88fed34938c);
impl core::ops::Deref for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectFactoryInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    pub unsafe fn CreateFromWin32Handle<T>(&self, win32handle: u64) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateFromWin32Handle)(windows_core::Interface::as_raw(self), win32handle, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromWin32Handle: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsDevicesAllJoynBusObjectInterop, IWindowsDevicesAllJoynBusObjectInterop_Vtbl, 0xd78aa3d5_5054_428f_99f2_ec3a5de3c3bc);
impl core::ops::Deref for IWindowsDevicesAllJoynBusObjectInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowsDevicesAllJoynBusObjectInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWindowsDevicesAllJoynBusObjectInterop {
    pub unsafe fn AddPropertyGetHandler(&self, context: *const core::ffi::c_void, interfacename: &windows_core::HSTRING, callback: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPropertyGetHandler)(windows_core::Interface::as_raw(self), context, core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn AddPropertySetHandler(&self, context: *const core::ffi::c_void, interfacename: &windows_core::HSTRING, callback: isize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddPropertySetHandler)(windows_core::Interface::as_raw(self), context, core::mem::transmute_copy(interfacename), callback).ok()
    }
    pub unsafe fn Win32Handle(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Win32Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddPropertyGetHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, isize) -> windows_core::HRESULT,
    pub AddPropertySetHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, isize) -> windows_core::HRESULT,
    pub Win32Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
