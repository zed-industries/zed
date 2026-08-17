pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsMediaLibrarySharingDevice, IWindowsMediaLibrarySharingDevice_Vtbl, 0x3dccc293_4fd9_4191_a25b_8e57c5d27bd4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsMediaLibrarySharingDevice {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsMediaLibrarySharingDevice, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevice {
    pub unsafe fn DeviceID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Authorization(&self) -> windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Authorization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthorization)(windows_core::Interface::as_raw(self), authorization).ok() }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Authorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevice_Impl: super::super::System::Com::IDispatch_Impl {
    fn DeviceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Authorization(&self) -> windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus>;
    fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDevice_Vtbl {
    pub const fn new<Identity: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceID<Identity: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevice_Impl::DeviceID(this) {
                    Ok(ok__) => {
                        deviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Authorization<Identity: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevice_Impl::Authorization(this) {
                    Ok(ok__) => {
                        authorization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAuthorization<Identity: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingDevice_Impl::SetAuthorization(this, core::mem::transmute_copy(&authorization)).into()
            }
        }
        unsafe extern "system" fn Properties<Identity: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevice_Impl::Properties(this) {
                    Ok(ok__) => {
                        deviceproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DeviceID: DeviceID::<Identity, OFFSET>,
            Authorization: Authorization::<Identity, OFFSET>,
            SetAuthorization: SetAuthorization::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevice as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDevice {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsMediaLibrarySharingDeviceProperties, IWindowsMediaLibrarySharingDeviceProperties_Vtbl, 0xc4623214_6b06_40c5_a623_b2ff4c076bfd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsMediaLibrarySharingDeviceProperties {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsMediaLibrarySharingDeviceProperties, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperties {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperties_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn GetProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub const fn new<Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDeviceProperties_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDeviceProperties_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDeviceProperties_Impl::GetProperty(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperties {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsMediaLibrarySharingDeviceProperty, IWindowsMediaLibrarySharingDeviceProperty_Vtbl, 0x81e26927_7a7d_40a7_81d4_bddc02960e3e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsMediaLibrarySharingDeviceProperty {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsMediaLibrarySharingDeviceProperty, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperty_Impl: super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub const fn new<Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDeviceProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDeviceProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Name: Name::<Identity, OFFSET>, Value: Value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperty {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsMediaLibrarySharingDevices, IWindowsMediaLibrarySharingDevices_Vtbl, 0x1803f9d6_fe6d_4546_bf5b_992fe8ec12d1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsMediaLibrarySharingDevices {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsMediaLibrarySharingDevices, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevices {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevices_Impl: super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsMediaLibrarySharingDevice>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn GetDevice(&self, deviceid: &windows_core::BSTR) -> windows_core::Result<IWindowsMediaLibrarySharingDevice>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDevices_Vtbl {
    pub const fn new<Identity: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Item<Identity: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevices_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevices_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingDevices_Impl::GetDevice(this, core::mem::transmute(&deviceid)) {
                    Ok(ok__) => {
                        device.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingDevices {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsMediaLibrarySharingServices, IWindowsMediaLibrarySharingServices_Vtbl, 0x01f5f85e_0a81_40da_a7c8_21ef3af8440c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsMediaLibrarySharingServices {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsMediaLibrarySharingServices, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServices {
    pub unsafe fn showShareMediaCPL(&self, device: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).showShareMediaCPL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(device)).ok() }
    }
    pub unsafe fn userHomeMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).userHomeMediaSharingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetuserHomeMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetuserHomeMediaSharingState)(windows_core::Interface::as_raw(self), sharingenabled).ok() }
    }
    pub unsafe fn userHomeMediaSharingLibraryName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).userHomeMediaSharingLibraryName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetuserHomeMediaSharingLibraryName(&self, libraryname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetuserHomeMediaSharingLibraryName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(libraryname)).ok() }
    }
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).computerHomeMediaSharingAllowedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetcomputerHomeMediaSharingAllowedState)(windows_core::Interface::as_raw(self), sharingallowed).ok() }
    }
    pub unsafe fn userInternetMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).userInternetMediaSharingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetuserInternetMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetuserInternetMediaSharingState)(windows_core::Interface::as_raw(self), sharingenabled).ok() }
    }
    pub unsafe fn computerInternetMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).computerInternetMediaSharingAllowedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetcomputerInternetMediaSharingAllowedState)(windows_core::Interface::as_raw(self), sharingallowed).ok() }
    }
    pub unsafe fn internetMediaSharingSecurityGroup(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).internetMediaSharingSecurityGroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetinternetMediaSharingSecurityGroup(&self, securitygroup: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetinternetMediaSharingSecurityGroup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(securitygroup)).ok() }
    }
    pub unsafe fn allowSharingToAllDevices(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).allowSharingToAllDevices)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetallowSharingToAllDevices(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetallowSharingToAllDevices)(windows_core::Interface::as_raw(self), sharingenabled).ok() }
    }
    pub unsafe fn setDefaultAuthorization(&self, macaddresses: &windows_core::BSTR, friendlyname: &windows_core::BSTR, authorization: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).setDefaultAuthorization)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(macaddresses), core::mem::transmute_copy(friendlyname), authorization).ok() }
    }
    pub unsafe fn setAuthorizationState(&self, macaddress: &windows_core::BSTR, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).setAuthorizationState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(macaddress), authorizationstate).ok() }
    }
    pub unsafe fn getAllDevices(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDevices> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAllDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn customSettingsApplied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).customSettingsApplied)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub showShareMediaCPL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub userHomeMediaSharingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetuserHomeMediaSharingState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub userHomeMediaSharingLibraryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetuserHomeMediaSharingLibraryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub computerHomeMediaSharingAllowedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetcomputerHomeMediaSharingAllowedState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub userInternetMediaSharingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetuserInternetMediaSharingState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub computerInternetMediaSharingAllowedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetcomputerInternetMediaSharingAllowedState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub internetMediaSharingSecurityGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetinternetMediaSharingSecurityGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub allowSharingToAllDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetallowSharingToAllDevices: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub setDefaultAuthorization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub setAuthorizationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub getAllDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub customSettingsApplied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingServices_Impl: super::super::System::Com::IDispatch_Impl {
    fn showShareMediaCPL(&self, device: &windows_core::BSTR) -> windows_core::Result<()>;
    fn userHomeMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserHomeMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn userHomeMediaSharingLibraryName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetuserHomeMediaSharingLibraryName(&self, libraryname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn computerHomeMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn userInternetMediaSharingState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserInternetMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn computerInternetMediaSharingAllowedState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn internetMediaSharingSecurityGroup(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetinternetMediaSharingSecurityGroup(&self, securitygroup: &windows_core::BSTR) -> windows_core::Result<()>;
    fn allowSharingToAllDevices(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetallowSharingToAllDevices(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setDefaultAuthorization(&self, macaddresses: &windows_core::BSTR, friendlyname: &windows_core::BSTR, authorization: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setAuthorizationState(&self, macaddress: &windows_core::BSTR, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAllDevices(&self) -> windows_core::Result<IWindowsMediaLibrarySharingDevices>;
    fn customSettingsApplied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingServices_Vtbl {
    pub const fn new<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn showShareMediaCPL<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::showShareMediaCPL(this, core::mem::transmute(&device)).into()
            }
        }
        unsafe extern "system" fn userHomeMediaSharingState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::userHomeMediaSharingState(this) {
                    Ok(ok__) => {
                        sharingenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetuserHomeMediaSharingState(this, core::mem::transmute_copy(&sharingenabled)).into()
            }
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, libraryname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::userHomeMediaSharingLibraryName(this) {
                    Ok(ok__) => {
                        libraryname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, libraryname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetuserHomeMediaSharingLibraryName(this, core::mem::transmute(&libraryname)).into()
            }
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::computerHomeMediaSharingAllowedState(this) {
                    Ok(ok__) => {
                        sharingallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetcomputerHomeMediaSharingAllowedState(this, core::mem::transmute_copy(&sharingallowed)).into()
            }
        }
        unsafe extern "system" fn userInternetMediaSharingState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::userInternetMediaSharingState(this) {
                    Ok(ok__) => {
                        sharingenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetuserInternetMediaSharingState(this, core::mem::transmute_copy(&sharingenabled)).into()
            }
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::computerInternetMediaSharingAllowedState(this) {
                    Ok(ok__) => {
                        sharingallowed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetcomputerInternetMediaSharingAllowedState(this, core::mem::transmute_copy(&sharingallowed)).into()
            }
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securitygroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::internetMediaSharingSecurityGroup(this) {
                    Ok(ok__) => {
                        securitygroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securitygroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetinternetMediaSharingSecurityGroup(this, core::mem::transmute(&securitygroup)).into()
            }
        }
        unsafe extern "system" fn allowSharingToAllDevices<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::allowSharingToAllDevices(this) {
                    Ok(ok__) => {
                        sharingenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::SetallowSharingToAllDevices(this, core::mem::transmute_copy(&sharingenabled)).into()
            }
        }
        unsafe extern "system" fn setDefaultAuthorization<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, macaddresses: *mut core::ffi::c_void, friendlyname: *mut core::ffi::c_void, authorization: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::setDefaultAuthorization(this, core::mem::transmute(&macaddresses), core::mem::transmute(&friendlyname), core::mem::transmute_copy(&authorization)).into()
            }
        }
        unsafe extern "system" fn setAuthorizationState<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, macaddress: *mut core::ffi::c_void, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowsMediaLibrarySharingServices_Impl::setAuthorizationState(this, core::mem::transmute(&macaddress), core::mem::transmute_copy(&authorizationstate)).into()
            }
        }
        unsafe extern "system" fn getAllDevices<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::getAllDevices(this) {
                    Ok(ok__) => {
                        devices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn customSettingsApplied<Identity: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customsettingsapplied: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowsMediaLibrarySharingServices_Impl::customSettingsApplied(this) {
                    Ok(ok__) => {
                        customsettingsapplied.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            showShareMediaCPL: showShareMediaCPL::<Identity, OFFSET>,
            userHomeMediaSharingState: userHomeMediaSharingState::<Identity, OFFSET>,
            SetuserHomeMediaSharingState: SetuserHomeMediaSharingState::<Identity, OFFSET>,
            userHomeMediaSharingLibraryName: userHomeMediaSharingLibraryName::<Identity, OFFSET>,
            SetuserHomeMediaSharingLibraryName: SetuserHomeMediaSharingLibraryName::<Identity, OFFSET>,
            computerHomeMediaSharingAllowedState: computerHomeMediaSharingAllowedState::<Identity, OFFSET>,
            SetcomputerHomeMediaSharingAllowedState: SetcomputerHomeMediaSharingAllowedState::<Identity, OFFSET>,
            userInternetMediaSharingState: userInternetMediaSharingState::<Identity, OFFSET>,
            SetuserInternetMediaSharingState: SetuserInternetMediaSharingState::<Identity, OFFSET>,
            computerInternetMediaSharingAllowedState: computerInternetMediaSharingAllowedState::<Identity, OFFSET>,
            SetcomputerInternetMediaSharingAllowedState: SetcomputerInternetMediaSharingAllowedState::<Identity, OFFSET>,
            internetMediaSharingSecurityGroup: internetMediaSharingSecurityGroup::<Identity, OFFSET>,
            SetinternetMediaSharingSecurityGroup: SetinternetMediaSharingSecurityGroup::<Identity, OFFSET>,
            allowSharingToAllDevices: allowSharingToAllDevices::<Identity, OFFSET>,
            SetallowSharingToAllDevices: SetallowSharingToAllDevices::<Identity, OFFSET>,
            setDefaultAuthorization: setDefaultAuthorization::<Identity, OFFSET>,
            setAuthorizationState: setAuthorizationState::<Identity, OFFSET>,
            getAllDevices: getAllDevices::<Identity, OFFSET>,
            customSettingsApplied: customSettingsApplied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingServices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWindowsMediaLibrarySharingServices {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
pub const WindowsMediaLibrarySharingServices: windows_core::GUID = windows_core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
