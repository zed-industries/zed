pub struct GameList;
impl GameList {
    pub fn FindAllAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllAsyncPackageFamilyName(packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<GameListEntry>>> {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsyncPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GameAdded<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<GameListChangedEventHandler>,
    {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameAdded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveGameAdded(token: i64) -> windows_core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveGameAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn GameRemoved<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<GameListRemovedEventHandler>,
    {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameRemoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveGameRemoved(token: i64) -> windows_core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveGameRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn GameUpdated<P0>(handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<GameListChangedEventHandler>,
    {
        Self::IGameListStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveGameUpdated(token: i64) -> windows_core::Result<()> {
        Self::IGameListStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveGameUpdated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn MergeEntriesAsync<P0, P1>(left: P0, right: P1) -> windows_core::Result<windows_future::IAsyncOperation<GameListEntry>>
    where
        P0: windows_core::Param<GameListEntry>,
        P1: windows_core::Param<GameListEntry>,
    {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MergeEntriesAsync)(windows_core::Interface::as_raw(this), left.param().abi(), right.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn UnmergeEntryAsync<P0>(mergedentry: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<GameListEntry>>>
    where
        P0: windows_core::Param<GameListEntry>,
    {
        Self::IGameListStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnmergeEntryAsync)(windows_core::Interface::as_raw(this), mergedentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGameListStatics<R, F: FnOnce(&IGameListStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameList, IGameListStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IGameListStatics2<R, F: FnOnce(&IGameListStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameList, IGameListStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameList {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameList";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameListCategory(pub i32);
impl GameListCategory {
    pub const Candidate: Self = Self(0i32);
    pub const ConfirmedBySystem: Self = Self(1i32);
    pub const ConfirmedByUser: Self = Self(2i32);
}
impl windows_core::TypeKind for GameListCategory {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameListCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListCategory;i4)");
}
windows_core::imp::define_interface!(GameListChangedEventHandler, GameListChangedEventHandler_Vtbl, 0x25f6a421_d8f5_4d91_b40e_53d5e86fde64);
impl windows_core::RuntimeType for GameListChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl GameListChangedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<GameListEntry>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = GameListChangedEventHandlerBox { vtable: &GameListChangedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, game: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<GameListEntry>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), game.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, game: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct GameListChangedEventHandlerBox<F: Fn(windows_core::Ref<GameListEntry>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const GameListChangedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<GameListEntry>) -> windows_core::Result<()> + Send + 'static> GameListChangedEventHandlerBox<F> {
    const VTABLE: GameListChangedEventHandler_Vtbl = GameListChangedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <GameListChangedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, game: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&game)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameListEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameListEntry, windows_core::IUnknown, windows_core::IInspectable, IGameListEntry);
impl GameListEntry {
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetCategoryAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchableState(&self) -> windows_core::Result<GameListEntryLaunchableState> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchableState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LauncherExecutable(&self) -> windows_core::Result<super::super::super::Storage::IStorageFile> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LauncherExecutable)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchParameters(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchParameters)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLauncherExecutableFileAsync<P0>(&self, executablefile: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetLauncherExecutableFileAsync)(windows_core::Interface::as_raw(this), executablefile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetLauncherExecutableFileWithParamsAsync<P0>(&self, executablefile: P0, launchparams: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetLauncherExecutableFileWithParamsAsync)(windows_core::Interface::as_raw(this), executablefile.param().abi(), core::mem::transmute_copy(launchparams), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TitleId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitleIdAsync(&self, id: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTitleIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GameModeConfiguration(&self) -> windows_core::Result<GameModeConfiguration> {
        let this = &windows_core::Interface::cast::<IGameListEntry2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameModeConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameListEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameListEntry>();
}
unsafe impl windows_core::Interface for GameListEntry {
    type Vtable = <IGameListEntry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameListEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameListEntry";
}
unsafe impl Send for GameListEntry {}
unsafe impl Sync for GameListEntry {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameListEntryLaunchableState(pub i32);
impl GameListEntryLaunchableState {
    pub const NotLaunchable: Self = Self(0i32);
    pub const ByLastRunningFullPath: Self = Self(1i32);
    pub const ByUserProvidedPath: Self = Self(2i32);
    pub const ByTile: Self = Self(3i32);
}
impl windows_core::TypeKind for GameListEntryLaunchableState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameListEntryLaunchableState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Preview.GamesEnumeration.GameListEntryLaunchableState;i4)");
}
windows_core::imp::define_interface!(GameListRemovedEventHandler, GameListRemovedEventHandler_Vtbl, 0x10c5648f_6c8f_4712_9b38_474bc22e76d8);
impl windows_core::RuntimeType for GameListRemovedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl GameListRemovedEventHandler {
    pub fn new<F: Fn(&windows_core::HSTRING) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = GameListRemovedEventHandlerBox { vtable: &GameListRemovedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke(&self, identifier: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(identifier)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GameListRemovedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, identifier: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct GameListRemovedEventHandlerBox<F: Fn(&windows_core::HSTRING) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const GameListRemovedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(&windows_core::HSTRING) -> windows_core::Result<()> + Send + 'static> GameListRemovedEventHandlerBox<F> {
    const VTABLE: GameListRemovedEventHandler_Vtbl = GameListRemovedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <GameListRemovedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, identifier: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute(&identifier)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameModeConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameModeConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl GameModeConfiguration {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RelatedProcessNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelatedProcessNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PercentGpuTimeAllocatedToGame(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentGpuTimeAllocatedToGame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPercentGpuTimeAllocatedToGame<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPercentGpuTimeAllocatedToGame)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PercentGpuMemoryAllocatedToGame(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentGpuMemoryAllocatedToGame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPercentGpuMemoryAllocatedToGame<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPercentGpuMemoryAllocatedToGame)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PercentGpuMemoryAllocatedToSystemCompositor(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentGpuMemoryAllocatedToSystemCompositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPercentGpuMemoryAllocatedToSystemCompositor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPercentGpuMemoryAllocatedToSystemCompositor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MaxCpuCount(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxCpuCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaxCpuCount<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxCpuCount)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CpuExclusivityMaskLow(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CpuExclusivityMaskLow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCpuExclusivityMaskLow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCpuExclusivityMaskLow)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CpuExclusivityMaskHigh(&self) -> windows_core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CpuExclusivityMaskHigh)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCpuExclusivityMaskHigh<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCpuExclusivityMaskHigh)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AffinitizeToExclusiveCpus(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AffinitizeToExclusiveCpus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAffinitizeToExclusiveCpus(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAffinitizeToExclusiveCpus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameModeConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameModeConfiguration>();
}
unsafe impl windows_core::Interface for GameModeConfiguration {
    type Vtable = <IGameModeConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameModeConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameModeConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeConfiguration";
}
unsafe impl Send for GameModeConfiguration {}
unsafe impl Sync for GameModeConfiguration {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameModeUserConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameModeUserConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl GameModeUserConfiguration {
    pub fn GamingRelatedProcessNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GamingRelatedProcessNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<GameModeUserConfiguration> {
        Self::IGameModeUserConfigurationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGameModeUserConfigurationStatics<R, F: FnOnce(&IGameModeUserConfigurationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameModeUserConfiguration, IGameModeUserConfigurationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GameModeUserConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameModeUserConfiguration>();
}
unsafe impl windows_core::Interface for GameModeUserConfiguration {
    type Vtable = <IGameModeUserConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameModeUserConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameModeUserConfiguration {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.GameModeUserConfiguration";
}
unsafe impl Send for GameModeUserConfiguration {}
unsafe impl Sync for GameModeUserConfiguration {}
windows_core::imp::define_interface!(IGameListEntry, IGameListEntry_Vtbl, 0x735924d3_811f_4494_b69c_c641a0c61543);
impl windows_core::RuntimeType for IGameListEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGameListEntry, windows_core::IUnknown, windows_core::IInspectable);
impl IGameListEntry {
    #[cfg(feature = "ApplicationModel")]
    pub fn DisplayInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LaunchAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<GameListCategory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCategoryAsync(&self, value: GameListCategory) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetCategoryAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl windows_core::RuntimeName for IGameListEntry {
    const NAME: &'static str = "Windows.Gaming.Preview.GamesEnumeration.IGameListEntry";
}
#[cfg(feature = "ApplicationModel")]
pub trait IGameListEntry_Impl: windows_core::IUnknownImpl {
    fn DisplayInfo(&self) -> windows_core::Result<super::super::super::ApplicationModel::AppDisplayInfo>;
    fn LaunchAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>>;
    fn Category(&self) -> windows_core::Result<GameListCategory>;
    fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>;
    fn SetCategoryAsync(&self, value: GameListCategory) -> windows_core::Result<windows_future::IAsyncAction>;
}
#[cfg(feature = "ApplicationModel")]
impl IGameListEntry_Vtbl {
    pub const fn new<Identity: IGameListEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayInfo<Identity: IGameListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameListEntry_Impl::DisplayInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LaunchAsync<Identity: IGameListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameListEntry_Impl::LaunchAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Category<Identity: IGameListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GameListCategory) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameListEntry_Impl::Category(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IGameListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameListEntry_Impl::Properties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCategoryAsync<Identity: IGameListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: GameListCategory, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGameListEntry_Impl::SetCategoryAsync(this, value) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGameListEntry, OFFSET>(),
            DisplayInfo: DisplayInfo::<Identity, OFFSET>,
            LaunchAsync: LaunchAsync::<Identity, OFFSET>,
            Category: Category::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            SetCategoryAsync: SetCategoryAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGameListEntry as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub DisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    DisplayInfo: usize,
    pub LaunchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameListCategory) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCategoryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, GameListCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameListEntry2, IGameListEntry2_Vtbl, 0xd84a8f8b_8749_4a25_90d3_f6c5a427886d);
impl windows_core::RuntimeType for IGameListEntry2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListEntry2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LaunchableState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameListEntryLaunchableState) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LauncherExecutable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LauncherExecutable: usize,
    pub LaunchParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetLauncherExecutableFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLauncherExecutableFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetLauncherExecutableFileWithParamsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetLauncherExecutableFileWithParamsAsync: usize,
    pub TitleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitleIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GameModeConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameListStatics, IGameListStatics_Vtbl, 0x2ddd0f6f_9c66_4b05_945c_d6ed78491b8c);
impl windows_core::RuntimeType for IGameListStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllAsyncPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GameAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGameAdded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GameRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGameRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GameUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGameUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameListStatics2, IGameListStatics2_Vtbl, 0x395f2098_ea1a_45aa_9268_a83905686f27);
impl windows_core::RuntimeType for IGameListStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameListStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MergeEntriesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnmergeEntryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameModeConfiguration, IGameModeConfiguration_Vtbl, 0x78e591af_b142_4ef0_8830_55bc2be4f5ea);
impl windows_core::RuntimeType for IGameModeConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RelatedProcessNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PercentGpuTimeAllocatedToGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPercentGpuTimeAllocatedToGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPercentGpuMemoryAllocatedToGame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPercentGpuMemoryAllocatedToSystemCompositor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxCpuCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaxCpuCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CpuExclusivityMaskLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCpuExclusivityMaskLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CpuExclusivityMaskHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCpuExclusivityMaskHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AffinitizeToExclusiveCpus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAffinitizeToExclusiveCpus: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameModeUserConfiguration, IGameModeUserConfiguration_Vtbl, 0x72d34af4_756b_470f_a0c2_ba62a90795db);
impl windows_core::RuntimeType for IGameModeUserConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GamingRelatedProcessNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameModeUserConfigurationStatics, IGameModeUserConfigurationStatics_Vtbl, 0x6e50d97c_66ea_478e_a4a1_f57c0e8d00e7);
impl windows_core::RuntimeType for IGameModeUserConfigurationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameModeUserConfigurationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
