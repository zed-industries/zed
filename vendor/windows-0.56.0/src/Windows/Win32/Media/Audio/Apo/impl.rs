pub trait IApoAcousticEchoCancellation_Impl: Sized {}
impl windows_core::RuntimeName for IApoAcousticEchoCancellation {}
impl IApoAcousticEchoCancellation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAcousticEchoCancellation_Impl, const OFFSET: isize>() -> IApoAcousticEchoCancellation_Vtbl {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApoAcousticEchoCancellation as windows_core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputConfiguration_Impl: Sized {
    fn AddAuxiliaryInput(&self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> windows_core::Result<()>;
    fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> windows_core::Result<()>;
    fn IsInputFormatSupported(&self, prequestedinputformat: Option<&IAudioMediaType>) -> windows_core::Result<IAudioMediaType>;
}
impl windows_core::RuntimeName for IApoAuxiliaryInputConfiguration {}
impl IApoAuxiliaryInputConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>() -> IApoAuxiliaryInputConfiguration_Vtbl {
        unsafe extern "system" fn AddAuxiliaryInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IApoAuxiliaryInputConfiguration_Impl::AddAuxiliaryInput(this, core::mem::transmute_copy(&dwinputid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pbydata), core::mem::transmute_copy(&pinputconnection)).into()
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IApoAuxiliaryInputConfiguration_Impl::RemoveAuxiliaryInput(this, core::mem::transmute_copy(&dwinputid)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestedinputformat: *mut core::ffi::c_void, ppsupportedinputformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IApoAuxiliaryInputConfiguration_Impl::IsInputFormatSupported(this, windows_core::from_raw_borrowed(&prequestedinputformat)) {
                Ok(ok__) => {
                    core::ptr::write(ppsupportedinputformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAuxiliaryInput: AddAuxiliaryInput::<Identity, Impl, OFFSET>,
            RemoveAuxiliaryInput: RemoveAuxiliaryInput::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputConfiguration as windows_core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputRT_Impl: Sized {
    fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY);
}
impl windows_core::RuntimeName for IApoAuxiliaryInputRT {}
impl IApoAuxiliaryInputRT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: isize>() -> IApoAuxiliaryInputRT_Vtbl {
        unsafe extern "system" fn AcceptInput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IApoAuxiliaryInputRT_Impl::AcceptInput(this, core::mem::transmute_copy(&dwinputid), core::mem::transmute_copy(&pinputconnection))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AcceptInput: AcceptInput::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputRT as windows_core::Interface>::IID
    }
}
pub trait IAudioDeviceModulesClient_Impl: Sized {
    fn SetAudioDeviceModulesManager(&self, paudiodevicemodulesmanager: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioDeviceModulesClient {}
impl IAudioDeviceModulesClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: isize>() -> IAudioDeviceModulesClient_Vtbl {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudiodevicemodulesmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioDeviceModulesClient_Impl::SetAudioDeviceModulesManager(this, windows_core::from_raw_borrowed(&paudiodevicemodulesmanager)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAudioDeviceModulesManager: SetAudioDeviceModulesManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioDeviceModulesClient as windows_core::Interface>::IID
    }
}
pub trait IAudioMediaType_Impl: Sized {
    fn IsCompressedFormat(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn IsEqual(&self, piaudiotype: Option<&IAudioMediaType>) -> windows_core::Result<u32>;
    fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX;
    fn GetUncompressedAudioFormat(&self, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioMediaType {}
impl IAudioMediaType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: isize>() -> IAudioMediaType_Vtbl {
        unsafe extern "system" fn IsCompressedFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioMediaType_Impl::IsCompressedFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(pfcompressed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piaudiotype: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioMediaType_Impl::IsEqual(this, windows_core::from_raw_borrowed(&piaudiotype)) {
                Ok(ok__) => {
                    core::ptr::write(pdwflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioMediaType_Impl::GetAudioFormat(this)
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioMediaType_Impl::GetUncompressedAudioFormat(this, core::mem::transmute_copy(&puncompressedaudioformat)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCompressedFormat: IsCompressedFormat::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            GetAudioFormat: GetAudioFormat::<Identity, Impl, OFFSET>,
            GetUncompressedAudioFormat: GetUncompressedAudioFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMediaType as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObject_Impl: Sized {
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetLatency(&self) -> windows_core::Result<i64>;
    fn GetRegistrationProperties(&self) -> windows_core::Result<*mut APO_REG_PROPERTIES>;
    fn Initialize(&self, cbdatasize: u32, pbydata: *const u8) -> windows_core::Result<()>;
    fn IsInputFormatSupported(&self, poppositeformat: Option<&IAudioMediaType>, prequestedinputformat: Option<&IAudioMediaType>) -> windows_core::Result<IAudioMediaType>;
    fn IsOutputFormatSupported(&self, poppositeformat: Option<&IAudioMediaType>, prequestedoutputformat: Option<&IAudioMediaType>) -> windows_core::Result<IAudioMediaType>;
    fn GetInputChannelCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioProcessingObject {}
impl IAudioProcessingObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>() -> IAudioProcessingObject_Vtbl {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetLatency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObject_Impl::GetLatency(this) {
                Ok(ok__) => {
                    core::ptr::write(ptime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObject_Impl::GetRegistrationProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(ppregprops, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObject_Impl::Initialize(this, core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pbydata)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poppositeformat: *mut core::ffi::c_void, prequestedinputformat: *mut core::ffi::c_void, ppsupportedinputformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObject_Impl::IsInputFormatSupported(this, windows_core::from_raw_borrowed(&poppositeformat), windows_core::from_raw_borrowed(&prequestedinputformat)) {
                Ok(ok__) => {
                    core::ptr::write(ppsupportedinputformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poppositeformat: *mut core::ffi::c_void, prequestedoutputformat: *mut core::ffi::c_void, ppsupportedoutputformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObject_Impl::IsOutputFormatSupported(this, windows_core::from_raw_borrowed(&poppositeformat), windows_core::from_raw_borrowed(&prequestedoutputformat)) {
                Ok(ok__) => {
                    core::ptr::write(ppsupportedoutputformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputChannelCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32channelcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObject_Impl::GetInputChannelCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pu32channelcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
            GetRegistrationProperties: GetRegistrationProperties::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, Impl, OFFSET>,
            GetInputChannelCount: GetInputChannelCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObject as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectConfiguration_Impl: Sized {
    fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> windows_core::Result<()>;
    fn UnlockForProcess(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioProcessingObjectConfiguration {}
impl IAudioProcessingObjectConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>() -> IAudioProcessingObjectConfiguration_Vtbl {
        unsafe extern "system" fn LockForProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectConfiguration_Impl::LockForProcess(this, core::mem::transmute_copy(&u32numinputconnections), core::mem::transmute_copy(&ppinputconnections), core::mem::transmute_copy(&u32numoutputconnections), core::mem::transmute_copy(&ppoutputconnections)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectConfiguration_Impl::UnlockForProcess(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LockForProcess: LockForProcess::<Identity, Impl, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectConfiguration as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectLoggingService_Impl: Sized {
    fn ApoLog(&self, level: APO_LOG_LEVEL, format: &windows_core::PCWSTR);
}
impl windows_core::RuntimeName for IAudioProcessingObjectLoggingService {}
impl IAudioProcessingObjectLoggingService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: isize>() -> IAudioProcessingObjectLoggingService_Vtbl {
        unsafe extern "system" fn ApoLog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: APO_LOG_LEVEL, format: windows_core::PCWSTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectLoggingService_Impl::ApoLog(this, core::mem::transmute_copy(&level), core::mem::transmute(&format))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ApoLog: ApoLog::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectLoggingService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioProcessingObjectNotifications_Impl: Sized {
    fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::Result<()>;
    fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAudioProcessingObjectNotifications {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioProcessingObjectNotifications_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>() -> IAudioProcessingObjectNotifications_Vtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectNotifications_Impl::GetApoNotificationRegistrationInfo(this, core::mem::transmute_copy(&aponotifications), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn HandleNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectNotifications_Impl::HandleNotification(this, core::mem::transmute_copy(&aponotification))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetApoNotificationRegistrationInfo: GetApoNotificationRegistrationInfo::<Identity, Impl, OFFSET>,
            HandleNotification: HandleNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectNotifications as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioProcessingObjectNotifications2_Impl: Sized + IAudioProcessingObjectNotifications_Impl {
    fn GetApoNotificationRegistrationInfo2(&self, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAudioProcessingObjectNotifications2 {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioProcessingObjectNotifications2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectNotifications2_Impl, const OFFSET: isize>() -> IAudioProcessingObjectNotifications2_Vtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectNotifications2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectNotifications2_Impl::GetApoNotificationRegistrationInfo2(this, core::mem::transmute_copy(&maxaponotificationtypesupported), core::mem::transmute_copy(&aponotifications), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: IAudioProcessingObjectNotifications_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetApoNotificationRegistrationInfo2: GetApoNotificationRegistrationInfo2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectNotifications2 as windows_core::Interface>::IID || iid == &<IAudioProcessingObjectNotifications as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRT_Impl: Sized {
    fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY);
    fn CalcInputFrames(&self, u32outputframecount: u32) -> u32;
    fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32;
}
impl windows_core::RuntimeName for IAudioProcessingObjectRT {}
impl IAudioProcessingObjectRT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>() -> IAudioProcessingObjectRT_Vtbl {
        unsafe extern "system" fn APOProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectRT_Impl::APOProcess(this, core::mem::transmute_copy(&u32numinputconnections), core::mem::transmute_copy(&ppinputconnections), core::mem::transmute_copy(&u32numoutputconnections), core::mem::transmute_copy(&ppoutputconnections))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32outputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectRT_Impl::CalcInputFrames(this, core::mem::transmute_copy(&u32outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32inputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioProcessingObjectRT_Impl::CalcOutputFrames(this, core::mem::transmute_copy(&u32inputframecount))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            APOProcess: APOProcess::<Identity, Impl, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, Impl, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRT as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRTQueueService_Impl: Sized {
    fn GetRealTimeWorkQueue(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioProcessingObjectRTQueueService {}
impl IAudioProcessingObjectRTQueueService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: isize>() -> IAudioProcessingObjectRTQueueService_Vtbl {
        unsafe extern "system" fn GetRealTimeWorkQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workqueueid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObjectRTQueueService_Impl::GetRealTimeWorkQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(workqueueid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRealTimeWorkQueue: GetRealTimeWorkQueue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRTQueueService as windows_core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectVBR_Impl: Sized {
    fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> windows_core::Result<u32>;
    fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioProcessingObjectVBR {}
impl IAudioProcessingObjectVBR_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>() -> IAudioProcessingObjectVBR_Vtbl {
        unsafe extern "system" fn CalcMaxInputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObjectVBR_Impl::CalcMaxInputFrames(this, core::mem::transmute_copy(&u32maxoutputframecount)) {
                Ok(ok__) => {
                    core::ptr::write(pu32inputframecount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioProcessingObjectVBR_Impl::CalcMaxOutputFrames(this, core::mem::transmute_copy(&u32maxinputframecount)) {
                Ok(ok__) => {
                    core::ptr::write(pu32outputframecount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CalcMaxInputFrames: CalcMaxInputFrames::<Identity, Impl, OFFSET>,
            CalcMaxOutputFrames: CalcMaxOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioProcessingObjectVBR as windows_core::Interface>::IID
    }
}
pub trait IAudioSystemEffects_Impl: Sized {}
impl windows_core::RuntimeName for IAudioSystemEffects {}
impl IAudioSystemEffects_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects_Impl, const OFFSET: isize>() -> IAudioSystemEffects_Vtbl {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffects as windows_core::Interface>::IID
    }
}
pub trait IAudioSystemEffects2_Impl: Sized + IAudioSystemEffects_Impl {
    fn GetEffectsList(&self, ppeffectsids: *mut *mut windows_core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSystemEffects2 {}
impl IAudioSystemEffects2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects2_Impl, const OFFSET: isize>() -> IAudioSystemEffects2_Vtbl {
        unsafe extern "system" fn GetEffectsList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffectsids: *mut *mut windows_core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioSystemEffects2_Impl::GetEffectsList(this, core::mem::transmute_copy(&ppeffectsids), core::mem::transmute_copy(&pceffects), core::mem::transmute_copy(&event)).into()
        }
        Self { base__: IAudioSystemEffects_Vtbl::new::<Identity, Impl, OFFSET>(), GetEffectsList: GetEffectsList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffects2 as windows_core::Interface>::IID || iid == &<IAudioSystemEffects as windows_core::Interface>::IID
    }
}
pub trait IAudioSystemEffects3_Impl: Sized + IAudioSystemEffects2_Impl {
    fn GetControllableSystemEffectsList(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetAudioSystemEffectState(&self, effectid: &windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSystemEffects3 {}
impl IAudioSystemEffects3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>() -> IAudioSystemEffects3_Vtbl {
        unsafe extern "system" fn GetControllableSystemEffectsList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioSystemEffects3_Impl::GetControllableSystemEffectsList(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&numeffects), core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAudioSystemEffects3_Impl::SetAudioSystemEffectState(this, core::mem::transmute(&effectid), core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: IAudioSystemEffects2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetControllableSystemEffectsList: GetControllableSystemEffectsList::<Identity, Impl, OFFSET>,
            SetAudioSystemEffectState: SetAudioSystemEffectState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffects3 as windows_core::Interface>::IID || iid == &<IAudioSystemEffects as windows_core::Interface>::IID || iid == &<IAudioSystemEffects2 as windows_core::Interface>::IID
    }
}
pub trait IAudioSystemEffectsCustomFormats_Impl: Sized {
    fn GetFormatCount(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, nformat: u32) -> windows_core::Result<IAudioMediaType>;
    fn GetFormatRepresentation(&self, nformat: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IAudioSystemEffectsCustomFormats {}
impl IAudioSystemEffectsCustomFormats_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>() -> IAudioSystemEffectsCustomFormats_Vtbl {
        unsafe extern "system" fn GetFormatCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcformats: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioSystemEffectsCustomFormats_Impl::GetFormatCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcformats, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nformat: u32, ppformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioSystemEffectsCustomFormats_Impl::GetFormat(this, core::mem::transmute_copy(&nformat)) {
                Ok(ok__) => {
                    core::ptr::write(ppformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRepresentation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAudioSystemEffectsCustomFormats_Impl::GetFormatRepresentation(this, core::mem::transmute_copy(&nformat)) {
                Ok(ok__) => {
                    core::ptr::write(ppwstrformatrep, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFormatCount: GetFormatCount::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetFormatRepresentation: GetFormatRepresentation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsCustomFormats as windows_core::Interface>::IID
    }
}
