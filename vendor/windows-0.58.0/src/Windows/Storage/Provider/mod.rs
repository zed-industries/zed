windows_core::imp::define_interface!(ICachedFileUpdaterStatics, ICachedFileUpdaterStatics_Vtbl, 0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
impl windows_core::RuntimeType for ICachedFileUpdaterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICachedFileUpdaterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetUpdateInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, ReadActivationMode, WriteActivationMode, CachedFileOptions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICachedFileUpdaterUI, ICachedFileUpdaterUI_Vtbl, 0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
impl windows_core::RuntimeType for ICachedFileUpdaterUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICachedFileUpdaterUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CachedFileTarget) -> windows_core::HRESULT,
    pub FileUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UIRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUIRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UIStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICachedFileUpdaterUI2, ICachedFileUpdaterUI2_Vtbl, 0x8856a21c_8699_4340_9f49_f7cad7fe8991);
impl windows_core::RuntimeType for ICachedFileUpdaterUI2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICachedFileUpdaterUI2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequest, IFileUpdateRequest_Vtbl, 0x40c82536_c1fe_4d93_a792_1e736bc70837);
impl windows_core::RuntimeType for IFileUpdateRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileUpdateRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub File: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileUpdateStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, FileUpdateStatus) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateLocalFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequest2, IFileUpdateRequest2_Vtbl, 0x82484648_bdbe_447b_a2ee_7afe6a032a94);
impl windows_core::RuntimeType for IFileUpdateRequest2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileUpdateRequest2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserInputNeededMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequestDeferral, IFileUpdateRequestDeferral_Vtbl, 0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
impl windows_core::RuntimeType for IFileUpdateRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileUpdateRequestDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileUpdateRequestedEventArgs, IFileUpdateRequestedEventArgs_Vtbl, 0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
impl windows_core::RuntimeType for IFileUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileUpdateRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderFileTypeInfo, IStorageProviderFileTypeInfo_Vtbl, 0x1955b9c1_0184_5a88_87df_4544f464365d);
impl windows_core::RuntimeType for IStorageProviderFileTypeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderFileTypeInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderFileTypeInfoFactory, IStorageProviderFileTypeInfoFactory_Vtbl, 0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
impl windows_core::RuntimeType for IStorageProviderFileTypeInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderFileTypeInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderGetContentInfoForPathResult, IStorageProviderGetContentInfoForPathResult_Vtbl, 0x2564711d_aa89_4d12_82e3_f72a92e33966);
impl windows_core::RuntimeType for IStorageProviderGetContentInfoForPathResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderGetContentInfoForPathResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderGetPathForContentUriResult, IStorageProviderGetPathForContentUriResult_Vtbl, 0x63711a9d_4118_45a6_acb6_22c49d019f40);
impl windows_core::RuntimeType for IStorageProviderGetPathForContentUriResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderGetPathForContentUriResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderUriSourceStatus) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertiesStatics, IStorageProviderItemPropertiesStatics_Vtbl, 0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
impl windows_core::RuntimeType for IStorageProviderItemPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderItemPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAsync: usize,
}
windows_core::imp::define_interface!(IStorageProviderItemProperty, IStorageProviderItemProperty_Vtbl, 0x476cb558_730b_4188_b7b5_63b716ed476d);
impl windows_core::RuntimeType for IStorageProviderItemProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderItemProperty_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertyDefinition, IStorageProviderItemPropertyDefinition_Vtbl, 0xc5b383bb_ff1f_4298_831e_ff1c08089690);
impl windows_core::RuntimeType for IStorageProviderItemPropertyDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderItemPropertyDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderItemPropertySource, IStorageProviderItemPropertySource_Vtbl, 0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
impl core::ops::Deref for IStorageProviderItemPropertySource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderItemPropertySource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderItemPropertySource {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemProperties(&self, itempath: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(itempath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderItemPropertySource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemProperties: usize,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderEntry, IStorageProviderKnownFolderEntry_Vtbl, 0xeffa7db0_1d44_596b_8464_928800c5e2d8);
impl windows_core::RuntimeType for IStorageProviderKnownFolderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderKnownFolderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KnownFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetKnownFolderId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderKnownFolderSyncStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderKnownFolderSyncStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfo, IStorageProviderKnownFolderSyncInfo_Vtbl, 0x98b017ce_ffc1_5b11_ae77_cc17afec1049);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderKnownFolderSyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetProviderDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownFolderEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownFolderEntries: usize,
    pub SyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSyncRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfoSource, IStorageProviderKnownFolderSyncInfoSource_Vtbl, 0x51359342_f7c0_53d0_bbb6_1cdc098ebda9);
impl core::ops::Deref for IStorageProviderKnownFolderSyncInfoSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderKnownFolderSyncInfoSource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderKnownFolderSyncInfoSource {
    pub fn GetKnownFolderSyncInfo(&self) -> windows_core::Result<StorageProviderKnownFolderSyncInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKnownFolderSyncInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KnownFolderSyncInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageProviderKnownFolderSyncInfoSource, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderSyncInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveKnownFolderSyncInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveKnownFolderSyncInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfoSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderKnownFolderSyncInfoSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetKnownFolderSyncInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KnownFolderSyncInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveKnownFolderSyncInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncInfoSourceFactory, IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl, 0xaaee03a7_a7f6_50be_a9b0_8e82d0c81082);
impl core::ops::Deref for IStorageProviderKnownFolderSyncInfoSourceFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderKnownFolderSyncInfoSourceFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderKnownFolderSyncInfoSourceFactory {
    pub fn GetKnownFolderSyncInfoSource(&self) -> windows_core::Result<IStorageProviderKnownFolderSyncInfoSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKnownFolderSyncInfoSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncInfoSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderKnownFolderSyncInfoSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetKnownFolderSyncInfoSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderKnownFolderSyncRequestArgs, IStorageProviderKnownFolderSyncRequestArgs_Vtbl, 0xeda6d569_b4e8_542f_ab8d_f3613f250a4a);
impl windows_core::RuntimeType for IStorageProviderKnownFolderSyncRequestArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderKnownFolderSyncRequestArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownFolders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownFolders: usize,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderMoreInfoUI, IStorageProviderMoreInfoUI_Vtbl, 0xef38e591_a7cb_5e7d_9b5e_22749842697c);
impl windows_core::RuntimeType for IStorageProviderMoreInfoUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderMoreInfoUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderPropertyCapabilities, IStorageProviderPropertyCapabilities_Vtbl, 0x658d2f0e_63b7_4567_acf9_51abe301dda5);
impl core::ops::Deref for IStorageProviderPropertyCapabilities {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderPropertyCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderPropertyCapabilities {
    pub fn IsPropertySupported(&self, propertycanonicalname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPropertySupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertycanonicalname), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderPropertyCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPropertySupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderQuotaUI, IStorageProviderQuotaUI_Vtbl, 0xba6295c3_312e_544f_9fd5_1f81b21f3649);
impl windows_core::RuntimeType for IStorageProviderQuotaUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderQuotaUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QuotaTotalInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetQuotaTotalInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub QuotaUsedInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetQuotaUsedInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub QuotaUsedLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetQuotaUsedLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub QuotaUsedColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    QuotaUsedColor: usize,
    #[cfg(feature = "UI")]
    pub SetQuotaUsedColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetQuotaUsedColor: usize,
}
windows_core::imp::define_interface!(IStorageProviderStatusUI, IStorageProviderStatusUI_Vtbl, 0xd6b6a758_198d_5b80_977f_5ff73da33118);
impl windows_core::RuntimeType for IStorageProviderStatusUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderStatusUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderState) -> windows_core::HRESULT,
    pub SetProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderState) -> windows_core::HRESULT,
    pub ProviderStateLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetProviderStateLabel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProviderStateIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderStateIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncStatusCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSyncStatusCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuotaUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuotaUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoreInfoUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMoreInfoUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderPrimaryCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetProviderPrimaryCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderSecondaryCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderSecondaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProviderSecondaryCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProviderSecondaryCommands: usize,
}
windows_core::imp::define_interface!(IStorageProviderStatusUISource, IStorageProviderStatusUISource_Vtbl, 0xa306c249_3d66_5e70_9007_e43df96051ff);
impl core::ops::Deref for IStorageProviderStatusUISource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderStatusUISource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderStatusUISource {
    pub fn GetStatusUI(&self) -> windows_core::Result<StorageProviderStatusUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StatusUIChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IStorageProviderStatusUISource, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusUIChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusUIChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusUIChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for IStorageProviderStatusUISource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderStatusUISource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatusUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StatusUIChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusUIChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderStatusUISourceFactory, IStorageProviderStatusUISourceFactory_Vtbl, 0x12e46b74_4e5a_58d1_a62f_0376e8ee7dd8);
impl core::ops::Deref for IStorageProviderStatusUISourceFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderStatusUISourceFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderStatusUISourceFactory {
    pub fn GetStatusUISource(&self, syncrootid: &windows_core::HSTRING) -> windows_core::Result<IStorageProviderStatusUISource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatusUISource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(syncrootid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IStorageProviderStatusUISourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderStatusUISourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatusUISource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo, IStorageProviderSyncRootInfo_Vtbl, 0x7c1305c4_99f9_41ac_8904_ab055d654926);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IconResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHydrationPolicy) -> windows_core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHydrationPolicy) -> windows_core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHydrationPolicyModifier) -> windows_core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHydrationPolicyModifier) -> windows_core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderPopulationPolicy) -> windows_core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderPopulationPolicy) -> windows_core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderInSyncPolicy) -> windows_core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderInSyncPolicy) -> windows_core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderHardlinkPolicy) -> windows_core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderHardlinkPolicy) -> windows_core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderProtectionMode) -> windows_core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, StorageProviderProtectionMode) -> windows_core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderItemPropertyDefinitions: usize,
    pub RecycleBinUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecycleBinUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo2, IStorageProviderSyncRootInfo2_Vtbl, 0xcf51b023_7cf1_5166_bdba_efd95f529e31);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootInfo3, IStorageProviderSyncRootInfo3_Vtbl, 0x507a6617_bef6_56fd_855e_75ace2e45cf5);
impl windows_core::RuntimeType for IStorageProviderSyncRootInfo3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderSyncRootInfo3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FallbackFileTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FallbackFileTypeInfo: usize,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootManagerStatics, IStorageProviderSyncRootManagerStatics_Vtbl, 0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
impl windows_core::RuntimeType for IStorageProviderSyncRootManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderSyncRootManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentSyncRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentSyncRoots: usize,
}
windows_core::imp::define_interface!(IStorageProviderSyncRootManagerStatics2, IStorageProviderSyncRootManagerStatics2_Vtbl, 0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
impl windows_core::RuntimeType for IStorageProviderSyncRootManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderSyncRootManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderUICommand, IStorageProviderUICommand_Vtbl, 0x0c3e0760_d846_568f_9484_105cc57b502b);
impl core::ops::Deref for IStorageProviderUICommand {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderUICommand, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderUICommand {
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Icon(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<StorageProviderUICommandState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IStorageProviderUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderUICommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StorageProviderUICommandState) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageProviderUriSource, IStorageProviderUriSource_Vtbl, 0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
impl core::ops::Deref for IStorageProviderUriSource {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageProviderUriSource, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageProviderUriSource {
    pub fn GetPathForContentUri<P0>(&self, contenturi: &windows_core::HSTRING, result: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderGetPathForContentUriResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetPathForContentUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contenturi), result.param().abi()).ok() }
    }
    pub fn GetContentInfoForPath<P0>(&self, path: &windows_core::HSTRING, result: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderGetContentInfoForPathResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetContentInfoForPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), result.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageProviderUriSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPathForContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CachedFileUpdater;
impl CachedFileUpdater {
    pub fn SetUpdateInformation<P0>(file: P0, contentid: &windows_core::HSTRING, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetUpdateInformation)(windows_core::Interface::as_raw(this), file.param().abi(), core::mem::transmute_copy(contentid), readmode, writemode, options).ok() })
    }
    #[doc(hidden)]
    pub fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CachedFileUpdaterUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CachedFileUpdaterUI, windows_core::IUnknown, windows_core::IInspectable);
impl CachedFileUpdaterUI {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UpdateTarget(&self) -> windows_core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTarget)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FileUpdateRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileUpdateRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFileUpdateRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileUpdateRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UIRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUIRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUIRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UIStatus(&self) -> windows_core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UIStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateRequest(&self) -> windows_core::Result<FileUpdateRequest> {
        let this = &windows_core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<FileUpdateRequestDeferral> {
        let this = &windows_core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterUI>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
    const IID: windows_core::GUID = <ICachedFileUpdaterUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileUpdateRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequest, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequest {
    pub fn ContentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn File(&self) -> windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).File)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: FileUpdateStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateLocalFile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateLocalFile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UserInputNeededMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInputNeededMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUserInputNeededMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUserInputNeededMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequest>();
}
unsafe impl windows_core::Interface for FileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
    const IID: windows_core::GUID = <IFileUpdateRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileUpdateRequestDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequestDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequestDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequestDeferral>();
}
unsafe impl windows_core::Interface for FileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
    const IID: windows_core::GUID = <IFileUpdateRequestDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileUpdateRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileUpdateRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FileUpdateRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileUpdateRequestedEventArgs>();
}
unsafe impl windows_core::Interface for FileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IFileUpdateRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderFileTypeInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderFileTypeInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderFileTypeInfo {
    pub fn FileExtension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileExtension)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(fileextension: &windows_core::HSTRING, iconresource: &windows_core::HSTRING) -> windows_core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(fileextension), core::mem::transmute_copy(iconresource), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderFileTypeInfo>();
}
unsafe impl windows_core::Interface for StorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderFileTypeInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
unsafe impl Send for StorageProviderFileTypeInfo {}
unsafe impl Sync for StorageProviderFileTypeInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderGetContentInfoForPathResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderGetContentInfoForPathResult, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderGetContentInfoForPathResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContentUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentUri(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderGetContentInfoForPathResult>();
}
unsafe impl windows_core::Interface for StorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderGetContentInfoForPathResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
unsafe impl Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl Sync for StorageProviderGetContentInfoForPathResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderGetPathForContentUriResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderGetPathForContentUriResult, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderGetPathForContentUriResult, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderGetPathForContentUriResult>();
}
unsafe impl windows_core::Interface for StorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderGetPathForContentUriResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
unsafe impl Send for StorageProviderGetPathForContentUriResult {}
unsafe impl Sync for StorageProviderGetPathForContentUriResult {}
pub struct StorageProviderItemProperties;
impl StorageProviderItemProperties {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAsync<P0, P1>(item: P0, itemproperties: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::IStorageItem>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>,
    {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAsync)(windows_core::Interface::as_raw(this), item.param().abi(), itemproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderItemProperty(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderItemProperty, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderItemProperty {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemProperty, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIconResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIconResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderItemProperty>();
}
unsafe impl windows_core::Interface for StorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderItemProperty as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
unsafe impl Send for StorageProviderItemProperty {}
unsafe impl Sync for StorageProviderItemProperty {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderItemPropertyDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderItemPropertyDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderItemPropertyDefinition, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayNameResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayNameResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayNameResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayNameResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderItemPropertyDefinition>();
}
unsafe impl windows_core::Interface for StorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderItemPropertyDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
unsafe impl Send for StorageProviderItemPropertyDefinition {}
unsafe impl Sync for StorageProviderItemPropertyDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderKnownFolderEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderEntry, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderEntry {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderKnownFolderEntry, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KnownFolderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKnownFolderId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKnownFolderId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<StorageProviderKnownFolderSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: StorageProviderKnownFolderSyncStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderEntry>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderEntry {
    type Vtable = IStorageProviderKnownFolderEntry_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderEntry {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderEntry";
}
unsafe impl Send for StorageProviderKnownFolderEntry {}
unsafe impl Sync for StorageProviderKnownFolderEntry {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderKnownFolderSyncInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderSyncInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderSyncInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderKnownFolderSyncInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownFolderEntries(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<StorageProviderKnownFolderEntry>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolderEntries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncRequested(&self) -> windows_core::Result<StorageProviderKnownFolderSyncRequestedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncRequested)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSyncRequested<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderKnownFolderSyncRequestedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSyncRequested)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderSyncInfo>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderSyncInfo {
    type Vtable = IStorageProviderKnownFolderSyncInfo_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderSyncInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderSyncInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderSyncInfo";
}
unsafe impl Send for StorageProviderKnownFolderSyncInfo {}
unsafe impl Sync for StorageProviderKnownFolderSyncInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderKnownFolderSyncRequestArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderKnownFolderSyncRequestArgs, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderKnownFolderSyncRequestArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownFolders(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KnownFolders)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Source(&self) -> windows_core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncRequestArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderKnownFolderSyncRequestArgs>();
}
unsafe impl windows_core::Interface for StorageProviderKnownFolderSyncRequestArgs {
    type Vtable = IStorageProviderKnownFolderSyncRequestArgs_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderKnownFolderSyncRequestArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderKnownFolderSyncRequestArgs {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderKnownFolderSyncRequestArgs";
}
unsafe impl Send for StorageProviderKnownFolderSyncRequestArgs {}
unsafe impl Sync for StorageProviderKnownFolderSyncRequestArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderMoreInfoUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderMoreInfoUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderMoreInfoUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderMoreInfoUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Command(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderMoreInfoUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderMoreInfoUI>();
}
unsafe impl windows_core::Interface for StorageProviderMoreInfoUI {
    type Vtable = IStorageProviderMoreInfoUI_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderMoreInfoUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderMoreInfoUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderMoreInfoUI";
}
unsafe impl Send for StorageProviderMoreInfoUI {}
unsafe impl Sync for StorageProviderMoreInfoUI {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderQuotaUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderQuotaUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderQuotaUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderQuotaUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn QuotaTotalInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaTotalInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQuotaTotalInBytes(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaTotalInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQuotaUsedInBytes(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn QuotaUsedLabel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedLabel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetQuotaUsedLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn QuotaUsedColor(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUsedColor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetQuotaUsedColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::UI::Color>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUsedColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderQuotaUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderQuotaUI>();
}
unsafe impl windows_core::Interface for StorageProviderQuotaUI {
    type Vtable = IStorageProviderQuotaUI_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderQuotaUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderQuotaUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderQuotaUI";
}
unsafe impl Send for StorageProviderQuotaUI {}
unsafe impl Sync for StorageProviderQuotaUI {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderStatusUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderStatusUI, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderStatusUI {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderStatusUI, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderState(&self) -> windows_core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProviderState(&self, value: StorageProviderState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderState)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProviderStateLabel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderStateLabel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderStateLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderStateLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderStateIcon(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderStateIcon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderStateIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderStateIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SyncStatusCommand(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncStatusCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSyncStatusCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSyncStatusCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn QuotaUI(&self) -> windows_core::Result<StorageProviderQuotaUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QuotaUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetQuotaUI<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderQuotaUI>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuotaUI)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MoreInfoUI(&self) -> windows_core::Result<StorageProviderMoreInfoUI> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUI)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMoreInfoUI<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderMoreInfoUI>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMoreInfoUI)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderPrimaryCommand(&self) -> windows_core::Result<IStorageProviderUICommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderPrimaryCommand)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderPrimaryCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStorageProviderUICommand>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderPrimaryCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderSecondaryCommands(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<IStorageProviderUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderSecondaryCommands)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProviderSecondaryCommands<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IVector<IStorageProviderUICommand>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderSecondaryCommands)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StorageProviderStatusUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderStatusUI>();
}
unsafe impl windows_core::Interface for StorageProviderStatusUI {
    type Vtable = IStorageProviderStatusUI_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderStatusUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderStatusUI {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatusUI";
}
unsafe impl Send for StorageProviderStatusUI {}
unsafe impl Sync for StorageProviderStatusUI {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StorageProviderSyncRootInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageProviderSyncRootInfo, windows_core::IUnknown, windows_core::IInspectable);
impl StorageProviderSyncRootInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Context(&self) -> windows_core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Path(&self) -> windows_core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPath<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPath)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DisplayNameResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayNameResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayNameResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayNameResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IconResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIconResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIconResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn HydrationPolicy(&self) -> windows_core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HydrationPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHydrationPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HydrationPolicyModifier(&self) -> windows_core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HydrationPolicyModifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHydrationPolicyModifier)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PopulationPolicy(&self) -> windows_core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PopulationPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPopulationPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InSyncPolicy(&self) -> windows_core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InSyncPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInSyncPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardlinkPolicy(&self) -> windows_core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardlinkPolicy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHardlinkPolicy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowSiblingsAsGroup(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowSiblingsAsGroup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowSiblingsAsGroup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVersion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProtectionMode(&self) -> windows_core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowPinning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowPinning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowPinning(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowPinning)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderItemPropertyDefinitions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StorageProviderItemPropertyDefinitions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RecycleBinUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecycleBinUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecycleBinUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecycleBinUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProviderId(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProviderId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FallbackFileTypeInfo(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &windows_core::Interface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FallbackFileTypeInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageProviderSyncRootInfo>();
}
unsafe impl windows_core::Interface for StorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
    const IID: windows_core::GUID = <IStorageProviderSyncRootInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
unsafe impl Send for StorageProviderSyncRootInfo {}
unsafe impl Sync for StorageProviderSyncRootInfo {}
pub struct StorageProviderSyncRootManager;
impl StorageProviderSyncRootManager {
    pub fn Register<P0>(syncrootinformation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderSyncRootInfo>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), syncrootinformation.param().abi()).ok() })
    }
    pub fn Unregister(id: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id)).ok() })
    }
    pub fn GetSyncRootInformationForFolder<P0>(folder: P0) -> windows_core::Result<StorageProviderSyncRootInfo>
    where
        P0: windows_core::Param<super::IStorageFolder>,
    {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSyncRootInformationForFolder)(windows_core::Interface::as_raw(this), folder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetSyncRootInformationForId(id: &windows_core::HSTRING) -> windows_core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSyncRootInformationForId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentSyncRoots() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentSyncRoots)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl windows_core::TypeKind for CachedFileOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CachedFileOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CachedFileOptions").field(&self.0).finish()
    }
}
impl CachedFileOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CachedFileOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl windows_core::TypeKind for CachedFileTarget {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CachedFileTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CachedFileTarget").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CachedFileTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl windows_core::TypeKind for FileUpdateStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FileUpdateStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FileUpdateStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl windows_core::TypeKind for ReadActivationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ReadActivationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ReadActivationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ReadActivationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl windows_core::TypeKind for StorageProviderHardlinkPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderHardlinkPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderHardlinkPolicy").field(&self.0).finish()
    }
}
impl StorageProviderHardlinkPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl windows_core::TypeKind for StorageProviderHydrationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderHydrationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl windows_core::TypeKind for StorageProviderHydrationPolicyModifier {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderHydrationPolicyModifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicyModifier").field(&self.0).finish()
    }
}
impl StorageProviderHydrationPolicyModifier {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl windows_core::TypeKind for StorageProviderInSyncPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderInSyncPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderInSyncPolicy").field(&self.0).finish()
    }
}
impl StorageProviderInSyncPolicy {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderKnownFolderSyncStatus(pub i32);
impl StorageProviderKnownFolderSyncStatus {
    pub const Available: Self = Self(0i32);
    pub const Enrolling: Self = Self(1i32);
    pub const Enrolled: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderKnownFolderSyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderKnownFolderSyncStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderKnownFolderSyncStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderKnownFolderSyncStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderPopulationPolicy {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderPopulationPolicy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderPopulationPolicy").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl windows_core::TypeKind for StorageProviderProtectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderProtectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderProtectionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl windows_core::TypeKind for StorageProviderState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderUICommandState(pub i32);
impl StorageProviderUICommandState {
    pub const Enabled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderUICommandState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderUICommandState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderUICommandState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderUICommandState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUICommandState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl windows_core::TypeKind for StorageProviderUriSourceStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StorageProviderUriSourceStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StorageProviderUriSourceStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl windows_core::TypeKind for UIStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UIStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UIStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UIStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl windows_core::TypeKind for WriteActivationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WriteActivationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WriteActivationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WriteActivationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
}
windows_core::imp::define_interface!(StorageProviderKnownFolderSyncRequestedHandler, StorageProviderKnownFolderSyncRequestedHandler_Vtbl, 0xc4cbb4f5_13dd_5c8e_8b96_336fc30c629b);
impl StorageProviderKnownFolderSyncRequestedHandler {
    pub fn new<F: FnMut(Option<&StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = StorageProviderKnownFolderSyncRequestedHandlerBox::<F> { vtable: &StorageProviderKnownFolderSyncRequestedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<StorageProviderKnownFolderSyncRequestArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct StorageProviderKnownFolderSyncRequestedHandlerBox<F: FnMut(Option<&StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const StorageProviderKnownFolderSyncRequestedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&StorageProviderKnownFolderSyncRequestArgs>) -> windows_core::Result<()> + Send + 'static> StorageProviderKnownFolderSyncRequestedHandlerBox<F> {
    const VTABLE: StorageProviderKnownFolderSyncRequestedHandler_Vtbl = StorageProviderKnownFolderSyncRequestedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <StorageProviderKnownFolderSyncRequestedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for StorageProviderKnownFolderSyncRequestedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct StorageProviderKnownFolderSyncRequestedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
