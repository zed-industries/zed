windows_core::imp::define_interface!(IFileOpenPickerUI, IFileOpenPickerUI_Vtbl, 0xdda45a10_f9d4_40c4_8af5_c5b6b5a61d1d);
impl windows_core::RuntimeType for IFileOpenPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileOpenPickerUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut AddFileResult) -> windows_core::HRESULT,
    pub RemoveFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContainsFile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub CanAddFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SelectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileSelectionMode) -> windows_core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub FileRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FileRemoved: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveFileRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveFileRemoved: usize,
    pub Closing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IFileRemovedEventArgs, IFileRemovedEventArgs_Vtbl, 0x13043da7_7fca_4c2b_9eca_6890f9f00185);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IFileRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IFileRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
}
windows_core::imp::define_interface!(IFileSavePickerUI, IFileSavePickerUI_Vtbl, 0x9656c1e7_3e56_43cc_8a39_33c73d9d542b);
impl windows_core::RuntimeType for IFileSavePickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFileSavePickerUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllowedFileTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllowedFileTypes: usize,
    pub SettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TrySetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut SetFileNameResult) -> windows_core::HRESULT,
    pub FileNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFileNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TargetFileRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTargetFileRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPickerClosingDeferral, IPickerClosingDeferral_Vtbl, 0x7af7f71e_1a67_4a31_ae80_e907708a619b);
impl windows_core::RuntimeType for IPickerClosingDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPickerClosingDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPickerClosingEventArgs, IPickerClosingEventArgs_Vtbl, 0x7e59f224_b332_4f12_8b9f_a8c2f06b32cd);
impl windows_core::RuntimeType for IPickerClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPickerClosingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ClosingOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPickerClosingOperation, IPickerClosingOperation_Vtbl, 0x4ce9fb84_beee_4e39_a773_fc5f0eae328d);
impl windows_core::RuntimeType for IPickerClosingOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPickerClosingOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFileRequest, ITargetFileRequest_Vtbl, 0x42bd3355_7f88_478b_8e81_690b20340678);
impl windows_core::RuntimeType for ITargetFileRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetFileRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFileRequestDeferral, ITargetFileRequestDeferral_Vtbl, 0x4aee9d91_bf15_4da9_95f6_f6b7d558225b);
impl windows_core::RuntimeType for ITargetFileRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetFileRequestDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFileRequestedEventArgs, ITargetFileRequestedEventArgs_Vtbl, 0xb163dbc1_1b51_4c89_a591_0fd40b3c57c9);
impl windows_core::RuntimeType for ITargetFileRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITargetFileRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileOpenPickerUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileOpenPickerUI, windows_core::IUnknown, windows_core::IInspectable);
impl FileOpenPickerUI {
    pub fn AddFile<P0>(&self, id: &windows_core::HSTRING, file: P0) -> windows_core::Result<AddFileResult>
    where
        P0: windows_core::Param<super::super::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddFile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), file.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFile(&self, id: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsFile(&self, id: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainsFile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).map(|| result__)
        }
    }
    pub fn CanAddFile<P0>(&self, file: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanAddFile)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedFileTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionMode(&self) -> windows_core::Result<FileSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SettingsIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsIdentifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
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
    #[cfg(feature = "deprecated")]
    pub fn FileRemoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveFileRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closing<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosing(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosing)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for FileOpenPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileOpenPickerUI>();
}
unsafe impl windows_core::Interface for FileOpenPickerUI {
    type Vtable = IFileOpenPickerUI_Vtbl;
    const IID: windows_core::GUID = <IFileOpenPickerUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOpenPickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileOpenPickerUI";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileRemovedEventArgs(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(FileRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl FileRemovedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for FileRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileRemovedEventArgs>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for FileRemovedEventArgs {
    type Vtable = IFileRemovedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IFileRemovedEventArgs as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for FileRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileRemovedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileSavePickerUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileSavePickerUI, windows_core::IUnknown, windows_core::IInspectable);
impl FileSavePickerUI {
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
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllowedFileTypes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedFileTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SettingsIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsIdentifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySetFileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<SetFileNameResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn FileNameChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileNameChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFileNameChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFileNameChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TargetFileRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetFileRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTargetFileRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTargetFileRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for FileSavePickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileSavePickerUI>();
}
unsafe impl windows_core::Interface for FileSavePickerUI {
    type Vtable = IFileSavePickerUI_Vtbl;
    const IID: windows_core::GUID = <IFileSavePickerUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileSavePickerUI {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.FileSavePickerUI";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PickerClosingDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PickerClosingDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl PickerClosingDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for PickerClosingDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPickerClosingDeferral>();
}
unsafe impl windows_core::Interface for PickerClosingDeferral {
    type Vtable = IPickerClosingDeferral_Vtbl;
    const IID: windows_core::GUID = <IPickerClosingDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PickerClosingDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingDeferral";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PickerClosingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PickerClosingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PickerClosingEventArgs {
    pub fn ClosingOperation(&self) -> windows_core::Result<PickerClosingOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClosingOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCanceled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PickerClosingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPickerClosingEventArgs>();
}
unsafe impl windows_core::Interface for PickerClosingEventArgs {
    type Vtable = IPickerClosingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPickerClosingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PickerClosingEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PickerClosingOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PickerClosingOperation, windows_core::IUnknown, windows_core::IInspectable);
impl PickerClosingOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<PickerClosingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PickerClosingOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPickerClosingOperation>();
}
unsafe impl windows_core::Interface for PickerClosingOperation {
    type Vtable = IPickerClosingOperation_Vtbl;
    const IID: windows_core::GUID = <IPickerClosingOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PickerClosingOperation {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.PickerClosingOperation";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetFileRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetFileRequest, windows_core::IUnknown, windows_core::IInspectable);
impl TargetFileRequest {
    pub fn TargetFile(&self) -> windows_core::Result<super::super::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTargetFile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::IStorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetFile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<TargetFileRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetFileRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetFileRequest>();
}
unsafe impl windows_core::Interface for TargetFileRequest {
    type Vtable = ITargetFileRequest_Vtbl;
    const IID: windows_core::GUID = <ITargetFileRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetFileRequest {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequest";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetFileRequestDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetFileRequestDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl TargetFileRequestDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for TargetFileRequestDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetFileRequestDeferral>();
}
unsafe impl windows_core::Interface for TargetFileRequestDeferral {
    type Vtable = ITargetFileRequestDeferral_Vtbl;
    const IID: windows_core::GUID = <ITargetFileRequestDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetFileRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestDeferral";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TargetFileRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TargetFileRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TargetFileRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<TargetFileRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TargetFileRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITargetFileRequestedEventArgs>();
}
unsafe impl windows_core::Interface for TargetFileRequestedEventArgs {
    type Vtable = ITargetFileRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITargetFileRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TargetFileRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Pickers.Provider.TargetFileRequestedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AddFileResult(pub i32);
impl AddFileResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const NotAllowed: Self = Self(2i32);
    pub const Unavailable: Self = Self(3i32);
}
impl windows_core::TypeKind for AddFileResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AddFileResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AddFileResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AddFileResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.AddFileResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FileSelectionMode(pub i32);
impl FileSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl windows_core::TypeKind for FileSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FileSelectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FileSelectionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FileSelectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.FileSelectionMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SetFileNameResult(pub i32);
impl SetFileNameResult {
    pub const Succeeded: Self = Self(0i32);
    pub const NotAllowed: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl windows_core::TypeKind for SetFileNameResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SetFileNameResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SetFileNameResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SetFileNameResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.Provider.SetFileNameResult;i4)");
}
