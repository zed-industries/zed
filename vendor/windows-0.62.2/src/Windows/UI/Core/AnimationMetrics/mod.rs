#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnimationDescription(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AnimationDescription, windows_core::IUnknown, windows_core::IInspectable);
impl AnimationDescription {
    pub fn Animations(&self) -> windows_core::Result<windows_collections::IVectorView<IPropertyAnimation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Animations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StaggerDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaggerDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StaggerDelayFactor(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StaggerDelayFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DelayLimit(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DelayLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ZOrder(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZOrder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(effect: AnimationEffect, target: AnimationEffectTarget) -> windows_core::Result<AnimationDescription> {
        Self::IAnimationDescriptionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), effect, target, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAnimationDescriptionFactory<R, F: FnOnce(&IAnimationDescriptionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AnimationDescription, IAnimationDescriptionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AnimationDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAnimationDescription>();
}
unsafe impl windows_core::Interface for AnimationDescription {
    type Vtable = <IAnimationDescription as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAnimationDescription as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AnimationDescription {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.AnimationDescription";
}
unsafe impl Send for AnimationDescription {}
unsafe impl Sync for AnimationDescription {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnimationEffect(pub i32);
impl AnimationEffect {
    pub const Expand: Self = Self(0i32);
    pub const Collapse: Self = Self(1i32);
    pub const Reposition: Self = Self(2i32);
    pub const FadeIn: Self = Self(3i32);
    pub const FadeOut: Self = Self(4i32);
    pub const AddToList: Self = Self(5i32);
    pub const DeleteFromList: Self = Self(6i32);
    pub const AddToGrid: Self = Self(7i32);
    pub const DeleteFromGrid: Self = Self(8i32);
    pub const AddToSearchGrid: Self = Self(9i32);
    pub const DeleteFromSearchGrid: Self = Self(10i32);
    pub const AddToSearchList: Self = Self(11i32);
    pub const DeleteFromSearchList: Self = Self(12i32);
    pub const ShowEdgeUI: Self = Self(13i32);
    pub const ShowPanel: Self = Self(14i32);
    pub const HideEdgeUI: Self = Self(15i32);
    pub const HidePanel: Self = Self(16i32);
    pub const ShowPopup: Self = Self(17i32);
    pub const HidePopup: Self = Self(18i32);
    pub const PointerDown: Self = Self(19i32);
    pub const PointerUp: Self = Self(20i32);
    pub const DragSourceStart: Self = Self(21i32);
    pub const DragSourceEnd: Self = Self(22i32);
    pub const TransitionContent: Self = Self(23i32);
    pub const Reveal: Self = Self(24i32);
    pub const Hide: Self = Self(25i32);
    pub const DragBetweenEnter: Self = Self(26i32);
    pub const DragBetweenLeave: Self = Self(27i32);
    pub const SwipeSelect: Self = Self(28i32);
    pub const SwipeDeselect: Self = Self(29i32);
    pub const SwipeReveal: Self = Self(30i32);
    pub const EnterPage: Self = Self(31i32);
    pub const TransitionPage: Self = Self(32i32);
    pub const CrossFade: Self = Self(33i32);
    pub const Peek: Self = Self(34i32);
    pub const UpdateBadge: Self = Self(35i32);
}
impl windows_core::TypeKind for AnimationEffect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AnimationEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffect;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnimationEffectTarget(pub i32);
impl AnimationEffectTarget {
    pub const Primary: Self = Self(0i32);
    pub const Added: Self = Self(1i32);
    pub const Affected: Self = Self(2i32);
    pub const Background: Self = Self(3i32);
    pub const Content: Self = Self(4i32);
    pub const Deleted: Self = Self(5i32);
    pub const Deselected: Self = Self(6i32);
    pub const DragSource: Self = Self(7i32);
    pub const Hidden: Self = Self(8i32);
    pub const Incoming: Self = Self(9i32);
    pub const Outgoing: Self = Self(10i32);
    pub const Outline: Self = Self(11i32);
    pub const Remaining: Self = Self(12i32);
    pub const Revealed: Self = Self(13i32);
    pub const RowIn: Self = Self(14i32);
    pub const RowOut: Self = Self(15i32);
    pub const Selected: Self = Self(16i32);
    pub const Selection: Self = Self(17i32);
    pub const Shown: Self = Self(18i32);
    pub const Tapped: Self = Self(19i32);
}
impl windows_core::TypeKind for AnimationEffectTarget {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AnimationEffectTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.AnimationEffectTarget;i4)");
}
windows_core::imp::define_interface!(IAnimationDescription, IAnimationDescription_Vtbl, 0x7d11a549_be3d_41de_b081_05c149962f9b);
impl windows_core::RuntimeType for IAnimationDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescription_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Animations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StaggerDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StaggerDelayFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub DelayLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAnimationDescriptionFactory, IAnimationDescriptionFactory_Vtbl, 0xc6e27abe_c1fb_48b5_9271_ecc70ac86ef0);
impl windows_core::RuntimeType for IAnimationDescriptionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationDescriptionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, AnimationEffect, AnimationEffectTarget, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpacityAnimation, IOpacityAnimation_Vtbl, 0x803aabe5_ee7e_455f_84e9_2506afb8d2b4);
impl windows_core::RuntimeType for IOpacityAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpacityAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitialOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyAnimation, IPropertyAnimation_Vtbl, 0x3a01b4da_4d8c_411e_b615_1ade683a9903);
impl windows_core::RuntimeType for IPropertyAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPropertyAnimation, windows_core::IUnknown, windows_core::IInspectable);
impl IPropertyAnimation {
    pub fn Type(&self) -> windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.IPropertyAnimation";
}
pub trait IPropertyAnimation_Impl: windows_core::IUnknownImpl {
    fn Type(&self) -> windows_core::Result<PropertyAnimationType>;
    fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point>;
    fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point>;
}
impl IPropertyAnimation_Vtbl {
    pub const fn new<Identity: IPropertyAnimation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PropertyAnimationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyAnimation_Impl::Type(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delay<Identity: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyAnimation_Impl::Delay(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Duration<Identity: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyAnimation_Impl::Duration(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Control1<Identity: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyAnimation_Impl::Control1(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Control2<Identity: IPropertyAnimation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyAnimation_Impl::Control2(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyAnimation, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Delay: Delay::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            Control1: Control1::<Identity, OFFSET>,
            Control2: Control2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyAnimation as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyAnimationType) -> windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Control1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
    pub Control2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScaleAnimation, IScaleAnimation_Vtbl, 0x023552c7_71ab_428c_9c9f_d31780964995);
impl windows_core::RuntimeType for IScaleAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleAnimation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitialScaleX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitialScaleY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalScaleX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub FinalScaleY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub NormalizedOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Point) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpacityAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OpacityAnimation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(OpacityAnimation, IPropertyAnimation);
impl OpacityAnimation {
    pub fn InitialOpacity(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialOpacity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinalOpacity(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinalOpacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<PropertyAnimationType> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for OpacityAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOpacityAnimation>();
}
unsafe impl windows_core::Interface for OpacityAnimation {
    type Vtable = <IOpacityAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOpacityAnimation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OpacityAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.OpacityAnimation";
}
unsafe impl Send for OpacityAnimation {}
unsafe impl Sync for OpacityAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PropertyAnimation, windows_core::IUnknown, windows_core::IInspectable, IPropertyAnimation);
impl PropertyAnimation {
    pub fn Type(&self) -> windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PropertyAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertyAnimation>();
}
unsafe impl windows_core::Interface for PropertyAnimation {
    type Vtable = <IPropertyAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPropertyAnimation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PropertyAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.PropertyAnimation";
}
unsafe impl Send for PropertyAnimation {}
unsafe impl Sync for PropertyAnimation {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PropertyAnimationType(pub i32);
impl PropertyAnimationType {
    pub const Scale: Self = Self(0i32);
    pub const Translation: Self = Self(1i32);
    pub const Opacity: Self = Self(2i32);
}
impl windows_core::TypeKind for PropertyAnimationType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PropertyAnimationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Core.AnimationMetrics.PropertyAnimationType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScaleAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScaleAnimation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ScaleAnimation, IPropertyAnimation);
impl ScaleAnimation {
    pub fn Type(&self) -> windows_core::Result<PropertyAnimationType> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IPropertyAnimation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InitialScaleX(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialScaleX)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InitialScaleY(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitialScaleY)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FinalScaleX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinalScaleX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FinalScaleY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FinalScaleY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NormalizedOrigin(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalizedOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ScaleAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScaleAnimation>();
}
unsafe impl windows_core::Interface for ScaleAnimation {
    type Vtable = <IScaleAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScaleAnimation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScaleAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.ScaleAnimation";
}
unsafe impl Send for ScaleAnimation {}
unsafe impl Sync for ScaleAnimation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationAnimation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TranslationAnimation, windows_core::IUnknown, windows_core::IInspectable, IPropertyAnimation);
impl TranslationAnimation {
    pub fn Type(&self) -> windows_core::Result<PropertyAnimationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Delay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Delay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control1(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control1)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Control2(&self) -> windows_core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control2)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TranslationAnimation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertyAnimation>();
}
unsafe impl windows_core::Interface for TranslationAnimation {
    type Vtable = <IPropertyAnimation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPropertyAnimation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TranslationAnimation {
    const NAME: &'static str = "Windows.UI.Core.AnimationMetrics.TranslationAnimation";
}
unsafe impl Send for TranslationAnimation {}
unsafe impl Sync for TranslationAnimation {}
