windows_core::imp::define_interface!(IPreallocatedWorkItem, IPreallocatedWorkItem_Vtbl, 0xb6daa9fc_bc5b_401a_a8b2_6e754d14daa6);
impl windows_core::RuntimeType for IPreallocatedWorkItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RunAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPreallocatedWorkItemFactory, IPreallocatedWorkItemFactory_Vtbl, 0xe3d32b45_dfea_469b_82c5_f6e3cefdeafb);
impl windows_core::RuntimeType for IPreallocatedWorkItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWorkItemWithPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::WorkItemPriority, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWorkItemWithPriorityAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::WorkItemPriority, super::WorkItemOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignalNotifier, ISignalNotifier_Vtbl, 0x14285e06_63a7_4713_b6d9_62f64b56fb8b);
impl windows_core::RuntimeType for ISignalNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISignalNotifierStatics, ISignalNotifierStatics_Vtbl, 0x1c4e4566_8400_46d3_a115_7d0c0dfc9f62);
impl windows_core::RuntimeType for ISignalNotifierStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifierStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AttachToEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachToEventWithTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachToSemaphore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttachToSemaphoreWithTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreallocatedWorkItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreallocatedWorkItem, windows_core::IUnknown, windows_core::IInspectable);
impl PreallocatedWorkItem {
    pub fn RunAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RunAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateWorkItem<P0>(handler: P0) -> windows_core::Result<PreallocatedWorkItem>
    where
        P0: windows_core::Param<super::WorkItemHandler>,
    {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWorkItem)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWorkItemWithPriority<P0>(handler: P0, priority: super::WorkItemPriority) -> windows_core::Result<PreallocatedWorkItem>
    where
        P0: windows_core::Param<super::WorkItemHandler>,
    {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWorkItemWithPriority)(windows_core::Interface::as_raw(this), handler.param().abi(), priority, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWorkItemWithPriorityAndOptions<P0>(handler: P0, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> windows_core::Result<PreallocatedWorkItem>
    where
        P0: windows_core::Param<super::WorkItemHandler>,
    {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWorkItemWithPriorityAndOptions)(windows_core::Interface::as_raw(this), handler.param().abi(), priority, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPreallocatedWorkItemFactory<R, F: FnOnce(&IPreallocatedWorkItemFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PreallocatedWorkItem, IPreallocatedWorkItemFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PreallocatedWorkItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreallocatedWorkItem>();
}
unsafe impl windows_core::Interface for PreallocatedWorkItem {
    type Vtable = <IPreallocatedWorkItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPreallocatedWorkItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PreallocatedWorkItem {
    const NAME: &'static str = "Windows.System.Threading.Core.PreallocatedWorkItem";
}
unsafe impl Send for PreallocatedWorkItem {}
unsafe impl Sync for PreallocatedWorkItem {}
windows_core::imp::define_interface!(SignalHandler, SignalHandler_Vtbl, 0x923c402e_4721_440e_9dda_55b6f2e07710);
impl windows_core::RuntimeType for SignalHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl SignalHandler {
    pub fn new<F: Fn(windows_core::Ref<SignalNotifier>, bool) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = SignalHandlerBox { vtable: &SignalHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, signalnotifier: P0, timedout: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SignalNotifier>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), signalnotifier.param().abi(), timedout).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SignalHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, signalnotifier: *mut core::ffi::c_void, timedout: bool) -> windows_core::HRESULT,
}
#[repr(C)]
struct SignalHandlerBox<F: Fn(windows_core::Ref<SignalNotifier>, bool) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const SignalHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn(windows_core::Ref<SignalNotifier>, bool) -> windows_core::Result<()> + Send + 'static> SignalHandlerBox<F> {
    const VTABLE: SignalHandler_Vtbl = SignalHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <SignalHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, signalnotifier: *mut core::ffi::c_void, timedout: bool) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&signalnotifier), timedout).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalNotifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SignalNotifier, windows_core::IUnknown, windows_core::IInspectable);
impl SignalNotifier {
    pub fn Enable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Enable)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Terminate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Terminate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AttachToEvent<P1>(name: &windows_core::HSTRING, handler: P1) -> windows_core::Result<SignalNotifier>
    where
        P1: windows_core::Param<SignalHandler>,
    {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachToEvent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AttachToEventWithTimeout<P1>(name: &windows_core::HSTRING, handler: P1, timeout: super::super::super::Foundation::TimeSpan) -> windows_core::Result<SignalNotifier>
    where
        P1: windows_core::Param<SignalHandler>,
    {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachToEventWithTimeout)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), handler.param().abi(), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AttachToSemaphore<P1>(name: &windows_core::HSTRING, handler: P1) -> windows_core::Result<SignalNotifier>
    where
        P1: windows_core::Param<SignalHandler>,
    {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachToSemaphore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AttachToSemaphoreWithTimeout<P1>(name: &windows_core::HSTRING, handler: P1, timeout: super::super::super::Foundation::TimeSpan) -> windows_core::Result<SignalNotifier>
    where
        P1: windows_core::Param<SignalHandler>,
    {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttachToSemaphoreWithTimeout)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), handler.param().abi(), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISignalNotifierStatics<R, F: FnOnce(&ISignalNotifierStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SignalNotifier, ISignalNotifierStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SignalNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISignalNotifier>();
}
unsafe impl windows_core::Interface for SignalNotifier {
    type Vtable = <ISignalNotifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISignalNotifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SignalNotifier {
    const NAME: &'static str = "Windows.System.Threading.Core.SignalNotifier";
}
unsafe impl Send for SignalNotifier {}
unsafe impl Sync for SignalNotifier {}
