pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: Sized {
    fn IsComThreadingRequired(&self) -> bool;
}
impl windows_core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProvider_Vtbl
    where
        Identity: ICoreAutomationConnectionBoundObjectProvider_Impl,
    {
        unsafe extern "system" fn IsComThreadingRequired<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreAutomationConnectionBoundObjectProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            let ok__ = ICoreAutomationConnectionBoundObjectProvider_Impl::IsComThreadingRequired(this);
            result__.write(core::mem::transmute_copy(&ok__));
            windows_core::HRESULT(0)
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreAutomationConnectionBoundObjectProvider as windows_core::Interface>::IID
    }
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: Sized {
    fn CallExtension(&self, extensionid: &windows_core::GUID, context: Option<&CoreAutomationRemoteOperationContext>, operandids: &[AutomationRemoteOperationOperandId]) -> windows_core::Result<()>;
    fn IsExtensionSupported(&self, extensionid: &windows_core::GUID) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
impl ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProvider_Vtbl
    where
        Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl,
    {
        unsafe extern "system" fn CallExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensionid: windows_core::GUID, context: *mut core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> windows_core::HRESULT
        where
            Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreAutomationRemoteOperationExtensionProvider_Impl::CallExtension(this, core::mem::transmute(&extensionid), windows_core::from_raw_borrowed(&context), core::slice::from_raw_parts(core::mem::transmute_copy(&operandids), operandIds_array_size as usize)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, extensionid: windows_core::GUID, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreAutomationRemoteOperationExtensionProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreAutomationRemoteOperationExtensionProvider_Impl::IsExtensionSupported(this, core::mem::transmute(&extensionid)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, OFFSET>(),
            CallExtension: CallExtension::<Identity, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationExtensionProvider as windows_core::Interface>::IID
    }
}
