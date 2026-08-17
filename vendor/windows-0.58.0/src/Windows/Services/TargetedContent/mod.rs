windows_core::imp::define_interface!(ITargetedContentAction, ITargetedContentAction_Vtbl, 0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
impl windows_core::RuntimeType for ITargetedContentAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentAvailabilityChangedEventArgs, ITargetedContentAvailabilityChangedEventArgs_Vtbl, 0xe0f59d26_5927_4450_965c_1ceb7becde65);
impl windows_core::RuntimeType for ITargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentAvailabilityChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentChangedEventArgs, ITargetedContentChangedEventArgs_Vtbl, 0x99d488c9_587e_4586_8ef7_b54ca9453a16);
impl windows_core::RuntimeType for ITargetedContentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasPreviousContentExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentCollection, ITargetedContentCollection_Vtbl, 0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
impl windows_core::RuntimeType for ITargetedContentCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, TargetedContentInteraction) -> windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
windows_core::imp::define_interface!(ITargetedContentContainer, ITargetedContentContainer_Vtbl, 0xbc2494c9_8837_47c2_850f_d79d64595926);
impl windows_core::RuntimeType for ITargetedContentContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Availability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TargetedContentAvailability) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectSingleObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentContainerStatics, ITargetedContentContainerStatics_Vtbl, 0x5b47e7fb_2140_4c1f_a736_c59583f227d8);
impl windows_core::RuntimeType for ITargetedContentContainerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentContainerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentImage, ITargetedContentImage_Vtbl, 0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
impl windows_core::RuntimeType for ITargetedContentImage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentImage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentItem, ITargetedContentItem_Vtbl, 0x38168dc4_276c_4c32_96ba_565c6e406e74);
impl windows_core::RuntimeType for ITargetedContentItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReportInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, TargetedContentInteraction) -> windows_core::HRESULT,
    pub ReportCustomInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Collections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Collections: usize,
}
windows_core::imp::define_interface!(ITargetedContentItemState, ITargetedContentItemState_Vtbl, 0x73935454_4c65_4b47_a441_472de53c79b6);
impl windows_core::RuntimeType for ITargetedContentItemState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentItemState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AppInstallationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TargetedContentAppInstallationState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentObject, ITargetedContentObject_Vtbl, 0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
impl windows_core::RuntimeType for ITargetedContentObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ObjectKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TargetedContentObjectKind) -> windows_core::HRESULT,
    pub Collection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentStateChangedEventArgs, ITargetedContentStateChangedEventArgs_Vtbl, 0x9a1cef3d_8073_4416_8df2_546835a6414f);
impl windows_core::RuntimeType for ITargetedContentStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentSubscription, ITargetedContentSubscription_Vtbl, 0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
impl windows_core::RuntimeType for ITargetedContentSubscription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentSubscription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetContentContainerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveContentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AvailabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentSubscriptionOptions, ITargetedContentSubscriptionOptions_Vtbl, 0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
impl windows_core::RuntimeType for ITargetedContentSubscriptionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentSubscriptionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SubscriptionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AllowPartialContentAvailability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowPartialContentAvailability: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CloudQueryParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CloudQueryParameters: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalFilters: usize,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentSubscriptionStatics, ITargetedContentSubscriptionStatics_Vtbl, 0xfaddfe80_360d_4916_b53c_7ea27090d02a);
impl windows_core::RuntimeType for ITargetedContentSubscriptionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentSubscriptionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetedContentValue, ITargetedContentValue_Vtbl, 0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
impl windows_core::RuntimeType for ITargetedContentValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetedContentValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ValueKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TargetedContentValueKind) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub String: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Number: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    File: usize,
    pub ImageFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Strings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Strings: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Uris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Uris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Numbers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Numbers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Booleans: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Booleans: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    Files: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImageFiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentAction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentAction, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentAction {
    pub fn InvokeAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentAction>();
}
unsafe impl windows_core::Interface for TargetedContentAction {
    type Vtable = ITargetedContentAction_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentAction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAction";
}
unsafe impl Send for TargetedContentAction {}
unsafe impl Sync for TargetedContentAction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentAvailabilityChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentAvailabilityChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentAvailabilityChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentAvailabilityChangedEventArgs>();
}
unsafe impl windows_core::Interface for TargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentAvailabilityChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
}
unsafe impl Send for TargetedContentAvailabilityChangedEventArgs {}
unsafe impl Sync for TargetedContentAvailabilityChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasPreviousContentExpired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasPreviousContentExpired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TargetedContentChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentChangedEventArgs>();
}
unsafe impl windows_core::Interface for TargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
}
unsafe impl Send for TargetedContentChangedEventArgs {}
unsafe impl Sync for TargetedContentChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentCollection, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentCollection {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportInteraction)(windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction(&self, custominteractionname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCustomInteraction)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(custominteractionname)).ok() }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentCollection>();
}
unsafe impl windows_core::Interface for TargetedContentCollection {
    type Vtable = ITargetedContentCollection_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentCollection";
}
unsafe impl Send for TargetedContentCollection {}
unsafe impl Sync for TargetedContentCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentContainer, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentContainer {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Availability(&self) -> windows_core::Result<TargetedContentAvailability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Availability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Content(&self) -> windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleObject(&self, path: &windows_core::HSTRING) -> windows_core::Result<TargetedContentObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleObject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAsync(contentid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        Self::ITargetedContentContainerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contentid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentContainerStatics<R, F: FnOnce(&ITargetedContentContainerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TargetedContentContainer, ITargetedContentContainerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TargetedContentContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentContainer>();
}
unsafe impl windows_core::Interface for TargetedContentContainer {
    type Vtable = ITargetedContentContainer_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentContainer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentContainer";
}
unsafe impl Send for TargetedContentContainer {}
unsafe impl Sync for TargetedContentContainer {}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentFile(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(TargetedContentFile, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(TargetedContentFile, super::super::Storage::Streams::IRandomAccessStreamReference);
#[cfg(feature = "Storage_Streams")]
impl TargetedContentFile {
    #[cfg(feature = "Storage_Streams")]
    pub fn OpenReadAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for TargetedContentFile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::Storage::Streams::IRandomAccessStreamReference>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for TargetedContentFile {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamReference_Vtbl;
    const IID: windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamReference as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for TargetedContentFile {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentFile";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for TargetedContentFile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentImage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentImage, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(TargetedContentImage, super::super::Storage::Streams::IRandomAccessStreamReference);
impl TargetedContentImage {
    #[cfg(feature = "Storage_Streams")]
    pub fn OpenReadAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &windows_core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TargetedContentImage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentImage>();
}
unsafe impl windows_core::Interface for TargetedContentImage {
    type Vtable = ITargetedContentImage_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentImage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentImage";
}
unsafe impl Send for TargetedContentImage {}
unsafe impl Sync for TargetedContentImage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentItem, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentItem {
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportInteraction)(windows_core::Interface::as_raw(this), interaction).ok() }
    }
    pub fn ReportCustomInteraction(&self, custominteractionname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCustomInteraction)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(custominteractionname)).ok() }
    }
    pub fn State(&self) -> windows_core::Result<TargetedContentItemState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collections)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentItem>();
}
unsafe impl windows_core::Interface for TargetedContentItem {
    type Vtable = ITargetedContentItem_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItem";
}
unsafe impl Send for TargetedContentItem {}
unsafe impl Sync for TargetedContentItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentItemState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentItemState, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentItemState {
    pub fn ShouldDisplay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldDisplay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppInstallationState(&self) -> windows_core::Result<TargetedContentAppInstallationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInstallationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TargetedContentItemState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentItemState>();
}
unsafe impl windows_core::Interface for TargetedContentItemState {
    type Vtable = ITargetedContentItemState_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentItemState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItemState";
}
unsafe impl Send for TargetedContentItemState {}
unsafe impl Sync for TargetedContentItemState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentObject, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentObject {
    pub fn ObjectKind(&self) -> windows_core::Result<TargetedContentObjectKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ObjectKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Collection(&self) -> windows_core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Collection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Item(&self) -> windows_core::Result<TargetedContentItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<TargetedContentValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentObject>();
}
unsafe impl windows_core::Interface for TargetedContentObject {
    type Vtable = ITargetedContentObject_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentObject";
}
unsafe impl Send for TargetedContentObject {}
unsafe impl Sync for TargetedContentObject {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentStateChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for TargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
}
unsafe impl Send for TargetedContentStateChangedEventArgs {}
unsafe impl Sync for TargetedContentStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentSubscription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentSubscription, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentSubscription {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetContentContainerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContentContainerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContentChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveContentChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn AvailabilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailabilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAvailabilityChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAvailabilityChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetAsync(subscriptionid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subscriptionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetOptions(subscriptionid: &windows_core::HSTRING) -> windows_core::Result<TargetedContentSubscriptionOptions> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(subscriptionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentSubscriptionStatics<R, F: FnOnce(&ITargetedContentSubscriptionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TargetedContentSubscription, ITargetedContentSubscriptionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TargetedContentSubscription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentSubscription>();
}
unsafe impl windows_core::Interface for TargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentSubscription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscription";
}
unsafe impl Send for TargetedContentSubscription {}
unsafe impl Sync for TargetedContentSubscription {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentSubscriptionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentSubscriptionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentSubscriptionOptions {
    pub fn SubscriptionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscriptionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AllowPartialContentAvailability(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowPartialContentAvailability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowPartialContentAvailability(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowPartialContentAvailability)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CloudQueryParameters(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloudQueryParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalFilters(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalFilters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Update(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for TargetedContentSubscriptionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentSubscriptionOptions>();
}
unsafe impl windows_core::Interface for TargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentSubscriptionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
}
unsafe impl Send for TargetedContentSubscriptionOptions {}
unsafe impl Sync for TargetedContentSubscriptionOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetedContentValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetedContentValue, windows_core::IUnknown, windows_core::IInspectable);
impl TargetedContentValue {
    pub fn ValueKind(&self) -> windows_core::Result<TargetedContentValueKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn String(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).String)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Number(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Number)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Boolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Boolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> windows_core::Result<TargetedContentFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImageFile(&self) -> windows_core::Result<TargetedContentImage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Action(&self) -> windows_core::Result<TargetedContentAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strings(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Strings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Uris(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Numbers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Numbers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Booleans(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Booleans)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Files(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Files)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImageFiles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImageFiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Actions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetedContentValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetedContentValue>();
}
unsafe impl windows_core::Interface for TargetedContentValue {
    type Vtable = ITargetedContentValue_Vtbl;
    const IID: windows_core::GUID = <ITargetedContentValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentValue";
}
unsafe impl Send for TargetedContentValue {}
unsafe impl Sync for TargetedContentValue {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl windows_core::TypeKind for TargetedContentAppInstallationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TargetedContentAppInstallationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TargetedContentAppInstallationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TargetedContentAppInstallationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAppInstallationState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl windows_core::TypeKind for TargetedContentAvailability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TargetedContentAvailability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TargetedContentAvailability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TargetedContentAvailability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAvailability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl windows_core::TypeKind for TargetedContentInteraction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TargetedContentInteraction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TargetedContentInteraction").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TargetedContentInteraction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentInteraction;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl windows_core::TypeKind for TargetedContentObjectKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TargetedContentObjectKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TargetedContentObjectKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TargetedContentObjectKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentObjectKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl windows_core::TypeKind for TargetedContentValueKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TargetedContentValueKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TargetedContentValueKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TargetedContentValueKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentValueKind;i4)");
}
