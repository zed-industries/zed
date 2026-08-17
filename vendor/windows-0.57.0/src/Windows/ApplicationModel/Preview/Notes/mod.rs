windows_core::imp::define_interface!(INotePlacementChangedPreviewEventArgs, INotePlacementChangedPreviewEventArgs_Vtbl, 0x491d57b7_f780_4e7f_a939_9a4caf965214);
impl windows_core::RuntimeType for INotePlacementChangedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotePlacementChangedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INoteVisibilityChangedPreviewEventArgs, INoteVisibilityChangedPreviewEventArgs_Vtbl, 0x0e34649e_3815_4ff6_83b3_a14d17120e24);
impl windows_core::RuntimeType for INoteVisibilityChangedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INoteVisibilityChangedPreviewEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotesWindowManagerPreview, INotesWindowManagerPreview_Vtbl, 0xdc2ac23e_4850_4f13_9cc7_ff487efdfcde);
impl windows_core::RuntimeType for INotesWindowManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotesWindowManagerPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsScreenLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShowNote: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ShowNoteRelativeTo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacement: usize,
    pub HideNote: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetNotePlacement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetNotePlacement: usize,
    pub TrySetNoteSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::super::Foundation::Size, *mut bool) -> windows_core::HRESULT,
    pub SetFocusToNextView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetNotesThumbnailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetNotesThumbnailAsync: usize,
    pub SystemLockStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSystemLockStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NotePlacementChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNotePlacementChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub NoteVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNoteVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotesWindowManagerPreview2, INotesWindowManagerPreview2_Vtbl, 0xedfe864a_1f54_4b09_9823_ff477f6fa3bc);
impl windows_core::RuntimeType for INotesWindowManagerPreview2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotesWindowManagerPreview2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowNoteRelativeToWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ShowNoteWithPlacementWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ShowNoteWithPlacementWithOptions: usize,
    pub SetFocusToPreviousView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetThumbnailImageForTaskSwitcherAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetThumbnailImageForTaskSwitcherAsync: usize,
}
windows_core::imp::define_interface!(INotesWindowManagerPreviewShowNoteOptions, INotesWindowManagerPreviewShowNoteOptions_Vtbl, 0x886b09d6_a6ae_4007_a56d_1ca70c84c0d2);
impl windows_core::RuntimeType for INotesWindowManagerPreviewShowNoteOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotesWindowManagerPreviewShowNoteOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowWithFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowWithFocus: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotesWindowManagerPreviewStatics, INotesWindowManagerPreviewStatics_Vtbl, 0x6668cc88_0a8e_4127_a38e_995445868a78);
impl windows_core::RuntimeType for INotesWindowManagerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotesWindowManagerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotePlacementChangedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotePlacementChangedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NotePlacementChangedPreviewEventArgs {
    pub fn ViewId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NotePlacementChangedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotePlacementChangedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for NotePlacementChangedPreviewEventArgs {
    type Vtable = INotePlacementChangedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <INotePlacementChangedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotePlacementChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotePlacementChangedPreviewEventArgs";
}
unsafe impl Send for NotePlacementChangedPreviewEventArgs {}
unsafe impl Sync for NotePlacementChangedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NoteVisibilityChangedPreviewEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NoteVisibilityChangedPreviewEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NoteVisibilityChangedPreviewEventArgs {
    pub fn ViewId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NoteVisibilityChangedPreviewEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INoteVisibilityChangedPreviewEventArgs>();
}
unsafe impl windows_core::Interface for NoteVisibilityChangedPreviewEventArgs {
    type Vtable = INoteVisibilityChangedPreviewEventArgs_Vtbl;
    const IID: windows_core::GUID = <INoteVisibilityChangedPreviewEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NoteVisibilityChangedPreviewEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NoteVisibilityChangedPreviewEventArgs";
}
unsafe impl Send for NoteVisibilityChangedPreviewEventArgs {}
unsafe impl Sync for NoteVisibilityChangedPreviewEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotesWindowManagerPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotesWindowManagerPreview, windows_core::IUnknown, windows_core::IInspectable);
impl NotesWindowManagerPreview {
    pub fn IsScreenLocked(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScreenLocked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShowNote(&self, noteviewid: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowNote)(windows_core::Interface::as_raw(this), noteviewid).ok() }
    }
    pub fn ShowNoteRelativeTo(&self, noteviewid: i32, anchornoteviewid: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowNoteRelativeTo)(windows_core::Interface::as_raw(this), noteviewid, anchornoteviewid).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacement<P0>(&self, noteviewid: i32, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ShowNoteWithPlacement)(windows_core::Interface::as_raw(this), noteviewid, data.param().abi()).ok() }
    }
    pub fn HideNote(&self, noteviewid: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).HideNote)(windows_core::Interface::as_raw(this), noteviewid).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetNotePlacement(&self, noteviewid: i32) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNotePlacement)(windows_core::Interface::as_raw(this), noteviewid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySetNoteSize(&self, noteviewid: i32, size: super::super::super::Foundation::Size) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetNoteSize)(windows_core::Interface::as_raw(this), noteviewid, size, &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusToNextView(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFocusToNextView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetNotesThumbnailAsync<P0>(&self, thumbnail: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetNotesThumbnailAsync)(windows_core::Interface::as_raw(this), thumbnail.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemLockStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemLockStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSystemLockStateChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSystemLockStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotePlacementChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NotePlacementChangedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotePlacementChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNotePlacementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNotePlacementChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NoteVisibilityChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<NotesWindowManagerPreview, NoteVisibilityChangedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoteVisibilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNoteVisibilityChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveNoteVisibilityChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ShowNoteRelativeToWithOptions<P0>(&self, noteviewid: i32, anchornoteviewid: i32, options: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<NotesWindowManagerPreviewShowNoteOptions>,
    {
        let this = &windows_core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ShowNoteRelativeToWithOptions)(windows_core::Interface::as_raw(this), noteviewid, anchornoteviewid, options.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ShowNoteWithPlacementWithOptions<P0, P1>(&self, noteviewid: i32, data: P0, options: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<NotesWindowManagerPreviewShowNoteOptions>,
    {
        let this = &windows_core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ShowNoteWithPlacementWithOptions)(windows_core::Interface::as_raw(this), noteviewid, data.param().abi(), options.param().abi()).ok() }
    }
    pub fn SetFocusToPreviousView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusToPreviousView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetThumbnailImageForTaskSwitcherAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = &windows_core::Interface::cast::<INotesWindowManagerPreview2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetThumbnailImageForTaskSwitcherAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentApp() -> windows_core::Result<NotesWindowManagerPreview> {
        Self::INotesWindowManagerPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentApp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INotesWindowManagerPreviewStatics<R, F: FnOnce(&INotesWindowManagerPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NotesWindowManagerPreview, INotesWindowManagerPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NotesWindowManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotesWindowManagerPreview>();
}
unsafe impl windows_core::Interface for NotesWindowManagerPreview {
    type Vtable = INotesWindowManagerPreview_Vtbl;
    const IID: windows_core::GUID = <INotesWindowManagerPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotesWindowManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreview";
}
unsafe impl Send for NotesWindowManagerPreview {}
unsafe impl Sync for NotesWindowManagerPreview {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotesWindowManagerPreviewShowNoteOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotesWindowManagerPreviewShowNoteOptions, windows_core::IUnknown, windows_core::IInspectable);
impl NotesWindowManagerPreviewShowNoteOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NotesWindowManagerPreviewShowNoteOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ShowWithFocus(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowWithFocus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowWithFocus(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowWithFocus)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for NotesWindowManagerPreviewShowNoteOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotesWindowManagerPreviewShowNoteOptions>();
}
unsafe impl windows_core::Interface for NotesWindowManagerPreviewShowNoteOptions {
    type Vtable = INotesWindowManagerPreviewShowNoteOptions_Vtbl;
    const IID: windows_core::GUID = <INotesWindowManagerPreviewShowNoteOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotesWindowManagerPreviewShowNoteOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Notes.NotesWindowManagerPreviewShowNoteOptions";
}
unsafe impl Send for NotesWindowManagerPreviewShowNoteOptions {}
unsafe impl Sync for NotesWindowManagerPreviewShowNoteOptions {}
