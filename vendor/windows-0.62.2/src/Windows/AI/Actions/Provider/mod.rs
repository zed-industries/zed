windows_core::imp::define_interface!(IActionFeedbackHandler, IActionFeedbackHandler_Vtbl, 0xa3fc3c51_a8c6_52c8_ad77_37bf3e2b565c);
impl windows_core::RuntimeType for IActionFeedbackHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IActionFeedbackHandler, windows_core::IUnknown, windows_core::IInspectable);
impl IActionFeedbackHandler {
    pub fn ProcessFeedbackAsync<P0, P1>(&self, context: P0, feedback: P1) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ActionInvocationContext>,
        P1: windows_core::Param<super::ActionFeedback>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessFeedbackAsync)(windows_core::Interface::as_raw(this), context.param().abi(), feedback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IActionFeedbackHandler {
    const NAME: &'static str = "Windows.AI.Actions.Provider.IActionFeedbackHandler";
}
pub trait IActionFeedbackHandler_Impl: windows_core::IUnknownImpl {
    fn ProcessFeedbackAsync(&self, context: windows_core::Ref<super::ActionInvocationContext>, feedback: windows_core::Ref<super::ActionFeedback>) -> windows_core::Result<windows_future::IAsyncAction>;
}
impl IActionFeedbackHandler_Vtbl {
    pub const fn new<Identity: IActionFeedbackHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessFeedbackAsync<Identity: IActionFeedbackHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void, feedback: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionFeedbackHandler_Impl::ProcessFeedbackAsync(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&feedback)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActionFeedbackHandler, OFFSET>(),
            ProcessFeedbackAsync: ProcessFeedbackAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionFeedbackHandler as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionFeedbackHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProcessFeedbackAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionProvider, IActionProvider_Vtbl, 0x62906c47_3d07_55f1_aefa_1522505afbbe);
impl windows_core::RuntimeType for IActionProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IActionProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IActionProvider {
    pub fn InvokeAsync<P0>(&self, context: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::ActionInvocationContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokeAsync)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IActionProvider {
    const NAME: &'static str = "Windows.AI.Actions.Provider.IActionProvider";
}
pub trait IActionProvider_Impl: windows_core::IUnknownImpl {
    fn InvokeAsync(&self, context: windows_core::Ref<super::ActionInvocationContext>) -> windows_core::Result<windows_future::IAsyncAction>;
}
impl IActionProvider_Vtbl {
    pub const fn new<Identity: IActionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeAsync<Identity: IActionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionProvider_Impl::InvokeAsync(this, core::mem::transmute_copy(&context)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActionProvider, OFFSET>(), InvokeAsync: InvokeAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvokeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
