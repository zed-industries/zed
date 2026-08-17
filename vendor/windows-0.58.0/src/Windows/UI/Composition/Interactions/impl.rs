pub trait ICompositionInteractionSource_Impl: Sized {}
impl windows_core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICompositionInteractionSource_Vtbl
    where
        Identity: ICompositionInteractionSource_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionInteractionSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as windows_core::Interface>::IID
    }
}
pub trait IInteractionTrackerOwner_Impl: Sized {
    fn CustomAnimationStateEntered(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerCustomAnimationStateEnteredArgs>) -> windows_core::Result<()>;
    fn IdleStateEntered(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerIdleStateEnteredArgs>) -> windows_core::Result<()>;
    fn InertiaStateEntered(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerInertiaStateEnteredArgs>) -> windows_core::Result<()>;
    fn InteractingStateEntered(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerInteractingStateEnteredArgs>) -> windows_core::Result<()>;
    fn RequestIgnored(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerRequestIgnoredArgs>) -> windows_core::Result<()>;
    fn ValuesChanged(&self, sender: Option<&InteractionTracker>, args: Option<&InteractionTrackerValuesChangedArgs>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
impl IInteractionTrackerOwner_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInteractionTrackerOwner_Vtbl
    where
        Identity: IInteractionTrackerOwner_Impl,
    {
        unsafe extern "system" fn CustomAnimationStateEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::CustomAnimationStateEntered(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::IdleStateEntered(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::InertiaStateEntered(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::InteractingStateEntered(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn RequestIgnored<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::RequestIgnored(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn ValuesChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInteractionTrackerOwner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInteractionTrackerOwner_Impl::ValuesChanged(this, windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInteractionTrackerOwner, OFFSET>(),
            CustomAnimationStateEntered: CustomAnimationStateEntered::<Identity, OFFSET>,
            IdleStateEntered: IdleStateEntered::<Identity, OFFSET>,
            InertiaStateEntered: InertiaStateEntered::<Identity, OFFSET>,
            InteractingStateEntered: InteractingStateEntered::<Identity, OFFSET>,
            RequestIgnored: RequestIgnored::<Identity, OFFSET>,
            ValuesChanged: ValuesChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInteractionTrackerOwner as windows_core::Interface>::IID
    }
}
