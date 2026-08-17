pub trait ICoreDropOperationTarget_Impl: Sized {
    fn EnterAsync(&self, draginfo: Option<&CoreDragInfo>, draguioverride: Option<&CoreDragUIOverride>) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(&self, draginfo: Option<&CoreDragInfo>, draguioverride: Option<&CoreDragUIOverride>) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(&self, draginfo: Option<&CoreDragInfo>) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(&self, draginfo: Option<&CoreDragInfo>) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
impl windows_core::RuntimeName for ICoreDropOperationTarget {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDropOperationTarget";
}
impl ICoreDropOperationTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>() -> ICoreDropOperationTarget_Vtbl {
        unsafe extern "system" fn EnterAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, draginfo: *mut core::ffi::c_void, draguioverride: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreDropOperationTarget_Impl::EnterAsync(this, windows_core::from_raw_borrowed(&draginfo), windows_core::from_raw_borrowed(&draguioverride)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, draginfo: *mut core::ffi::c_void, draguioverride: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreDropOperationTarget_Impl::OverAsync(this, windows_core::from_raw_borrowed(&draginfo), windows_core::from_raw_borrowed(&draguioverride)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, draginfo: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreDropOperationTarget_Impl::LeaveAsync(this, windows_core::from_raw_borrowed(&draginfo)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, draginfo: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreDropOperationTarget_Impl::DropAsync(this, windows_core::from_raw_borrowed(&draginfo)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreDropOperationTarget, OFFSET>(),
            EnterAsync: EnterAsync::<Identity, Impl, OFFSET>,
            OverAsync: OverAsync::<Identity, Impl, OFFSET>,
            LeaveAsync: LeaveAsync::<Identity, Impl, OFFSET>,
            DropAsync: DropAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreDropOperationTarget as windows_core::Interface>::IID
    }
}
