pub trait IBackgroundCondition_Impl: Sized {}
impl windows_core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundCondition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCondition_Impl, const OFFSET: isize>() -> IBackgroundCondition_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundCondition, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCondition as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTask_Impl: Sized {
    fn Run(&self, taskinstance: Option<&IBackgroundTaskInstance>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: isize>() -> IBackgroundTask_Vtbl {
        unsafe extern "system" fn Run<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskinstance: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTask_Impl::Run(this, windows_core::from_raw_borrowed(&taskinstance)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTask, OFFSET>(), Run: Run::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTask as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTaskInstance_Impl: Sized {
    fn InstanceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Task(&self) -> windows_core::Result<BackgroundTaskRegistration>;
    fn Progress(&self) -> windows_core::Result<u32>;
    fn SetProgress(&self, value: u32) -> windows_core::Result<()>;
    fn TriggerDetails(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn Canceled(&self, cancelhandler: Option<&BackgroundTaskCanceledEventHandler>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn SuspendedCount(&self) -> windows_core::Result<u32>;
    fn GetDeferral(&self) -> windows_core::Result<BackgroundTaskDeferral>;
}
impl windows_core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
impl IBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn InstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::InstanceId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::Task(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::Progress(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTaskInstance_Impl::SetProgress(this, value).into()
        }
        unsafe extern "system" fn TriggerDetails<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::TriggerDetails(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancelhandler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::Canceled(this, windows_core::from_raw_borrowed(&cancelhandler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTaskInstance_Impl::RemoveCanceled(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SuspendedCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::SuspendedCount(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance_Impl::GetDeferral(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance, OFFSET>(),
            InstanceId: InstanceId::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            SetProgress: SetProgress::<Identity, Impl, OFFSET>,
            TriggerDetails: TriggerDetails::<Identity, Impl, OFFSET>,
            Canceled: Canceled::<Identity, Impl, OFFSET>,
            RemoveCanceled: RemoveCanceled::<Identity, Impl, OFFSET>,
            SuspendedCount: SuspendedCount::<Identity, Impl, OFFSET>,
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTaskInstance2_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
impl IBackgroundTaskInstance2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance2_Vtbl {
        unsafe extern "system" fn GetThrottleCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance2_Impl::GetThrottleCount(this, counter) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance2, OFFSET>(),
            GetThrottleCount: GetThrottleCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait IBackgroundTaskInstance4_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn User(&self) -> windows_core::Result<super::super::System::User>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
#[cfg(feature = "System")]
impl IBackgroundTaskInstance4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance4_Vtbl {
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskInstance4_Impl::User(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance4, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance4 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTaskRegistration_Impl: Sized {
    fn TaskId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Progress(&self, handler: Option<&BackgroundTaskProgressEventHandler>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn Completed(&self, handler: Option<&BackgroundTaskCompletedEventHandler>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn Unregister(&self, canceltask: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
impl IBackgroundTaskRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration_Vtbl {
        unsafe extern "system" fn TaskId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration_Impl::TaskId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration_Impl::Progress(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTaskRegistration_Impl::RemoveProgress(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Completed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration_Impl::Completed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTaskRegistration_Impl::RemoveCompleted(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Unregister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, canceltask: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBackgroundTaskRegistration_Impl::Unregister(this, canceltask).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration, OFFSET>(),
            TaskId: TaskId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            RemoveProgress: RemoveProgress::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            RemoveCompleted: RemoveCompleted::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTaskRegistration2_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn Trigger(&self) -> windows_core::Result<IBackgroundTrigger>;
}
impl windows_core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
impl IBackgroundTaskRegistration2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration2_Vtbl {
        unsafe extern "system" fn Trigger<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration2_Impl::Trigger(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration2, OFFSET>(), Trigger: Trigger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration2 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTaskRegistration3_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(&self) -> windows_core::Result<BackgroundTaskRegistrationGroup>;
}
impl windows_core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
impl IBackgroundTaskRegistration3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration3_Vtbl {
        unsafe extern "system" fn TaskGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBackgroundTaskRegistration3_Impl::TaskGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration3, OFFSET>(), TaskGroup: TaskGroup::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration3 as windows_core::Interface>::IID
    }
}
pub trait IBackgroundTrigger_Impl: Sized {}
impl windows_core::RuntimeName for IBackgroundTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTrigger";
}
impl IBackgroundTrigger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTrigger_Impl, const OFFSET: isize>() -> IBackgroundTrigger_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTrigger, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundTrigger as windows_core::Interface>::IID
    }
}
