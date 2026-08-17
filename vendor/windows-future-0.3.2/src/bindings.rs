windows_core::imp::define_interface!(
    AsyncActionCompletedHandler,
    AsyncActionCompletedHandler_Vtbl,
    0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7
);
impl windows_core::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl AsyncActionCompletedHandler {
    pub fn new<
        F: Fn(windows_core::Ref<IAsyncAction>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncActionCompletedHandlerBox {
            vtable: &AsyncActionCompletedHandlerBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncAction>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                asyncstatus,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionCompletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT,
}
#[repr(C)]
struct AsyncActionCompletedHandlerBox<
    F: Fn(windows_core::Ref<IAsyncAction>, AsyncStatus) -> windows_core::Result<()> + Send + 'static,
> {
    vtable: *const AsyncActionCompletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        F: Fn(windows_core::Ref<IAsyncAction>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncActionCompletedHandlerBox<F>
{
    const VTABLE: AsyncActionCompletedHandler_Vtbl = AsyncActionCompletedHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncActionCompletedHandler as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncActionProgressHandler<TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TProgress>,
)
where
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface
    for AsyncActionProgressHandler<TProgress>
{
    type Vtable = AsyncActionProgressHandler_Vtbl<TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for AsyncActionProgressHandler<TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({6d844858-0cff-4590-ae89-95a5a5c8b4b8}")
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static> AsyncActionProgressHandler<TProgress> {
    pub fn new<
        F: Fn(
                windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
                windows_core::Ref<TProgress>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncActionProgressHandlerBox {
            vtable: &AsyncActionProgressHandlerBox::<TProgress, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncActionWithProgress<TProgress>>,
        P1: windows_core::Param<TProgress>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                progressinfo.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionProgressHandler_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        progressinfo: windows_core::AbiType<TProgress>,
    ) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncActionProgressHandlerBox<
    TProgress,
    F: Fn(
            windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
            windows_core::Ref<TProgress>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncActionProgressHandler_Vtbl<TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        TProgress: windows_core::RuntimeType + 'static,
        F: Fn(
                windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
                windows_core::Ref<TProgress>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncActionProgressHandlerBox<TProgress, F>
{
    const VTABLE: AsyncActionProgressHandler_Vtbl<TProgress> =
        AsyncActionProgressHandler_Vtbl::<TProgress> {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: Self::QueryInterface,
                AddRef: Self::AddRef,
                Release: Self::Release,
            },
            Invoke: Self::Invoke,
            TProgress: core::marker::PhantomData::<TProgress>,
        };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid
                == <AsyncActionProgressHandler<TProgress> as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        progressinfo: windows_core::AbiType<TProgress>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(
                core::mem::transmute_copy(&asyncinfo),
                core::mem::transmute_copy(&progressinfo),
            )
            .into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TProgress>,
)
where
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface
    for AsyncActionWithProgressCompletedHandler<TProgress>
{
    type Vtable = AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for AsyncActionWithProgressCompletedHandler<TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9c029f91-cc84-44fd-ac26-0a6c4e555281}")
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static>
    AsyncActionWithProgressCompletedHandler<TProgress>
{
    pub fn new<
        F: Fn(
                windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
                AsyncStatus,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncActionWithProgressCompletedHandlerBox {
            vtable: &AsyncActionWithProgressCompletedHandlerBox::<TProgress, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncActionWithProgress<TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                asyncstatus,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncActionWithProgressCompletedHandlerBox<
    TProgress,
    F: Fn(
            windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
            AsyncStatus,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        TProgress: windows_core::RuntimeType + 'static,
        F: Fn(
                windows_core::Ref<IAsyncActionWithProgress<TProgress>>,
                AsyncStatus,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncActionWithProgressCompletedHandlerBox<TProgress, F>
{
    const VTABLE: AsyncActionWithProgressCompletedHandler_Vtbl<TProgress> =
        AsyncActionWithProgressCompletedHandler_Vtbl::<TProgress> {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: Self::QueryInterface,
                AddRef: Self::AddRef,
                Release: Self::Release,
            },
            Invoke: Self::Invoke,
            TProgress: core::marker::PhantomData::<TProgress>,
        };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            * interface = if * iid == < AsyncActionWithProgressCompletedHandler < TProgress > as windows_core::Interface >::IID || * iid == < windows_core::IUnknown as windows_core::Interface >::IID || * iid == < windows_core::imp::IAgileObject as windows_core::Interface >::IID { & mut ( * this ) . vtable as * mut _ as _ } else if * iid == < windows_core::imp::IMarshal as windows_core::Interface >::IID { ( * this ) . count . add_ref ( ) ; return windows_core::imp::marshaler ( core::mem::transmute ( & mut ( * this ) . vtable as * mut _ as * mut core::ffi::c_void ) , interface ) ; } else { core::ptr::null_mut ( ) } ;
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationCompletedHandler<TResult>(
    windows_core::IUnknown,
    core::marker::PhantomData<TResult>,
)
where
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface
    for AsyncOperationCompletedHandler<TResult>
{
    type Vtable = AsyncOperationCompletedHandler_Vtbl<TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for AsyncOperationCompletedHandler<TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({fcdcf02c-e5d8-4478-915a-4d90b74b83a5}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
    pub fn new<
        F: Fn(windows_core::Ref<IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncOperationCompletedHandlerBox {
            vtable: &AsyncOperationCompletedHandlerBox::<TResult, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperation<TResult>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                asyncstatus,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationCompletedHandler_Vtbl<TResult>
where
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
}
#[repr(C)]
struct AsyncOperationCompletedHandlerBox<
    TResult,
    F: Fn(windows_core::Ref<IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    TResult: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationCompletedHandler_Vtbl<TResult>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        F: Fn(windows_core::Ref<IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncOperationCompletedHandlerBox<TResult, F>
{
    const VTABLE: AsyncOperationCompletedHandler_Vtbl<TResult> =
        AsyncOperationCompletedHandler_Vtbl::<TResult> {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: Self::QueryInterface,
                AddRef: Self::AddRef,
                Release: Self::Release,
            },
            Invoke: Self::Invoke,
            TResult: core::marker::PhantomData::<TResult>,
        };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid
                == <AsyncOperationCompletedHandler<TResult> as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TResult>,
    core::marker::PhantomData<TProgress>,
)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::Interface for AsyncOperationProgressHandler<TResult, TProgress>
{
    type Vtable = AsyncOperationProgressHandler_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({55690902-0aab-421a-8778-f8ce5026d758}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > AsyncOperationProgressHandler<TResult, TProgress>
{
    pub fn new<
        F: Fn(
                windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
                windows_core::Ref<TProgress>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncOperationProgressHandlerBox {
            vtable: &AsyncOperationProgressHandlerBox::<TResult, TProgress, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperationWithProgress<TResult, TProgress>>,
        P1: windows_core::Param<TProgress>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                progressinfo.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationProgressHandler_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        progressinfo: windows_core::AbiType<TProgress>,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncOperationProgressHandlerBox<
    TResult,
    TProgress,
    F: Fn(
            windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
            windows_core::Ref<TProgress>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationProgressHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
        F: Fn(
                windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
                windows_core::Ref<TProgress>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncOperationProgressHandlerBox<TResult, TProgress, F>
{
    const VTABLE: AsyncOperationProgressHandler_Vtbl<TResult, TProgress> =
        AsyncOperationProgressHandler_Vtbl::<TResult, TProgress> {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: Self::QueryInterface,
                AddRef: Self::AddRef,
                Release: Self::Release,
            },
            Invoke: Self::Invoke,
            TResult: core::marker::PhantomData::<TResult>,
            TProgress: core::marker::PhantomData::<TProgress>,
        };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            * interface = if * iid == < AsyncOperationProgressHandler < TResult , TProgress > as windows_core::Interface >::IID || * iid == < windows_core::IUnknown as windows_core::Interface >::IID || * iid == < windows_core::imp::IAgileObject as windows_core::Interface >::IID { & mut ( * this ) . vtable as * mut _ as _ } else if * iid == < windows_core::imp::IMarshal as windows_core::Interface >::IID { ( * this ) . count . add_ref ( ) ; return windows_core::imp::marshaler ( core::mem::transmute ( & mut ( * this ) . vtable as * mut _ as * mut core::ffi::c_void ) , interface ) ; } else { core::ptr::null_mut ( ) } ;
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        progressinfo: windows_core::AbiType<TProgress>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(
                core::mem::transmute_copy(&asyncinfo),
                core::mem::transmute_copy(&progressinfo),
            )
            .into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TResult>,
    core::marker::PhantomData<TProgress>,
)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::Interface for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
{
    type Vtable = AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::RuntimeType for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({e85df41d-6aa7-46e3-a8e2-f009d840c627}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
{
    pub fn new<
        F: Fn(
                windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
                AsyncStatus,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AsyncOperationWithProgressCompletedHandlerBox {
            vtable: &AsyncOperationWithProgressCompletedHandlerBox::<TResult, TProgress, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperationWithProgress<TResult, TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                asyncinfo.param().abi(),
                asyncstatus,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncOperationWithProgressCompletedHandlerBox<
    TResult,
    TProgress,
    F: Fn(
            windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
            AsyncStatus,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
        F: Fn(
                windows_core::Ref<IAsyncOperationWithProgress<TResult, TProgress>>,
                AsyncStatus,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > AsyncOperationWithProgressCompletedHandlerBox<TResult, TProgress, F>
{
    const VTABLE: AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress> =
        AsyncOperationWithProgressCompletedHandler_Vtbl::<TResult, TProgress> {
            base__: windows_core::IUnknown_Vtbl {
                QueryInterface: Self::QueryInterface,
                AddRef: Self::AddRef,
                Release: Self::Release,
            },
            Invoke: Self::Invoke,
            TResult: core::marker::PhantomData::<TResult>,
            TProgress: core::marker::PhantomData::<TProgress>,
        };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            * interface = if * iid == < AsyncOperationWithProgressCompletedHandler < TResult , TProgress > as windows_core::Interface >::IID || * iid == < windows_core::IUnknown as windows_core::Interface >::IID || * iid == < windows_core::imp::IAgileObject as windows_core::Interface >::IID { & mut ( * this ) . vtable as * mut _ as _ } else if * iid == < windows_core::imp::IMarshal as windows_core::Interface >::IID { ( * this ) . count . add_ref ( ) ; return windows_core::imp::marshaler ( core::mem::transmute ( & mut ( * this ) . vtable as * mut _ as * mut core::ffi::c_void ) , interface ) ; } else { core::ptr::null_mut ( ) } ;
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        asyncinfo: *mut core::ffi::c_void,
        asyncstatus: AsyncStatus,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl windows_core::TypeKind for AsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AsyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
windows_core::imp::define_interface!(
    IAsyncAction,
    IAsyncAction_Vtbl,
    0x5a648006_843a_4da9_865b_9d26e5dfad7b
);
impl windows_core::RuntimeType for IAsyncAction {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAsyncAction,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(IAsyncAction, IAsyncInfo);
impl IAsyncAction {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionCompletedHandler>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl Send for IAsyncAction {}
unsafe impl Sync for IAsyncAction {}
impl windows_core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
pub trait IAsyncAction_Impl: IAsyncInfo_Impl {
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<AsyncActionCompletedHandler>,
    ) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: IAsyncAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncAction_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<Identity: IAsyncAction_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::GetResults(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncAction, OFFSET>(),
            SetCompleted: SetCompleted::<Identity, OFFSET>,
            Completed: Completed::<Identity, OFFSET>,
            GetResults: GetResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncAction as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncActionWithProgress<TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TProgress>,
)
where
    TProgress: windows_core::RuntimeType + 'static;
impl<TProgress: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncActionWithProgress<TProgress>
{
}
impl<TProgress: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncActionWithProgress<TProgress>
{
}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface
    for IAsyncActionWithProgress<TProgress>
{
    type Vtable = IAsyncActionWithProgress_Vtbl<TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for IAsyncActionWithProgress<TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({1f6db258-e803-48a1-9546-eb7353398884}")
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IAsyncInfo>
    for IAsyncActionWithProgress<TProgress>
{
    const QUERY: bool = true;
}
impl<TProgress: windows_core::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    pub fn SetProgress<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionProgressHandler<TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetProgress)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Progress(&self) -> windows_core::Result<AsyncActionProgressHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionWithProgressCompletedHandler<TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(
        &self,
    ) -> windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> Send
    for IAsyncActionWithProgress<TProgress>
{
}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> Sync
    for IAsyncActionWithProgress<TProgress>
{
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeName
    for IAsyncActionWithProgress<TProgress>
{
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
pub trait IAsyncActionWithProgress_Impl<TProgress>: IAsyncInfo_Impl
where
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(
        &self,
        handler: windows_core::Ref<AsyncActionProgressHandler<TProgress>>,
    ) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<AsyncActionWithProgressCompletedHandler<TProgress>>,
    ) -> windows_core::Result<()>;
    fn Completed(&self)
        -> windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl<TProgress: windows_core::RuntimeType + 'static> IAsyncActionWithProgress_Vtbl<TProgress> {
    pub const fn new<Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(
    ) -> Self {
        unsafe extern "system" fn SetProgress<
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncActionWithProgress_Impl<TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::SetProgress(
                    this,
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn Progress<
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncActionWithProgress_Impl<TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncActionWithProgress_Impl::Progress(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompleted<
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncActionWithProgress_Impl<TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::SetCompleted(
                    this,
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn Completed<
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncActionWithProgress_Impl<TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncActionWithProgress_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncActionWithProgress_Impl<TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::GetResults(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAsyncActionWithProgress<TProgress>,
                OFFSET,
            >(),
            SetProgress: SetProgress::<TProgress, Identity, OFFSET>,
            Progress: Progress::<TProgress, Identity, OFFSET>,
            SetCompleted: SetCompleted::<TProgress, Identity, OFFSET>,
            Completed: Completed::<TProgress, Identity, OFFSET>,
            GetResults: GetResults::<TProgress, Identity, OFFSET>,
            TProgress: core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncActionWithProgress<TProgress> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncActionWithProgress_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
windows_core::imp::define_interface!(
    IAsyncInfo,
    IAsyncInfo_Vtbl,
    0x00000036_0000_0000_c000_000000000046
);
impl windows_core::RuntimeType for IAsyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IAsyncInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IAsyncInfo {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
pub trait IAsyncInfo_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: IAsyncInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Id(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut AsyncStatus,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Status(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::ErrorCode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAsyncInfo_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncInfo, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncInfo as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AsyncStatus,
    ) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncOperation<TResult>(windows_core::IUnknown, core::marker::PhantomData<TResult>)
where
    TResult: windows_core::RuntimeType + 'static;
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncOperation<TResult>
{
}
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncOperation<TResult>
{
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface
    for IAsyncOperation<TResult>
{
    type Vtable = IAsyncOperation_Vtbl<TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for IAsyncOperation<TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IAsyncInfo>
    for IAsyncOperation<TResult>
{
    const QUERY: bool = true;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationCompletedHandler<TResult>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncOperationCompletedHandler<TResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Sync for IAsyncOperation<TResult> {}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeName
    for IAsyncOperation<TResult>
{
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
pub trait IAsyncOperation_Impl<TResult>: IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
{
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<AsyncOperationCompletedHandler<TResult>>,
    ) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperation_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperation_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<TResult>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperation_Impl::GetResults(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAsyncOperation<TResult>,
                OFFSET,
            >(),
            SetCompleted: SetCompleted::<TResult, Identity, OFFSET>,
            Completed: Completed::<TResult, Identity, OFFSET>,
            GetResults: GetResults::<TResult, Identity, OFFSET>,
            TResult: core::marker::PhantomData::<TResult>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncOperation<TResult> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperation_Vtbl<TResult>
where
    TResult: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(
    windows_core::IUnknown,
    core::marker::PhantomData<TResult>,
    core::marker::PhantomData<TProgress>,
)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::imp::CanInto<windows_core::IUnknown>
    for IAsyncOperationWithProgress<TResult, TProgress>
{
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::imp::CanInto<windows_core::IInspectable>
    for IAsyncOperationWithProgress<TResult, TProgress>
{
}
unsafe impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::Interface for IAsyncOperationWithProgress<TResult, TProgress>
{
    type Vtable = IAsyncOperationWithProgress_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b";")
        .push_other(TProgress::SIGNATURE)
        .push_slice(b")");
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::imp::CanInto<IAsyncInfo> for IAsyncOperationWithProgress<TResult, TProgress>
{
    const QUERY: bool = true;
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > IAsyncOperationWithProgress<TResult, TProgress>
{
    pub fn SetProgress<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationProgressHandler<TResult, TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetProgress)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Progress(
        &self,
    ) -> windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCompleted)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(
        &self,
    ) -> windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > Send for IAsyncOperationWithProgress<TResult, TProgress>
{
}
unsafe impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > Sync for IAsyncOperationWithProgress<TResult, TProgress>
{
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > windows_core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress>
{
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
pub trait IAsyncOperationWithProgress_Impl<TResult, TProgress>: IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(
        &self,
        handler: windows_core::Ref<AsyncOperationProgressHandler<TResult, TProgress>>,
    ) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>,
    ) -> windows_core::Result<()>;
    fn Completed(
        &self,
    ) -> windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<
        TResult: windows_core::RuntimeType + 'static,
        TProgress: windows_core::RuntimeType + 'static,
    > IAsyncOperationWithProgress_Vtbl<TResult, TProgress>
{
    pub const fn new<
        Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
        const OFFSET: isize,
    >() -> Self {
        unsafe extern "system" fn SetProgress<
            TResult: windows_core::RuntimeType + 'static,
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperationWithProgress_Impl::SetProgress(
                    this,
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn Progress<
            TResult: windows_core::RuntimeType + 'static,
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperationWithProgress_Impl::Progress(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompleted<
            TResult: windows_core::RuntimeType + 'static,
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperationWithProgress_Impl::SetCompleted(
                    this,
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn Completed<
            TResult: windows_core::RuntimeType + 'static,
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperationWithProgress_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<
            TResult: windows_core::RuntimeType + 'static,
            TProgress: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<TResult>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperationWithProgress_Impl::GetResults(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAsyncOperationWithProgress<TResult, TProgress>,
                OFFSET,
            >(),
            SetProgress: SetProgress::<TResult, TProgress, Identity, OFFSET>,
            Progress: Progress::<TResult, TProgress, Identity, OFFSET>,
            SetCompleted: SetCompleted::<TResult, TProgress, Identity, OFFSET>,
            Completed: Completed::<TResult, TProgress, Identity, OFFSET>,
            GetResults: GetResults::<TResult, TProgress, Identity, OFFSET>,
            TResult: core::marker::PhantomData::<TResult>,
            TProgress: core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncOperationWithProgress<TResult, TProgress> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperationWithProgress_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
