#[cfg(feature = "Storage_Pickers_Provider")]
pub mod Provider;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileExtensionVector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileExtensionVector, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVector<windows_core::HSTRING>);
windows_core::imp::required_hierarchy!(FileExtensionVector, windows_collections::IIterable<windows_core::HSTRING>);
impl FileExtensionVector {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf(&self, value: &windows_core::HSTRING, index: &mut u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt(&self, index: u32, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, core::mem::transmute_copy(value)).ok() }
    }
    pub fn InsertAt(&self, index: u32, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [windows_core::HSTRING]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[windows_core::HSTRING]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for FileExtensionVector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<windows_core::HSTRING>>();
}
unsafe impl windows_core::Interface for FileExtensionVector {
    type Vtable = <windows_collections::IVector<windows_core::HSTRING> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVector<windows_core::HSTRING> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileExtensionVector {
    const NAME: &'static str = "Windows.Storage.Pickers.FileExtensionVector";
}
unsafe impl Send for FileExtensionVector {}
unsafe impl Sync for FileExtensionVector {}
impl IntoIterator for FileExtensionVector {
    type Item = windows_core::HSTRING;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &FileExtensionVector {
    type Item = windows_core::HSTRING;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOpenPicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileOpenPicker, windows_core::IUnknown, windows_core::IInspectable);
impl FileOpenPicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileOpenPicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> windows_core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsIdentifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSettingsIdentifier)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedStartLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuggestedStartLocation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommitButtonText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCommitButtonText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommitButtonText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileTypeFilter(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileTypeFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PickSingleFileAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PickMultipleFilesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickMultipleFilesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PickSingleFileAndContinue(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PickSingleFileAndContinue)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PickMultipleFilesAndContinue(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PickMultipleFilesAndContinue)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IFileOpenPicker3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ResumePickSingleFileAsync() -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        Self::IFileOpenPickerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResumePickSingleFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> windows_core::Result<FileOpenPicker>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileOpenPickerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PickSingleFileAsync2(&self, pickeroperationid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<IFileOpenPickerWithOperationId>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleFileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(pickeroperationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    fn IFileOpenPickerStatics<R, F: FnOnce(&IFileOpenPickerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileOpenPicker, IFileOpenPickerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFileOpenPickerStatics2<R, F: FnOnce(&IFileOpenPickerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileOpenPicker, IFileOpenPickerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileOpenPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileOpenPicker>();
}
unsafe impl windows_core::Interface for FileOpenPicker {
    type Vtable = <IFileOpenPicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileOpenPicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileOpenPicker";
}
unsafe impl Send for FileOpenPicker {}
unsafe impl Sync for FileOpenPicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilePickerFileTypesOrderedMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy ! ( FilePickerFileTypesOrderedMap , windows_core::IUnknown , windows_core::IInspectable , windows_collections:: IMap < windows_core::HSTRING , windows_collections:: IVector < windows_core::HSTRING > > );
windows_core::imp::required_hierarchy!(FilePickerFileTypesOrderedMap, windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>>);
impl FilePickerFileTypesOrderedMap {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows_collections::IVector<windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for FilePickerFileTypesOrderedMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IMap<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>>();
}
unsafe impl windows_core::Interface for FilePickerFileTypesOrderedMap {
    type Vtable = <windows_collections::IMap<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IMap<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FilePickerFileTypesOrderedMap {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerFileTypesOrderedMap";
}
unsafe impl Send for FilePickerFileTypesOrderedMap {}
unsafe impl Sync for FilePickerFileTypesOrderedMap {}
impl IntoIterator for FilePickerFileTypesOrderedMap {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &FilePickerFileTypesOrderedMap {
    type Item = windows_collections::IKeyValuePair<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilePickerSelectedFilesArray(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(FilePickerSelectedFilesArray, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVectorView<super::StorageFile>);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(FilePickerSelectedFilesArray, windows_collections::IIterable<super::StorageFile>);
#[cfg(feature = "Storage_Streams")]
impl FilePickerSelectedFilesArray {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<super::StorageFile>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<super::StorageFile>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<super::StorageFile>]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for FilePickerSelectedFilesArray {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVectorView<super::StorageFile>>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for FilePickerSelectedFilesArray {
    type Vtable = <windows_collections::IVectorView<super::StorageFile> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVectorView<super::StorageFile> as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for FilePickerSelectedFilesArray {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerSelectedFilesArray";
}
#[cfg(feature = "Storage_Streams")]
unsafe impl Send for FilePickerSelectedFilesArray {}
#[cfg(feature = "Storage_Streams")]
unsafe impl Sync for FilePickerSelectedFilesArray {}
#[cfg(feature = "Storage_Streams")]
impl IntoIterator for FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl IntoIterator for &FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileSavePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileSavePicker, windows_core::IUnknown, windows_core::IInspectable);
impl FileSavePicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileSavePicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SettingsIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsIdentifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSettingsIdentifier)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedStartLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuggestedStartLocation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommitButtonText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCommitButtonText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommitButtonText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileTypeChoices(&self) -> windows_core::Result<windows_collections::IMap<windows_core::HSTRING, windows_collections::IVector<windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileTypeChoices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultFileExtension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultFileExtension)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDefaultFileExtension(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultFileExtension)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SuggestedSaveFile(&self) -> windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedSaveFile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSuggestedSaveFile<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::StorageFile>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuggestedSaveFile)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SuggestedFileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedFileName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSuggestedFileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuggestedFileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PickSaveFileAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSaveFileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IFileSavePicker2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PickSaveFileAndContinue(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileSavePicker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PickSaveFileAndContinue)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EnterpriseId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IFileSavePicker3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterpriseId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetEnterpriseId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFileSavePicker3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetEnterpriseId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IFileSavePicker4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> windows_core::Result<FileSavePicker>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileSavePickerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IFileSavePickerStatics<R, F: FnOnce(&IFileSavePickerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileSavePicker, IFileSavePickerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileSavePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileSavePicker>();
}
unsafe impl windows_core::Interface for FileSavePicker {
    type Vtable = <IFileSavePicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileSavePicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileSavePicker";
}
unsafe impl Send for FileSavePicker {}
unsafe impl Sync for FileSavePicker {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FolderPicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FolderPicker, windows_core::IUnknown, windows_core::IInspectable);
impl FolderPicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FolderPicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> windows_core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SettingsIdentifier)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSettingsIdentifier)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedStartLocation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSuggestedStartLocation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommitButtonText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCommitButtonText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommitButtonText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FileTypeFilter(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileTypeFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn PickSingleFolderAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleFolderAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IFolderPicker2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContinuationData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PickFolderAndContinue(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFolderPicker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PickFolderAndContinue)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IFolderPicker3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> windows_core::Result<FolderPicker>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFolderPickerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IFolderPickerStatics<R, F: FnOnce(&IFolderPickerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FolderPicker, IFolderPickerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FolderPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFolderPicker>();
}
unsafe impl windows_core::Interface for FolderPicker {
    type Vtable = <IFolderPicker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFolderPicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FolderPicker";
}
unsafe impl Send for FolderPicker {}
unsafe impl Sync for FolderPicker {}
windows_core::imp::define_interface!(IFileOpenPicker, IFileOpenPicker_Vtbl, 0x2ca8278a_12c5_4c5f_8977_94547793c241);
impl windows_core::RuntimeType for IFileOpenPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PickerViewMode) -> windows_core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, PickerViewMode) -> windows_core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PickerLocationId) -> windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, PickerLocationId) -> windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileTypeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PickSingleFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PickSingleFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PickMultipleFilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PickMultipleFilesAsync: usize,
}
windows_core::imp::define_interface!(IFileOpenPicker2, IFileOpenPicker2_Vtbl, 0x8ceb6cd2_b446_46f7_b265_90f8e55ad650);
impl windows_core::RuntimeType for IFileOpenPicker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    pub PickSingleFileAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PickMultipleFilesAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileOpenPicker3, IFileOpenPicker3_Vtbl, 0xd9a5c5b3_c5dc_5b98_bd80_a8d0ca0584d8);
impl windows_core::RuntimeType for IFileOpenPicker3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IFileOpenPickerStatics, IFileOpenPickerStatics_Vtbl, 0x6821573b_2f02_4833_96d4_abbfad72b67b);
impl windows_core::RuntimeType for IFileOpenPickerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ResumePickSingleFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResumePickSingleFileAsync: usize,
}
windows_core::imp::define_interface!(IFileOpenPickerStatics2, IFileOpenPickerStatics2_Vtbl, 0xe8917415_eddd_5c98_b6f3_366fdfcad392);
impl windows_core::RuntimeType for IFileOpenPickerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
windows_core::imp::define_interface!(IFileOpenPickerWithOperationId, IFileOpenPickerWithOperationId_Vtbl, 0x3f57b569_2522_4ca5_aa73_a15509f1fcbf);
impl windows_core::RuntimeType for IFileOpenPickerWithOperationId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerWithOperationId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub PickSingleFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PickSingleFileAsync: usize,
}
windows_core::imp::define_interface!(IFileSavePicker, IFileSavePicker_Vtbl, 0x3286ffcb_617f_4cc5_af6a_b3fdf29ad145);
impl windows_core::RuntimeType for IFileSavePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PickerLocationId) -> windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, PickerLocationId) -> windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileTypeChoices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultFileExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultFileExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SuggestedSaveFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SuggestedSaveFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSuggestedSaveFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSuggestedSaveFile: usize,
    pub SuggestedFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSuggestedFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PickSaveFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PickSaveFileAsync: usize,
}
windows_core::imp::define_interface!(IFileSavePicker2, IFileSavePicker2_Vtbl, 0x0ec313a2_d24b_449a_8197_e89104fd42cc);
impl windows_core::RuntimeType for IFileSavePicker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    pub PickSaveFileAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileSavePicker3, IFileSavePicker3_Vtbl, 0x698aec69_ba3c_4e51_bd90_4abcbbf4cfaf);
impl windows_core::RuntimeType for IFileSavePicker3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileSavePicker4, IFileSavePicker4_Vtbl, 0xe7d83a5a_ddfa_5de0_8b70_c842c21988ec);
impl windows_core::RuntimeType for IFileSavePicker4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IFileSavePickerStatics, IFileSavePickerStatics_Vtbl, 0x28e3cf9e_961c_5e2c_aed7_e64737f4ce37);
impl windows_core::RuntimeType for IFileSavePickerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
windows_core::imp::define_interface!(IFolderPicker, IFolderPicker_Vtbl, 0x084f7799_f3fb_400a_99b1_7b4a772fd60d);
impl windows_core::RuntimeType for IFolderPicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PickerViewMode) -> windows_core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, PickerViewMode) -> windows_core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PickerLocationId) -> windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(*mut core::ffi::c_void, PickerLocationId) -> windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileTypeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub PickSingleFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    PickSingleFolderAsync: usize,
}
windows_core::imp::define_interface!(IFolderPicker2, IFolderPicker2_Vtbl, 0x8eb3ba97_dc85_4616_be94_9660881f2f5d);
impl windows_core::RuntimeType for IFolderPicker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    pub PickFolderAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFolderPicker3, IFolderPicker3_Vtbl, 0x673b1e29_d326_53c0_bd24_a25c714cee36);
impl windows_core::RuntimeType for IFolderPicker3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IFolderPickerStatics, IFolderPickerStatics_Vtbl, 0x9be34740_7ca1_5942_a3c8_46f2551ecff3);
impl windows_core::RuntimeType for IFolderPickerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PickerLocationId(pub i32);
impl PickerLocationId {
    pub const DocumentsLibrary: Self = Self(0i32);
    pub const ComputerFolder: Self = Self(1i32);
    pub const Desktop: Self = Self(2i32);
    pub const Downloads: Self = Self(3i32);
    pub const HomeGroup: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const PicturesLibrary: Self = Self(6i32);
    pub const VideosLibrary: Self = Self(7i32);
    pub const Objects3D: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl windows_core::TypeKind for PickerLocationId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PickerLocationId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerLocationId;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PickerViewMode(pub i32);
impl PickerViewMode {
    pub const List: Self = Self(0i32);
    pub const Thumbnail: Self = Self(1i32);
}
impl windows_core::TypeKind for PickerViewMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PickerViewMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerViewMode;i4)");
}
