windows_core::imp::define_interface!(INDClient, INDClient_Vtbl, 0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
impl windows_core::RuntimeType for INDClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegistrationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ProximityDetectionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LicenseFetchCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ReRegistrationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LicenseFetchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReRegistrationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDClientFactory, INDClientFactory_Vtbl, 0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
impl windows_core::RuntimeType for INDClientFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDClosedCaptionDataReceivedEventArgs, INDClosedCaptionDataReceivedEventArgs_Vtbl, 0x4738d29f_c345_4649_8468_b8c5fc357190);
impl windows_core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDClosedCaptionDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl INDClosedCaptionDataReceivedEventArgs {
    pub fn ClosedCaptionDataFormat(&self) -> windows_core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionDataFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PresentationTimestamp(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClosedCaptionData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeName for INDClosedCaptionDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
}
pub trait INDClosedCaptionDataReceivedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ClosedCaptionDataFormat(&self) -> windows_core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&self) -> windows_core::Result<i64>;
    fn ClosedCaptionData(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
impl INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClosedCaptionDataFormat<Identity: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDClosedCaptionDataReceivedEventArgs_Impl::ClosedCaptionDataFormat(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PresentationTimestamp<Identity: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDClosedCaptionDataReceivedEventArgs_Impl::PresentationTimestamp(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClosedCaptionData<Identity: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDClosedCaptionDataReceivedEventArgs_Impl::ClosedCaptionData(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDClosedCaptionDataReceivedEventArgs, OFFSET>(),
            ClosedCaptionDataFormat: ClosedCaptionDataFormat::<Identity, OFFSET>,
            PresentationTimestamp: PresentationTimestamp::<Identity, OFFSET>,
            ClosedCaptionData: ClosedCaptionData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDClosedCaptionDataReceivedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDClosedCaptionFormat) -> windows_core::HRESULT,
    pub PresentationTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ClosedCaptionData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDCustomData, INDCustomData_Vtbl, 0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
impl windows_core::RuntimeType for INDCustomData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDCustomData, windows_core::IUnknown, windows_core::IInspectable);
impl INDCustomData {
    pub fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomDataTypeID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeName for INDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomData";
}
pub trait INDCustomData_Impl: windows_core::IUnknownImpl {
    fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
impl INDCustomData_Vtbl {
    pub const fn new<Identity: INDCustomData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CustomDataTypeID<Identity: INDCustomData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDCustomData_Impl::CustomDataTypeID(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CustomData<Identity: INDCustomData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDCustomData_Impl::CustomData(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDCustomData, OFFSET>(),
            CustomDataTypeID: CustomDataTypeID::<Identity, OFFSET>,
            CustomData: CustomData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDCustomData as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomDataTypeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub CustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDCustomDataFactory, INDCustomDataFactory_Vtbl, 0xd65405ab_3424_4833_8c9a_af5fdeb22872);
impl windows_core::RuntimeType for INDCustomDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDDownloadEngine, INDDownloadEngine_Vtbl, 0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
impl windows_core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDDownloadEngine, windows_core::IUnknown, windows_core::IInspectable);
impl INDDownloadEngine {
    pub fn Open<P0>(&self, uri: P0, sessionidbytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Open)(windows_core::Interface::as_raw(this), uri.param().abi(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr()).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Seek(&self, startposition: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), startposition).ok() }
    }
    pub fn CanSeek(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BufferFullMinThresholdInSamples(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferFullMinThresholdInSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BufferFullMaxThresholdInSamples(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferFullMaxThresholdInSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Notifier(&self) -> windows_core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for INDDownloadEngine {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngine";
}
pub trait INDDownloadEngine_Impl: windows_core::IUnknownImpl {
    fn Open(&self, uri: windows_core::Ref<super::super::super::Foundation::Uri>, sessionIDBytes: &[u8]) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Seek(&self, startPosition: &super::super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn CanSeek(&self) -> windows_core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&self) -> windows_core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&self) -> windows_core::Result<u32>;
    fn Notifier(&self) -> windows_core::Result<NDDownloadEngineNotifier>;
}
impl INDDownloadEngine_Vtbl {
    pub const fn new<Identity: INDDownloadEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, sessionidbytes_array_size: u32, sessionidbytes: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngine_Impl::Open(this, core::mem::transmute_copy(&uri), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionidbytes_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngine_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngine_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngine_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngine_Impl::Seek(this, core::mem::transmute(&startposition)).into()
            }
        }
        unsafe extern "system" fn CanSeek<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDDownloadEngine_Impl::CanSeek(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDDownloadEngine_Impl::BufferFullMinThresholdInSamples(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDDownloadEngine_Impl::BufferFullMaxThresholdInSamples(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notifier<Identity: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDDownloadEngine_Impl::Notifier(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDDownloadEngine, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            CanSeek: CanSeek::<Identity, OFFSET>,
            BufferFullMinThresholdInSamples: BufferFullMinThresholdInSamples::<Identity, OFFSET>,
            BufferFullMaxThresholdInSamples: BufferFullMaxThresholdInSamples::<Identity, OFFSET>,
            Notifier: Notifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDDownloadEngine as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Notifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDDownloadEngineNotifier, INDDownloadEngineNotifier_Vtbl, 0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
impl windows_core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDDownloadEngineNotifier, windows_core::IUnknown, windows_core::IInspectable);
impl INDDownloadEngineNotifier {
    pub fn OnStreamOpened(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnStreamOpened)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnDataReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    pub fn OnEndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnEndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OnNetworkError(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnNetworkError)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for INDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
}
pub trait INDDownloadEngineNotifier_Impl: windows_core::IUnknownImpl {
    fn OnStreamOpened(&self) -> windows_core::Result<()>;
    fn OnPlayReadyObjectReceived(&self, dataBytes: &[u8]) -> windows_core::Result<()>;
    fn OnContentIDReceived(&self, licenseFetchDescriptor: windows_core::Ref<INDLicenseFetchDescriptor>) -> windows_core::Result<()>;
    fn OnDataReceived(&self, dataBytes: &[u8], bytesReceived: u32) -> windows_core::Result<()>;
    fn OnEndOfStream(&self) -> windows_core::Result<()>;
    fn OnNetworkError(&self) -> windows_core::Result<()>;
}
impl INDDownloadEngineNotifier_Vtbl {
    pub const fn new<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStreamOpened<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnStreamOpened(this).into()
            }
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, databytes_array_size: u32, databytes: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnPlayReadyObjectReceived(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), databytes_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn OnContentIDReceived<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchdescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnContentIDReceived(this, core::mem::transmute_copy(&licensefetchdescriptor)).into()
            }
        }
        unsafe extern "system" fn OnDataReceived<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, databytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnDataReceived(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), databytes_array_size as usize), bytesreceived).into()
            }
        }
        unsafe extern "system" fn OnEndOfStream<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnEndOfStream(this).into()
            }
        }
        unsafe extern "system" fn OnNetworkError<Identity: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDDownloadEngineNotifier_Impl::OnNetworkError(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDDownloadEngineNotifier, OFFSET>(),
            OnStreamOpened: OnStreamOpened::<Identity, OFFSET>,
            OnPlayReadyObjectReceived: OnPlayReadyObjectReceived::<Identity, OFFSET>,
            OnContentIDReceived: OnContentIDReceived::<Identity, OFFSET>,
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnEndOfStream: OnEndOfStream::<Identity, OFFSET>,
            OnNetworkError: OnNetworkError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDDownloadEngineNotifier as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnStreamOpened: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnContentIDReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32) -> windows_core::HRESULT,
    pub OnEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDLicenseFetchCompletedEventArgs, INDLicenseFetchCompletedEventArgs_Vtbl, 0x1ee30a1a_11b2_4558_8865_e3a516922517);
impl windows_core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDLicenseFetchCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl INDLicenseFetchCompletedEventArgs {
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for INDLicenseFetchCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
}
pub trait INDLicenseFetchCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
}
impl INDLicenseFetchCompletedEventArgs_Vtbl {
    pub const fn new<Identity: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResponseCustomData<Identity: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDLicenseFetchCompletedEventArgs_Impl::ResponseCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDLicenseFetchDescriptor, INDLicenseFetchDescriptor_Vtbl, 0x5498d33a_e686_4935_a567_7ca77ad20fa4);
impl windows_core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDLicenseFetchDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl INDLicenseFetchDescriptor {
    pub fn ContentIDType(&self) -> windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentIDType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ContentID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for INDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
}
pub trait INDLicenseFetchDescriptor_Impl: windows_core::IUnknownImpl {
    fn ContentIDType(&self) -> windows_core::Result<NDContentIDType>;
    fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(&self, licenseFetchChallengeCustomData: windows_core::Ref<INDCustomData>) -> windows_core::Result<()>;
}
impl INDLicenseFetchDescriptor_Vtbl {
    pub const fn new<Identity: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentIDType<Identity: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDContentIDType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDLicenseFetchDescriptor_Impl::ContentIDType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentID<Identity: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDLicenseFetchDescriptor_Impl::ContentID(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Identity: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDLicenseFetchDescriptor_Impl::LicenseFetchChallengeCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Identity: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchchallengecustomdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDLicenseFetchDescriptor_Impl::SetLicenseFetchChallengeCustomData(this, core::mem::transmute_copy(&licensefetchchallengecustomdata)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchDescriptor, OFFSET>(),
            ContentIDType: ContentIDType::<Identity, OFFSET>,
            ContentID: ContentID::<Identity, OFFSET>,
            LicenseFetchChallengeCustomData: LicenseFetchChallengeCustomData::<Identity, OFFSET>,
            SetLicenseFetchChallengeCustomData: SetLicenseFetchChallengeCustomData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchDescriptor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentIDType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDContentIDType) -> windows_core::HRESULT,
    pub ContentID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDLicenseFetchDescriptorFactory, INDLicenseFetchDescriptorFactory_Vtbl, 0xd0031202_cfac_4f00_ae6a_97af80b848f2);
impl windows_core::RuntimeType for INDLicenseFetchDescriptorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, NDContentIDType, u32, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDLicenseFetchResult, INDLicenseFetchResult_Vtbl, 0x21d39698_aa62_45ff_a5ff_8037e5433825);
impl windows_core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDLicenseFetchResult, windows_core::IUnknown, windows_core::IInspectable);
impl INDLicenseFetchResult {
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for INDLicenseFetchResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
}
pub trait INDLicenseFetchResult_Impl: windows_core::IUnknownImpl {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
}
impl INDLicenseFetchResult_Vtbl {
    pub const fn new<Identity: INDLicenseFetchResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResponseCustomData<Identity: INDLicenseFetchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDLicenseFetchResult_Impl::ResponseCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchResult, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDMessenger, INDMessenger_Vtbl, 0xd42df95d_a75b_47bf_8249_bc83820da38a);
impl windows_core::RuntimeType for INDMessenger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDMessenger, windows_core::IUnknown, windows_core::IInspectable);
impl INDMessenger {
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for INDMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDMessenger";
}
pub trait INDMessenger_Impl: windows_core::IUnknownImpl {
    fn SendRegistrationRequestAsync(&self, sessionIDBytes: &[u8], challengeDataBytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&self, pdType: NDProximityDetectionType, transmitterChannelBytes: &[u8], sessionIDBytes: &[u8], challengeDataBytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&self, pdType: NDProximityDetectionType, transmitterChannelBytes: &[u8], sessionIDBytes: &[u8], responseDataBytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&self, sessionIDBytes: &[u8], challengeDataBytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>>;
}
impl INDMessenger_Vtbl {
    pub const fn new<Identity: INDMessenger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendRegistrationRequestAsync<Identity: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionidbytes_array_size: u32, sessionidbytes: *const u8, challengedatabytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDMessenger_Impl::SendRegistrationRequestAsync(this, core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionidbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengedatabytes_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendProximityDetectionStartAsync<Identity: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterchannelbytes_array_size: u32, transmitterchannelbytes: *const u8, sessionidbytes_array_size: u32, sessionidbytes: *const u8, challengedatabytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDMessenger_Impl::SendProximityDetectionStartAsync(this, pdtype, core::slice::from_raw_parts(core::mem::transmute_copy(&transmitterchannelbytes), transmitterchannelbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionidbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengedatabytes_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Identity: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterchannelbytes_array_size: u32, transmitterchannelbytes: *const u8, sessionidbytes_array_size: u32, sessionidbytes: *const u8, responsedatabytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDMessenger_Impl::SendProximityDetectionResponseAsync(this, pdtype, core::slice::from_raw_parts(core::mem::transmute_copy(&transmitterchannelbytes), transmitterchannelbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionidbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&responsedatabytes), responsedatabytes_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Identity: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionidbytes_array_size: u32, sessionidbytes: *const u8, challengedatabytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDMessenger_Impl::SendLicenseFetchRequestAsync(this, core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionidbytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengedatabytes_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDMessenger, OFFSET>(),
            SendRegistrationRequestAsync: SendRegistrationRequestAsync::<Identity, OFFSET>,
            SendProximityDetectionStartAsync: SendProximityDetectionStartAsync::<Identity, OFFSET>,
            SendProximityDetectionResponseAsync: SendProximityDetectionResponseAsync::<Identity, OFFSET>,
            SendLicenseFetchRequestAsync: SendLicenseFetchRequestAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDMessenger as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, NDProximityDetectionType, u32, *const u8, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, NDProximityDetectionType, u32, *const u8, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDProximityDetectionCompletedEventArgs, INDProximityDetectionCompletedEventArgs_Vtbl, 0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
impl windows_core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDProximityDetectionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl INDProximityDetectionCompletedEventArgs {
    pub fn ProximityDetectionRetryCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityDetectionRetryCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for INDProximityDetectionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
}
pub trait INDProximityDetectionCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ProximityDetectionRetryCount(&self) -> windows_core::Result<u32>;
}
impl INDProximityDetectionCompletedEventArgs_Vtbl {
    pub const fn new<Identity: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProximityDetectionRetryCount<Identity: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDProximityDetectionCompletedEventArgs_Impl::ProximityDetectionRetryCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDProximityDetectionCompletedEventArgs, OFFSET>(),
            ProximityDetectionRetryCount: ProximityDetectionRetryCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDProximityDetectionCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDRegistrationCompletedEventArgs, INDRegistrationCompletedEventArgs_Vtbl, 0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
impl windows_core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDRegistrationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl INDRegistrationCompletedEventArgs {
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransmitterProperties(&self) -> windows_core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitterProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TransmitterCertificateAccepted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitterCertificateAccepted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransmitterCertificateAccepted)(windows_core::Interface::as_raw(this), accept).ok() }
    }
}
impl windows_core::RuntimeName for INDRegistrationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
}
pub trait INDRegistrationCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
    fn TransmitterProperties(&self) -> windows_core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(&self) -> windows_core::Result<bool>;
    fn SetTransmitterCertificateAccepted(&self, accept: bool) -> windows_core::Result<()>;
}
impl INDRegistrationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResponseCustomData<Identity: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDRegistrationCompletedEventArgs_Impl::ResponseCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmitterProperties<Identity: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDRegistrationCompletedEventArgs_Impl::TransmitterProperties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmitterCertificateAccepted<Identity: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDRegistrationCompletedEventArgs_Impl::TransmitterCertificateAccepted(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Identity: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accept: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDRegistrationCompletedEventArgs_Impl::SetTransmitterCertificateAccepted(this, accept).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDRegistrationCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, OFFSET>,
            TransmitterProperties: TransmitterProperties::<Identity, OFFSET>,
            TransmitterCertificateAccepted: TransmitterCertificateAccepted::<Identity, OFFSET>,
            SetTransmitterCertificateAccepted: SetTransmitterCertificateAccepted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDRegistrationCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransmitterProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDSendResult, INDSendResult_Vtbl, 0xe3685517_a584_479d_90b7_d689c7bf7c80);
impl windows_core::RuntimeType for INDSendResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDSendResult, windows_core::IUnknown, windows_core::IInspectable);
impl INDSendResult {
    pub fn Response(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeName for INDSendResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDSendResult";
}
pub trait INDSendResult_Impl: windows_core::IUnknownImpl {
    fn Response(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
impl INDSendResult_Vtbl {
    pub const fn new<Identity: INDSendResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Response<Identity: INDSendResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDSendResult_Impl::Response(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, INDSendResult, OFFSET>(), Response: Response::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDSendResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDStartResult, INDStartResult_Vtbl, 0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
impl windows_core::RuntimeType for INDStartResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDStartResult, windows_core::IUnknown, windows_core::IInspectable);
impl INDStartResult {
    #[cfg(feature = "Media_Core")]
    pub fn MediaStreamSource(&self) -> windows_core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaStreamSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_Core")]
impl windows_core::RuntimeName for INDStartResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStartResult";
}
#[cfg(feature = "Media_Core")]
pub trait INDStartResult_Impl: windows_core::IUnknownImpl {
    fn MediaStreamSource(&self) -> windows_core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(feature = "Media_Core")]
impl INDStartResult_Vtbl {
    pub const fn new<Identity: INDStartResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MediaStreamSource<Identity: INDStartResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDStartResult_Impl::MediaStreamSource(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStartResult, OFFSET>(), MediaStreamSource: MediaStreamSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStartResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub MediaStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaStreamSource: usize,
}
windows_core::imp::define_interface!(INDStorageFileHelper, INDStorageFileHelper_Vtbl, 0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
impl windows_core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDStorageFileHelper, windows_core::IUnknown, windows_core::IInspectable);
impl INDStorageFileHelper {
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileURLs<P0>(&self, file: P0) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileURLs)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for INDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStorageFileHelper";
}
#[cfg(feature = "Storage_Streams")]
pub trait INDStorageFileHelper_Impl: windows_core::IUnknownImpl {
    fn GetFileURLs(&self, file: windows_core::Ref<super::super::super::Storage::IStorageFile>) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>>;
}
#[cfg(feature = "Storage_Streams")]
impl INDStorageFileHelper_Vtbl {
    pub const fn new<Identity: INDStorageFileHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileURLs<Identity: INDStorageFileHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDStorageFileHelper_Impl::GetFileURLs(this, core::mem::transmute_copy(&file)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStorageFileHelper, OFFSET>(), GetFileURLs: GetFileURLs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStorageFileHelper as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileURLs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileURLs: usize,
}
windows_core::imp::define_interface!(INDStreamParser, INDStreamParser_Vtbl, 0xe0baa198_9796_41c9_8695_59437e67e66a);
impl windows_core::RuntimeType for INDStreamParser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDStreamParser, windows_core::IUnknown, windows_core::IInspectable);
impl INDStreamParser {
    pub fn ParseData(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseData)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn GetStreamInformation<P0>(&self, descriptor: P0, streamtype: &mut NDMediaStreamType) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStreamInformation)(windows_core::Interface::as_raw(this), descriptor.param().abi(), streamtype, &mut result__).map(|| result__)
        }
    }
    pub fn BeginOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Notifier(&self) -> windows_core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_Core")]
impl windows_core::RuntimeName for INDStreamParser {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParser";
}
#[cfg(feature = "Media_Core")]
pub trait INDStreamParser_Impl: windows_core::IUnknownImpl {
    fn ParseData(&self, dataBytes: &[u8]) -> windows_core::Result<()>;
    fn GetStreamInformation(&self, descriptor: windows_core::Ref<super::super::Core::IMediaStreamDescriptor>, streamType: &mut NDMediaStreamType) -> windows_core::Result<u32>;
    fn BeginOfStream(&self) -> windows_core::Result<()>;
    fn EndOfStream(&self) -> windows_core::Result<()>;
    fn Notifier(&self) -> windows_core::Result<NDStreamParserNotifier>;
}
#[cfg(feature = "Media_Core")]
impl INDStreamParser_Vtbl {
    pub const fn new<Identity: INDStreamParser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseData<Identity: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, databytes_array_size: u32, databytes: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParser_Impl::ParseData(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), databytes_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn GetStreamInformation<Identity: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDStreamParser_Impl::GetStreamInformation(this, core::mem::transmute_copy(&descriptor), core::mem::transmute_copy(&streamtype)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginOfStream<Identity: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParser_Impl::BeginOfStream(this).into()
            }
        }
        unsafe extern "system" fn EndOfStream<Identity: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParser_Impl::EndOfStream(this).into()
            }
        }
        unsafe extern "system" fn Notifier<Identity: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDStreamParser_Impl::Notifier(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStreamParser, OFFSET>(),
            ParseData: ParseData::<Identity, OFFSET>,
            GetStreamInformation: GetStreamInformation::<Identity, OFFSET>,
            BeginOfStream: BeginOfStream::<Identity, OFFSET>,
            EndOfStream: EndOfStream::<Identity, OFFSET>,
            Notifier: Notifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStreamParser as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ParseData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub GetStreamInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut NDMediaStreamType, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    GetStreamInformation: usize,
    pub BeginOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDStreamParserNotifier, INDStreamParserNotifier_Vtbl, 0xc167acd0_2ce6_426c_ace5_5e9275fea715);
impl windows_core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDStreamParserNotifier, windows_core::IUnknown, windows_core::IInspectable);
impl INDStreamParserNotifier {
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: windows_core::Param<windows_collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(windows_core::Interface::as_raw(this), audiostreamdescriptors.param().abi(), videostreamdescriptors.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnSampleParsed<P2>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnSampleParsed)(windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: windows_core::GUID, probytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(windows_core::Interface::as_raw(this), descriptor.param().abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "Media_Core")]
impl windows_core::RuntimeName for INDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
}
#[cfg(feature = "Media_Core")]
pub trait INDStreamParserNotifier_Impl: windows_core::IUnknownImpl {
    fn OnContentIDReceived(&self, licenseFetchDescriptor: windows_core::Ref<INDLicenseFetchDescriptor>) -> windows_core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&self, audioStreamDescriptors: windows_core::Ref<windows_collections::IVector<super::super::Core::AudioStreamDescriptor>>, videoStreamDescriptors: windows_core::Ref<windows_collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> windows_core::Result<()>;
    fn OnSampleParsed(&self, streamID: u32, streamType: NDMediaStreamType, streamSample: windows_core::Ref<super::super::Core::MediaStreamSample>, pts: i64, ccFormat: NDClosedCaptionFormat, ccDataBytes: &[u8]) -> windows_core::Result<()>;
    fn OnBeginSetupDecryptor(&self, descriptor: windows_core::Ref<super::super::Core::IMediaStreamDescriptor>, keyID: &windows_core::GUID, proBytes: &[u8]) -> windows_core::Result<()>;
}
#[cfg(feature = "Media_Core")]
impl INDStreamParserNotifier_Vtbl {
    pub const fn new<Identity: INDStreamParserNotifier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnContentIDReceived<Identity: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchdescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParserNotifier_Impl::OnContentIDReceived(this, core::mem::transmute_copy(&licensefetchdescriptor)).into()
            }
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Identity: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiostreamdescriptors: *mut core::ffi::c_void, videostreamdescriptors: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParserNotifier_Impl::OnMediaStreamDescriptorCreated(this, core::mem::transmute_copy(&audiostreamdescriptors), core::mem::transmute_copy(&videostreamdescriptors)).into()
            }
        }
        unsafe extern "system" fn OnSampleParsed<Identity: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes_array_size: u32, ccdatabytes: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParserNotifier_Impl::OnSampleParsed(this, streamid, streamtype, core::mem::transmute_copy(&streamsample), pts, ccformat, core::slice::from_raw_parts(core::mem::transmute_copy(&ccdatabytes), ccdatabytes_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Identity: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void, keyid: windows_core::GUID, probytes_array_size: u32, probytes: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INDStreamParserNotifier_Impl::OnBeginSetupDecryptor(this, core::mem::transmute_copy(&descriptor), core::mem::transmute(&keyid), core::slice::from_raw_parts(core::mem::transmute_copy(&probytes), probytes_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStreamParserNotifier, OFFSET>(),
            OnContentIDReceived: OnContentIDReceived::<Identity, OFFSET>,
            OnMediaStreamDescriptorCreated: OnMediaStreamDescriptorCreated::<Identity, OFFSET>,
            OnSampleParsed: OnSampleParsed::<Identity, OFFSET>,
            OnBeginSetupDecryptor: OnBeginSetupDecryptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStreamParserNotifier as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnContentIDReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(feature = "Media_Core")]
    pub OnSampleParsed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, NDMediaStreamType, *mut core::ffi::c_void, i64, NDClosedCaptionFormat, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    OnSampleParsed: usize,
    #[cfg(feature = "Media_Core")]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    OnBeginSetupDecryptor: usize,
}
windows_core::imp::define_interface!(INDTCPMessengerFactory, INDTCPMessengerFactory_Vtbl, 0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
impl windows_core::RuntimeType for INDTCPMessengerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INDTransmitterProperties, INDTransmitterProperties_Vtbl, 0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
impl windows_core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(INDTransmitterProperties, windows_core::IUnknown, windows_core::IInspectable);
impl INDTransmitterProperties {
    pub fn CertificateType(&self) -> windows_core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CertificateType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PlatformIdentifier(&self) -> windows_core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlatformIdentifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedFeatures(&self) -> windows_core::Result<windows_core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SupportedFeatures)(windows_core::Interface::as_raw(this), windows_core::Array::<NDCertificateFeature>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SecurityLevel(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecurityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SecurityVersion(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecurityVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ClientID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ClientID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn ModelDigest(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ModelDigest)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn ModelManufacturerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelManufacturerName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ModelName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ModelNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for INDTransmitterProperties {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTransmitterProperties";
}
pub trait INDTransmitterProperties_Impl: windows_core::IUnknownImpl {
    fn CertificateType(&self) -> windows_core::Result<NDCertificateType>;
    fn PlatformIdentifier(&self) -> windows_core::Result<NDCertificatePlatformID>;
    fn SupportedFeatures(&self) -> windows_core::Result<windows_core::Array<NDCertificateFeature>>;
    fn SecurityLevel(&self) -> windows_core::Result<u32>;
    fn SecurityVersion(&self) -> windows_core::Result<u32>;
    fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn ClientID(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn ModelDigest(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn ModelManufacturerName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ModelName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ModelNumber(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl INDTransmitterProperties_Vtbl {
    pub const fn new<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CertificateType<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDCertificateType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::CertificateType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PlatformIdentifier<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::PlatformIdentifier(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedFeatures<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::SupportedFeatures(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SecurityLevel<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::SecurityLevel(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SecurityVersion<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::SecurityVersion(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ExpirationDate(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientID<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ClientID(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelDigest<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ModelDigest(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelManufacturerName<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ModelManufacturerName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelName<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ModelName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INDTransmitterProperties_Impl::ModelNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDTransmitterProperties, OFFSET>(),
            CertificateType: CertificateType::<Identity, OFFSET>,
            PlatformIdentifier: PlatformIdentifier::<Identity, OFFSET>,
            SupportedFeatures: SupportedFeatures::<Identity, OFFSET>,
            SecurityLevel: SecurityLevel::<Identity, OFFSET>,
            SecurityVersion: SecurityVersion::<Identity, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, OFFSET>,
            ClientID: ClientID::<Identity, OFFSET>,
            ModelDigest: ModelDigest::<Identity, OFFSET>,
            ModelManufacturerName: ModelManufacturerName::<Identity, OFFSET>,
            ModelName: ModelName::<Identity, OFFSET>,
            ModelNumber: ModelNumber::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDTransmitterProperties as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CertificateType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDCertificateType) -> windows_core::HRESULT,
    pub PlatformIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDCertificatePlatformID) -> windows_core::HRESULT,
    pub SupportedFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut NDCertificateFeature) -> windows_core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SecurityVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ClientID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ModelDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ModelManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeader, IPlayReadyContentHeader_Vtbl, 0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
impl windows_core::RuntimeType for IPlayReadyContentHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayReadyEncryptionAlgorithm) -> windows_core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayReadyDecryptorSetup) -> windows_core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeader2, IPlayReadyContentHeader2_Vtbl, 0x359c79f4_2180_498c_965b_e754d875eab2);
impl windows_core::RuntimeType for IPlayReadyContentHeader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeaderFactory, IPlayReadyContentHeaderFactory_Vtbl, 0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
impl windows_core::RuntimeType for IPlayReadyContentHeaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromComponents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, PlayReadyEncryptionAlgorithm, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeaderFactory2, IPlayReadyContentHeaderFactory2_Vtbl, 0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
impl windows_core::RuntimeType for IPlayReadyContentHeaderFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, u32, *const windows_core::HSTRING, PlayReadyEncryptionAlgorithm, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentResolver, IPlayReadyContentResolver_Vtbl, 0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
impl windows_core::RuntimeType for IPlayReadyContentResolver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomain, IPlayReadyDomain_Vtbl, 0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
impl windows_core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyDomain, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayReadyDomain {
    pub fn AccountId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Revision(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DomainJoinUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainJoinUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IPlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomain";
}
pub trait IPlayReadyDomain_Impl: windows_core::IUnknownImpl {
    fn AccountId(&self) -> windows_core::Result<windows_core::GUID>;
    fn ServiceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Revision(&self) -> windows_core::Result<u32>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn DomainJoinUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri>;
}
impl IPlayReadyDomain_Vtbl {
    pub const fn new<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccountId<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyDomain_Impl::AccountId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceId<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyDomain_Impl::ServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Revision<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyDomain_Impl::Revision(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyDomain_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainJoinUrl<Identity: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyDomain_Impl::DomainJoinUrl(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyDomain, OFFSET>(),
            AccountId: AccountId::<Identity, OFFSET>,
            ServiceId: ServiceId::<Identity, OFFSET>,
            Revision: Revision::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            DomainJoinUrl: DomainJoinUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyDomain as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainJoinUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomainIterableFactory, IPlayReadyDomainIterableFactory_Vtbl, 0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
impl windows_core::RuntimeType for IPlayReadyDomainIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomainJoinServiceRequest, IPlayReadyDomainJoinServiceRequest_Vtbl, 0x171b4a5a_405f_4739_b040_67b9f0c38758);
impl windows_core::RuntimeType for IPlayReadyDomainJoinServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomainLeaveServiceRequest, IPlayReadyDomainLeaveServiceRequest_Vtbl, 0x062d58be_97ad_4917_aa03_46d4c252d464);
impl windows_core::RuntimeType for IPlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyITADataGenerator, IPlayReadyITADataGenerator_Vtbl, 0x24446b8e_10b9_4530_b25b_901a8029a9b2);
impl windows_core::RuntimeType for IPlayReadyITADataGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GenerateData: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *mut core::ffi::c_void, PlayReadyITADataFormat, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GenerateData: usize,
}
windows_core::imp::define_interface!(IPlayReadyIndividualizationServiceRequest, IPlayReadyIndividualizationServiceRequest_Vtbl, 0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
impl windows_core::RuntimeType for IPlayReadyIndividualizationServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPlayReadyLicense, IPlayReadyLicense_Vtbl, 0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
impl windows_core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyLicense, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayReadyLicense {
    pub fn FullyEvaluated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullyEvaluated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsableForPlay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsableForPlay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DomainAccountID(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainAccountID)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ChainDepth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChainDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKIDAtChainDepth)(windows_core::Interface::as_raw(this), chaindepth, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense";
}
pub trait IPlayReadyLicense_Impl: windows_core::IUnknownImpl {
    fn FullyEvaluated(&self) -> windows_core::Result<bool>;
    fn UsableForPlay(&self) -> windows_core::Result<bool>;
    fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&self) -> windows_core::Result<u32>;
    fn DomainAccountID(&self) -> windows_core::Result<windows_core::GUID>;
    fn ChainDepth(&self) -> windows_core::Result<u32>;
    fn GetKIDAtChainDepth(&self, chainDepth: u32) -> windows_core::Result<windows_core::GUID>;
}
impl IPlayReadyLicense_Vtbl {
    pub const fn new<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FullyEvaluated<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::FullyEvaluated(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsableForPlay<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::UsableForPlay(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::ExpirationDate(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpireAfterFirstPlay<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::ExpireAfterFirstPlay(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DomainAccountID<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::DomainAccountID(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChainDepth<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::ChainDepth(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKIDAtChainDepth<Identity: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaindepth: u32, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicense_Impl::GetKIDAtChainDepth(this, chaindepth) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicense, OFFSET>(),
            FullyEvaluated: FullyEvaluated::<Identity, OFFSET>,
            UsableForPlay: UsableForPlay::<Identity, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, OFFSET>,
            ExpireAfterFirstPlay: ExpireAfterFirstPlay::<Identity, OFFSET>,
            DomainAccountID: DomainAccountID::<Identity, OFFSET>,
            ChainDepth: ChainDepth::<Identity, OFFSET>,
            GetKIDAtChainDepth: GetKIDAtChainDepth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicense as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FullyEvaluated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UsableForPlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpireAfterFirstPlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DomainAccountID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ChainDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetKIDAtChainDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicense2, IPlayReadyLicense2_Vtbl, 0x30f4e7a7_d8e3_48a0_bcda_ff9f40530436);
impl windows_core::RuntimeType for IPlayReadyLicense2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SecureStopId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseAcquisitionServiceRequest, IPlayReadyLicenseAcquisitionServiceRequest_Vtbl, 0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
impl windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyLicenseAcquisitionServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPlayReadyLicenseAcquisitionServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl IPlayReadyLicenseAcquisitionServiceRequest {
    pub fn ContentHeader(&self) -> windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentHeader)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainServiceId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
}
pub trait IPlayReadyLicenseAcquisitionServiceRequest_Impl: super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn ContentHeader(&self) -> windows_core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&self, value: windows_core::Ref<PlayReadyContentHeader>) -> windows_core::Result<()>;
    fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetDomainServiceId(&self, value: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub const fn new<Identity: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentHeader<Identity: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicenseAcquisitionServiceRequest_Impl::ContentHeader(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentHeader<Identity: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayReadyLicenseAcquisitionServiceRequest_Impl::SetContentHeader(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DomainServiceId<Identity: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicenseAcquisitionServiceRequest_Impl::DomainServiceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDomainServiceId<Identity: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayReadyLicenseAcquisitionServiceRequest_Impl::SetDomainServiceId(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseAcquisitionServiceRequest, OFFSET>(),
            ContentHeader: ContentHeader::<Identity, OFFSET>,
            SetContentHeader: SetContentHeader::<Identity, OFFSET>,
            DomainServiceId: DomainServiceId::<Identity, OFFSET>,
            SetDomainServiceId: SetDomainServiceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseAcquisitionServiceRequest as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseAcquisitionServiceRequest2, IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl, 0xb7fa5eb5_fe0c_b225_bc60_5a9edd32ceb5);
impl windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseAcquisitionServiceRequest3, IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl, 0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
impl windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLicenseIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseIterableFactory, IPlayReadyLicenseIterableFactory_Vtbl, 0xd4179f08_0837_4978_8e68_be4293c8d7a6);
impl windows_core::RuntimeType for IPlayReadyLicenseIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseManagement, IPlayReadyLicenseManagement_Vtbl, 0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
impl windows_core::RuntimeType for IPlayReadyLicenseManagement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeleteLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSession, IPlayReadyLicenseSession_Vtbl, 0xa1723a39_87fa_4fdd_abbb_a9720e845259);
impl windows_core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyLicenseSession, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLAServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(windows_core::Interface::as_raw(this), mpm.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IPlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
}
pub trait IPlayReadyLicenseSession_Impl: windows_core::IUnknownImpl {
    fn CreateLAServiceRequest(&self) -> windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&self, mpm: windows_core::Ref<super::MediaProtectionManager>) -> windows_core::Result<()>;
}
impl IPlayReadyLicenseSession_Vtbl {
    pub const fn new<Identity: IPlayReadyLicenseSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLAServiceRequest<Identity: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicenseSession_Impl::CreateLAServiceRequest(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConfigureMediaProtectionManager<Identity: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mpm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayReadyLicenseSession_Impl::ConfigureMediaProtectionManager(this, core::mem::transmute_copy(&mpm)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseSession, OFFSET>(),
            CreateLAServiceRequest: CreateLAServiceRequest::<Identity, OFFSET>,
            ConfigureMediaProtectionManager: ConfigureMediaProtectionManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLAServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSession2, IPlayReadyLicenseSession2_Vtbl, 0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
impl windows_core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyLicenseSession2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPlayReadyLicenseSession2, IPlayReadyLicenseSession);
impl IPlayReadyLicenseSession2 {
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLicenseIterable)(windows_core::Interface::as_raw(this), contentheader.param().abi(), fullyevaluated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateLAServiceRequest(&self) -> windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLAServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProtectionManager>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(windows_core::Interface::as_raw(this), mpm.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IPlayReadyLicenseSession2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
}
pub trait IPlayReadyLicenseSession2_Impl: IPlayReadyLicenseSession_Impl {
    fn CreateLicenseIterable(&self, contentHeader: windows_core::Ref<PlayReadyContentHeader>, fullyEvaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>;
}
impl IPlayReadyLicenseSession2_Vtbl {
    pub const fn new<Identity: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateLicenseIterable<Identity: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentheader: *mut core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyLicenseSession2_Impl::CreateLicenseIterable(this, core::mem::transmute_copy(&contentheader), fullyevaluated) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseSession2, OFFSET>(),
            CreateLicenseIterable: CreateLicenseIterable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLicenseIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSessionFactory, IPlayReadyLicenseSessionFactory_Vtbl, 0x62492699_6527_429e_98be_48d798ac2739);
impl windows_core::RuntimeType for IPlayReadyLicenseSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPlayReadyMeteringReportServiceRequest, IPlayReadyMeteringReportServiceRequest_Vtbl, 0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
impl windows_core::RuntimeType for IPlayReadyMeteringReportServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MeteringCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetMeteringCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyRevocationServiceRequest, IPlayReadyRevocationServiceRequest_Vtbl, 0x543d66ac_faf0_4560_84a5_0e4acec939e4);
impl windows_core::RuntimeType for IPlayReadyRevocationServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPlayReadySecureStopIterableFactory, IPlayReadySecureStopIterableFactory_Vtbl, 0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
impl windows_core::RuntimeType for IPlayReadySecureStopIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequest_Vtbl, 0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
impl windows_core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadySecureStopServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPlayReadySecureStopServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl IPlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionID)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stopped(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PublisherCertificate(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).PublisherCertificate)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
}
pub trait IPlayReadySecureStopServiceRequest_Impl: super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn SessionID(&self) -> windows_core::Result<windows_core::GUID>;
    fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&self) -> windows_core::Result<bool>;
    fn PublisherCertificate(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
impl IPlayReadySecureStopServiceRequest_Vtbl {
    pub const fn new<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionID<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadySecureStopServiceRequest_Impl::SessionID(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartTime<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadySecureStopServiceRequest_Impl::StartTime(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateTime<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadySecureStopServiceRequest_Impl::UpdateTime(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stopped<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadySecureStopServiceRequest_Impl::Stopped(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PublisherCertificate<Identity: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadySecureStopServiceRequest_Impl::PublisherCertificate(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadySecureStopServiceRequest, OFFSET>(),
            SessionID: SessionID::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            UpdateTime: UpdateTime::<Identity, OFFSET>,
            Stopped: Stopped::<Identity, OFFSET>,
            PublisherCertificate: PublisherCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadySecureStopServiceRequest as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub UpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PublisherCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadySecureStopServiceRequestFactory, IPlayReadySecureStopServiceRequestFactory_Vtbl, 0x0e448ac9_e67e_494e_9f49_6285438c76cf);
impl windows_core::RuntimeType for IPlayReadySecureStopServiceRequestFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyServiceRequest, IPlayReadyServiceRequest_Vtbl, 0x8bad2836_a703_45a6_a180_76f3565aa725);
impl windows_core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPlayReadyServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPlayReadyServiceRequest, super::IMediaProtectionServiceRequest);
impl IPlayReadyServiceRequest {
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPlayReadyServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
}
pub trait IPlayReadyServiceRequest_Impl: super::IMediaProtectionServiceRequest_Impl {
    fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: windows_core::Ref<super::super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction>;
    fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&self, responseBytes: &[u8]) -> windows_core::Result<windows_core::HRESULT>;
}
impl IPlayReadyServiceRequest_Vtbl {
    pub const fn new<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Uri<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::Uri(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUri<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayReadyServiceRequest_Impl::SetUri(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ResponseCustomData<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::ResponseCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChallengeCustomData<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::ChallengeCustomData(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChallengeCustomData<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayReadyServiceRequest_Impl::SetChallengeCustomData(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn BeginServiceRequest<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::BeginServiceRequest(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextServiceRequest<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::NextServiceRequest(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateManualEnablingChallenge<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::GenerateManualEnablingChallenge(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessManualEnablingResponse<Identity: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, responsebytes_array_size: u32, responsebytes: *const u8, result__: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPlayReadyServiceRequest_Impl::ProcessManualEnablingResponse(this, core::slice::from_raw_parts(core::mem::transmute_copy(&responsebytes), responsebytes_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyServiceRequest, OFFSET>(),
            Uri: Uri::<Identity, OFFSET>,
            SetUri: SetUri::<Identity, OFFSET>,
            ResponseCustomData: ResponseCustomData::<Identity, OFFSET>,
            ChallengeCustomData: ChallengeCustomData::<Identity, OFFSET>,
            SetChallengeCustomData: SetChallengeCustomData::<Identity, OFFSET>,
            BeginServiceRequest: BeginServiceRequest::<Identity, OFFSET>,
            NextServiceRequest: NextServiceRequest::<Identity, OFFSET>,
            GenerateManualEnablingChallenge: GenerateManualEnablingChallenge::<Identity, OFFSET>,
            ProcessManualEnablingResponse: ProcessManualEnablingResponse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyServiceRequest as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenerateManualEnablingChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessManualEnablingResponse: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadySoapMessage, IPlayReadySoapMessage_Vtbl, 0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
impl windows_core::RuntimeType for IPlayReadySoapMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetMessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageHeaders: usize,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics, IPlayReadyStatics_Vtbl, 0x5e69c00d_247c_469a_8f31_5c1a1571d9c6);
impl windows_core::RuntimeType for IPlayReadyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DomainJoinServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DomainLeaveServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IndividualizationServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub LicenseAcquirerServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub MeteringReportServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RevocationServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub MediaProtectionSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub PlayReadySecurityVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics2, IPlayReadyStatics2_Vtbl, 0x1f8d6a92_5f9a_423e_9466_b33969af7a3d);
impl windows_core::RuntimeType for IPlayReadyStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics3, IPlayReadyStatics3_Vtbl, 0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
impl windows_core::RuntimeType for IPlayReadyStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SecureStopServiceRequestType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CheckSupportedHardware: unsafe extern "system" fn(*mut core::ffi::c_void, PlayReadyHardwareDRMFeatures, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics4, IPlayReadyStatics4_Vtbl, 0x50a91300_d824_4231_9d5e_78ef8844c7d7);
impl windows_core::RuntimeType for IPlayReadyStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics5, IPlayReadyStatics5_Vtbl, 0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
impl windows_core::RuntimeType for IPlayReadyStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDCertificateFeature(pub i32);
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
impl windows_core::TypeKind for NDCertificateFeature {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDCertificatePlatformID(pub i32);
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
impl windows_core::TypeKind for NDCertificatePlatformID {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDCertificateType(pub i32);
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
impl windows_core::TypeKind for NDCertificateType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDCertificateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDClient(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDClient, windows_core::IUnknown, windows_core::IInspectable);
impl NDClient {
    pub fn RegistrationCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRegistrationCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRegistrationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ProximityDetectionCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityDetectionCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProximityDetectionCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProximityDetectionCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn LicenseFetchCompleted<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveLicenseFetchCompleted(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLicenseFetchCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ReRegistrationNeeded<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReRegistrationNeeded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReRegistrationNeeded(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReRegistrationNeeded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ClosedCaptionDataReceived<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionDataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosedCaptionDataReceived(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosedCaptionDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StartAsync<P0, P2, P3>(&self, contenturl: P0, startasyncoptions: u32, registrationcustomdata: P2, licensefetchdescriptor: P3) -> windows_core::Result<windows_future::IAsyncOperation<INDStartResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P2: windows_core::Param<INDCustomData>,
        P3: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), contenturl.param().abi(), startasyncoptions, registrationcustomdata.param().abi(), licensefetchdescriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LicenseFetchAsync<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<windows_future::IAsyncOperation<INDLicenseFetchResult>>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchAsync)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReRegistrationAsync<P0>(&self, registrationcustomdata: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReRegistrationAsync)(windows_core::Interface::as_raw(this), registrationcustomdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateInstance<P0, P1, P2>(downloadengine: P0, streamparser: P1, pmessenger: P2) -> windows_core::Result<NDClient>
    where
        P0: windows_core::Param<INDDownloadEngine>,
        P1: windows_core::Param<INDStreamParser>,
        P2: windows_core::Param<INDMessenger>,
    {
        Self::INDClientFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), downloadengine.param().abi(), streamparser.param().abi(), pmessenger.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDClient, INDClientFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NDClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDClient>();
}
unsafe impl windows_core::Interface for NDClient {
    type Vtable = <INDClient as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDClient as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDClosedCaptionFormat(pub i32);
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl windows_core::TypeKind for NDClosedCaptionFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDContentIDType(pub i32);
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl windows_core::TypeKind for NDContentIDType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDContentIDType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDCustomData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDCustomData, windows_core::IUnknown, windows_core::IInspectable, INDCustomData);
impl NDCustomData {
    pub fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomDataTypeID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CreateInstance(customdatatypeidbytes: &[u8], customdatabytes: &[u8]) -> windows_core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), customdatatypeidbytes.len().try_into().unwrap(), customdatatypeidbytes.as_ptr(), customdatabytes.len().try_into().unwrap(), customdatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDCustomData, INDCustomDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NDCustomData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDCustomData>();
}
unsafe impl windows_core::Interface for NDCustomData {
    type Vtable = <INDCustomData as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDCustomData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDDownloadEngineNotifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDDownloadEngineNotifier, windows_core::IUnknown, windows_core::IInspectable, INDDownloadEngineNotifier);
impl NDDownloadEngineNotifier {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDDownloadEngineNotifier, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OnStreamOpened(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnStreamOpened)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnDataReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    pub fn OnEndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnEndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OnNetworkError(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnNetworkError)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDDownloadEngineNotifier>();
}
unsafe impl windows_core::Interface for NDDownloadEngineNotifier {
    type Vtable = <INDDownloadEngineNotifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDDownloadEngineNotifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDLicenseFetchDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDLicenseFetchDescriptor, windows_core::IUnknown, windows_core::IInspectable, INDLicenseFetchDescriptor);
impl NDLicenseFetchDescriptor {
    pub fn ContentIDType(&self) -> windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentIDType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ContentID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.param().abi()).ok() }
    }
    pub fn CreateInstance<P2>(contentidtype: NDContentIDType, contentidbytes: &[u8], licensefetchchallengecustomdata: P2) -> windows_core::Result<NDLicenseFetchDescriptor>
    where
        P2: windows_core::Param<INDCustomData>,
    {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), contentidtype, contentidbytes.len().try_into().unwrap(), contentidbytes.as_ptr(), licensefetchchallengecustomdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDLicenseFetchDescriptor>();
}
unsafe impl windows_core::Interface for NDLicenseFetchDescriptor {
    type Vtable = <INDLicenseFetchDescriptor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDLicenseFetchDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDMediaStreamType(pub i32);
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl windows_core::TypeKind for NDMediaStreamType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDProximityDetectionType(pub i32);
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
impl windows_core::TypeKind for NDProximityDetectionType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NDStartAsyncOptions(pub i32);
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
impl windows_core::TypeKind for NDStartAsyncOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDStorageFileHelper(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDStorageFileHelper, windows_core::IUnknown, windows_core::IInspectable, INDStorageFileHelper);
impl NDStorageFileHelper {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDStorageFileHelper, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileURLs<P0>(&self, file: P0) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileURLs)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDStorageFileHelper>();
}
unsafe impl windows_core::Interface for NDStorageFileHelper {
    type Vtable = <INDStorageFileHelper as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDStorageFileHelper as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDStreamParserNotifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDStreamParserNotifier, windows_core::IUnknown, windows_core::IInspectable, INDStreamParserNotifier);
impl NDStreamParserNotifier {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDStreamParserNotifier, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: windows_core::Param<windows_collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(windows_core::Interface::as_raw(this), audiostreamdescriptors.param().abi(), videostreamdescriptors.param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnSampleParsed<P2>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> windows_core::Result<()>
    where
        P2: windows_core::Param<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnSampleParsed)(windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: windows_core::GUID, probytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(windows_core::Interface::as_raw(this), descriptor.param().abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDStreamParserNotifier>();
}
unsafe impl windows_core::Interface for NDStreamParserNotifier {
    type Vtable = <INDStreamParserNotifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDStreamParserNotifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NDTCPMessenger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NDTCPMessenger, windows_core::IUnknown, windows_core::IInspectable, INDMessenger);
impl NDTCPMessenger {
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<windows_future::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(remotehostname: &windows_core::HSTRING, remotehostport: u32) -> windows_core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(remotehostname), remotehostport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDMessenger>();
}
unsafe impl windows_core::Interface for NDTCPMessenger {
    type Vtable = <INDMessenger as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INDMessenger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyContentHeader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyContentHeader, windows_core::IUnknown, windows_core::IInspectable);
impl PlayReadyContentHeader {
    pub fn KeyId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyIdString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyIdString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn LicenseAcquisitionUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseAcquisitionUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseAcquisitionUserInterfaceUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EncryptionType(&self) -> windows_core::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EncryptionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CustomAttributes(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomAttributes)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DecryptorSetup(&self) -> windows_core::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecryptorSetup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSerializedHeader(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetSerializedHeader)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn HeaderWithEmbeddedUpdates(&self) -> windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderWithEmbeddedUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyIds(&self) -> windows_core::Result<windows_core::Array<windows_core::GUID>> {
        let this = &windows_core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).KeyIds)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::GUID>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn KeyIdStrings(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).KeyIdStrings)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CreateInstanceFromWindowsMediaDrmHeader<P1, P2>(headerbytes: &[u8], licenseacquisitionurl: P1, licenseacquisitionuserinterfaceurl: P2, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
        P2: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromWindowsMediaDrmHeader)(windows_core::Interface::as_raw(this), headerbytes.len().try_into().unwrap(), headerbytes.as_ptr(), licenseacquisitionurl.param().abi(), licenseacquisitionuserinterfaceurl.param().abi(), core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceFromComponents<P3, P4>(contentkeyid: windows_core::GUID, contentkeyidstring: &windows_core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P3, licenseacquisitionuserinterfaceurl: P4, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P3: windows_core::Param<super::super::super::Foundation::Uri>,
        P4: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromComponents)(windows_core::Interface::as_raw(this), contentkeyid, core::mem::transmute_copy(contentkeyidstring), contentencryptionalgorithm, licenseacquisitionurl.param().abi(), licenseacquisitionuserinterfaceurl.param().abi(), core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[u8]) -> windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromPlayReadyHeader)(windows_core::Interface::as_raw(this), headerbytes.len().try_into().unwrap(), headerbytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceFromComponents2<P4, P5>(dwflags: u32, contentkeyids: &[windows_core::GUID], contentkeyidstrings: &[windows_core::HSTRING], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P4, licenseacquisitionuserinterfaceurl: P5, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P4: windows_core::Param<super::super::super::Foundation::Uri>,
        P5: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromComponents2)(windows_core::Interface::as_raw(this), dwflags, contentkeyids.len().try_into().unwrap(), contentkeyids.as_ptr(), contentkeyidstrings.len().try_into().unwrap(), core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, licenseacquisitionurl.param().abi(), licenseacquisitionuserinterfaceurl.param().abi(), core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyContentHeader>();
}
unsafe impl windows_core::Interface for PlayReadyContentHeader {
    type Vtable = <IPlayReadyContentHeader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyContentHeader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
pub struct PlayReadyContentResolver;
impl PlayReadyContentResolver {
    pub fn ServiceRequest<P0>(contentheader: P0) -> windows_core::Result<IPlayReadyServiceRequest>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceRequest)(windows_core::Interface::as_raw(this), contentheader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl windows_core::TypeKind for PlayReadyDecryptorSetup {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyDomain(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomain, windows_core::IUnknown, windows_core::IInspectable, IPlayReadyDomain);
impl PlayReadyDomain {
    pub fn AccountId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Revision(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revision)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DomainJoinUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainJoinUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayReadyDomain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyDomain>();
}
unsafe impl windows_core::Interface for PlayReadyDomain {
    type Vtable = <IPlayReadyDomain as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyDomain as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyDomainIterable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomainIterable, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterable<IPlayReadyDomain>);
impl PlayReadyDomainIterable {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(domainaccountid: windows_core::GUID) -> windows_core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), domainaccountid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterable<IPlayReadyDomain>>();
}
unsafe impl windows_core::Interface for PlayReadyDomainIterable {
    type Vtable = <windows_collections::IIterable<IPlayReadyDomain> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterable<IPlayReadyDomain> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
impl IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyDomainIterator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomainIterator, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterator<IPlayReadyDomain>);
impl PlayReadyDomainIterator {
    pub fn Current(&self) -> windows_core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, items: &mut [Option<IPlayReadyDomain>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterator<IPlayReadyDomain>>();
}
unsafe impl windows_core::Interface for PlayReadyDomainIterator {
    type Vtable = <windows_collections::IIterator<IPlayReadyDomain> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterator<IPlayReadyDomain> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyDomainJoinServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomainJoinServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyDomainJoinServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyDomainJoinServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DomainAccountId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainAccountId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainAccountId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainFriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDomainFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainServiceId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyDomainJoinServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyDomainJoinServiceRequest {
    type Vtable = <IPlayReadyDomainJoinServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyDomainJoinServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyDomainLeaveServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomainLeaveServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyDomainLeaveServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyDomainLeaveServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DomainAccountId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainAccountId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainAccountId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainServiceId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyDomainLeaveServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyDomainLeaveServiceRequest {
    type Vtable = <IPlayReadyDomainLeaveServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyDomainLeaveServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl windows_core::TypeKind for PlayReadyEncryptionAlgorithm {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl windows_core::TypeKind for PlayReadyHardwareDRMFeatures {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl windows_core::TypeKind for PlayReadyITADataFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyITADataGenerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyITADataGenerator, windows_core::IUnknown, windows_core::IInspectable);
impl PlayReadyITADataGenerator {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyITADataGenerator, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GenerateData<P2>(&self, guidcpsystemid: windows_core::GUID, countofstreams: u32, configuration: P2, format: PlayReadyITADataFormat) -> windows_core::Result<windows_core::Array<u8>>
    where
        P2: windows_core::Param<super::super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GenerateData)(windows_core::Interface::as_raw(this), guidcpsystemid, countofstreams, configuration.param().abi(), format, windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyITADataGenerator>();
}
unsafe impl windows_core::Interface for PlayReadyITADataGenerator {
    type Vtable = <IPlayReadyITADataGenerator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyITADataGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyIndividualizationServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyIndividualizationServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyIndividualizationServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyIndividualizationServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyIndividualizationServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyIndividualizationServiceRequest {
    type Vtable = <IPlayReadyIndividualizationServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyIndividualizationServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyLicense(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicense, windows_core::IUnknown, windows_core::IInspectable, IPlayReadyLicense);
impl PlayReadyLicense {
    pub fn FullyEvaluated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullyEvaluated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UsableForPlay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UsableForPlay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DomainAccountID(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainAccountID)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ChainDepth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChainDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKIDAtChainDepth)(windows_core::Interface::as_raw(this), chaindepth, &mut result__).map(|| result__)
        }
    }
    pub fn SecureStopId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecureStopId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SecurityLevel(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecurityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InMemoryOnly(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InMemoryOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExpiresInRealTime(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpiresInRealTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyLicense {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyLicense>();
}
unsafe impl windows_core::Interface for PlayReadyLicense {
    type Vtable = <IPlayReadyLicense as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyLicense as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, windows_core::IUnknown, windows_core::IInspectable, IPlayReadyLicenseAcquisitionServiceRequest);
windows_core::imp::required_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContentHeader(&self) -> windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentHeader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentHeader<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentHeader)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainServiceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDomainServiceId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SessionId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLicenseIterable)(windows_core::Interface::as_raw(this), contentheader.param().abi(), fullyevaluated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyLicenseAcquisitionServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = <IPlayReadyLicenseAcquisitionServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyLicenseAcquisitionServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyLicenseIterable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterable, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterable<IPlayReadyLicense>);
impl PlayReadyLicenseIterable {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseIterable, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0>(contentheader: P0, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), contentheader.param().abi(), fullyevaluated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterable<IPlayReadyLicense>>();
}
unsafe impl windows_core::Interface for PlayReadyLicenseIterable {
    type Vtable = <windows_collections::IIterable<IPlayReadyLicense> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterable<IPlayReadyLicense> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
impl IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyLicenseIterator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterator, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterator<IPlayReadyLicense>);
impl PlayReadyLicenseIterator {
    pub fn Current(&self) -> windows_core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, items: &mut [Option<IPlayReadyLicense>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterator<IPlayReadyLicense>>();
}
unsafe impl windows_core::Interface for PlayReadyLicenseIterator {
    type Vtable = <windows_collections::IIterator<IPlayReadyLicense> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterator<IPlayReadyLicense> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
pub struct PlayReadyLicenseManagement;
impl PlayReadyLicenseManagement {
    pub fn DeleteLicenses<P0>(contentheader: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteLicenses)(windows_core::Interface::as_raw(this), contentheader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyLicenseSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseSession, windows_core::IUnknown, windows_core::IInspectable, IPlayReadyLicenseSession);
windows_core::imp::required_hierarchy!(PlayReadyLicenseSession, IPlayReadyLicenseSession2);
impl PlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLAServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(windows_core::Interface::as_raw(this), mpm.param().abi()).ok() }
    }
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLicenseIterable)(windows_core::Interface::as_raw(this), contentheader.param().abi(), fullyevaluated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0>(configuration: P0) -> windows_core::Result<PlayReadyLicenseSession>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), configuration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyLicenseSession>();
}
unsafe impl windows_core::Interface for PlayReadyLicenseSession {
    type Vtable = <IPlayReadyLicenseSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyLicenseSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyMeteringReportServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyMeteringReportServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyMeteringReportServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyMeteringReportServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MeteringCertificate(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).MeteringCertificate)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMeteringCertificate)(windows_core::Interface::as_raw(this), meteringcertbytes.len().try_into().unwrap(), meteringcertbytes.as_ptr()).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyMeteringReportServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyMeteringReportServiceRequest {
    type Vtable = <IPlayReadyMeteringReportServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyMeteringReportServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadyRevocationServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyRevocationServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyRevocationServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyRevocationServiceRequest, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyRevocationServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadyRevocationServiceRequest {
    type Vtable = <IPlayReadyRevocationServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadyRevocationServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadySecureStopIterable(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterable, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterable<IPlayReadySecureStopServiceRequest>);
impl PlayReadySecureStopIterable {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(publishercertbytes: &[u8]) -> windows_core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterable<IPlayReadySecureStopServiceRequest>>();
}
unsafe impl windows_core::Interface for PlayReadySecureStopIterable {
    type Vtable = <windows_collections::IIterable<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterable<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
impl IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadySecureStopIterator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterator, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IIterator<IPlayReadySecureStopServiceRequest>);
impl PlayReadySecureStopIterator {
    pub fn Current(&self) -> windows_core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, items: &mut [Option<IPlayReadySecureStopServiceRequest>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IIterator<IPlayReadySecureStopServiceRequest>>();
}
unsafe impl windows_core::Interface for PlayReadySecureStopIterator {
    type Vtable = <windows_collections::IIterator<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IIterator<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadySecureStopServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadySecureStopServiceRequest, windows_core::IUnknown, windows_core::IInspectable, IPlayReadySecureStopServiceRequest);
windows_core::imp::required_hierarchy!(PlayReadySecureStopServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyServiceRequest);
impl PlayReadySecureStopServiceRequest {
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SessionID(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionID)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stopped(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PublisherCertificate(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).PublisherCertificate)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CreateInstance(publishercertbytes: &[u8]) -> windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceFromSessionID(sessionid: windows_core::GUID, publishercertbytes: &[u8]) -> windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromSessionID)(windows_core::Interface::as_raw(this), sessionid, publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BeginServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextServiceRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).map(|| result__)
        }
    }
    fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadySecureStopServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = <IPlayReadySecureStopServiceRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadySecureStopServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayReadySoapMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadySoapMessage, windows_core::IUnknown, windows_core::IInspectable);
impl PlayReadySoapMessage {
    pub fn GetMessageBody(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetMessageBody)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MessageHeaders(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageHeaders)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadySoapMessage>();
}
unsafe impl windows_core::Interface for PlayReadySoapMessage {
    type Vtable = <IPlayReadySoapMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlayReadySoapMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
pub struct PlayReadyStatics;
impl PlayReadyStatics {
    pub fn DomainJoinServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainJoinServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DomainLeaveServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DomainLeaveServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IndividualizationServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndividualizationServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn LicenseAcquirerServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseAcquirerServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MeteringReportServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeteringReportServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RevocationServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RevocationServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediaProtectionSystemId() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaProtectionSystemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PlayReadySecurityVersion() -> windows_core::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayReadySecurityVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PlayReadyCertificateSecurityLevel() -> windows_core::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlayReadyCertificateSecurityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SecureStopServiceRequestType() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecureStopServiceRequestType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> windows_core::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckSupportedHardware)(windows_core::Interface::as_raw(this), hwdrmfeature, &mut result__).map(|| result__)
        })
    }
    pub fn InputTrustAuthorityToCreate() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputTrustAuthorityToCreate)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn ProtectionSystemId() -> windows_core::Result<windows_core::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystemId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn HardwareDRMDisabledAtTime() -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareDRMDisabledAtTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HardwareDRMDisabledUntilTime() -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareDRMDisabledUntilTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ResetHardwareDRMDisabled() -> windows_core::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (windows_core::Interface::vtable(this).ResetHardwareDRMDisabled)(windows_core::Interface::as_raw(this)).ok() })
    }
    fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
