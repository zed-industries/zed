pub trait IInertiaProcessor_Impl: Sized {
    fn InitialOriginX(&self) -> windows_core::Result<f32>;
    fn SetInitialOriginX(&self, x: f32) -> windows_core::Result<()>;
    fn InitialOriginY(&self) -> windows_core::Result<f32>;
    fn SetInitialOriginY(&self, y: f32) -> windows_core::Result<()>;
    fn InitialVelocityX(&self) -> windows_core::Result<f32>;
    fn SetInitialVelocityX(&self, x: f32) -> windows_core::Result<()>;
    fn InitialVelocityY(&self) -> windows_core::Result<f32>;
    fn SetInitialVelocityY(&self, y: f32) -> windows_core::Result<()>;
    fn InitialAngularVelocity(&self) -> windows_core::Result<f32>;
    fn SetInitialAngularVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn InitialExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn SetInitialExpansionVelocity(&self, velocity: f32) -> windows_core::Result<()>;
    fn InitialRadius(&self) -> windows_core::Result<f32>;
    fn SetInitialRadius(&self, radius: f32) -> windows_core::Result<()>;
    fn BoundaryLeft(&self) -> windows_core::Result<f32>;
    fn SetBoundaryLeft(&self, left: f32) -> windows_core::Result<()>;
    fn BoundaryTop(&self) -> windows_core::Result<f32>;
    fn SetBoundaryTop(&self, top: f32) -> windows_core::Result<()>;
    fn BoundaryRight(&self) -> windows_core::Result<f32>;
    fn SetBoundaryRight(&self, right: f32) -> windows_core::Result<()>;
    fn BoundaryBottom(&self) -> windows_core::Result<f32>;
    fn SetBoundaryBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn ElasticMarginLeft(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginLeft(&self, left: f32) -> windows_core::Result<()>;
    fn ElasticMarginTop(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginTop(&self, top: f32) -> windows_core::Result<()>;
    fn ElasticMarginRight(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginRight(&self, right: f32) -> windows_core::Result<()>;
    fn ElasticMarginBottom(&self) -> windows_core::Result<f32>;
    fn SetElasticMarginBottom(&self, bottom: f32) -> windows_core::Result<()>;
    fn DesiredDisplacement(&self) -> windows_core::Result<f32>;
    fn SetDesiredDisplacement(&self, displacement: f32) -> windows_core::Result<()>;
    fn DesiredRotation(&self) -> windows_core::Result<f32>;
    fn SetDesiredRotation(&self, rotation: f32) -> windows_core::Result<()>;
    fn DesiredExpansion(&self) -> windows_core::Result<f32>;
    fn SetDesiredExpansion(&self, expansion: f32) -> windows_core::Result<()>;
    fn DesiredDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn DesiredAngularDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn DesiredExpansionDeceleration(&self) -> windows_core::Result<f32>;
    fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> windows_core::Result<()>;
    fn InitialTimestamp(&self) -> windows_core::Result<u32>;
    fn SetInitialTimestamp(&self, timestamp: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Process(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn ProcessTime(&self, timestamp: u32) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn CompleteTime(&self, timestamp: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInertiaProcessor {}
impl IInertiaProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInertiaProcessor_Vtbl
    where
        Identity: IInertiaProcessor_Impl,
    {
        unsafe extern "system" fn InitialOriginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialOriginX(this) {
                Ok(ok__) => {
                    x.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialOriginX(this, core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialOriginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialOriginY(this) {
                Ok(ok__) => {
                    y.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialOriginY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialOriginY(this, core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialVelocityX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialVelocityX(this) {
                Ok(ok__) => {
                    x.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialVelocityX(this, core::mem::transmute_copy(&x)).into()
        }
        unsafe extern "system" fn InitialVelocityY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialVelocityY(this) {
                Ok(ok__) => {
                    y.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialVelocityY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, y: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialVelocityY(this, core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn InitialAngularVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialAngularVelocity(this) {
                Ok(ok__) => {
                    velocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialAngularVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialAngularVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialExpansionVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialExpansionVelocity(this) {
                Ok(ok__) => {
                    velocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialExpansionVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialExpansionVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn InitialRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialRadius(this) {
                Ok(ok__) => {
                    radius.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, radius: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialRadius(this, core::mem::transmute_copy(&radius)).into()
        }
        unsafe extern "system" fn BoundaryLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::BoundaryLeft(this) {
                Ok(ok__) => {
                    left.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetBoundaryLeft(this, core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn BoundaryTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::BoundaryTop(this) {
                Ok(ok__) => {
                    top.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetBoundaryTop(this, core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn BoundaryRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::BoundaryRight(this) {
                Ok(ok__) => {
                    right.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetBoundaryRight(this, core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn BoundaryBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::BoundaryBottom(this) {
                Ok(ok__) => {
                    bottom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoundaryBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetBoundaryBottom(this, core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn ElasticMarginLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::ElasticMarginLeft(this) {
                Ok(ok__) => {
                    left.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginLeft<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetElasticMarginLeft(this, core::mem::transmute_copy(&left)).into()
        }
        unsafe extern "system" fn ElasticMarginTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::ElasticMarginTop(this) {
                Ok(ok__) => {
                    top.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginTop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, top: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetElasticMarginTop(this, core::mem::transmute_copy(&top)).into()
        }
        unsafe extern "system" fn ElasticMarginRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::ElasticMarginRight(this) {
                Ok(ok__) => {
                    right.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginRight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, right: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetElasticMarginRight(this, core::mem::transmute_copy(&right)).into()
        }
        unsafe extern "system" fn ElasticMarginBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::ElasticMarginBottom(this) {
                Ok(ok__) => {
                    bottom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElasticMarginBottom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bottom: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetElasticMarginBottom(this, core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredDisplacement(this) {
                Ok(ok__) => {
                    displacement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displacement: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredDisplacement(this, core::mem::transmute_copy(&displacement)).into()
        }
        unsafe extern "system" fn DesiredRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredRotation(this) {
                Ok(ok__) => {
                    rotation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredRotation(this, core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn DesiredExpansion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredExpansion(this) {
                Ok(ok__) => {
                    expansion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansion: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredExpansion(this, core::mem::transmute_copy(&expansion)).into()
        }
        unsafe extern "system" fn DesiredDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredDeceleration(this) {
                Ok(ok__) => {
                    deceleration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredAngularDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredAngularDeceleration(this) {
                Ok(ok__) => {
                    deceleration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAngularDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredAngularDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn DesiredExpansionDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: *mut f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::DesiredExpansionDeceleration(this) {
                Ok(ok__) => {
                    deceleration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansionDeceleration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deceleration: f32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetDesiredExpansionDeceleration(this, core::mem::transmute_copy(&deceleration)).into()
        }
        unsafe extern "system" fn InitialTimestamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: *mut u32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::InitialTimestamp(this) {
                Ok(ok__) => {
                    timestamp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialTimestamp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::SetInitialTimestamp(this, core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Process<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::Process(this) {
                Ok(ok__) => {
                    completed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInertiaProcessor_Impl::ProcessTime(this, core::mem::transmute_copy(&timestamp)) {
                Ok(ok__) => {
                    completed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::Complete(this).into()
        }
        unsafe extern "system" fn CompleteTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timestamp: u32) -> windows_core::HRESULT
        where
            Identity: IInertiaProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInertiaProcessor_Impl::CompleteTime(this, core::mem::transmute_copy(&timestamp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitialOriginX: InitialOriginX::<Identity, OFFSET>,
            SetInitialOriginX: SetInitialOriginX::<Identity, OFFSET>,
            InitialOriginY: InitialOriginY::<Identity, OFFSET>,
            SetInitialOriginY: SetInitialOriginY::<Identity, OFFSET>,
            InitialVelocityX: InitialVelocityX::<Identity, OFFSET>,
            SetInitialVelocityX: SetInitialVelocityX::<Identity, OFFSET>,
            InitialVelocityY: InitialVelocityY::<Identity, OFFSET>,
            SetInitialVelocityY: SetInitialVelocityY::<Identity, OFFSET>,
            InitialAngularVelocity: InitialAngularVelocity::<Identity, OFFSET>,
            SetInitialAngularVelocity: SetInitialAngularVelocity::<Identity, OFFSET>,
            InitialExpansionVelocity: InitialExpansionVelocity::<Identity, OFFSET>,
            SetInitialExpansionVelocity: SetInitialExpansionVelocity::<Identity, OFFSET>,
            InitialRadius: InitialRadius::<Identity, OFFSET>,
            SetInitialRadius: SetInitialRadius::<Identity, OFFSET>,
            BoundaryLeft: BoundaryLeft::<Identity, OFFSET>,
            SetBoundaryLeft: SetBoundaryLeft::<Identity, OFFSET>,
            BoundaryTop: BoundaryTop::<Identity, OFFSET>,
            SetBoundaryTop: SetBoundaryTop::<Identity, OFFSET>,
            BoundaryRight: BoundaryRight::<Identity, OFFSET>,
            SetBoundaryRight: SetBoundaryRight::<Identity, OFFSET>,
            BoundaryBottom: BoundaryBottom::<Identity, OFFSET>,
            SetBoundaryBottom: SetBoundaryBottom::<Identity, OFFSET>,
            ElasticMarginLeft: ElasticMarginLeft::<Identity, OFFSET>,
            SetElasticMarginLeft: SetElasticMarginLeft::<Identity, OFFSET>,
            ElasticMarginTop: ElasticMarginTop::<Identity, OFFSET>,
            SetElasticMarginTop: SetElasticMarginTop::<Identity, OFFSET>,
            ElasticMarginRight: ElasticMarginRight::<Identity, OFFSET>,
            SetElasticMarginRight: SetElasticMarginRight::<Identity, OFFSET>,
            ElasticMarginBottom: ElasticMarginBottom::<Identity, OFFSET>,
            SetElasticMarginBottom: SetElasticMarginBottom::<Identity, OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Identity, OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Identity, OFFSET>,
            DesiredRotation: DesiredRotation::<Identity, OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Identity, OFFSET>,
            DesiredExpansion: DesiredExpansion::<Identity, OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Identity, OFFSET>,
            DesiredDeceleration: DesiredDeceleration::<Identity, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, OFFSET>,
            DesiredAngularDeceleration: DesiredAngularDeceleration::<Identity, OFFSET>,
            SetDesiredAngularDeceleration: SetDesiredAngularDeceleration::<Identity, OFFSET>,
            DesiredExpansionDeceleration: DesiredExpansionDeceleration::<Identity, OFFSET>,
            SetDesiredExpansionDeceleration: SetDesiredExpansionDeceleration::<Identity, OFFSET>,
            InitialTimestamp: InitialTimestamp::<Identity, OFFSET>,
            SetInitialTimestamp: SetInitialTimestamp::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            ProcessTime: ProcessTime::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            CompleteTime: CompleteTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaProcessor as windows_core::Interface>::IID
    }
}
pub trait IManipulationProcessor_Impl: Sized {
    fn SupportedManipulations(&self) -> windows_core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS>;
    fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::Result<()>;
    fn PivotPointX(&self) -> windows_core::Result<f32>;
    fn SetPivotPointX(&self, pivotpointx: f32) -> windows_core::Result<()>;
    fn PivotPointY(&self) -> windows_core::Result<f32>;
    fn SetPivotPointY(&self, pivotpointy: f32) -> windows_core::Result<()>;
    fn PivotRadius(&self) -> windows_core::Result<f32>;
    fn SetPivotRadius(&self, pivotradius: f32) -> windows_core::Result<()>;
    fn CompleteManipulation(&self) -> windows_core::Result<()>;
    fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> windows_core::Result<()>;
    fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::Result<()>;
    fn GetVelocityX(&self) -> windows_core::Result<f32>;
    fn GetVelocityY(&self) -> windows_core::Result<f32>;
    fn GetExpansionVelocity(&self) -> windows_core::Result<f32>;
    fn GetAngularVelocity(&self) -> windows_core::Result<f32>;
    fn MinimumScaleRotateRadius(&self) -> windows_core::Result<f32>;
    fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManipulationProcessor {}
impl IManipulationProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManipulationProcessor_Vtbl
    where
        Identity: IManipulationProcessor_Impl,
    {
        unsafe extern "system" fn SupportedManipulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::SupportedManipulations(this) {
                Ok(ok__) => {
                    manipulations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedManipulations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::SetSupportedManipulations(this, core::mem::transmute_copy(&manipulations)).into()
        }
        unsafe extern "system" fn PivotPointX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::PivotPointX(this) {
                Ok(ok__) => {
                    pivotpointx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointx: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::SetPivotPointX(this, core::mem::transmute_copy(&pivotpointx)).into()
        }
        unsafe extern "system" fn PivotPointY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::PivotPointY(this) {
                Ok(ok__) => {
                    pivotpointy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotPointY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotpointy: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::SetPivotPointY(this, core::mem::transmute_copy(&pivotpointy)).into()
        }
        unsafe extern "system" fn PivotRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::PivotRadius(this) {
                Ok(ok__) => {
                    pivotradius.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivotRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pivotradius: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::SetPivotRadius(this, core::mem::transmute_copy(&pivotradius)).into()
        }
        unsafe extern "system" fn CompleteManipulation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::CompleteManipulation(this).into()
        }
        unsafe extern "system" fn ProcessDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessDown(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessMove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessMove(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessUp(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ProcessDownWithTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessDownWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessMoveWithTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessMoveWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn ProcessUpWithTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::ProcessUpWithTime(this, core::mem::transmute_copy(&manipulatorid), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn GetVelocityX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityx: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::GetVelocityX(this) {
                Ok(ok__) => {
                    velocityx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocityY<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocityy: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::GetVelocityY(this) {
                Ok(ok__) => {
                    velocityy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpansionVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expansionvelocity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::GetExpansionVelocity(this) {
                Ok(ok__) => {
                    expansionvelocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAngularVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, angularvelocity: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::GetAngularVelocity(this) {
                Ok(ok__) => {
                    angularvelocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimumScaleRotateRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: *mut f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManipulationProcessor_Impl::MinimumScaleRotateRadius(this) {
                Ok(ok__) => {
                    minradius.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScaleRotateRadius<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minradius: f32) -> windows_core::HRESULT
        where
            Identity: IManipulationProcessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManipulationProcessor_Impl::SetMinimumScaleRotateRadius(this, core::mem::transmute_copy(&minradius)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedManipulations: SupportedManipulations::<Identity, OFFSET>,
            SetSupportedManipulations: SetSupportedManipulations::<Identity, OFFSET>,
            PivotPointX: PivotPointX::<Identity, OFFSET>,
            SetPivotPointX: SetPivotPointX::<Identity, OFFSET>,
            PivotPointY: PivotPointY::<Identity, OFFSET>,
            SetPivotPointY: SetPivotPointY::<Identity, OFFSET>,
            PivotRadius: PivotRadius::<Identity, OFFSET>,
            SetPivotRadius: SetPivotRadius::<Identity, OFFSET>,
            CompleteManipulation: CompleteManipulation::<Identity, OFFSET>,
            ProcessDown: ProcessDown::<Identity, OFFSET>,
            ProcessMove: ProcessMove::<Identity, OFFSET>,
            ProcessUp: ProcessUp::<Identity, OFFSET>,
            ProcessDownWithTime: ProcessDownWithTime::<Identity, OFFSET>,
            ProcessMoveWithTime: ProcessMoveWithTime::<Identity, OFFSET>,
            ProcessUpWithTime: ProcessUpWithTime::<Identity, OFFSET>,
            GetVelocityX: GetVelocityX::<Identity, OFFSET>,
            GetVelocityY: GetVelocityY::<Identity, OFFSET>,
            GetExpansionVelocity: GetExpansionVelocity::<Identity, OFFSET>,
            GetAngularVelocity: GetAngularVelocity::<Identity, OFFSET>,
            MinimumScaleRotateRadius: MinimumScaleRotateRadius::<Identity, OFFSET>,
            SetMinimumScaleRotateRadius: SetMinimumScaleRotateRadius::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationProcessor as windows_core::Interface>::IID
    }
}
pub trait _IManipulationEvents_Impl: Sized {
    fn ManipulationStarted(&self, x: f32, y: f32) -> windows_core::Result<()>;
    fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
    fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for _IManipulationEvents {}
impl _IManipulationEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _IManipulationEvents_Vtbl
    where
        Identity: _IManipulationEvents_Impl,
    {
        unsafe extern "system" fn ManipulationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32) -> windows_core::HRESULT
        where
            Identity: _IManipulationEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _IManipulationEvents_Impl::ManipulationStarted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT
        where
            Identity: _IManipulationEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _IManipulationEvents_Impl::ManipulationDelta(
                this,
                core::mem::transmute_copy(&x),
                core::mem::transmute_copy(&y),
                core::mem::transmute_copy(&translationdeltax),
                core::mem::transmute_copy(&translationdeltay),
                core::mem::transmute_copy(&scaledelta),
                core::mem::transmute_copy(&expansiondelta),
                core::mem::transmute_copy(&rotationdelta),
                core::mem::transmute_copy(&cumulativetranslationx),
                core::mem::transmute_copy(&cumulativetranslationy),
                core::mem::transmute_copy(&cumulativescale),
                core::mem::transmute_copy(&cumulativeexpansion),
                core::mem::transmute_copy(&cumulativerotation),
            )
            .into()
        }
        unsafe extern "system" fn ManipulationCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> windows_core::HRESULT
        where
            Identity: _IManipulationEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _IManipulationEvents_Impl::ManipulationCompleted(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&cumulativetranslationx), core::mem::transmute_copy(&cumulativetranslationy), core::mem::transmute_copy(&cumulativescale), core::mem::transmute_copy(&cumulativeexpansion), core::mem::transmute_copy(&cumulativerotation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ManipulationStarted: ManipulationStarted::<Identity, OFFSET>,
            ManipulationDelta: ManipulationDelta::<Identity, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_IManipulationEvents as windows_core::Interface>::IID
    }
}
