windows_core::imp::define_interface!(IRemoteTextConnection, IRemoteTextConnection_Vtbl, 0x4e7bb02a_183e_5e66_b5e4_3e6e5c570cf1);
impl windows_core::RuntimeType for IRemoteTextConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteTextConnection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RegisterThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnregisterThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteTextConnectionFactory, IRemoteTextConnectionFactory_Vtbl, 0x88e075c2_0cae_596c_850f_78d345cd728b);
impl windows_core::RuntimeType for IRemoteTextConnectionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRemoteTextConnectionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RemoteTextConnection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteTextConnection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteTextConnection, super::super::super::Foundation::IClosable);
impl RemoteTextConnection {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
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
    pub fn RegisterThread(&self, threadid: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RegisterThread)(windows_core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn UnregisterThread(&self, threadid: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UnregisterThread)(windows_core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn ReportDataReceived(&self, pdudata: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportDataReceived)(windows_core::Interface::as_raw(this), pdudata.len().try_into().unwrap(), pdudata.as_ptr()).ok() }
    }
    pub fn CreateInstance<P0>(connectionid: windows_core::GUID, pduforwarder: P0) -> windows_core::Result<RemoteTextConnection>
    where
        P0: windows_core::Param<RemoteTextConnectionDataHandler>,
    {
        Self::IRemoteTextConnectionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), connectionid, pduforwarder.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRemoteTextConnectionFactory<R, F: FnOnce(&IRemoteTextConnectionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemoteTextConnection, IRemoteTextConnectionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RemoteTextConnection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteTextConnection>();
}
unsafe impl windows_core::Interface for RemoteTextConnection {
    type Vtable = IRemoteTextConnection_Vtbl;
    const IID: windows_core::GUID = <IRemoteTextConnection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteTextConnection {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.RemoteTextConnection";
}
unsafe impl Send for RemoteTextConnection {}
unsafe impl Sync for RemoteTextConnection {}
windows_core::imp::define_interface!(RemoteTextConnectionDataHandler, RemoteTextConnectionDataHandler_Vtbl, 0x099ffbc8_8bcb_41b5_b056_57e77021bf1b);
impl RemoteTextConnectionDataHandler {
    pub fn new<F: FnMut(&[u8]) -> windows_core::Result<bool> + Send + 'static>(invoke: F) -> Self {
        let com = RemoteTextConnectionDataHandlerBox::<F> { vtable: &RemoteTextConnectionDataHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke(&self, pdudata: &[u8]) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), pdudata.len().try_into().unwrap(), pdudata.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
struct RemoteTextConnectionDataHandlerBox<F: FnMut(&[u8]) -> windows_core::Result<bool> + Send + 'static> {
    vtable: *const RemoteTextConnectionDataHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(&[u8]) -> windows_core::Result<bool> + Send + 'static> RemoteTextConnectionDataHandlerBox<F> {
    const VTABLE: RemoteTextConnectionDataHandler_Vtbl = RemoteTextConnectionDataHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <RemoteTextConnectionDataHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        match (this.invoke)(core::slice::from_raw_parts(core::mem::transmute_copy(&pdudata), pduData_array_size as usize)) {
            Ok(ok__) => {
                core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
            Err(err) => err.into(),
        }
    }
}
impl windows_core::RuntimeType for RemoteTextConnectionDataHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RemoteTextConnectionDataHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut bool) -> windows_core::HRESULT,
}
