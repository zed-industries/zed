#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
windows_core::imp::define_interface!(IUserDataTask, IUserDataTask_Vtbl, 0x7c6585d1_e0d4_4f99_aee2_bc2d5ddadf4c);
impl windows_core::RuntimeType for IUserDataTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CompletedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompletedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskDetailsKind) -> windows_core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskDetailsKind) -> windows_core::HRESULT,
    pub DueDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDueDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskKind) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskPriority) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskPriority) -> windows_core::HRESULT,
    pub RecurrenceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecurrenceProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegenerationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRegenerationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReminder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskSensitivity) -> windows_core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskSensitivity) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskBatch, IUserDataTaskBatch_Vtbl, 0x382da5fe_20b5_431c_8f42_a5d292ec930c);
impl windows_core::RuntimeType for IUserDataTaskBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskBatch_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Tasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tasks: usize,
}
windows_core::imp::define_interface!(IUserDataTaskList, IUserDataTaskList_Vtbl, 0x49412e39_7c1d_4df1_bed3_314b7cbf5e4e);
impl windows_core::RuntimeType for IUserDataTaskList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskListOtherAppReadAccess) -> windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskListOtherAppReadAccess) -> windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskListOtherAppWriteAccess) -> windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskListOtherAppWriteAccess) -> windows_core::HRESULT,
    pub LimitedWriteOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTaskReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTaskReaderWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskListLimitedWriteOperations, IUserDataTaskListLimitedWriteOperations_Vtbl, 0x7aa267f2_6078_4183_919e_4f29f19cfae9);
impl windows_core::RuntimeType for IUserDataTaskListLimitedWriteOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskListLimitedWriteOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCompleteTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryCreateOrUpdateTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDeleteTaskAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySkipOccurrenceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskListSyncManager, IUserDataTaskListSyncManager_Vtbl, 0x8e591a95_1dcf_469f_93ec_ba48bb553c6b);
impl windows_core::RuntimeType for IUserDataTaskListSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskListSyncManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LastAttemptedSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskListSyncStatus) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskListSyncStatus) -> windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskManager, IUserDataTaskManager_Vtbl, 0x8451c914_e60b_48a9_9211_7fb8a56cb84c);
impl windows_core::RuntimeType for IUserDataTaskManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestStoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskStoreAccessType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IUserDataTaskManagerStatics, IUserDataTaskManagerStatics_Vtbl, 0xb35539f8_c502_47fc_a81e_100883719d55);
impl windows_core::RuntimeType for IUserDataTaskManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IUserDataTaskQueryOptions, IUserDataTaskQueryOptions_Vtbl, 0x959f27ed_909a_4d30_8c1b_331d8fe667e2);
impl windows_core::RuntimeType for IUserDataTaskQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskQueryOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SortProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskQuerySortProperty) -> windows_core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskQuerySortProperty) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskQueryKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskQueryKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskReader, IUserDataTaskReader_Vtbl, 0x03e688b1_4ccf_4500_883b_e76290cfed63);
impl windows_core::RuntimeType for IUserDataTaskReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadBatchAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskRecurrenceProperties, IUserDataTaskRecurrenceProperties_Vtbl, 0x73df80b0_27c6_40ce_b149_9cd41485a69e);
impl windows_core::RuntimeType for IUserDataTaskRecurrenceProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskRecurrenceProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskRecurrenceUnit) -> windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskRecurrenceUnit) -> windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskRegenerationProperties, IUserDataTaskRegenerationProperties_Vtbl, 0x92ab0007_090e_4704_bb5c_84fc0b0d9c31);
impl windows_core::RuntimeType for IUserDataTaskRegenerationProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskRegenerationProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataTaskRegenerationUnit) -> windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataTaskRegenerationUnit) -> windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Until: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataTaskStore, IUserDataTaskStore_Vtbl, 0xf06a9cb0_f1db_45ba_8a62_086004c0213d);
impl windows_core::RuntimeType for IUserDataTaskStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataTaskStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateListAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateListInAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindListsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindListsAsync: usize,
    pub GetListAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTask(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTask, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTask {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataTask, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ListId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CompletedDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletedDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompletedDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompletedDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Details(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDetails(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDetails)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DetailsKind(&self) -> windows_core::Result<UserDataTaskDetailsKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetailsKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDetailsKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DueDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DueDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDueDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDueDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<UserDataTaskKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Priority(&self) -> windows_core::Result<UserDataTaskPriority> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPriority(&self, value: UserDataTaskPriority) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RecurrenceProperties(&self) -> windows_core::Result<UserDataTaskRecurrenceProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecurrenceProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRecurrenceProperties<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UserDataTaskRecurrenceProperties>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRecurrenceProperties)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RegenerationProperties(&self) -> windows_core::Result<UserDataTaskRegenerationProperties> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegenerationProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRegenerationProperties<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UserDataTaskRegenerationProperties>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRegenerationProperties)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Reminder(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reminder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetReminder<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReminder)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Sensitivity(&self) -> windows_core::Result<UserDataTaskSensitivity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Sensitivity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSensitivity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubject(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSubject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartDate(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartDate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartDate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UserDataTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTask>();
}
unsafe impl windows_core::Interface for UserDataTask {
    type Vtable = IUserDataTask_Vtbl;
    const IID: windows_core::GUID = <IUserDataTask as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTask";
}
unsafe impl Send for UserDataTask {}
unsafe impl Sync for UserDataTask {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskBatch(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskBatch, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskBatch {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tasks(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tasks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataTaskBatch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskBatch>();
}
unsafe impl windows_core::Interface for UserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskBatch as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
}
unsafe impl Send for UserDataTaskBatch {}
unsafe impl Sync for UserDataTaskBatch {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskList, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskList {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserDataAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserDataAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OtherAppReadAccess(&self) -> windows_core::Result<UserDataTaskListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppReadAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppReadAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> windows_core::Result<UserDataTaskListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OtherAppWriteAccess)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOtherAppWriteAccess)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LimitedWriteOperations(&self) -> windows_core::Result<UserDataTaskListLimitedWriteOperations> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LimitedWriteOperations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncManager(&self) -> windows_core::Result<UserDataTaskListSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterSyncManagerAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterSyncManagerAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTaskReader(&self) -> windows_core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTaskReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTaskReaderWithOptions<P0>(&self, options: P0) -> windows_core::Result<UserDataTaskReader>
    where
        P0: windows_core::Param<UserDataTaskQueryOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTaskReaderWithOptions)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTaskAsync(&self, userdatatask: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTaskAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdatatask), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveTaskAsync<P0>(&self, userdatatask: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<UserDataTask>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveTaskAsync)(windows_core::Interface::as_raw(this), userdatatask.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteTaskAsync(&self, userdatataskid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteTaskAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdatataskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataTaskList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskList>();
}
unsafe impl windows_core::Interface for UserDataTaskList {
    type Vtable = IUserDataTaskList_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
}
unsafe impl Send for UserDataTaskList {}
unsafe impl Sync for UserDataTaskList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskListLimitedWriteOperations(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskListLimitedWriteOperations, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskListLimitedWriteOperations {
    pub fn TryCompleteTaskAsync(&self, userdatataskid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCompleteTaskAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdatataskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryCreateOrUpdateTaskAsync<P0>(&self, userdatatask: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<UserDataTask>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateOrUpdateTaskAsync)(windows_core::Interface::as_raw(this), userdatatask.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryDeleteTaskAsync(&self, userdatataskid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDeleteTaskAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdatataskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySkipOccurrenceAsync(&self, userdatataskid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySkipOccurrenceAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(userdatataskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataTaskListLimitedWriteOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskListLimitedWriteOperations>();
}
unsafe impl windows_core::Interface for UserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskListLimitedWriteOperations as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
}
unsafe impl Send for UserDataTaskListLimitedWriteOperations {}
unsafe impl Sync for UserDataTaskListLimitedWriteOperations {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskListSyncManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskListSyncManager, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskListSyncManager {
    pub fn LastAttemptedSyncTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastAttemptedSyncTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLastAttemptedSyncTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LastSuccessfulSyncTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSuccessfulSyncTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLastSuccessfulSyncTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<UserDataTaskListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStatus)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SyncAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSyncStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for UserDataTaskListSyncManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskListSyncManager>();
}
unsafe impl windows_core::Interface for UserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskListSyncManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
}
unsafe impl Send for UserDataTaskListSyncManager {}
unsafe impl Sync for UserDataTaskListSyncManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskManager, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskManager {
    pub fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestStoreAsync)(windows_core::Interface::as_raw(this), accesstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<UserDataTaskManager>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserDataTaskManagerStatics<R, F: FnOnce(&IUserDataTaskManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataTaskManager, IUserDataTaskManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserDataTaskManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskManager>();
}
unsafe impl windows_core::Interface for UserDataTaskManager {
    type Vtable = IUserDataTaskManager_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
}
unsafe impl Send for UserDataTaskManager {}
unsafe impl Sync for UserDataTaskManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskQueryOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskQueryOptions, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskQueryOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataTaskQueryOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SortProperty(&self) -> windows_core::Result<UserDataTaskQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SortProperty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSortProperty)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<UserDataTaskQueryKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: UserDataTaskQueryKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for UserDataTaskQueryOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskQueryOptions>();
}
unsafe impl windows_core::Interface for UserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskQueryOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
}
unsafe impl Send for UserDataTaskQueryOptions {}
unsafe impl Sync for UserDataTaskQueryOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskReader, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskReader {
    pub fn ReadBatchAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBatchAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataTaskReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskReader>();
}
unsafe impl windows_core::Interface for UserDataTaskReader {
    type Vtable = IUserDataTaskReader_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
}
unsafe impl Send for UserDataTaskReader {}
unsafe impl Sync for UserDataTaskReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskRecurrenceProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskRecurrenceProperties, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskRecurrenceProperties {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataTaskRecurrenceProperties, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> windows_core::Result<UserDataTaskRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occurrences)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOccurrences<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOccurrences)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Until(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Until)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUntil<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUntil)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Interval(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DaysOfWeek(&self) -> windows_core::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DaysOfWeek)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDaysOfWeek<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDaysOfWeek)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn WeekOfMonth(&self) -> windows_core::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WeekOfMonth)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetWeekOfMonth<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWeekOfMonth)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Month(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Month)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMonth<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMonth)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Day(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Day)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDay<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDay)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UserDataTaskRecurrenceProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskRecurrenceProperties>();
}
unsafe impl windows_core::Interface for UserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskRecurrenceProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
}
unsafe impl Send for UserDataTaskRecurrenceProperties {}
unsafe impl Sync for UserDataTaskRecurrenceProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskRegenerationProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskRegenerationProperties, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskRegenerationProperties {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataTaskRegenerationProperties, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> windows_core::Result<UserDataTaskRegenerationUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occurrences)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOccurrences<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOccurrences)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Until(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Until)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUntil<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUntil)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Interval(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Interval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for UserDataTaskRegenerationProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskRegenerationProperties>();
}
unsafe impl windows_core::Interface for UserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskRegenerationProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
}
unsafe impl Send for UserDataTaskRegenerationProperties {}
unsafe impl Sync for UserDataTaskRegenerationProperties {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserDataTaskStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataTaskStore, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataTaskStore {
    pub fn CreateListAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateListAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateListInAccountAsync(&self, name: &windows_core::HSTRING, userdataaccountid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateListInAccountAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(userdataaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindListsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindListsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetListAsync(&self, tasklistid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetListAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tasklistid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataTaskStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataTaskStore>();
}
unsafe impl windows_core::Interface for UserDataTaskStore {
    type Vtable = IUserDataTaskStore_Vtbl;
    const IID: windows_core::GUID = <IUserDataTaskStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
}
unsafe impl Send for UserDataTaskStore {}
unsafe impl Sync for UserDataTaskStore {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskDaysOfWeek(pub u32);
impl UserDataTaskDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl windows_core::TypeKind for UserDataTaskDaysOfWeek {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskDaysOfWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskDaysOfWeek").field(&self.0).finish()
    }
}
impl UserDataTaskDaysOfWeek {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UserDataTaskDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UserDataTaskDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for UserDataTaskDaysOfWeek {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataTaskDetailsKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskDetailsKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskDetailsKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskDetailsKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: Self = Self(0i32);
    pub const Recurring: Self = Self(1i32);
    pub const Regenerating: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataTaskKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataTaskListOtherAppReadAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskListOtherAppReadAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppReadAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskListOtherAppReadAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataTaskListOtherAppWriteAccess {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskListOtherAppWriteAccess {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskListOtherAppWriteAccess {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskListSyncStatus(pub i32);
impl UserDataTaskListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl windows_core::TypeKind for UserDataTaskListSyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskListSyncStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskListSyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: Self = Self(0i32);
    pub const Low: Self = Self(-1i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataTaskPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: Self = Self(0i32);
    pub const Incomplete: Self = Self(1i32);
    pub const Complete: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataTaskQueryKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskQueryKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskQueryKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskQueryKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: Self = Self(0i32);
}
impl windows_core::TypeKind for UserDataTaskQuerySortProperty {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskQuerySortProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskQuerySortProperty").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskQuerySortProperty {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskRecurrenceUnit(pub i32);
impl UserDataTaskRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl windows_core::TypeKind for UserDataTaskRecurrenceUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskRecurrenceUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskRecurrenceUnit").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskRecurrenceUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const Yearly: Self = Self(4i32);
}
impl windows_core::TypeKind for UserDataTaskRegenerationUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskRegenerationUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskRegenerationUnit").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskRegenerationUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataTaskSensitivity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskSensitivity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskSensitivity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskSensitivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: Self = Self(0i32);
    pub const AllTasksLimitedReadWrite: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataTaskStoreAccessType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskStoreAccessType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskStoreAccessType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskStoreAccessType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl windows_core::TypeKind for UserDataTaskWeekOfMonth {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataTaskWeekOfMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataTaskWeekOfMonth").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataTaskWeekOfMonth {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth;i4)");
}
