#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDClient, INDClient_Vtbl, 0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClient_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RegistrationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RegistrationCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveRegistrationCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub ProximityDetectionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ProximityDetectionCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveProximityDetectionCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveLicenseFetchCompleted: usize,
    #[cfg(feature = "deprecated")]
    pub ReRegistrationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReRegistrationNeeded: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveReRegistrationNeeded: usize,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveClosedCaptionDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StartAsync: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ReRegistrationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ReRegistrationAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDClientFactory, INDClientFactory_Vtbl, 0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDClientFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClientFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDClosedCaptionDataReceivedEventArgs, INDClosedCaptionDataReceivedEventArgs_Vtbl, 0x4738d29f_c345_4649_8468_b8c5fc357190);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDClosedCaptionDataReceivedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDClosedCaptionDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionDataFormat(&self) -> windows_core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionDataFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PresentationTimestamp(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresentationTimestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDClosedCaptionFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionDataFormat: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionData: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDCustomData, INDCustomData_Vtbl, 0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDCustomData {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDCustomData, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDCustomData {
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomDataTypeID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDCustomData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDCustomData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CustomDataTypeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomDataTypeID: usize,
    #[cfg(feature = "deprecated")]
    pub CustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomData: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDCustomDataFactory, INDCustomDataFactory_Vtbl, 0xd65405ab_3424_4833_8c9a_af5fdeb22872);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDCustomDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDCustomDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDDownloadEngine, INDDownloadEngine_Vtbl, 0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDDownloadEngine {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDDownloadEngine, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDDownloadEngine {
    #[cfg(feature = "deprecated")]
    pub fn Open<P0>(&self, uri: P0, sessionidbytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Open)(windows_core::Interface::as_raw(this), uri.param().abi(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Resume(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Resume)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Seek(&self, startposition: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), startposition).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSeek)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMinThresholdInSamples(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferFullMinThresholdInSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMaxThresholdInSamples(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferFullMaxThresholdInSamples)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> windows_core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDDownloadEngine_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Open: usize,
    #[cfg(feature = "deprecated")]
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Pause: usize,
    #[cfg(feature = "deprecated")]
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Resume: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
    #[cfg(feature = "deprecated")]
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Seek: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMinThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMaxThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDDownloadEngineNotifier, INDDownloadEngineNotifier_Vtbl, 0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDDownloadEngineNotifier {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDDownloadEngineNotifier, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier {
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnStreamOpened)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnDataReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnEndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnNetworkError)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDDownloadEngineNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnStreamOpened: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnStreamOpened: usize,
    #[cfg(feature = "deprecated")]
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnPlayReadyObjectReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnEndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub OnNetworkError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnNetworkError: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDLicenseFetchCompletedEventArgs, INDLicenseFetchCompletedEventArgs_Vtbl, 0x1ee30a1a_11b2_4558_8865_e3a516922517);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDLicenseFetchCompletedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDLicenseFetchCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDLicenseFetchDescriptor, INDLicenseFetchDescriptor_Vtbl, 0x5498d33a_e686_4935_a567_7ca77ad20fa4);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDLicenseFetchDescriptor {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDLicenseFetchDescriptor, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentIDType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ContentID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ContentIDType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDContentIDType) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentIDType: usize,
    #[cfg(feature = "deprecated")]
    pub ContentID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentID: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchChallengeCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLicenseFetchChallengeCustomData: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDLicenseFetchDescriptorFactory, INDLicenseFetchDescriptorFactory_Vtbl, 0xd0031202_cfac_4f00_ae6a_97af80b848f2);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDLicenseFetchDescriptorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchDescriptorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, NDContentIDType, u32, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDLicenseFetchResult, INDLicenseFetchResult_Vtbl, 0x21d39698_aa62_45ff_a5ff_8037e5433825);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDLicenseFetchResult {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDLicenseFetchResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDLicenseFetchResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDMessenger, INDMessenger_Vtbl, 0xd42df95d_a75b_47bf_8249_bc83820da38a);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDMessenger {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDMessenger, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDMessenger {
    #[cfg(feature = "deprecated")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDMessenger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDMessenger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SendRegistrationRequestAsync: usize,
    #[cfg(feature = "deprecated")]
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, NDProximityDetectionType, u32, *const u8, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SendProximityDetectionStartAsync: usize,
    #[cfg(feature = "deprecated")]
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, NDProximityDetectionType, u32, *const u8, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SendProximityDetectionResponseAsync: usize,
    #[cfg(feature = "deprecated")]
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SendLicenseFetchRequestAsync: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDProximityDetectionCompletedEventArgs, INDProximityDetectionCompletedEventArgs_Vtbl, 0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDProximityDetectionCompletedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDProximityDetectionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ProximityDetectionRetryCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityDetectionRetryCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDProximityDetectionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ProximityDetectionRetryCount: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDRegistrationCompletedEventArgs, INDRegistrationCompletedEventArgs_Vtbl, 0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDRegistrationCompletedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDRegistrationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TransmitterProperties(&self) -> windows_core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitterProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TransmitterCertificateAccepted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransmitterCertificateAccepted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransmitterCertificateAccepted)(windows_core::Interface::as_raw(this), accept).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDRegistrationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterProperties: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterCertificateAccepted: usize,
    #[cfg(feature = "deprecated")]
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTransmitterCertificateAccepted: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDSendResult, INDSendResult_Vtbl, 0xe3685517_a584_479d_90b7_d689c7bf7c80);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDSendResult {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDSendResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDSendResult {
    #[cfg(feature = "deprecated")]
    pub fn Response(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Response)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDSendResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDSendResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Response: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Response: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDStartResult, INDStartResult_Vtbl, 0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDStartResult {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDStartResult, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDStartResult {
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn MediaStreamSource(&self) -> windows_core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaStreamSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDStartResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStartResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub MediaStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    MediaStreamSource: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDStorageFileHelper, INDStorageFileHelper_Vtbl, 0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDStorageFileHelper {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDStorageFileHelper, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDStorageFileHelper {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0>(&self, file: P0) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>>
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
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStorageFileHelper_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub GetFileURLs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    GetFileURLs: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDStreamParser, INDStreamParser_Vtbl, 0xe0baa198_9796_41c9_8695_59437e67e66a);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDStreamParser {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDStreamParser, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDStreamParser {
    #[cfg(feature = "deprecated")]
    pub fn ParseData(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ParseData)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
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
    #[cfg(feature = "deprecated")]
    pub fn BeginOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BeginOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn EndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> windows_core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDStreamParser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStreamParser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ParseData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ParseData: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub GetStreamInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut NDMediaStreamType, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    GetStreamInformation: usize,
    #[cfg(feature = "deprecated")]
    pub BeginOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BeginOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub EndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDStreamParserNotifier, INDStreamParserNotifier_Vtbl, 0xc167acd0_2ce6_426c_ace5_5e9275fea715);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDStreamParserNotifier {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDStreamParserNotifier, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDStreamParserNotifier {
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(windows_core::Interface::as_raw(this), audiostreamdescriptors.param().abi(), videostreamdescriptors.param().abi()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed<P0>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P0, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnSampleParsed)(windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: windows_core::GUID, probytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(windows_core::Interface::as_raw(this), descriptor.param().abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDStreamParserNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated")))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnSampleParsed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, NDMediaStreamType, *mut core::ffi::c_void, i64, NDClosedCaptionFormat, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnSampleParsed: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnBeginSetupDecryptor: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDTCPMessengerFactory, INDTCPMessengerFactory_Vtbl, 0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDTCPMessengerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDTCPMessengerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(INDTransmitterProperties, INDTransmitterProperties_Vtbl, 0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
#[cfg(feature = "deprecated")]
impl core::ops::Deref for INDTransmitterProperties {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(INDTransmitterProperties, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl INDTransmitterProperties {
    #[cfg(feature = "deprecated")]
    pub fn CertificateType(&self) -> windows_core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CertificateType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlatformIdentifier(&self) -> windows_core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlatformIdentifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SupportedFeatures(&self) -> windows_core::Result<windows_core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SupportedFeatures)(windows_core::Interface::as_raw(this), windows_core::Array::<NDCertificateFeature>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SecurityLevel(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecurityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SecurityVersion(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecurityVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ExpirationDate(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClientID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ClientID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelDigest(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ModelDigest)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelManufacturerName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelManufacturerName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct INDTransmitterProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CertificateType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDCertificateType) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CertificateType: usize,
    #[cfg(feature = "deprecated")]
    pub PlatformIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NDCertificatePlatformID) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlatformIdentifier: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut NDCertificateFeature) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedFeatures: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityVersion: usize,
    #[cfg(feature = "deprecated")]
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExpirationDate: usize,
    #[cfg(feature = "deprecated")]
    pub ClientID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClientID: usize,
    #[cfg(feature = "deprecated")]
    pub ModelDigest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelDigest: usize,
    #[cfg(feature = "deprecated")]
    pub ModelManufacturerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelManufacturerName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
}
windows_core::imp::define_interface!(IPlayReadyContentHeader, IPlayReadyContentHeader_Vtbl, 0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
impl windows_core::RuntimeType for IPlayReadyContentHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyContentHeader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayReadyEncryptionAlgorithm) -> windows_core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlayReadyDecryptorSetup) -> windows_core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeader2, IPlayReadyContentHeader2_Vtbl, 0x359c79f4_2180_498c_965b_e754d875eab2);
impl windows_core::RuntimeType for IPlayReadyContentHeader2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyContentHeader2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeaderFactory, IPlayReadyContentHeaderFactory_Vtbl, 0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
impl windows_core::RuntimeType for IPlayReadyContentHeaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyContentHeaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromComponents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>, PlayReadyEncryptionAlgorithm, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentHeaderFactory2, IPlayReadyContentHeaderFactory2_Vtbl, 0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
impl windows_core::RuntimeType for IPlayReadyContentHeaderFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyContentHeaderFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>, PlayReadyEncryptionAlgorithm, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyContentResolver, IPlayReadyContentResolver_Vtbl, 0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
impl windows_core::RuntimeType for IPlayReadyContentResolver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyContentResolver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomain, IPlayReadyDomain_Vtbl, 0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
impl core::ops::Deref for IPlayReadyDomain {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyDomain_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DomainJoinUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomainIterableFactory, IPlayReadyDomainIterableFactory_Vtbl, 0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
impl windows_core::RuntimeType for IPlayReadyDomainIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyDomainIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPlayReadyDomainJoinServiceRequest, IPlayReadyDomainJoinServiceRequest_Vtbl, 0x171b4a5a_405f_4739_b040_67b9f0c38758);
impl windows_core::RuntimeType for IPlayReadyDomainJoinServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyDomainJoinServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyDomainLeaveServiceRequest, IPlayReadyDomainLeaveServiceRequest_Vtbl, 0x062d58be_97ad_4917_aa03_46d4c252d464);
impl windows_core::RuntimeType for IPlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadyIndividualizationServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPlayReadyLicense, IPlayReadyLicense_Vtbl, 0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
impl core::ops::Deref for IPlayReadyLicense {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadyLicense2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SecureStopId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseAcquisitionServiceRequest, IPlayReadyLicenseAcquisitionServiceRequest_Vtbl, 0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
impl core::ops::Deref for IPlayReadyLicenseAcquisitionServiceRequest {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
impl windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseAcquisitionServiceRequest3, IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl, 0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
impl windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
windows_core::imp::define_interface!(IPlayReadyLicenseIterableFactory, IPlayReadyLicenseIterableFactory_Vtbl, 0xd4179f08_0837_4978_8e68_be4293c8d7a6);
impl windows_core::RuntimeType for IPlayReadyLicenseIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyLicenseIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPlayReadyLicenseManagement, IPlayReadyLicenseManagement_Vtbl, 0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
impl windows_core::RuntimeType for IPlayReadyLicenseManagement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyLicenseManagement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeleteLicenses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSession, IPlayReadyLicenseSession_Vtbl, 0xa1723a39_87fa_4fdd_abbb_a9720e845259);
impl core::ops::Deref for IPlayReadyLicenseSession {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyLicenseSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateLAServiceRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSession2, IPlayReadyLicenseSession2_Vtbl, 0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
impl core::ops::Deref for IPlayReadyLicenseSession2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPlayReadyLicenseSession2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPlayReadyLicenseSession2, IPlayReadyLicenseSession);
impl IPlayReadyLicenseSession2 {
    #[cfg(feature = "Foundation_Collections")]
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
impl windows_core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyLicenseSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
windows_core::imp::define_interface!(IPlayReadyLicenseSessionFactory, IPlayReadyLicenseSessionFactory_Vtbl, 0x62492699_6527_429e_98be_48d798ac2739);
impl windows_core::RuntimeType for IPlayReadyLicenseSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadyRevocationServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPlayReadySecureStopIterableFactory, IPlayReadySecureStopIterableFactory_Vtbl, 0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
impl windows_core::RuntimeType for IPlayReadySecureStopIterableFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadySecureStopIterableFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequest_Vtbl, 0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
impl core::ops::Deref for IPlayReadySecureStopServiceRequest {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
impl windows_core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadySecureStopServiceRequestFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyServiceRequest, IPlayReadyServiceRequest_Vtbl, 0x8bad2836_a703_45a6_a180_76f3565aa725);
impl core::ops::Deref for IPlayReadyServiceRequest {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
impl windows_core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResponseCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
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
pub struct IPlayReadyStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics3, IPlayReadyStatics3_Vtbl, 0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
impl windows_core::RuntimeType for IPlayReadyStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
pub struct IPlayReadyStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlayReadyStatics5, IPlayReadyStatics5_Vtbl, 0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
impl windows_core::RuntimeType for IPlayReadyStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPlayReadyStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDClient(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDClient, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl NDClient {
    #[cfg(feature = "deprecated")]
    pub fn RegistrationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveRegistrationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRegistrationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ProximityDetectionCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProximityDetectionCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveProximityDetectionCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProximityDetectionCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveLicenseFetchCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveLicenseFetchCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ReRegistrationNeeded<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReRegistrationNeeded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveReRegistrationNeeded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReRegistrationNeeded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionDataReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosedCaptionDataReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveClosedCaptionDataReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosedCaptionDataReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn StartAsync<P0, P1, P2>(&self, contenturl: P0, startasyncoptions: u32, registrationcustomdata: P1, licensefetchdescriptor: P2) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<INDCustomData>,
        P2: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), contenturl.param().abi(), startasyncoptions, registrationcustomdata.param().abi(), licensefetchdescriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchAsync<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchAsync)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ReRegistrationAsync<P0>(&self, registrationcustomdata: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReRegistrationAsync)(windows_core::Interface::as_raw(this), registrationcustomdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
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
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDClient, INDClientFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDClient {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDClient>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDClient {
    type Vtable = INDClient_Vtbl;
    const IID: windows_core::GUID = <INDClient as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDCustomData(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDCustomData, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDCustomData, INDCustomData);
#[cfg(feature = "deprecated")]
impl NDCustomData {
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomDataTypeID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).CustomData)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(customdatatypeidbytes: &[u8], customdatabytes: &[u8]) -> windows_core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), customdatatypeidbytes.len().try_into().unwrap(), customdatatypeidbytes.as_ptr(), customdatabytes.len().try_into().unwrap(), customdatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDCustomData, INDCustomDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDCustomData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDCustomData>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDCustomData {
    type Vtable = INDCustomData_Vtbl;
    const IID: windows_core::GUID = <INDCustomData as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDDownloadEngineNotifier(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDDownloadEngineNotifier, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDDownloadEngineNotifier, INDDownloadEngineNotifier);
#[cfg(feature = "deprecated")]
impl NDDownloadEngineNotifier {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDDownloadEngineNotifier, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnStreamOpened)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnDataReceived)(windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnEndOfStream)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnNetworkError)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDDownloadEngineNotifier>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
    const IID: windows_core::GUID = <INDDownloadEngineNotifier as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDLicenseFetchDescriptor(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDLicenseFetchDescriptor, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDLicenseFetchDescriptor, INDLicenseFetchDescriptor);
#[cfg(feature = "deprecated")]
impl NDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentIDType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).ContentID)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<P0>(contentidtype: NDContentIDType, contentidbytes: &[u8], licensefetchchallengecustomdata: P0) -> windows_core::Result<NDLicenseFetchDescriptor>
    where
        P0: windows_core::Param<INDCustomData>,
    {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), contentidtype, contentidbytes.len().try_into().unwrap(), contentidbytes.as_ptr(), licensefetchchallengecustomdata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDLicenseFetchDescriptor>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
    const IID: windows_core::GUID = <INDLicenseFetchDescriptor as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDStorageFileHelper(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDStorageFileHelper, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDStorageFileHelper, INDStorageFileHelper);
#[cfg(feature = "deprecated")]
impl NDStorageFileHelper {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDStorageFileHelper, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0>(&self, file: P0) -> windows_core::Result<super::super::super::Foundation::Collections::IVector<windows_core::HSTRING>>
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
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDStorageFileHelper>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
    const IID: windows_core::GUID = <INDStorageFileHelper as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDStreamParserNotifier(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDStreamParserNotifier, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDStreamParserNotifier, INDStreamParserNotifier);
#[cfg(feature = "deprecated")]
impl NDStreamParserNotifier {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDStreamParserNotifier, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnContentIDReceived)(windows_core::Interface::as_raw(this), licensefetchdescriptor.param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: windows_core::Param<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(windows_core::Interface::as_raw(this), audiostreamdescriptors.param().abi(), videostreamdescriptors.param().abi()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed<P0>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P0, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnSampleParsed)(windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: windows_core::GUID, probytes: &[u8]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(windows_core::Interface::as_raw(this), descriptor.param().abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDStreamParserNotifier>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
    const IID: windows_core::GUID = <INDStreamParserNotifier as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NDTCPMessenger(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(NDTCPMessenger, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(NDTCPMessenger, INDMessenger);
#[cfg(feature = "deprecated")]
impl NDTCPMessenger {
    #[cfg(feature = "deprecated")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(remotehostname: &windows_core::HSTRING, remotehostport: u32) -> windows_core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(remotehostname), remotehostport, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INDMessenger>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for NDTCPMessenger {
    type Vtable = INDMessenger_Vtbl;
    const IID: windows_core::GUID = <INDMessenger as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).KeyIdString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).CustomAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn CreateInstanceFromWindowsMediaDrmHeader<P0, P1>(headerbytes: &[u8], licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromWindowsMediaDrmHeader)(windows_core::Interface::as_raw(this), headerbytes.len().try_into().unwrap(), headerbytes.as_ptr(), licenseacquisitionurl.param().abi(), licenseacquisitionuserinterfaceurl.param().abi(), core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceFromComponents<P0, P1>(contentkeyid: windows_core::GUID, contentkeyidstring: &windows_core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
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
    pub fn CreateInstanceFromComponents2<P0, P1>(dwflags: u32, contentkeyids: &[windows_core::GUID], contentkeyidstrings: &[windows_core::HSTRING], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &windows_core::HSTRING, domainserviceid: windows_core::GUID) -> windows_core::Result<PlayReadyContentHeader>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromComponents2)(windows_core::Interface::as_raw(this), dwflags, contentkeyids.len().try_into().unwrap(), contentkeyids.as_ptr(), contentkeyidstrings.len().try_into().unwrap(), core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, licenseacquisitionurl.param().abi(), licenseacquisitionuserinterfaceurl.param().abi(), core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyContentHeader>();
}
unsafe impl windows_core::Interface for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
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
    #[doc(hidden)]
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyDomain(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyDomain, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyDomain, IPlayReadyDomain);
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
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    type Vtable = IPlayReadyDomain_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyDomain as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyDomainIterable(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadyDomainIterable, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadyDomainIterable, super::super::super::Foundation::Collections::IIterable::<IPlayReadyDomain>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterable {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(domainaccountid: windows_core::GUID) -> windows_core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), domainaccountid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadyDomainIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyDomain>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyDomainIterator(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadyDomainIterator, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadyDomainIterator, super::super::super::Foundation::Collections::IIterator::<IPlayReadyDomain>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> windows_core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [Option<IPlayReadyDomain>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadyDomainIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyDomain>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).DomainFriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyDomainJoinServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyDomainLeaveServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
    pub fn GenerateData<P0>(&self, guidcpsystemid: windows_core::GUID, countofstreams: u32, configuration: P0, format: PlayReadyITADataFormat) -> windows_core::Result<windows_core::Array<u8>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IPropertySet>,
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
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyITADataGenerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyIndividualizationServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyLicense(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicense, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyLicense, IPlayReadyLicense);
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
    type Vtable = IPlayReadyLicense_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyLicense as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadyLicenseAcquisitionServiceRequest, IPlayReadyServiceRequest);
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
    #[cfg(feature = "Foundation_Collections")]
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyLicenseAcquisitionServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyLicenseIterable(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterable, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadyLicenseIterable, super::super::super::Foundation::Collections::IIterable::<IPlayReadyLicense>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterable {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseIterable, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0>(contentheader: P0, fullyevaluated: bool) -> windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), contentheader.param().abi(), fullyevaluated, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadyLicenseIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyLicense>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyLicenseIterator(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterator, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadyLicenseIterator, super::super::super::Foundation::Collections::IIterator::<IPlayReadyLicense>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> windows_core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [Option<IPlayReadyLicense>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadyLicenseIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyLicense>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
pub struct PlayReadyLicenseManagement;
impl PlayReadyLicenseManagement {
    pub fn DeleteLicenses<P0>(contentheader: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteLicenses)(windows_core::Interface::as_raw(this), contentheader.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadyLicenseSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadyLicenseSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadyLicenseSession, IPlayReadyLicenseSession, IPlayReadyLicenseSession2);
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
    #[cfg(feature = "Foundation_Collections")]
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
    #[doc(hidden)]
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadyLicenseSession>();
}
unsafe impl windows_core::Interface for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyLicenseSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyMeteringReportServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadyRevocationServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadySecureStopIterable(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterable, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadySecureStopIterable, super::super::super::Foundation::Collections::IIterable::<IPlayReadySecureStopServiceRequest>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterable {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(publishercertbytes: &[u8]) -> windows_core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadySecureStopIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadySecureStopServiceRequest>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadySecureStopIterator(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterator, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(PlayReadySecureStopIterator, super::super::super::Foundation::Collections::IIterator::<IPlayReadySecureStopServiceRequest>);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> windows_core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [Option<IPlayReadySecureStopServiceRequest>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for PlayReadySecureStopIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadySecureStopServiceRequest>;
    const IID: windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PlayReadySecureStopServiceRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlayReadySecureStopServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PlayReadySecureStopServiceRequest, super::IMediaProtectionServiceRequest, IPlayReadySecureStopServiceRequest, IPlayReadyServiceRequest);
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
            (windows_core::Interface::vtable(this).ResponseCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChallengeCustomData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChallengeCustomData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetChallengeCustomData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetChallengeCustomData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BeginServiceRequest(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncAction> {
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
    #[doc(hidden)]
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlayReadySecureStopServiceRequest>();
}
unsafe impl windows_core::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
    const IID: windows_core::GUID = <IPlayReadySecureStopServiceRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
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
    type Vtable = IPlayReadySoapMessage_Vtbl;
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
            (windows_core::Interface::vtable(this).InputTrustAuthorityToCreate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[doc(hidden)]
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDCertificateFeature(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDCertificateFeature {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDCertificateFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDCertificateFeature").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDCertificatePlatformID(pub i32);
#[cfg(feature = "deprecated")]
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
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDCertificatePlatformID {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDCertificatePlatformID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDCertificatePlatformID").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDCertificateType(pub i32);
#[cfg(feature = "deprecated")]
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
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDCertificateType {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDCertificateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDCertificateType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDCertificateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDClosedCaptionFormat(pub i32);
#[cfg(feature = "deprecated")]
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDClosedCaptionFormat {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDClosedCaptionFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDClosedCaptionFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDContentIDType(pub i32);
#[cfg(feature = "deprecated")]
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDContentIDType {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDContentIDType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDContentIDType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDContentIDType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDMediaStreamType(pub i32);
#[cfg(feature = "deprecated")]
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDMediaStreamType {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDMediaStreamType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDMediaStreamType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDProximityDetectionType(pub i32);
#[cfg(feature = "deprecated")]
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDProximityDetectionType {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDProximityDetectionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDProximityDetectionType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NDStartAsyncOptions(pub i32);
#[cfg(feature = "deprecated")]
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl windows_core::TypeKind for NDStartAsyncOptions {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl core::fmt::Debug for NDStartAsyncOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NDStartAsyncOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl windows_core::TypeKind for PlayReadyDecryptorSetup {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlayReadyDecryptorSetup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlayReadyDecryptorSetup").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
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
impl core::fmt::Debug for PlayReadyEncryptionAlgorithm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlayReadyEncryptionAlgorithm").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl windows_core::TypeKind for PlayReadyHardwareDRMFeatures {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlayReadyHardwareDRMFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlayReadyHardwareDRMFeatures").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl windows_core::TypeKind for PlayReadyITADataFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlayReadyITADataFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlayReadyITADataFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
