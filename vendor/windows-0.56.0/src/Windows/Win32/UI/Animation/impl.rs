pub trait IUIAnimationInterpolator_Impl: Sized {
    fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn InterpolateValue(&self, offset: f64) -> windows_core::Result<f64>;
    fn InterpolateVelocity(&self, offset: f64) -> windows_core::Result<f64>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator {}
impl IUIAnimationInterpolator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator_Vtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator_Impl::GetDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(duration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(velocity, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationInterpolator2_Impl: Sized {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateValue(&self, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateVelocity(&self, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPrimitiveInterpolation(&self, interpolation: Option<&IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator2 {}
impl IUIAnimationInterpolator2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    core::ptr::write(dimension, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationInterpolator2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(duration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::GetFinalValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolation: *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::GetPrimitiveInterpolation(this, windows_core::from_raw_borrowed(&interpolation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetDependencies<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationInterpolator2_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, Impl, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, Impl, OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationLoopIterationChangeHandler2_Impl: Sized {
    fn OnLoopIterationChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationLoopIterationChangeHandler2 {}
impl IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2_Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationLoopIterationChangeHandler2_Impl::OnLoopIterationChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&id), core::mem::transmute_copy(&newiterationcount), core::mem::transmute_copy(&olditerationcount)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManager_Impl: Sized {
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler>) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager {}
impl IUIAnimationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>() -> IUIAnimationManager_Vtbl {
        unsafe extern "system" fn CreateAnimationVariable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    core::ptr::write(variable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    core::ptr::write(variable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManager2_Impl: Sized {
    fn CreateAnimationVectorVariable(&self, initialvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(&self) -> windows_core::Result<f64>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager2 {}
impl IUIAnimationManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>() -> IUIAnimationManager2_Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::CreateAnimationVectorVariable(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    core::ptr::write(variable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    core::ptr::write(variable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    core::ptr::write(variable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::EstimateNextEventTime(this) {
                Ok(ok__) => {
                    core::ptr::write(seconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationManager2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManager2_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Identity, Impl, OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, Impl, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, Impl, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, Impl, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, Impl, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, Impl, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, Impl, OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, Impl, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, Impl, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, Impl, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, Impl, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, Impl, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler_Impl: Sized {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler {}
impl IUIAnimationManagerEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManagerEventHandler_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationManagerEventHandler2_Impl: Sized {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler2 {}
impl IUIAnimationManagerEventHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler2_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationManagerEventHandler2_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPrimitiveInterpolation_Impl: Sized {
    fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPrimitiveInterpolation {}
impl IUIAnimationPrimitiveInterpolation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>() -> IUIAnimationPrimitiveInterpolation_Vtbl {
        unsafe extern "system" fn AddCubic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationPrimitiveInterpolation_Impl::AddCubic(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationPrimitiveInterpolation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddCubic: AddCubic::<Identity, Impl, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPrimitiveInterpolation as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison_Impl: Sized {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard>, newstoryboard: Option<&IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison {}
impl IUIAnimationPriorityComparison_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationPriorityComparison_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationPriorityComparison2_Impl: Sized {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard2>, newstoryboard: Option<&IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison2 {}
impl IUIAnimationPriorityComparison2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison2_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationPriorityComparison2_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboard_Impl: Sized {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard {}
impl IUIAnimationStoryboard_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(keyframe, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    core::ptr::write(keyframe, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&repetitioncount)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Schedule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    core::ptr::write(elapsedtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboard2_Impl: Sized {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: Option<&IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn SetSkipDuration(&self, secondsduration: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard2 {}
impl IUIAnimationStoryboard2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard2_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard2_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    core::ptr::write(keyframe, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard2_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    core::ptr::write(keyframe, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&crepetition), core::mem::transmute_copy(&repeatmode), windows_core::from_raw_borrowed(&piterationchangehandler), core::mem::transmute_copy(&id), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn SetSkipDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, secondsduration: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::SetSkipDuration(this, core::mem::transmute_copy(&secondsduration)).into()
        }
        unsafe extern "system" fn Schedule<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    core::ptr::write(status, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationStoryboard2_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    core::ptr::write(elapsedtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboard2_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, Impl, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, Impl, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, Impl, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, Impl, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, Impl, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, Impl, OFFSET>,
            HoldVariable: HoldVariable::<Identity, Impl, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, Impl, OFFSET>,
            SetSkipDuration: SetSkipDuration::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            Conclude: Conclude::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, Impl, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler_Impl: Sized {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler {}
impl IUIAnimationStoryboardEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationStoryboardEventHandler2_Impl: Sized {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler2 {}
impl IUIAnimationStoryboardEventHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, Impl, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimer_Impl: Sized {
    fn SetTimerUpdateHandler(&self, updatehandler: Option<&IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::Result<()>;
    fn SetTimerEventHandler(&self, handler: Option<&IUIAnimationTimerEventHandler>) -> windows_core::Result<()>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<()>;
    fn GetTime(&self) -> windows_core::Result<f64>;
    fn SetFrameRateThreshold(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimer {}
impl IUIAnimationTimer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>() -> IUIAnimationTimer_Vtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatehandler: *mut core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::SetTimerUpdateHandler(this, windows_core::from_raw_borrowed(&updatehandler), core::mem::transmute_copy(&idlebehavior)).into()
        }
        unsafe extern "system" fn SetTimerEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::SetTimerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Disable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::Disable(this).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::IsEnabled(this).into()
        }
        unsafe extern "system" fn GetTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTimer_Impl::GetTime(this) {
                Ok(ok__) => {
                    core::ptr::write(seconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimer_Impl::SetFrameRateThreshold(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Identity, Impl, OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimer as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerClientEventHandler_Impl: Sized {
    fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerClientEventHandler {}
impl IUIAnimationTimerClientEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerClientEventHandler_Vtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerClientEventHandler_Impl::OnTimerClientStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerEventHandler_Impl: Sized {
    fn OnPreUpdate(&self) -> windows_core::Result<()>;
    fn OnPostUpdate(&self) -> windows_core::Result<()>;
    fn OnRenderingTooSlow(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerEventHandler {}
impl IUIAnimationTimerEventHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerEventHandler_Vtbl {
        unsafe extern "system" fn OnPreUpdate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerEventHandler_Impl::OnPreUpdate(this).into()
        }
        unsafe extern "system" fn OnPostUpdate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerEventHandler_Impl::OnPostUpdate(this).into()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerEventHandler_Impl::OnRenderingTooSlow(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPreUpdate: OnPreUpdate::<Identity, Impl, OFFSET>,
            OnPostUpdate: OnPostUpdate::<Identity, Impl, OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerEventHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTimerUpdateHandler_Impl: Sized {
    fn OnUpdate(&self, timenow: f64) -> windows_core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(&self, handler: Option<&IUIAnimationTimerClientEventHandler>) -> windows_core::Result<()>;
    fn ClearTimerClientEventHandler(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerUpdateHandler {}
impl IUIAnimationTimerUpdateHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerUpdateHandler_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTimerUpdateHandler_Impl::OnUpdate(this, core::mem::transmute_copy(&timenow)) {
                Ok(ok__) => {
                    core::ptr::write(result, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerUpdateHandler_Impl::SetTimerClientEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTimerUpdateHandler_Impl::ClearTimerClientEventHandler(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Identity, Impl, OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerUpdateHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransition_Impl: Sized {
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition {}
impl IUIAnimationTransition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>() -> IUIAnimationTransition_Vtbl {
        unsafe extern "system" fn SetInitialValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransition_Impl::GetDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(duration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransition2_Impl: Sized {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVectorValue(&self, value: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn SetInitialVectorVelocity(&self, velocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition2 {}
impl IUIAnimationTransition2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>() -> IUIAnimationTransition2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransition2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    core::ptr::write(dimension, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition2_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition2_Impl::SetInitialVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition2_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition2_Impl::SetInitialVectorVelocity(this, core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationTransition2_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransition2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(duration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            SetInitialValue: SetInitialValue::<Identity, Impl, OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Identity, Impl, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, Impl, OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Identity, Impl, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory_Impl: Sized {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator>) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory {}
impl IUIAnimationTransitionFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionFactory_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionFactory2_Impl: Sized {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator2>) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory2 {}
impl IUIAnimationTransitionFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory2_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionFactory2_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary_Impl: Sized {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary {}
impl IUIAnimationTransitionLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationTransitionLibrary2_Impl: Sized {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateInstantaneousVectorTransition(&self, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary2 {}
impl IUIAnimationTransitionLibrary2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary2_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousVectorTransition(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteVectorTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    core::ptr::write(transition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    core::ptr::write(pptransition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    core::ptr::write(pptransition, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, Impl, OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Identity, Impl, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, Impl, OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Identity, Impl, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, Impl, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, Impl, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, Impl, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, Impl, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, Impl, OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Identity, Impl, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, Impl, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Identity, Impl, OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariable_Impl: Sized {
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler>) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariable {}
impl IUIAnimationVariable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>() -> IUIAnimationVariable_Vtbl {
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    core::ptr::write(finalvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    core::ptr::write(previousvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(finalvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(previousvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
pub trait IUIAnimationVariable2_Impl: Sized {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetVectorValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurve(&self, animation: Option<&super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> windows_core::Result<()>;
    fn GetVectorCurve(&self, animation: *const Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetFinalVectorValue(&self, finalvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousVectorValue(&self, previousvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetIntegerVectorValue(&self, value: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerVectorValue(&self, finalvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerVectorValue(&self, previousvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetLowerBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableCurveChangeHandler(&self, handler: Option<&IUIAnimationVariableCurveChangeHandler2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl windows_core::RuntimeName for IUIAnimationVariable2 {}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl IUIAnimationVariable2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>() -> IUIAnimationVariable2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    core::ptr::write(dimension, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurve<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetCurve(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn GetVectorCurve<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *const *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetVectorCurve(this, core::mem::transmute_copy(&animation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    core::ptr::write(finalvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetFinalVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    core::ptr::write(previousvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetPreviousVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(value, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetIntegerVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(finalvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetFinalIntegerVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    core::ptr::write(previousvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetPreviousIntegerVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUIAnimationVariable2_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    core::ptr::write(storyboard, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetLowerBoundVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetLowerBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBoundVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetUpperBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariable2_Impl::SetVariableCurveChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetVectorValue: GetVectorValue::<Identity, Impl, OFFSET>,
            GetCurve: GetCurve::<Identity, Impl, OFFSET>,
            GetVectorCurve: GetVectorCurve::<Identity, Impl, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, Impl, OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, Impl, OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Identity, Impl, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, Impl, OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, Impl, OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, Impl, OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Identity, Impl, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, Impl, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, Impl, OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Identity, Impl, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, Impl, OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Identity, Impl, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, Impl, OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler_Impl: Sized {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler {}
impl IUIAnimationVariableChangeHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: f64, previousvalue: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariableChangeHandler_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableChangeHandler2_Impl: Sized {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler2 {}
impl IUIAnimationVariableChangeHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler2_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariableChangeHandler2_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableCurveChangeHandler2_Impl: Sized {
    fn OnCurveChanged(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableCurveChangeHandler2 {}
impl IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2_Vtbl {
        unsafe extern "system" fn OnCurveChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariableCurveChangeHandler2_Impl::OnCurveChanged(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCurveChanged: OnCurveChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler_Impl: Sized {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler {}
impl IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: i32, previousvalue: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariableIntegerChangeHandler_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as windows_core::Interface>::IID
    }
}
pub trait IUIAnimationVariableIntegerChangeHandler2_Impl: Sized {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler2 {}
impl IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IUIAnimationVariableIntegerChangeHandler2_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as windows_core::Interface>::IID
    }
}
