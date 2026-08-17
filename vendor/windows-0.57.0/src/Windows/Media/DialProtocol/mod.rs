windows_core::imp::define_interface!(IDialApp, IDialApp_Vtbl, 0x555ffbd3_45b7_49f3_bbd7_302db6084646);
impl windows_core::RuntimeType for IDialApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RequestLaunchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialAppStateDetails, IDialAppStateDetails_Vtbl, 0xddc4a4a1_f5de_400d_bea4_8c8466bb2961);
impl windows_core::RuntimeType for IDialAppStateDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialAppStateDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DialAppState) -> windows_core::HRESULT,
    pub FullXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialDevice, IDialDevice_Vtbl, 0xfff0edaf_759f_41d2_a20a_7f29ce0b3784);
impl windows_core::RuntimeType for IDialDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDialApp: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialDevice2, IDialDevice2_Vtbl, 0xbab7f3d5_5bfb_4eba_8b32_b57c5c5ee5c9);
impl windows_core::RuntimeType for IDialDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
windows_core::imp::define_interface!(IDialDevicePicker, IDialDevicePicker_Vtbl, 0xba7e520a_ff59_4f4b_bdac_d89f495ad6e1);
impl windows_core::RuntimeType for IDialDevicePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDevicePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Appearance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Appearance: usize,
    pub DialDeviceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDialDeviceSelected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DisconnectButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DialDevicePickerDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDialDevicePickerDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub ShowWithPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    ShowWithPlacement: usize,
    pub PickSingleDialDeviceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Popups")]
    pub PickSingleDialDeviceAsyncWithPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect, super::super::UI::Popups::Placement, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PickSingleDialDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DialDeviceDisplayStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialDevicePickerFilter, IDialDevicePickerFilter_Vtbl, 0xc17c93ba_86c0_485d_b8d6_0f9a8f641590);
impl windows_core::RuntimeType for IDialDevicePickerFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDevicePickerFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedAppNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedAppNames: usize,
}
windows_core::imp::define_interface!(IDialDeviceSelectedEventArgs, IDialDeviceSelectedEventArgs_Vtbl, 0x480b92ad_ac76_47eb_9c06_a19304da0247);
impl windows_core::RuntimeType for IDialDeviceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDeviceSelectedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedDialDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialDeviceStatics, IDialDeviceStatics_Vtbl, 0xaa69cc95_01f8_4758_8461_2bbd1cdc3cf3);
impl windows_core::RuntimeType for IDialDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInfoSupportsDialAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInfoSupportsDialAsync: usize,
}
windows_core::imp::define_interface!(IDialDisconnectButtonClickedEventArgs, IDialDisconnectButtonClickedEventArgs_Vtbl, 0x52765152_9c81_4e55_adc2_0ebe99cde3b6);
impl windows_core::RuntimeType for IDialDisconnectButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialDisconnectButtonClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialReceiverApp, IDialReceiverApp_Vtbl, 0xfd3e7c57_5045_470e_b304_4dd9b13e7d11);
impl windows_core::RuntimeType for IDialReceiverApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialReceiverApp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAdditionalDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAdditionalDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalDataAsync: usize,
}
windows_core::imp::define_interface!(IDialReceiverApp2, IDialReceiverApp2_Vtbl, 0x530c5805_9130_42ac_a504_1977dcb2ea8a);
impl windows_core::RuntimeType for IDialReceiverApp2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialReceiverApp2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetUniqueDeviceNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDialReceiverAppStatics, IDialReceiverAppStatics_Vtbl, 0x53183a3c_4c36_4d02_b28a_f2a9da38ec52);
impl windows_core::RuntimeType for IDialReceiverAppStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDialReceiverAppStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialApp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialApp, windows_core::IUnknown, windows_core::IInspectable);
impl DialApp {
    pub fn AppName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestLaunchAsync(&self, appargument: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialAppLaunchResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestLaunchAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appargument), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StopAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialAppStopResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StopAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppStateAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialAppStateDetails>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppStateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialApp>();
}
unsafe impl windows_core::Interface for DialApp {
    type Vtable = IDialApp_Vtbl;
    const IID: windows_core::GUID = <IDialApp as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialApp";
}
unsafe impl Send for DialApp {}
unsafe impl Sync for DialApp {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialAppStateDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialAppStateDetails, windows_core::IUnknown, windows_core::IInspectable);
impl DialAppStateDetails {
    pub fn State(&self) -> windows_core::Result<DialAppState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FullXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialAppStateDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialAppStateDetails>();
}
unsafe impl windows_core::Interface for DialAppStateDetails {
    type Vtable = IDialAppStateDetails_Vtbl;
    const IID: windows_core::GUID = <IDialAppStateDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialAppStateDetails {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialAppStateDetails";
}
unsafe impl Send for DialAppStateDetails {}
unsafe impl Sync for DialAppStateDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialDevice, windows_core::IUnknown, windows_core::IInspectable);
impl DialDevice {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDialApp(&self, appname: &windows_core::HSTRING) -> windows_core::Result<DialApp> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDialApp)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FriendlyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &windows_core::Interface::cast::<IDialDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector(appname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(value: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInfoSupportsDialAsync<P0>(device: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::Devices::Enumeration::DeviceInformation>,
    {
        Self::IDialDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceInfoSupportsDialAsync)(windows_core::Interface::as_raw(this), device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDialDeviceStatics<R, F: FnOnce(&IDialDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DialDevice, IDialDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DialDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialDevice>();
}
unsafe impl windows_core::Interface for DialDevice {
    type Vtable = IDialDevice_Vtbl;
    const IID: windows_core::GUID = <IDialDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialDevice {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevice";
}
unsafe impl Send for DialDevice {}
unsafe impl Sync for DialDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialDevicePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialDevicePicker, windows_core::IUnknown, windows_core::IInspectable);
impl DialDevicePicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DialDevicePicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Filter(&self) -> windows_core::Result<DialDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Filter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Appearance(&self) -> windows_core::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appearance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DialDeviceSelected<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDeviceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialDeviceSelected)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDialDeviceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDialDeviceSelected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DisconnectButtonClicked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DialDevicePicker, DialDisconnectButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisconnectButtonClicked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisconnectButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisconnectButtonClicked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DialDevicePickerDismissed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DialDevicePicker, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DialDevicePickerDismissed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDialDevicePickerDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDialDevicePickerDismissed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this), selection).ok() }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn ShowWithPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowWithPlacement)(windows_core::Interface::as_raw(this), selection, preferredplacement).ok() }
    }
    pub fn PickSingleDialDeviceAsync(&self, selection: super::super::Foundation::Rect) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleDialDeviceAsync)(windows_core::Interface::as_raw(this), selection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Popups")]
    pub fn PickSingleDialDeviceAsyncWithPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DialDevice>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleDialDeviceAsyncWithPlacement)(windows_core::Interface::as_raw(this), selection, preferredplacement, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Hide(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Hide)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetDisplayStatus<P0>(&self, device: P0, status: DialDeviceDisplayStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DialDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayStatus)(windows_core::Interface::as_raw(this), device.param().abi(), status).ok() }
    }
}
impl windows_core::RuntimeType for DialDevicePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialDevicePicker>();
}
unsafe impl windows_core::Interface for DialDevicePicker {
    type Vtable = IDialDevicePicker_Vtbl;
    const IID: windows_core::GUID = <IDialDevicePicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialDevicePicker {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePicker";
}
unsafe impl Send for DialDevicePicker {}
unsafe impl Sync for DialDevicePicker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialDevicePickerFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialDevicePickerFilter, windows_core::IUnknown, windows_core::IInspectable);
impl DialDevicePickerFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedAppNames(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedAppNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialDevicePickerFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialDevicePickerFilter>();
}
unsafe impl windows_core::Interface for DialDevicePickerFilter {
    type Vtable = IDialDevicePickerFilter_Vtbl;
    const IID: windows_core::GUID = <IDialDevicePickerFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDevicePickerFilter";
}
unsafe impl Send for DialDevicePickerFilter {}
unsafe impl Sync for DialDevicePickerFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialDeviceSelectedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialDeviceSelectedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DialDeviceSelectedEventArgs {
    pub fn SelectedDialDevice(&self) -> windows_core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectedDialDevice)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialDeviceSelectedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialDeviceSelectedEventArgs>();
}
unsafe impl windows_core::Interface for DialDeviceSelectedEventArgs {
    type Vtable = IDialDeviceSelectedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDialDeviceSelectedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDeviceSelectedEventArgs";
}
unsafe impl Send for DialDeviceSelectedEventArgs {}
unsafe impl Sync for DialDeviceSelectedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialDisconnectButtonClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialDisconnectButtonClickedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DialDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> windows_core::Result<DialDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DialDisconnectButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialDisconnectButtonClickedEventArgs>();
}
unsafe impl windows_core::Interface for DialDisconnectButtonClickedEventArgs {
    type Vtable = IDialDisconnectButtonClickedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDialDisconnectButtonClickedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialDisconnectButtonClickedEventArgs";
}
unsafe impl Send for DialDisconnectButtonClickedEventArgs {}
unsafe impl Sync for DialDisconnectButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DialReceiverApp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DialReceiverApp, windows_core::IUnknown, windows_core::IInspectable);
impl DialReceiverApp {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAdditionalDataAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAdditionalDataAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalDataAsync<P0>(&self, additionaldata: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAdditionalDataAsync)(windows_core::Interface::as_raw(this), additionaldata.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetUniqueDeviceNameAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IDialReceiverApp2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUniqueDeviceNameAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<DialReceiverApp> {
        Self::IDialReceiverAppStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDialReceiverAppStatics<R, F: FnOnce(&IDialReceiverAppStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DialReceiverApp, IDialReceiverAppStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DialReceiverApp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDialReceiverApp>();
}
unsafe impl windows_core::Interface for DialReceiverApp {
    type Vtable = IDialReceiverApp_Vtbl;
    const IID: windows_core::GUID = <IDialReceiverApp as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DialReceiverApp {
    const NAME: &'static str = "Windows.Media.DialProtocol.DialReceiverApp";
}
unsafe impl Send for DialReceiverApp {}
unsafe impl Sync for DialReceiverApp {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: Self = Self(0i32);
    pub const FailedToLaunch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for DialAppLaunchResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DialAppLaunchResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DialAppLaunchResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DialAppLaunchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppLaunchResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for DialAppState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DialAppState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DialAppState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DialAppState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: Self = Self(0i32);
    pub const StopFailed: Self = Self(1i32);
    pub const OperationNotSupported: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for DialAppStopResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DialAppStopResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DialAppStopResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DialAppStopResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialAppStopResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Disconnected: Self = Self(4i32);
    pub const Error: Self = Self(5i32);
}
impl windows_core::TypeKind for DialDeviceDisplayStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DialDeviceDisplayStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DialDeviceDisplayStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DialDeviceDisplayStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.DialProtocol.DialDeviceDisplayStatus;i4)");
}
