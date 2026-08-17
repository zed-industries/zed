windows_core::imp::define_interface!(IScreenReaderPositionChangedEventArgs, IScreenReaderPositionChangedEventArgs_Vtbl, 0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
impl windows_core::RuntimeType for IScreenReaderPositionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScreenReaderPositionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub IsReadingText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScreenReaderService, IScreenReaderService_Vtbl, 0x19475427_eac0_50d3_bdd9_9b487a226256);
impl windows_core::RuntimeType for IScreenReaderService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScreenReaderService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentScreenReaderPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScreenReaderPositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScreenReaderPositionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ScreenReaderPositionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScreenReaderPositionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ScreenReaderPositionChangedEventArgs {
    pub fn ScreenPositionInRawPixels(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenPositionInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReadingText(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadingText)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ScreenReaderPositionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScreenReaderPositionChangedEventArgs>();
}
unsafe impl windows_core::Interface for ScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IScreenReaderPositionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
}
unsafe impl Send for ScreenReaderPositionChangedEventArgs {}
unsafe impl Sync for ScreenReaderPositionChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ScreenReaderService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScreenReaderService, windows_core::IUnknown, windows_core::IInspectable);
impl ScreenReaderService {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScreenReaderService, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CurrentScreenReaderPosition(&self) -> windows_core::Result<ScreenReaderPositionChangedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentScreenReaderPosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ScreenReaderPositionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenReaderPositionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScreenReaderPositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScreenReaderPositionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ScreenReaderService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScreenReaderService>();
}
unsafe impl windows_core::Interface for ScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
    const IID: windows_core::GUID = <IScreenReaderService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderService";
}
unsafe impl Send for ScreenReaderService {}
unsafe impl Sync for ScreenReaderService {}
