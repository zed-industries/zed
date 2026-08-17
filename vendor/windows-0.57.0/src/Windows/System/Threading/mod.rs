#[cfg(feature = "System_Threading_Core")]
pub mod Core;
windows_core::imp::define_interface!(IThreadPoolStatics, IThreadPoolStatics_Vtbl, 0xb6bf67dd_84bd_44f8_ac1c_93ebcb9dba91);
impl windows_core::RuntimeType for IThreadPoolStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IThreadPoolStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RunAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunWithPriorityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WorkItemPriority, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunWithPriorityAndOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WorkItemPriority, WorkItemOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IThreadPoolTimer, IThreadPoolTimer_Vtbl, 0x594ebe78_55ea_4a88_a50d_3402ae1f9cf2);
impl windows_core::RuntimeType for IThreadPoolTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IThreadPoolTimer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Period: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IThreadPoolTimerStatics, IThreadPoolTimerStatics_Vtbl, 0x1a8a9d02_e482_461b_b8c7_8efad1cce590);
impl windows_core::RuntimeType for IThreadPoolTimerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IThreadPoolTimerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePeriodicTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePeriodicTimerWithCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTimerWithCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct ThreadPool;
impl ThreadPool {
    pub fn RunAsync<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<WorkItemHandler>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunAsync)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RunWithPriorityAsync<P0>(handler: P0, priority: WorkItemPriority) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<WorkItemHandler>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunWithPriorityAsync)(windows_core::Interface::as_raw(this), handler.param().abi(), priority, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RunWithPriorityAndOptionsAsync<P0>(handler: P0, priority: WorkItemPriority, options: WorkItemOptions) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<WorkItemHandler>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunWithPriorityAndOptionsAsync)(windows_core::Interface::as_raw(this), handler.param().abi(), priority, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IThreadPoolStatics<R, F: FnOnce(&IThreadPoolStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ThreadPool, IThreadPoolStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ThreadPool {
    const NAME: &'static str = "Windows.System.Threading.ThreadPool";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ThreadPoolTimer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ThreadPoolTimer, windows_core::IUnknown, windows_core::IInspectable);
impl ThreadPoolTimer {
    pub fn Period(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Period)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreatePeriodicTimer<P0>(handler: P0, period: super::super::Foundation::TimeSpan) -> windows_core::Result<ThreadPoolTimer>
    where
        P0: windows_core::Param<TimerElapsedHandler>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeriodicTimer)(windows_core::Interface::as_raw(this), handler.param().abi(), period, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTimer<P0>(handler: P0, delay: super::super::Foundation::TimeSpan) -> windows_core::Result<ThreadPoolTimer>
    where
        P0: windows_core::Param<TimerElapsedHandler>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTimer)(windows_core::Interface::as_raw(this), handler.param().abi(), delay, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePeriodicTimerWithCompletion<P0, P1>(handler: P0, period: super::super::Foundation::TimeSpan, destroyed: P1) -> windows_core::Result<ThreadPoolTimer>
    where
        P0: windows_core::Param<TimerElapsedHandler>,
        P1: windows_core::Param<TimerDestroyedHandler>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePeriodicTimerWithCompletion)(windows_core::Interface::as_raw(this), handler.param().abi(), period, destroyed.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTimerWithCompletion<P0, P1>(handler: P0, delay: super::super::Foundation::TimeSpan, destroyed: P1) -> windows_core::Result<ThreadPoolTimer>
    where
        P0: windows_core::Param<TimerElapsedHandler>,
        P1: windows_core::Param<TimerDestroyedHandler>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTimerWithCompletion)(windows_core::Interface::as_raw(this), handler.param().abi(), delay, destroyed.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IThreadPoolTimerStatics<R, F: FnOnce(&IThreadPoolTimerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ThreadPoolTimer, IThreadPoolTimerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ThreadPoolTimer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IThreadPoolTimer>();
}
unsafe impl windows_core::Interface for ThreadPoolTimer {
    type Vtable = IThreadPoolTimer_Vtbl;
    const IID: windows_core::GUID = <IThreadPoolTimer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.ThreadPoolTimer";
}
unsafe impl Send for ThreadPoolTimer {}
unsafe impl Sync for ThreadPoolTimer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: Self = Self(0u32);
    pub const TimeSliced: Self = Self(1u32);
}
impl windows_core::TypeKind for WorkItemOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WorkItemOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WorkItemOptions").field(&self.0).finish()
    }
}
impl WorkItemOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WorkItemOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WorkItemOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WorkItemOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WorkItemOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WorkItemOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for WorkItemOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for WorkItemPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WorkItemPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WorkItemPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WorkItemPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemPriority;i4)");
}
windows_core::imp::define_interface!(TimerDestroyedHandler, TimerDestroyedHandler_Vtbl, 0x34ed19fa_8384_4eb9_8209_fb5094eeec35);
impl TimerDestroyedHandler {
    pub fn new<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = TimerDestroyedHandlerBox::<F> { vtable: &TimerDestroyedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, timer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ThreadPoolTimer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), timer.param().abi()).ok() }
    }
}
#[repr(C)]
struct TimerDestroyedHandlerBox<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const TimerDestroyedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static> TimerDestroyedHandlerBox<F> {
    const VTABLE: TimerDestroyedHandler_Vtbl = TimerDestroyedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <TimerDestroyedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, timer: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&timer)).into()
    }
}
impl windows_core::RuntimeType for TimerDestroyedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct TimerDestroyedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(TimerElapsedHandler, TimerElapsedHandler_Vtbl, 0xfaaea667_fbeb_49cb_adb2_71184c556e43);
impl TimerElapsedHandler {
    pub fn new<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = TimerElapsedHandlerBox::<F> { vtable: &TimerElapsedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, timer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ThreadPoolTimer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), timer.param().abi()).ok() }
    }
}
#[repr(C)]
struct TimerElapsedHandlerBox<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const TimerElapsedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ThreadPoolTimer>) -> windows_core::Result<()> + Send + 'static> TimerElapsedHandlerBox<F> {
    const VTABLE: TimerElapsedHandler_Vtbl = TimerElapsedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <TimerElapsedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, timer: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&timer)).into()
    }
}
impl windows_core::RuntimeType for TimerElapsedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct TimerElapsedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(WorkItemHandler, WorkItemHandler_Vtbl, 0x1d1a8b8b_fa66_414f_9cbd_b65fc99d17fa);
impl WorkItemHandler {
    pub fn new<F: FnMut(Option<&super::super::Foundation::IAsyncAction>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = WorkItemHandlerBox::<F> { vtable: &WorkItemHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, operation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IAsyncAction>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), operation.param().abi()).ok() }
    }
}
#[repr(C)]
struct WorkItemHandlerBox<F: FnMut(Option<&super::super::Foundation::IAsyncAction>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const WorkItemHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&super::super::Foundation::IAsyncAction>) -> windows_core::Result<()> + Send + 'static> WorkItemHandlerBox<F> {
    const VTABLE: WorkItemHandler_Vtbl = WorkItemHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <WorkItemHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, operation: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&operation)).into()
    }
}
impl windows_core::RuntimeType for WorkItemHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct WorkItemHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
