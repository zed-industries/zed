#[cfg(feature = "deprecated")]
pub trait INDClosedCaptionDataReceivedEventArgs_Impl: Sized {
    fn ClosedCaptionDataFormat(&self) -> windows_core::Result<NDClosedCaptionFormat>;
    fn PresentationTimestamp(&self) -> windows_core::Result<i64>;
    fn ClosedCaptionData(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDClosedCaptionDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>() -> INDClosedCaptionDataReceivedEventArgs_Vtbl {
        unsafe extern "system" fn ClosedCaptionDataFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDClosedCaptionDataReceivedEventArgs_Impl::ClosedCaptionDataFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationTimestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDClosedCaptionDataReceivedEventArgs_Impl::PresentationTimestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosedCaptionData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDClosedCaptionDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDClosedCaptionDataReceivedEventArgs_Impl::ClosedCaptionData(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDClosedCaptionDataReceivedEventArgs, OFFSET>(),
            ClosedCaptionDataFormat: ClosedCaptionDataFormat::<Identity, Impl, OFFSET>,
            PresentationTimestamp: PresentationTimestamp::<Identity, Impl, OFFSET>,
            ClosedCaptionData: ClosedCaptionData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDClosedCaptionDataReceivedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDCustomData_Impl: Sized {
    fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDCustomData";
}
#[cfg(feature = "deprecated")]
impl INDCustomData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: isize>() -> INDCustomData_Vtbl {
        unsafe extern "system" fn CustomDataTypeID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDCustomData_Impl::CustomDataTypeID(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDCustomData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDCustomData_Impl::CustomData(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDCustomData, OFFSET>(),
            CustomDataTypeID: CustomDataTypeID::<Identity, Impl, OFFSET>,
            CustomData: CustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDCustomData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngine_Impl: Sized {
    fn Open(&self, uri: Option<&super::super::super::Foundation::Uri>, sessionidbytes: &[u8]) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Seek(&self, startposition: &super::super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn CanSeek(&self) -> windows_core::Result<bool>;
    fn BufferFullMinThresholdInSamples(&self) -> windows_core::Result<u32>;
    fn BufferFullMaxThresholdInSamples(&self) -> windows_core::Result<u32>;
    fn Notifier(&self) -> windows_core::Result<NDDownloadEngineNotifier>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDDownloadEngine {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngine";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>() -> INDDownloadEngine_Vtbl {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngine_Impl::Open(this, windows_core::from_raw_borrowed(&uri), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngine_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngine_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngine_Impl::Close(this).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngine_Impl::Seek(this, core::mem::transmute(&startposition)).into()
        }
        unsafe extern "system" fn CanSeek<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDDownloadEngine_Impl::CanSeek(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMinThresholdInSamples<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDDownloadEngine_Impl::BufferFullMinThresholdInSamples(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferFullMaxThresholdInSamples<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDDownloadEngine_Impl::BufferFullMaxThresholdInSamples(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDDownloadEngine_Impl::Notifier(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDDownloadEngine, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CanSeek: CanSeek::<Identity, Impl, OFFSET>,
            BufferFullMinThresholdInSamples: BufferFullMinThresholdInSamples::<Identity, Impl, OFFSET>,
            BufferFullMaxThresholdInSamples: BufferFullMaxThresholdInSamples::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDDownloadEngine as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDDownloadEngineNotifier_Impl: Sized {
    fn OnStreamOpened(&self) -> windows_core::Result<()>;
    fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> windows_core::Result<()>;
    fn OnContentIDReceived(&self, licensefetchdescriptor: Option<&INDLicenseFetchDescriptor>) -> windows_core::Result<()>;
    fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> windows_core::Result<()>;
    fn OnEndOfStream(&self) -> windows_core::Result<()>;
    fn OnNetworkError(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>() -> INDDownloadEngineNotifier_Vtbl {
        unsafe extern "system" fn OnStreamOpened<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnStreamOpened(this).into()
        }
        unsafe extern "system" fn OnPlayReadyObjectReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnPlayReadyObjectReceived(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), dataBytes_array_size as usize)).into()
        }
        unsafe extern "system" fn OnContentIDReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchdescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnContentIDReceived(this, windows_core::from_raw_borrowed(&licensefetchdescriptor)).into()
        }
        unsafe extern "system" fn OnDataReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnDataReceived(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), dataBytes_array_size as usize), bytesreceived).into()
        }
        unsafe extern "system" fn OnEndOfStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnEndOfStream(this).into()
        }
        unsafe extern "system" fn OnNetworkError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDDownloadEngineNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDDownloadEngineNotifier_Impl::OnNetworkError(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDDownloadEngineNotifier, OFFSET>(),
            OnStreamOpened: OnStreamOpened::<Identity, Impl, OFFSET>,
            OnPlayReadyObjectReceived: OnPlayReadyObjectReceived::<Identity, Impl, OFFSET>,
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnDataReceived: OnDataReceived::<Identity, Impl, OFFSET>,
            OnEndOfStream: OnEndOfStream::<Identity, Impl, OFFSET>,
            OnNetworkError: OnNetworkError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDDownloadEngineNotifier as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDLicenseFetchCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>() -> INDLicenseFetchCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDLicenseFetchCompletedEventArgs_Impl::ResponseCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchDescriptor_Impl: Sized {
    fn ContentIDType(&self) -> windows_core::Result<NDContentIDType>;
    fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>>;
    fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData>;
    fn SetLicenseFetchChallengeCustomData(&self, licensefetchchallengecustomdata: Option<&INDCustomData>) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>() -> INDLicenseFetchDescriptor_Vtbl {
        unsafe extern "system" fn ContentIDType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDContentIDType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDLicenseFetchDescriptor_Impl::ContentIDType(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDLicenseFetchDescriptor_Impl::ContentID(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseFetchChallengeCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDLicenseFetchDescriptor_Impl::LicenseFetchChallengeCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLicenseFetchChallengeCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchchallengecustomdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDLicenseFetchDescriptor_Impl::SetLicenseFetchChallengeCustomData(this, windows_core::from_raw_borrowed(&licensefetchchallengecustomdata)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchDescriptor, OFFSET>(),
            ContentIDType: ContentIDType::<Identity, Impl, OFFSET>,
            ContentID: ContentID::<Identity, Impl, OFFSET>,
            LicenseFetchChallengeCustomData: LicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
            SetLicenseFetchChallengeCustomData: SetLicenseFetchChallengeCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchDescriptor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDLicenseFetchResult_Impl: Sized {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDLicenseFetchResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
}
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchResult_Impl, const OFFSET: isize>() -> INDLicenseFetchResult_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDLicenseFetchResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDLicenseFetchResult_Impl::ResponseCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDLicenseFetchResult, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDLicenseFetchResult as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDMessenger_Impl: Sized {
    fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
    fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDMessenger";
}
#[cfg(feature = "deprecated")]
impl INDMessenger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: isize>() -> INDMessenger_Vtbl {
        unsafe extern "system" fn SendRegistrationRequestAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDMessenger_Impl::SendRegistrationRequestAsync(this, core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionStartAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDMessenger_Impl::SendProximityDetectionStartAsync(this, pdtype, core::slice::from_raw_parts(core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendProximityDetectionResponseAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDMessenger_Impl::SendProximityDetectionResponseAsync(this, pdtype, core::slice::from_raw_parts(core::mem::transmute_copy(&transmitterchannelbytes), transmitterChannelBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&responsedatabytes), responseDataBytes_array_size as usize)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendLicenseFetchRequestAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDMessenger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDMessenger_Impl::SendLicenseFetchRequestAsync(this, core::slice::from_raw_parts(core::mem::transmute_copy(&sessionidbytes), sessionIDBytes_array_size as usize), core::slice::from_raw_parts(core::mem::transmute_copy(&challengedatabytes), challengeDataBytes_array_size as usize)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDMessenger, OFFSET>(),
            SendRegistrationRequestAsync: SendRegistrationRequestAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionStartAsync: SendProximityDetectionStartAsync::<Identity, Impl, OFFSET>,
            SendProximityDetectionResponseAsync: SendProximityDetectionResponseAsync::<Identity, Impl, OFFSET>,
            SendLicenseFetchRequestAsync: SendLicenseFetchRequestAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDMessenger as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDProximityDetectionCompletedEventArgs_Impl: Sized {
    fn ProximityDetectionRetryCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDProximityDetectionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>() -> INDProximityDetectionCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ProximityDetectionRetryCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDProximityDetectionCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDProximityDetectionCompletedEventArgs_Impl::ProximityDetectionRetryCount(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDProximityDetectionCompletedEventArgs, OFFSET>(),
            ProximityDetectionRetryCount: ProximityDetectionRetryCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDProximityDetectionCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDRegistrationCompletedEventArgs_Impl: Sized {
    fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData>;
    fn TransmitterProperties(&self) -> windows_core::Result<INDTransmitterProperties>;
    fn TransmitterCertificateAccepted(&self) -> windows_core::Result<bool>;
    fn SetTransmitterCertificateAccepted(&self, accept: bool) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDRegistrationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
}
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>() -> INDRegistrationCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResponseCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDRegistrationCompletedEventArgs_Impl::ResponseCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDRegistrationCompletedEventArgs_Impl::TransmitterProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitterCertificateAccepted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDRegistrationCompletedEventArgs_Impl::TransmitterCertificateAccepted(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmitterCertificateAccepted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDRegistrationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accept: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDRegistrationCompletedEventArgs_Impl::SetTransmitterCertificateAccepted(this, accept).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDRegistrationCompletedEventArgs, OFFSET>(),
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            TransmitterProperties: TransmitterProperties::<Identity, Impl, OFFSET>,
            TransmitterCertificateAccepted: TransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
            SetTransmitterCertificateAccepted: SetTransmitterCertificateAccepted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDRegistrationCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDSendResult_Impl: Sized {
    fn Response(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDSendResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDSendResult";
}
#[cfg(feature = "deprecated")]
impl INDSendResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDSendResult_Impl, const OFFSET: isize>() -> INDSendResult_Vtbl {
        unsafe extern "system" fn Response<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDSendResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDSendResult_Impl::Response(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, INDSendResult, OFFSET>(), Response: Response::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDSendResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStartResult_Impl: Sized {
    fn MediaStreamSource(&self) -> windows_core::Result<super::super::Core::MediaStreamSource>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl windows_core::RuntimeName for INDStartResult {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStartResult";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStartResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStartResult_Impl, const OFFSET: isize>() -> INDStartResult_Vtbl {
        unsafe extern "system" fn MediaStreamSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStartResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDStartResult_Impl::MediaStreamSource(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStartResult, OFFSET>(),
            MediaStreamSource: MediaStreamSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStartResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
pub trait INDStorageFileHelper_Impl: Sized {
    fn GetFileURLs(&self, file: Option<&super::super::super::Storage::IStorageFile>) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl windows_core::RuntimeName for INDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStorageFileHelper";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
impl INDStorageFileHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStorageFileHelper_Impl, const OFFSET: isize>() -> INDStorageFileHelper_Vtbl {
        unsafe extern "system" fn GetFileURLs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStorageFileHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDStorageFileHelper_Impl::GetFileURLs(this, windows_core::from_raw_borrowed(&file)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStorageFileHelper, OFFSET>(), GetFileURLs: GetFileURLs::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStorageFileHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParser_Impl: Sized {
    fn ParseData(&self, databytes: &[u8]) -> windows_core::Result<()>;
    fn GetStreamInformation(&self, descriptor: Option<&super::super::Core::IMediaStreamDescriptor>, streamtype: &mut NDMediaStreamType) -> windows_core::Result<u32>;
    fn BeginOfStream(&self) -> windows_core::Result<()>;
    fn EndOfStream(&self) -> windows_core::Result<()>;
    fn Notifier(&self) -> windows_core::Result<NDStreamParserNotifier>;
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl windows_core::RuntimeName for INDStreamParser {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParser";
}
#[cfg(all(feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>() -> INDStreamParser_Vtbl {
        unsafe extern "system" fn ParseData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParser_Impl::ParseData(this, core::slice::from_raw_parts(core::mem::transmute_copy(&databytes), dataBytes_array_size as usize)).into()
        }
        unsafe extern "system" fn GetStreamInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDStreamParser_Impl::GetStreamInformation(this, windows_core::from_raw_borrowed(&descriptor), core::mem::transmute_copy(&streamtype)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginOfStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParser_Impl::BeginOfStream(this).into()
        }
        unsafe extern "system" fn EndOfStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParser_Impl::EndOfStream(this).into()
        }
        unsafe extern "system" fn Notifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDStreamParser_Impl::Notifier(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStreamParser, OFFSET>(),
            ParseData: ParseData::<Identity, Impl, OFFSET>,
            GetStreamInformation: GetStreamInformation::<Identity, Impl, OFFSET>,
            BeginOfStream: BeginOfStream::<Identity, Impl, OFFSET>,
            EndOfStream: EndOfStream::<Identity, Impl, OFFSET>,
            Notifier: Notifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStreamParser as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
pub trait INDStreamParserNotifier_Impl: Sized {
    fn OnContentIDReceived(&self, licensefetchdescriptor: Option<&INDLicenseFetchDescriptor>) -> windows_core::Result<()>;
    fn OnMediaStreamDescriptorCreated(&self, audiostreamdescriptors: Option<&super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, videostreamdescriptors: Option<&super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>) -> windows_core::Result<()>;
    fn OnSampleParsed(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Option<&super::super::Core::MediaStreamSample>, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> windows_core::Result<()>;
    fn OnBeginSetupDecryptor(&self, descriptor: Option<&super::super::Core::IMediaStreamDescriptor>, keyid: &windows_core::GUID, probytes: &[u8]) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl windows_core::RuntimeName for INDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
impl INDStreamParserNotifier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>() -> INDStreamParserNotifier_Vtbl {
        unsafe extern "system" fn OnContentIDReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, licensefetchdescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParserNotifier_Impl::OnContentIDReceived(this, windows_core::from_raw_borrowed(&licensefetchdescriptor)).into()
        }
        unsafe extern "system" fn OnMediaStreamDescriptorCreated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiostreamdescriptors: *mut core::ffi::c_void, videostreamdescriptors: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParserNotifier_Impl::OnMediaStreamDescriptorCreated(this, windows_core::from_raw_borrowed(&audiostreamdescriptors), windows_core::from_raw_borrowed(&videostreamdescriptors)).into()
        }
        unsafe extern "system" fn OnSampleParsed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParserNotifier_Impl::OnSampleParsed(this, streamid, streamtype, windows_core::from_raw_borrowed(&streamsample), pts, ccformat, core::slice::from_raw_parts(core::mem::transmute_copy(&ccdatabytes), ccDataBytes_array_size as usize)).into()
        }
        unsafe extern "system" fn OnBeginSetupDecryptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDStreamParserNotifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void, keyid: windows_core::GUID, proBytes_array_size: u32, probytes: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INDStreamParserNotifier_Impl::OnBeginSetupDecryptor(this, windows_core::from_raw_borrowed(&descriptor), core::mem::transmute(&keyid), core::slice::from_raw_parts(core::mem::transmute_copy(&probytes), proBytes_array_size as usize)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDStreamParserNotifier, OFFSET>(),
            OnContentIDReceived: OnContentIDReceived::<Identity, Impl, OFFSET>,
            OnMediaStreamDescriptorCreated: OnMediaStreamDescriptorCreated::<Identity, Impl, OFFSET>,
            OnSampleParsed: OnSampleParsed::<Identity, Impl, OFFSET>,
            OnBeginSetupDecryptor: OnBeginSetupDecryptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDStreamParserNotifier as windows_core::Interface>::IID
    }
}
#[cfg(feature = "deprecated")]
pub trait INDTransmitterProperties_Impl: Sized {
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
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for INDTransmitterProperties {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.INDTransmitterProperties";
}
#[cfg(feature = "deprecated")]
impl INDTransmitterProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>() -> INDTransmitterProperties_Vtbl {
        unsafe extern "system" fn CertificateType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDCertificateType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::CertificateType(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::PlatformIdentifier(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::SupportedFeatures(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::SecurityLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::SecurityVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ExpirationDate(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ClientID(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelDigest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ModelDigest(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelManufacturerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ModelManufacturerName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ModelName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INDTransmitterProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INDTransmitterProperties_Impl::ModelNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INDTransmitterProperties, OFFSET>(),
            CertificateType: CertificateType::<Identity, Impl, OFFSET>,
            PlatformIdentifier: PlatformIdentifier::<Identity, Impl, OFFSET>,
            SupportedFeatures: SupportedFeatures::<Identity, Impl, OFFSET>,
            SecurityLevel: SecurityLevel::<Identity, Impl, OFFSET>,
            SecurityVersion: SecurityVersion::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ClientID: ClientID::<Identity, Impl, OFFSET>,
            ModelDigest: ModelDigest::<Identity, Impl, OFFSET>,
            ModelManufacturerName: ModelManufacturerName::<Identity, Impl, OFFSET>,
            ModelName: ModelName::<Identity, Impl, OFFSET>,
            ModelNumber: ModelNumber::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDTransmitterProperties as windows_core::Interface>::IID
    }
}
pub trait IPlayReadyDomain_Impl: Sized {
    fn AccountId(&self) -> windows_core::Result<windows_core::GUID>;
    fn ServiceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Revision(&self) -> windows_core::Result<u32>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn DomainJoinUrl(&self) -> windows_core::Result<super::super::super::Foundation::Uri>;
}
impl windows_core::RuntimeName for IPlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyDomain";
}
impl IPlayReadyDomain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>() -> IPlayReadyDomain_Vtbl {
        unsafe extern "system" fn AccountId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyDomain_Impl::AccountId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyDomain_Impl::ServiceId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyDomain_Impl::Revision(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyDomain_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainJoinUrl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyDomain_Impl::DomainJoinUrl(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyDomain, OFFSET>(),
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            ServiceId: ServiceId::<Identity, Impl, OFFSET>,
            Revision: Revision::<Identity, Impl, OFFSET>,
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            DomainJoinUrl: DomainJoinUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyDomain as windows_core::Interface>::IID
    }
}
pub trait IPlayReadyLicense_Impl: Sized {
    fn FullyEvaluated(&self) -> windows_core::Result<bool>;
    fn UsableForPlay(&self) -> windows_core::Result<bool>;
    fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
    fn ExpireAfterFirstPlay(&self) -> windows_core::Result<u32>;
    fn DomainAccountID(&self) -> windows_core::Result<windows_core::GUID>;
    fn ChainDepth(&self) -> windows_core::Result<u32>;
    fn GetKIDAtChainDepth(&self, chaindepth: u32) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IPlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicense";
}
impl IPlayReadyLicense_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>() -> IPlayReadyLicense_Vtbl {
        unsafe extern "system" fn FullyEvaluated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::FullyEvaluated(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableForPlay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::UsableForPlay(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::ExpirationDate(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpireAfterFirstPlay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::ExpireAfterFirstPlay(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainAccountID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::DomainAccountID(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChainDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::ChainDepth(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKIDAtChainDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicense_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, chaindepth: u32, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicense_Impl::GetKIDAtChainDepth(this, chaindepth) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicense, OFFSET>(),
            FullyEvaluated: FullyEvaluated::<Identity, Impl, OFFSET>,
            UsableForPlay: UsableForPlay::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            ExpireAfterFirstPlay: ExpireAfterFirstPlay::<Identity, Impl, OFFSET>,
            DomainAccountID: DomainAccountID::<Identity, Impl, OFFSET>,
            ChainDepth: ChainDepth::<Identity, Impl, OFFSET>,
            GetKIDAtChainDepth: GetKIDAtChainDepth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicense as windows_core::Interface>::IID
    }
}
pub trait IPlayReadyLicenseAcquisitionServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn ContentHeader(&self) -> windows_core::Result<PlayReadyContentHeader>;
    fn SetContentHeader(&self, value: Option<&PlayReadyContentHeader>) -> windows_core::Result<()>;
    fn DomainServiceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetDomainServiceId(&self, value: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
}
impl IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
        unsafe extern "system" fn ContentHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicenseAcquisitionServiceRequest_Impl::ContentHeader(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentHeader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlayReadyLicenseAcquisitionServiceRequest_Impl::SetContentHeader(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn DomainServiceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicenseAcquisitionServiceRequest_Impl::DomainServiceId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainServiceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseAcquisitionServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlayReadyLicenseAcquisitionServiceRequest_Impl::SetDomainServiceId(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseAcquisitionServiceRequest, OFFSET>(),
            ContentHeader: ContentHeader::<Identity, Impl, OFFSET>,
            SetContentHeader: SetContentHeader::<Identity, Impl, OFFSET>,
            DomainServiceId: DomainServiceId::<Identity, Impl, OFFSET>,
            SetDomainServiceId: SetDomainServiceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseAcquisitionServiceRequest as windows_core::Interface>::IID
    }
}
pub trait IPlayReadyLicenseSession_Impl: Sized {
    fn CreateLAServiceRequest(&self) -> windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest>;
    fn ConfigureMediaProtectionManager(&self, mpm: Option<&super::MediaProtectionManager>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
}
impl IPlayReadyLicenseSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>() -> IPlayReadyLicenseSession_Vtbl {
        unsafe extern "system" fn CreateLAServiceRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicenseSession_Impl::CreateLAServiceRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureMediaProtectionManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mpm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlayReadyLicenseSession_Impl::ConfigureMediaProtectionManager(this, windows_core::from_raw_borrowed(&mpm)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseSession, OFFSET>(),
            CreateLAServiceRequest: CreateLAServiceRequest::<Identity, Impl, OFFSET>,
            ConfigureMediaProtectionManager: ConfigureMediaProtectionManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPlayReadyLicenseSession2_Impl: Sized + IPlayReadyLicenseSession_Impl {
    fn CreateLicenseIterable(&self, contentheader: Option<&PlayReadyContentHeader>, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPlayReadyLicenseSession2 {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
}
#[cfg(feature = "Foundation_Collections")]
impl IPlayReadyLicenseSession2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>() -> IPlayReadyLicenseSession2_Vtbl {
        unsafe extern "system" fn CreateLicenseIterable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyLicenseSession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentheader: *mut core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyLicenseSession2_Impl::CreateLicenseIterable(this, windows_core::from_raw_borrowed(&contentheader), fullyevaluated) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyLicenseSession2, OFFSET>(),
            CreateLicenseIterable: CreateLicenseIterable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyLicenseSession2 as windows_core::Interface>::IID
    }
}
pub trait IPlayReadySecureStopServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl + IPlayReadyServiceRequest_Impl {
    fn SessionID(&self) -> windows_core::Result<windows_core::GUID>;
    fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn UpdateTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime>;
    fn Stopped(&self) -> windows_core::Result<bool>;
    fn PublisherCertificate(&self) -> windows_core::Result<windows_core::Array<u8>>;
}
impl windows_core::RuntimeName for IPlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
}
impl IPlayReadySecureStopServiceRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadySecureStopServiceRequest_Vtbl {
        unsafe extern "system" fn SessionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadySecureStopServiceRequest_Impl::SessionID(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadySecureStopServiceRequest_Impl::StartTime(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadySecureStopServiceRequest_Impl::UpdateTime(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stopped<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadySecureStopServiceRequest_Impl::Stopped(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadySecureStopServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadySecureStopServiceRequest_Impl::PublisherCertificate(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadySecureStopServiceRequest, OFFSET>(),
            SessionID: SessionID::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            UpdateTime: UpdateTime::<Identity, Impl, OFFSET>,
            Stopped: Stopped::<Identity, Impl, OFFSET>,
            PublisherCertificate: PublisherCertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadySecureStopServiceRequest as windows_core::Interface>::IID
    }
}
pub trait IPlayReadyServiceRequest_Impl: Sized + super::IMediaProtectionServiceRequest_Impl {
    fn Uri(&self) -> windows_core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: Option<&super::super::super::Foundation::Uri>) -> windows_core::Result<()>;
    fn ResponseCustomData(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>;
    fn NextServiceRequest(&self) -> windows_core::Result<IPlayReadyServiceRequest>;
    fn GenerateManualEnablingChallenge(&self) -> windows_core::Result<PlayReadySoapMessage>;
    fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> windows_core::Result<windows_core::HRESULT>;
}
impl windows_core::RuntimeName for IPlayReadyServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
}
impl IPlayReadyServiceRequest_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>() -> IPlayReadyServiceRequest_Vtbl {
        unsafe extern "system" fn Uri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::Uri(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlayReadyServiceRequest_Impl::SetUri(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn ResponseCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::ResponseCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChallengeCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::ChallengeCustomData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChallengeCustomData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlayReadyServiceRequest_Impl::SetChallengeCustomData(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginServiceRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::BeginServiceRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextServiceRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::NextServiceRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateManualEnablingChallenge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::GenerateManualEnablingChallenge(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessManualEnablingResponse<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlayReadyServiceRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPlayReadyServiceRequest_Impl::ProcessManualEnablingResponse(this, core::slice::from_raw_parts(core::mem::transmute_copy(&responsebytes), responseBytes_array_size as usize)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayReadyServiceRequest, OFFSET>(),
            Uri: Uri::<Identity, Impl, OFFSET>,
            SetUri: SetUri::<Identity, Impl, OFFSET>,
            ResponseCustomData: ResponseCustomData::<Identity, Impl, OFFSET>,
            ChallengeCustomData: ChallengeCustomData::<Identity, Impl, OFFSET>,
            SetChallengeCustomData: SetChallengeCustomData::<Identity, Impl, OFFSET>,
            BeginServiceRequest: BeginServiceRequest::<Identity, Impl, OFFSET>,
            NextServiceRequest: NextServiceRequest::<Identity, Impl, OFFSET>,
            GenerateManualEnablingChallenge: GenerateManualEnablingChallenge::<Identity, Impl, OFFSET>,
            ProcessManualEnablingResponse: ProcessManualEnablingResponse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayReadyServiceRequest as windows_core::Interface>::IID
    }
}
