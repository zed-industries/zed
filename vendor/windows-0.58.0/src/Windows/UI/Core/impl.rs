pub trait ICoreAcceleratorKeys_Impl: Sized {
    fn AcceleratorKeyActivated(&self, handler: Option<&super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.ICoreAcceleratorKeys";
}
impl ICoreAcceleratorKeys_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreAcceleratorKeys_Vtbl
    where
        Identity: ICoreAcceleratorKeys_Impl,
    {
        unsafe extern "system" fn AcceleratorKeyActivated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreAcceleratorKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreAcceleratorKeys_Impl::AcceleratorKeyActivated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceleratorKeyActivated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreAcceleratorKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreAcceleratorKeys_Impl::RemoveAcceleratorKeyActivated(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreAcceleratorKeys, OFFSET>(),
            AcceleratorKeyActivated: AcceleratorKeyActivated::<Identity, OFFSET>,
            RemoveAcceleratorKeyActivated: RemoveAcceleratorKeyActivated::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreInputSourceBase_Vtbl
    where
        Identity: ICoreInputSourceBase_Impl,
    {
        unsafe extern "system" fn Dispatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreInputSourceBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreInputSourceBase_Impl::Dispatcher(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreInputSourceBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreInputSourceBase_Impl::IsInputEnabled(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: ICoreInputSourceBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreInputSourceBase_Impl::SetIsInputEnabled(this, value).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreInputSourceBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreInputSourceBase_Impl::InputEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreInputSourceBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreInputSourceBase_Impl::RemoveInputEnabled(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreInputSourceBase, OFFSET>(),
            Dispatcher: Dispatcher::<Identity, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, OFFSET>,
            InputEnabled: InputEnabled::<Identity, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorePointerInputSource_Vtbl
    where
        Identity: ICorePointerInputSource_Impl,
    {
        unsafe extern "system" fn ReleasePointerCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::ReleasePointerCapture(this).into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::SetPointerCapture(this).into()
        }
        unsafe extern "system" fn HasCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::HasCapture(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerPosition(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCursor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerCursor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::SetPointerCursor(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerCaptureLost(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerCaptureLost(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerEntered(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerEntered(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerExited(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerExited(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerMoved(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerMoved(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerPressed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerPressed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerReleased(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource_Impl::PointerWheelChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerInputSource_Impl::RemovePointerWheelChanged(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerInputSource, OFFSET>(),
            ReleasePointerCapture: ReleasePointerCapture::<Identity, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, OFFSET>,
            HasCapture: HasCapture::<Identity, OFFSET>,
            PointerPosition: PointerPosition::<Identity, OFFSET>,
            PointerCursor: PointerCursor::<Identity, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, OFFSET>,
            PointerEntered: PointerEntered::<Identity, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, OFFSET>,
            PointerExited: PointerExited::<Identity, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, OFFSET>,
            PointerMoved: PointerMoved::<Identity, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, OFFSET>,
            PointerPressed: PointerPressed::<Identity, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, OFFSET>,
            PointerReleased: PointerReleased::<Identity, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorePointerInputSource2_Vtbl
    where
        Identity: ICorePointerInputSource2_Impl,
    {
        unsafe extern "system" fn DispatcherQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorePointerInputSource2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerInputSource2_Impl::DispatcherQueue(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerInputSource2, OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorePointerRedirector_Vtbl
    where
        Identity: ICorePointerRedirector_Impl,
    {
        unsafe extern "system" fn PointerRoutedAway<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerRedirector_Impl::PointerRoutedAway(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerRedirector_Impl::RemovePointerRoutedAway(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerRedirector_Impl::PointerRoutedTo(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerRedirector_Impl::RemovePointerRoutedTo(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorePointerRedirector_Impl::PointerRoutedReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICorePointerRedirector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorePointerRedirector_Impl::RemovePointerRoutedReleased(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICorePointerRedirector, OFFSET>(),
            PointerRoutedAway: PointerRoutedAway::<Identity, OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Identity, OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Identity, OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Identity, OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Identity, OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreWindow_Vtbl
    where
        Identity: ICoreWindow_Impl,
    {
        unsafe extern "system" fn AutomationHostProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::AutomationHostProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::Bounds(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::CustomProperties(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::Dispatcher(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::FlowDirection(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CoreWindowFlowDirection) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::SetFlowDirection(this, value).into()
        }
        unsafe extern "system" fn IsInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::IsInputEnabled(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::SetIsInputEnabled(this, value).into()
        }
        unsafe extern "system" fn PointerCursor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerCursor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::SetPointerCursor(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn PointerPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerPosition(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::Visible(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::Close(this).into()
        }
        unsafe extern "system" fn GetAsyncKeyState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::GetAsyncKeyState(this, virtualkey) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::GetKeyState(this, virtualkey) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::ReleasePointerCapture(this).into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::SetPointerCapture(this).into()
        }
        unsafe extern "system" fn Activated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::Activated(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveActivated(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn AutomationProviderRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::AutomationProviderRequested(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutomationProviderRequested<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveAutomationProviderRequested(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn CharacterReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::CharacterReceived(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveCharacterReceived(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Closed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::Closed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveClosed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::InputEnabled(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveInputEnabled(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::KeyDown(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveKeyDown(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::KeyUp(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveKeyUp(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerCaptureLost(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerCaptureLost(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerEntered(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerEntered(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerExited(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerExited(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerMoved(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerMoved(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerPressed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerPressed(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerReleased(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerReleased(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn TouchHitTesting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::TouchHitTesting(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveTouchHitTesting(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::PointerWheelChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemovePointerWheelChanged(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SizeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::SizeChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveSizeChanged(this, core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindow_Impl::VisibilityChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: ICoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindow_Impl::RemoveVisibilityChanged(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindow, OFFSET>(),
            AutomationHostProvider: AutomationHostProvider::<Identity, OFFSET>,
            Bounds: Bounds::<Identity, OFFSET>,
            CustomProperties: CustomProperties::<Identity, OFFSET>,
            Dispatcher: Dispatcher::<Identity, OFFSET>,
            FlowDirection: FlowDirection::<Identity, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, OFFSET>,
            PointerCursor: PointerCursor::<Identity, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, OFFSET>,
            PointerPosition: PointerPosition::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetAsyncKeyState: GetAsyncKeyState::<Identity, OFFSET>,
            GetKeyState: GetKeyState::<Identity, OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Identity, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, OFFSET>,
            Activated: Activated::<Identity, OFFSET>,
            RemoveActivated: RemoveActivated::<Identity, OFFSET>,
            AutomationProviderRequested: AutomationProviderRequested::<Identity, OFFSET>,
            RemoveAutomationProviderRequested: RemoveAutomationProviderRequested::<Identity, OFFSET>,
            CharacterReceived: CharacterReceived::<Identity, OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, OFFSET>,
            Closed: Closed::<Identity, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, OFFSET>,
            InputEnabled: InputEnabled::<Identity, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Identity, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, OFFSET>,
            PointerEntered: PointerEntered::<Identity, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, OFFSET>,
            PointerExited: PointerExited::<Identity, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, OFFSET>,
            PointerMoved: PointerMoved::<Identity, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, OFFSET>,
            PointerPressed: PointerPressed::<Identity, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, OFFSET>,
            PointerReleased: PointerReleased::<Identity, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, OFFSET>,
            TouchHitTesting: TouchHitTesting::<Identity, OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Identity, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, OFFSET>,
            SizeChanged: SizeChanged::<Identity, OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Identity, OFFSET>,
            VisibilityChanged: VisibilityChanged::<Identity, OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreWindowEventArgs_Vtbl
    where
        Identity: ICoreWindowEventArgs_Impl,
    {
        unsafe extern "system" fn Handled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: ICoreWindowEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICoreWindowEventArgs_Impl::Handled(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: ICoreWindowEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreWindowEventArgs_Impl::SetHandled(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreWindowEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInitializeWithCoreWindow_Vtbl
    where
        Identity: IInitializeWithCoreWindow_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInitializeWithCoreWindow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInitializeWithCoreWindow_Impl::Initialize(this, windows_core::from_raw_borrowed(&window)).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInitializeWithCoreWindow, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeWithCoreWindow as windows_core::Interface>::IID
    }
}
