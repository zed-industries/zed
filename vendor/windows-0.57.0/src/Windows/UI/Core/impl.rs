pub trait ICoreAcceleratorKeys_Impl: Sized {
    fn AcceleratorKeyActivated(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.ICoreAcceleratorKeys";
}
impl ICoreAcceleratorKeys_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>() -> ICoreAcceleratorKeys_Vtbl {
        unsafe extern "system" fn AcceleratorKeyActivated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreAcceleratorKeys_Impl::AcceleratorKeyActivated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceleratorKeyActivated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreAcceleratorKeys_Impl::RemoveAcceleratorKeyActivated(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAcceleratorKeys, OFFSET>(),
            AcceleratorKeyActivated: AcceleratorKeyActivated::<Identity, Impl, OFFSET>,
            RemoveAcceleratorKeyActivated: RemoveAcceleratorKeyActivated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreAcceleratorKeys as windows_core::Interface>::IID
    }
}
pub trait ICoreInputSourceBase_Impl: Sized {
    fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher>;
    fn IsInputEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn InputEnabled(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, InputEnabledEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreInputSourceBase {
    const NAME: &'static str = "Windows.UI.Core.ICoreInputSourceBase";
}
impl ICoreInputSourceBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>() -> ICoreInputSourceBase_Vtbl {
        unsafe extern "system" fn Dispatcher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreInputSourceBase_Impl::Dispatcher(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreInputSourceBase_Impl::IsInputEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreInputSourceBase_Impl::SetIsInputEnabled(this, value).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreInputSourceBase_Impl::InputEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreInputSourceBase_Impl::RemoveInputEnabled(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputSourceBase, OFFSET>(),
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreInputSourceBase as windows_core::Interface>::IID
    }
}
pub trait ICorePointerInputSource_Impl: Sized {
    fn ReleasePointerCapture(&self) -> windows_core::Result<()>;
    fn SetPointerCapture(&self) -> windows_core::Result<()>;
    fn HasCapture(&self) -> windows_core::Result<bool>;
    fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point>;
    fn PointerCursor(&self) -> windows_core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: Option<&CoreCursor>) -> windows_core::Result<()>;
    fn PointerCaptureLost(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerEntered(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerExited(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerMoved(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerPressed(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerReleased(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerWheelChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<windows_core::IInspectable, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorePointerInputSource {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerInputSource";
}
impl ICorePointerInputSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>() -> ICorePointerInputSource_Vtbl {
        unsafe extern "system" fn ReleasePointerCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::ReleasePointerCapture(this).into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::SetPointerCapture(this).into()
        }
        unsafe extern "system" fn HasCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::HasCapture(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerPosition(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerCursor(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::SetPointerCursor(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerCaptureLost(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerCaptureLost(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerEntered(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerEntered(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerExited(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerExited(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerMoved(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerMoved(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerPressed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerPressed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerReleased(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource_Impl::PointerWheelChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerInputSource_Impl::RemovePointerWheelChanged(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerInputSource, OFFSET>(),
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            HasCapture: HasCapture::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorePointerInputSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "System")]
pub trait ICorePointerInputSource2_Impl: Sized + ICorePointerInputSource_Impl {
    fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(feature = "System")]
impl windows_core::RuntimeName for ICorePointerInputSource2 {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerInputSource2";
}
#[cfg(feature = "System")]
impl ICorePointerInputSource2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: isize>() -> ICorePointerInputSource2_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerInputSource2_Impl::DispatcherQueue(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerInputSource2, OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorePointerInputSource2 as windows_core::Interface>::IID
    }
}
pub trait ICorePointerRedirector_Impl: Sized {
    fn PointerRoutedAway(&self, handler: Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedAway(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerRoutedTo(&self, handler: Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedTo(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerRoutedReleased(&self, handler: Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorePointerRedirector {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerRedirector";
}
impl ICorePointerRedirector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>() -> ICorePointerRedirector_Vtbl {
        unsafe extern "system" fn PointerRoutedAway<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerRedirector_Impl::PointerRoutedAway(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerRedirector_Impl::RemovePointerRoutedAway(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerRedirector_Impl::PointerRoutedTo(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerRedirector_Impl::RemovePointerRoutedTo(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICorePointerRedirector_Impl::PointerRoutedReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICorePointerRedirector_Impl::RemovePointerRoutedReleased(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerRedirector, OFFSET>(),
            PointerRoutedAway: PointerRoutedAway::<Identity, Impl, OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Identity, Impl, OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Identity, Impl, OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Identity, Impl, OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Identity, Impl, OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorePointerRedirector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
pub trait ICoreWindow_Impl: Sized {
    fn AutomationHostProvider(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn Bounds(&self) -> windows_core::Result<super::super::Foundation::Rect>;
    fn CustomProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn Dispatcher(&self) -> windows_core::Result<CoreDispatcher>;
    fn FlowDirection(&self) -> windows_core::Result<CoreWindowFlowDirection>;
    fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> windows_core::Result<()>;
    fn IsInputEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn PointerCursor(&self) -> windows_core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: Option<&CoreCursor>) -> windows_core::Result<()>;
    fn PointerPosition(&self) -> windows_core::Result<super::super::Foundation::Point>;
    fn Visible(&self) -> windows_core::Result<bool>;
    fn Activate(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates>;
    fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> windows_core::Result<CoreVirtualKeyStates>;
    fn ReleasePointerCapture(&self) -> windows_core::Result<()>;
    fn SetPointerCapture(&self) -> windows_core::Result<()>;
    fn Activated(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn AutomationProviderRequested(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutomationProviderRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn CharacterReceived(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn Closed(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn InputEnabled(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn KeyDown(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn KeyUp(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerCaptureLost(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerEntered(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerExited(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerMoved(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerPressed(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerReleased(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn TouchHitTesting(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn PointerWheelChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn SizeChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn VisibilityChanged(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl windows_core::RuntimeName for ICoreWindow {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow";
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl ICoreWindow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>() -> ICoreWindow_Vtbl {
        unsafe extern "system" fn AutomationHostProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::AutomationHostProvider(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bounds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::Bounds(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::CustomProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::Dispatcher(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::FlowDirection(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CoreWindowFlowDirection) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::SetFlowDirection(this, value).into()
        }
        unsafe extern "system" fn IsInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::IsInputEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::SetIsInputEnabled(this, value).into()
        }
        unsafe extern "system" fn PointerCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerCursor(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::SetPointerCursor(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn PointerPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerPosition(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::Visible(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetAsyncKeyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::GetAsyncKeyState(this, virtualkey) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::GetKeyState(this, virtualkey) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::ReleasePointerCapture(this).into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::SetPointerCapture(this).into()
        }
        unsafe extern "system" fn Activated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::Activated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveActivated(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn AutomationProviderRequested<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::AutomationProviderRequested(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutomationProviderRequested<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveAutomationProviderRequested(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn CharacterReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::CharacterReceived(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveCharacterReceived(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Closed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::Closed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveClosed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::InputEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveInputEnabled(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::KeyDown(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveKeyDown(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyUp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::KeyUp(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveKeyUp(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerCaptureLost(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerCaptureLost(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerEntered(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerEntered(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerExited(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerExited(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerMoved(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerMoved(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerPressed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerPressed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerReleased(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn TouchHitTesting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::TouchHitTesting(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveTouchHitTesting(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::PointerWheelChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemovePointerWheelChanged(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SizeChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::SizeChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveSizeChanged(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindow_Impl::VisibilityChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindow_Impl::RemoveVisibilityChanged(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindow, OFFSET>(),
            AutomationHostProvider: AutomationHostProvider::<Identity, Impl, OFFSET>,
            Bounds: Bounds::<Identity, Impl, OFFSET>,
            CustomProperties: CustomProperties::<Identity, Impl, OFFSET>,
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            FlowDirection: FlowDirection::<Identity, Impl, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetAsyncKeyState: GetAsyncKeyState::<Identity, Impl, OFFSET>,
            GetKeyState: GetKeyState::<Identity, Impl, OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            Activated: Activated::<Identity, Impl, OFFSET>,
            RemoveActivated: RemoveActivated::<Identity, Impl, OFFSET>,
            AutomationProviderRequested: AutomationProviderRequested::<Identity, Impl, OFFSET>,
            RemoveAutomationProviderRequested: RemoveAutomationProviderRequested::<Identity, Impl, OFFSET>,
            CharacterReceived: CharacterReceived::<Identity, Impl, OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            TouchHitTesting: TouchHitTesting::<Identity, Impl, OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
            SizeChanged: SizeChanged::<Identity, Impl, OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Identity, Impl, OFFSET>,
            VisibilityChanged: VisibilityChanged::<Identity, Impl, OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindow as windows_core::Interface>::IID
    }
}
pub trait ICoreWindowEventArgs_Impl: Sized {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowEventArgs";
}
impl ICoreWindowEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>() -> ICoreWindowEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICoreWindowEventArgs_Impl::Handled(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreWindowEventArgs_Impl::SetHandled(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindowEventArgs, OFFSET>(),
            Handled: Handled::<Identity, Impl, OFFSET>,
            SetHandled: SetHandled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreWindowEventArgs as windows_core::Interface>::IID
    }
}
pub trait IInitializeWithCoreWindow_Impl: Sized {
    fn Initialize(&self, window: Option<&CoreWindow>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInitializeWithCoreWindow {
    const NAME: &'static str = "Windows.UI.Core.IInitializeWithCoreWindow";
}
impl IInitializeWithCoreWindow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: isize>() -> IInitializeWithCoreWindow_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IInitializeWithCoreWindow_Impl::Initialize(this, windows_core::from_raw_borrowed(&window)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInitializeWithCoreWindow, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithCoreWindow as windows_core::Interface>::IID
    }
}
