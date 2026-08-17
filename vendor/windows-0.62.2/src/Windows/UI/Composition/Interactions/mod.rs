#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionConditionalValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionConditionalValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompositionConditionalValue, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl CompositionConditionalValue {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Condition(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Condition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCondition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<CompositionConditionalValue>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    fn ICompositionConditionalValueStatics<R, F: FnOnce(&ICompositionConditionalValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompositionConditionalValue, ICompositionConditionalValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionConditionalValue>();
}
unsafe impl windows_core::Interface for CompositionConditionalValue {
    type Vtable = <ICompositionConditionalValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionConditionalValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionConditionalValue";
}
unsafe impl Send for CompositionConditionalValue {}
unsafe impl Sync for CompositionConditionalValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionInteractionSourceCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionInteractionSourceCollection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompositionInteractionSourceCollection, super::IAnimationObject, super::super::super::Foundation::IClosable, windows_collections::IIterable<ICompositionInteractionSource>, super::CompositionObject);
impl CompositionInteractionSourceCollection {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Count(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Count)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Add<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICompositionInteractionSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Add)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Remove<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICompositionInteractionSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAll(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAll)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<ICompositionInteractionSource>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<ICompositionInteractionSource>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionInteractionSourceCollection>();
}
unsafe impl windows_core::Interface for CompositionInteractionSourceCollection {
    type Vtable = <ICompositionInteractionSourceCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionInteractionSourceCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
unsafe impl Send for CompositionInteractionSourceCollection {}
unsafe impl Sync for CompositionInteractionSourceCollection {}
impl IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
windows_core::imp::define_interface!(ICompositionConditionalValue, ICompositionConditionalValue_Vtbl, 0x43250538_eb73_4561_a71d_1a43eaeb7a9b);
impl windows_core::RuntimeType for ICompositionConditionalValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionConditionalValueStatics, ICompositionConditionalValueStatics_Vtbl, 0x090c4b72_8467_4d0a_9065_ac46b80a5522);
impl windows_core::RuntimeType for ICompositionConditionalValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionInteractionSource, ICompositionInteractionSource_Vtbl, 0x043b2431_06e3_495a_ba54_409f0017fac0);
impl windows_core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ICompositionInteractionSource, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
pub trait ICompositionInteractionSource_Impl: windows_core::IUnknownImpl {}
impl ICompositionInteractionSource_Vtbl {
    pub const fn new<Identity: ICompositionInteractionSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionInteractionSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionInteractionSourceCollection, ICompositionInteractionSourceCollection_Vtbl, 0x1b468e4b_a5bf_47d8_a547_3894155a158c);
impl windows_core::RuntimeType for ICompositionInteractionSourceCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionSourceConfiguration, IInteractionSourceConfiguration_Vtbl, 0xa78347e5_a9d1_4d02_985e_b930cd0b9da4);
impl windows_core::RuntimeType for IInteractionSourceConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PositionXSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceRedirectionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTracker, IInteractionTracker_Vtbl, 0x2a8e8cb1_1000_4416_8363_cc27fb877308);
impl windows_core::RuntimeType for IInteractionTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InteractionSources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPositionRoundingSuggested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetMaxPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    pub MaxScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMaxScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub MinPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetMinPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    pub MinScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMinScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub NaturalRestingPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub NaturalRestingScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub PositionInertiaDecayRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPositionInertiaDecayRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ScaleInertiaDecayRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScaleInertiaDecayRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub AdjustPositionXIfGreaterThanThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub AdjustPositionYIfGreaterThanThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub ConfigurePositionXInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigurePositionYInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureScaleInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryUpdatePosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
    pub TryUpdatePositionBy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
    pub TryUpdatePositionWithAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TryUpdatePositionWithAdditionalVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
    pub TryUpdateScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
    pub TryUpdateScaleWithAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
    pub TryUpdateScaleWithAdditionalVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32, windows_numerics::Vector3, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTracker2, IInteractionTracker2_Vtbl, 0x25769a3e_ce6d_448c_8386_92620d240756);
impl windows_core::RuntimeType for IInteractionTracker2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConfigureCenterPointXInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureCenterPointYInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTracker3, IInteractionTracker3_Vtbl, 0xe6c5d7a2_5c4b_42c6_84b7_f69441b18091);
impl windows_core::RuntimeType for IInteractionTracker3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConfigureVector2PositionInertiaModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTracker4, IInteractionTracker4_Vtbl, 0xebd222bc_04af_4ac7_847d_06ea36e80a16);
impl windows_core::RuntimeType for IInteractionTracker4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, InteractionTrackerClampingOption, *mut i32) -> windows_core::HRESULT,
    pub TryUpdatePositionByWithOption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, InteractionTrackerClampingOption, *mut i32) -> windows_core::HRESULT,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTracker5, IInteractionTracker5_Vtbl, 0xd3ef5da2_a254_40e4_88d5_44e4e16b5809);
impl windows_core::RuntimeType for IInteractionTracker5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryUpdatePositionWithOption: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3, InteractionTrackerClampingOption, InteractionTrackerPositionUpdateOption, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerCustomAnimationStateEnteredArgs, IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl, 0x8d1c8cf1_d7b0_434c_a5d2_2d7611864834);
impl windows_core::RuntimeType for IInteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerCustomAnimationStateEnteredArgs2, IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl, 0x47d579b7_0985_5e99_b024_2f32c380c1a4);
impl windows_core::RuntimeType for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerIdleStateEnteredArgs, IInteractionTrackerIdleStateEnteredArgs_Vtbl, 0x50012faa_1510_4142_a1a5_019b09f8857b);
impl windows_core::RuntimeType for IInteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerIdleStateEnteredArgs2, IInteractionTrackerIdleStateEnteredArgs2_Vtbl, 0xf2e771ed_b803_5137_9435_1c96e48721e9);
impl windows_core::RuntimeType for IInteractionTrackerIdleStateEnteredArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaModifier, IInteractionTrackerInertiaModifier_Vtbl, 0xa0e2c920_26b4_4da2_8b61_5e683979bbe2);
impl windows_core::RuntimeType for IInteractionTrackerInertiaModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaModifierFactory, IInteractionTrackerInertiaModifierFactory_Vtbl, 0x993818fe_c94e_4b86_87f3_922665ba46b9);
impl windows_core::RuntimeType for IInteractionTrackerInertiaModifierFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotion_Vtbl, 0x04922fdc_f154_4cb8_bf33_cc1ba611e6db);
impl windows_core::RuntimeType for IInteractionTrackerInertiaMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Motion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaMotionStatics, IInteractionTrackerInertiaMotionStatics_Vtbl, 0x8cc83dd6_ba7b_431a_844b_6eac9130f99a);
impl windows_core::RuntimeType for IInteractionTrackerInertiaMotionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotion_Vtbl, 0x70acdaae_27dc_48ed_a3c3_6d61c9a029d2);
impl windows_core::RuntimeType for IInteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaNaturalMotionStatics, IInteractionTrackerInertiaNaturalMotionStatics_Vtbl, 0xcfda55b0_5e3e_4289_932d_ee5f50e74283);
impl windows_core::RuntimeType for IInteractionTrackerInertiaNaturalMotionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValue_Vtbl, 0x86f7ec09_5096_4170_9cc8_df2fe101bb93);
impl windows_core::RuntimeType for IInteractionTrackerInertiaRestingValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestingValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestingValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaRestingValueStatics, IInteractionTrackerInertiaRestingValueStatics_Vtbl, 0x18ed4699_0745_4096_bcab_3a4e99569bcf);
impl windows_core::RuntimeType for IInteractionTrackerInertiaRestingValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaStateEnteredArgs, IInteractionTrackerInertiaStateEnteredArgs_Vtbl, 0x87108cf2_e7ff_4f7d_9ffd_d72f1e409b63);
impl windows_core::RuntimeType for IInteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModifiedRestingPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModifiedRestingScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NaturalRestingPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub NaturalRestingScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub PositionVelocityInPixelsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ScaleVelocityInPercentPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaStateEnteredArgs2, IInteractionTrackerInertiaStateEnteredArgs2_Vtbl, 0xb1eb32f6_c26c_41f6_a189_fabc22b323cc);
impl windows_core::RuntimeType for IInteractionTrackerInertiaStateEnteredArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsInertiaFromImpulse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInertiaStateEnteredArgs3, IInteractionTrackerInertiaStateEnteredArgs3_Vtbl, 0x48ac1c2f_47bd_59af_a58c_79bd2eb9ef71);
impl windows_core::RuntimeType for IInteractionTrackerInertiaStateEnteredArgs3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInteractingStateEnteredArgs, IInteractionTrackerInteractingStateEnteredArgs_Vtbl, 0xa7263939_a17b_4011_99fd_b5c24f143748);
impl windows_core::RuntimeType for IInteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerInteractingStateEnteredArgs2, IInteractionTrackerInteractingStateEnteredArgs2_Vtbl, 0x509652d6_d488_59cd_819f_f52310295b11);
impl windows_core::RuntimeType for IInteractionTrackerInteractingStateEnteredArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFromBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerOwner, IInteractionTrackerOwner_Vtbl, 0xdb2e8af3_4deb_4e53_b29c_b06c9f96d651);
impl windows_core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IInteractionTrackerOwner, windows_core::IUnknown, windows_core::IInspectable);
impl IInteractionTrackerOwner {
    pub fn CustomAnimationStateEntered<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerCustomAnimationStateEnteredArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CustomAnimationStateEntered)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
    pub fn IdleStateEntered<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerIdleStateEnteredArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).IdleStateEntered)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
    pub fn InertiaStateEntered<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerInertiaStateEnteredArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InertiaStateEntered)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
    pub fn InteractingStateEntered<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerInteractingStateEnteredArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InteractingStateEntered)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
    pub fn RequestIgnored<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerRequestIgnoredArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestIgnored)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
    pub fn ValuesChanged<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTrackerValuesChangedArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ValuesChanged)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
pub trait IInteractionTrackerOwner_Impl: windows_core::IUnknownImpl {
    fn CustomAnimationStateEntered(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerCustomAnimationStateEnteredArgs>) -> windows_core::Result<()>;
    fn IdleStateEntered(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerIdleStateEnteredArgs>) -> windows_core::Result<()>;
    fn InertiaStateEntered(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerInertiaStateEnteredArgs>) -> windows_core::Result<()>;
    fn InteractingStateEntered(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerInteractingStateEnteredArgs>) -> windows_core::Result<()>;
    fn RequestIgnored(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerRequestIgnoredArgs>) -> windows_core::Result<()>;
    fn ValuesChanged(&self, sender: windows_core::Ref<InteractionTracker>, args: windows_core::Ref<InteractionTrackerValuesChangedArgs>) -> windows_core::Result<()>;
}
impl IInteractionTrackerOwner_Vtbl {
    pub const fn new<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CustomAnimationStateEntered<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::CustomAnimationStateEntered(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn IdleStateEntered<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::IdleStateEntered(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn InertiaStateEntered<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::InertiaStateEntered(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn InteractingStateEntered<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::InteractingStateEntered(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn RequestIgnored<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::RequestIgnored(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
        }
        unsafe extern "system" fn ValuesChanged<Identity: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInteractionTrackerOwner_Impl::ValuesChanged(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
            }
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
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomAnimationStateEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IdleStateEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InertiaStateEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InteractingStateEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestIgnored: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValuesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerRequestIgnoredArgs, IInteractionTrackerRequestIgnoredArgs_Vtbl, 0x80dd82f1_ce25_488f_91dd_cb6455ccff2e);
impl windows_core::RuntimeType for IInteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerStatics, IInteractionTrackerStatics_Vtbl, 0xbba5d7b7_6590_4498_8d6c_eb62b514c92a);
impl windows_core::RuntimeType for IInteractionTrackerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerStatics2, IInteractionTrackerStatics2_Vtbl, 0x35e53720_46b7_5cb0_b505_f3d6884a6163);
impl windows_core::RuntimeType for IInteractionTrackerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetBindingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, InteractionBindingAxisModes) -> windows_core::HRESULT,
    pub GetBindingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut InteractionBindingAxisModes) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerValuesChangedArgs, IInteractionTrackerValuesChangedArgs_Vtbl, 0xcf1578ef_d3df_4501_b9e6_f02fb22f73d0);
impl windows_core::RuntimeType for IInteractionTrackerValuesChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub RequestId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerVector2InertiaModifier, IInteractionTrackerVector2InertiaModifier_Vtbl, 0x87e08ab0_3086_4853_a4b7_77882ad5d7e3);
impl windows_core::RuntimeType for IInteractionTrackerVector2InertiaModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInteractionTrackerVector2InertiaModifierFactory, IInteractionTrackerVector2InertiaModifierFactory_Vtbl, 0x7401d6c4_6c6d_48df_bc3e_171e227e7d7f);
impl windows_core::RuntimeType for IInteractionTrackerVector2InertiaModifierFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotion_Vtbl, 0x5f17695c_162d_4c07_9400_c282b28276ca);
impl windows_core::RuntimeType for IInteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NaturalMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNaturalMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInteractionTrackerVector2InertiaNaturalMotionStatics, IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl, 0x82001a48_09c0_434f_8189_141c66df362f);
impl windows_core::RuntimeType for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualInteractionSource, IVisualInteractionSource_Vtbl, 0xca0e8a86_d8d6_4111_b088_70347bd2b0ed);
impl windows_core::RuntimeType for IVisualInteractionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPositionXRailsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPositionXRailsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsPositionYRailsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsPositionYRailsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ManipulationRedirectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VisualInteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub SetManipulationRedirectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, VisualInteractionSourceRedirectionMode) -> windows_core::HRESULT,
    pub PositionXChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionChainingMode) -> windows_core::HRESULT,
    pub SetPositionXChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionChainingMode) -> windows_core::HRESULT,
    pub PositionXSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceMode) -> windows_core::HRESULT,
    pub SetPositionXSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceMode) -> windows_core::HRESULT,
    pub PositionYChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionChainingMode) -> windows_core::HRESULT,
    pub SetPositionYChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionChainingMode) -> windows_core::HRESULT,
    pub PositionYSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceMode) -> windows_core::HRESULT,
    pub SetPositionYSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceMode) -> windows_core::HRESULT,
    pub ScaleChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionChainingMode) -> windows_core::HRESULT,
    pub SetScaleChainingMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionChainingMode) -> windows_core::HRESULT,
    pub ScaleSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InteractionSourceMode) -> windows_core::HRESULT,
    pub SetScaleSourceMode: unsafe extern "system" fn(*mut core::ffi::c_void, InteractionSourceMode) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub TryRedirectForManipulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    TryRedirectForManipulation: usize,
}
windows_core::imp::define_interface!(IVisualInteractionSource2, IVisualInteractionSource2_Vtbl, 0xaa914893_a73c_414d_80d0_249bad2fbd93);
impl windows_core::RuntimeType for IVisualInteractionSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeltaPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub DeltaScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub PositionVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ScaleVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ConfigureCenterPointXModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureCenterPointYModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureDeltaPositionXModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureDeltaPositionYModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigureDeltaScaleModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualInteractionSource3, IVisualInteractionSource3_Vtbl, 0xd941ef2a_0d5c_4057_92d7_c9711533204f);
impl windows_core::RuntimeType for IVisualInteractionSource3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerWheelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualInteractionSourceObjectFactory, IVisualInteractionSourceObjectFactory_Vtbl, 0xb2ca917c_e98a_41f2_b3c9_891c9266c8f6);
impl windows_core::RuntimeType for IVisualInteractionSourceObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IVisualInteractionSourceStatics, IVisualInteractionSourceStatics_Vtbl, 0x369965e1_8645_4f75_ba00_6479cd10c8e6);
impl windows_core::RuntimeType for IVisualInteractionSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualInteractionSourceStatics2, IVisualInteractionSourceStatics2_Vtbl, 0xa979c032_5764_55e0_bc1f_0778786dcfde);
impl windows_core::RuntimeType for IVisualInteractionSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromIVisualElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: Self = Self(0u32);
    pub const PositionX: Self = Self(1u32);
    pub const PositionY: Self = Self(2u32);
    pub const Scale: Self = Self(4u32);
}
impl windows_core::TypeKind for InteractionBindingAxisModes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionBindingAxisModes;u4)");
}
impl InteractionBindingAxisModes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl windows_core::TypeKind for InteractionChainingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionChainingMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionSourceConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionSourceConfiguration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionSourceConfiguration, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl InteractionSourceConfiguration {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn PositionXSourceMode(&self) -> windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionXSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceRedirectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionXSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionYSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceRedirectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionYSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> windows_core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceRedirectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaleSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionSourceConfiguration>();
}
unsafe impl windows_core::Interface for InteractionSourceConfiguration {
    type Vtable = <IInteractionSourceConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionSourceConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionSourceConfiguration";
}
unsafe impl Send for InteractionSourceConfiguration {}
unsafe impl Sync for InteractionSourceConfiguration {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: Self = Self(0i32);
    pub const EnabledWithInertia: Self = Self(1i32);
    pub const EnabledWithoutInertia: Self = Self(2i32);
}
impl windows_core::TypeKind for InteractionSourceMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl windows_core::TypeKind for InteractionSourceRedirectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTracker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTracker, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTracker, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl InteractionTracker {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn InteractionSources(&self) -> windows_core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InteractionSources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPositionRoundingSuggested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPosition(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxPosition(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinPosition(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinPosition(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMinPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMinScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NaturalRestingPosition(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalRestingPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalRestingScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Owner(&self) -> windows_core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Owner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PositionInertiaDecayRate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<windows_numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionInertiaDecayRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPositionInertiaDecayRate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<windows_numerics::Vector3>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionInertiaDecayRate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PositionVelocityInPixelsPerSecond(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Scale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScaleInertiaDecayRate(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleInertiaDecayRate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetScaleInertiaDecayRate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::IReference<f32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaleInertiaDecayRate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AdjustPositionXIfGreaterThanThreshold)(windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(&self, adjustment: f32, positionthreshold: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AdjustPositionYIfGreaterThanThreshold)(windows_core::Interface::as_raw(this), adjustment, positionthreshold).ok() }
    }
    pub fn ConfigurePositionXInertiaModifiers<P0>(&self, modifiers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigurePositionXInertiaModifiers)(windows_core::Interface::as_raw(this), modifiers.param().abi()).ok() }
    }
    pub fn ConfigurePositionYInertiaModifiers<P0>(&self, modifiers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigurePositionYInertiaModifiers)(windows_core::Interface::as_raw(this), modifiers.param().abi()).ok() }
    }
    pub fn ConfigureScaleInertiaModifiers<P0>(&self, modifiers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<InteractionTrackerInertiaModifier>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ConfigureScaleInertiaModifiers)(windows_core::Interface::as_raw(this), modifiers.param().abi()).ok() }
    }
    pub fn TryUpdatePosition(&self, value: windows_numerics::Vector3) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePosition)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdatePositionBy(&self, amount: windows_numerics::Vector3) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionBy)(windows_core::Interface::as_raw(this), amount, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdatePositionWithAnimation<P0>(&self, animation: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionWithAnimation)(windows_core::Interface::as_raw(this), animation.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdatePositionWithAdditionalVelocity(&self, velocityinpixelspersecond: windows_numerics::Vector3) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionWithAdditionalVelocity)(windows_core::Interface::as_raw(this), velocityinpixelspersecond, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdateScale(&self, value: f32, centerpoint: windows_numerics::Vector3) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateScale)(windows_core::Interface::as_raw(this), value, centerpoint, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdateScaleWithAnimation<P0>(&self, animation: P0, centerpoint: windows_numerics::Vector3) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateScaleWithAnimation)(windows_core::Interface::as_raw(this), animation.param().abi(), centerpoint, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdateScaleWithAdditionalVelocity(&self, velocityinpercentpersecond: f32, centerpoint: windows_numerics::Vector3) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdateScaleWithAdditionalVelocity)(windows_core::Interface::as_raw(this), velocityinpercentpersecond, centerpoint, &mut result__).map(|| result__)
        }
    }
    pub fn ConfigureCenterPointXInertiaModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureCenterPointXInertiaModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureCenterPointYInertiaModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureCenterPointYInertiaModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureVector2PositionInertiaModifiers<P0>(&self, modifiers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<InteractionTrackerVector2InertiaModifier>>,
    {
        let this = &windows_core::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureVector2PositionInertiaModifiers)(windows_core::Interface::as_raw(this), modifiers.param().abi()).ok() }
    }
    pub fn TryUpdatePositionWithOption(&self, value: windows_numerics::Vector3, option: InteractionTrackerClampingOption) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(windows_core::Interface::as_raw(this), value, option, &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdatePositionByWithOption(&self, amount: windows_numerics::Vector3, option: InteractionTrackerClampingOption) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionByWithOption)(windows_core::Interface::as_raw(this), amount, option, &mut result__).map(|| result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInertiaFromImpulse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryUpdatePositionWithOption2(&self, value: windows_numerics::Vector3, option: InteractionTrackerClampingOption, posupdateoption: InteractionTrackerPositionUpdateOption) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUpdatePositionWithOption)(windows_core::Interface::as_raw(this), value, option, posupdateoption, &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<InteractionTracker>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithOwner<P0, P1>(compositor: P0, owner: P1) -> windows_core::Result<InteractionTracker>
    where
        P0: windows_core::Param<super::Compositor>,
        P1: windows_core::Param<IInteractionTrackerOwner>,
    {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithOwner)(windows_core::Interface::as_raw(this), compositor.param().abi(), owner.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetBindingMode<P0, P1>(boundtracker1: P0, boundtracker2: P1, axismode: InteractionBindingAxisModes) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTracker>,
    {
        Self::IInteractionTrackerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetBindingMode)(windows_core::Interface::as_raw(this), boundtracker1.param().abi(), boundtracker2.param().abi(), axismode).ok() })
    }
    pub fn GetBindingMode<P0, P1>(boundtracker1: P0, boundtracker2: P1) -> windows_core::Result<InteractionBindingAxisModes>
    where
        P0: windows_core::Param<InteractionTracker>,
        P1: windows_core::Param<InteractionTracker>,
    {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBindingMode)(windows_core::Interface::as_raw(this), boundtracker1.param().abi(), boundtracker2.param().abi(), &mut result__).map(|| result__)
        })
    }
    fn IInteractionTrackerStatics<R, F: FnOnce(&IInteractionTrackerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTracker, IInteractionTrackerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInteractionTrackerStatics2<R, F: FnOnce(&IInteractionTrackerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTracker, IInteractionTrackerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InteractionTracker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTracker>();
}
unsafe impl windows_core::Interface for InteractionTracker {
    type Vtable = <IInteractionTracker as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTracker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTracker";
}
unsafe impl Send for InteractionTracker {}
unsafe impl Sync for InteractionTracker {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl windows_core::TypeKind for InteractionTrackerClampingOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerCustomAnimationStateEnteredArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFromBinding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTrackerCustomAnimationStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFromBinding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerCustomAnimationStateEnteredArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = <IInteractionTrackerCustomAnimationStateEnteredArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerCustomAnimationStateEnteredArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
unsafe impl Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerIdleStateEnteredArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerIdleStateEnteredArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFromBinding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFromBinding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerIdleStateEnteredArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = <IInteractionTrackerIdleStateEnteredArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerIdleStateEnteredArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
unsafe impl Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl Sync for InteractionTrackerIdleStateEnteredArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInertiaModifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaModifier, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerInertiaModifier, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl InteractionTrackerInertiaModifier {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInertiaModifier>();
}
unsafe impl windows_core::Interface for InteractionTrackerInertiaModifier {
    type Vtable = <IInteractionTrackerInertiaModifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInertiaModifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
unsafe impl Send for InteractionTrackerInertiaModifier {}
unsafe impl Sync for InteractionTrackerInertiaModifier {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInertiaMotion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaMotion, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerInertiaMotion, super::IAnimationObject, super::super::super::Foundation::IClosable, InteractionTrackerInertiaModifier, super::CompositionObject);
impl InteractionTrackerInertiaMotion {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Condition(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Condition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCondition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Motion(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Motion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMotion<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMotion)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<InteractionTrackerInertiaMotion>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInteractionTrackerInertiaMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTrackerInertiaMotion, IInteractionTrackerInertiaMotionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInertiaMotion>();
}
unsafe impl windows_core::Interface for InteractionTrackerInertiaMotion {
    type Vtable = <IInteractionTrackerInertiaMotion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInertiaMotion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
unsafe impl Send for InteractionTrackerInertiaMotion {}
unsafe impl Sync for InteractionTrackerInertiaMotion {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInertiaNaturalMotion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaNaturalMotion, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerInertiaNaturalMotion, super::IAnimationObject, super::super::super::Foundation::IClosable, InteractionTrackerInertiaModifier, super::CompositionObject);
impl InteractionTrackerInertiaNaturalMotion {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Condition(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Condition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCondition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> windows_core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalMotion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ScalarNaturalMotionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNaturalMotion)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<InteractionTrackerInertiaNaturalMotion>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInteractionTrackerInertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTrackerInertiaNaturalMotion, IInteractionTrackerInertiaNaturalMotionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInertiaNaturalMotion>();
}
unsafe impl windows_core::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = <IInteractionTrackerInertiaNaturalMotion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInertiaNaturalMotion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
unsafe impl Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl Sync for InteractionTrackerInertiaNaturalMotion {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInertiaRestingValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaRestingValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerInertiaRestingValue, super::IAnimationObject, super::super::super::Foundation::IClosable, InteractionTrackerInertiaModifier, super::CompositionObject);
impl InteractionTrackerInertiaRestingValue {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Condition(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Condition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCondition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RestingValue(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RestingValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRestingValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRestingValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<InteractionTrackerInertiaRestingValue>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInteractionTrackerInertiaRestingValueStatics<R, F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTrackerInertiaRestingValue, IInteractionTrackerInertiaRestingValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInertiaRestingValue>();
}
unsafe impl windows_core::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = <IInteractionTrackerInertiaRestingValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInertiaRestingValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
unsafe impl Send for InteractionTrackerInertiaRestingValue {}
unsafe impl Sync for InteractionTrackerInertiaRestingValue {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInertiaStateEnteredArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInertiaStateEnteredArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerInertiaStateEnteredArgs {
    pub fn ModifiedRestingPosition(&self) -> windows_core::Result<super::super::super::Foundation::IReference<windows_numerics::Vector3>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModifiedRestingPosition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ModifiedRestingScale(&self) -> windows_core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModifiedRestingScale)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NaturalRestingPosition(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalRestingPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalRestingScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PositionVelocityInPixelsPerSecond(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionVelocityInPixelsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleVelocityInPercentPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInertiaFromImpulse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFromBinding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFromBinding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInertiaStateEnteredArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = <IInteractionTrackerInertiaStateEnteredArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInertiaStateEnteredArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
unsafe impl Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerInteractingStateEnteredArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerInteractingStateEnteredArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFromBinding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInteractionTrackerInteractingStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFromBinding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerInteractingStateEnteredArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = <IInteractionTrackerInteractingStateEnteredArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerInteractingStateEnteredArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
unsafe impl Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: Self = Self(0i32);
    pub const AllowActiveCustomScaleAnimation: Self = Self(1i32);
}
impl windows_core::TypeKind for InteractionTrackerPositionUpdateOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerRequestIgnoredArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerRequestIgnoredArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerRequestIgnoredArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = <IInteractionTrackerRequestIgnoredArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerRequestIgnoredArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
unsafe impl Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl Sync for InteractionTrackerRequestIgnoredArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerValuesChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerValuesChangedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InteractionTrackerValuesChangedArgs {
    pub fn Position(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequestId(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Scale(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerValuesChangedArgs>();
}
unsafe impl windows_core::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = <IInteractionTrackerValuesChangedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerValuesChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
unsafe impl Send for InteractionTrackerValuesChangedArgs {}
unsafe impl Sync for InteractionTrackerValuesChangedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerVector2InertiaModifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerVector2InertiaModifier, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerVector2InertiaModifier, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionObject);
impl InteractionTrackerVector2InertiaModifier {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerVector2InertiaModifier>();
}
unsafe impl windows_core::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = <IInteractionTrackerVector2InertiaModifier as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerVector2InertiaModifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
unsafe impl Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl Sync for InteractionTrackerVector2InertiaModifier {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InteractionTrackerVector2InertiaNaturalMotion, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InteractionTrackerVector2InertiaNaturalMotion, super::IAnimationObject, super::super::super::Foundation::IClosable, InteractionTrackerVector2InertiaModifier, super::CompositionObject);
impl InteractionTrackerVector2InertiaNaturalMotion {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Condition(&self) -> windows_core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Condition)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCondition<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ExpressionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCondition)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NaturalMotion(&self) -> windows_core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NaturalMotion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNaturalMotion<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Vector2NaturalMotionAnimation>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNaturalMotion)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(compositor: P0) -> windows_core::Result<InteractionTrackerVector2InertiaNaturalMotion>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), compositor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInteractionTrackerVector2InertiaNaturalMotionStatics<R, F: FnOnce(&IInteractionTrackerVector2InertiaNaturalMotionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InteractionTrackerVector2InertiaNaturalMotion, IInteractionTrackerVector2InertiaNaturalMotionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInteractionTrackerVector2InertiaNaturalMotion>();
}
unsafe impl windows_core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = <IInteractionTrackerVector2InertiaNaturalMotion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInteractionTrackerVector2InertiaNaturalMotion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
unsafe impl Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisualInteractionSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VisualInteractionSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VisualInteractionSource, super::IAnimationObject, super::super::super::Foundation::IClosable, ICompositionInteractionSource, super::CompositionObject);
impl VisualInteractionSource {
    pub fn PopulatePropertyInfo<P1>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::AnimationPropertyInfo>,
    {
        let this = &windows_core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PopulatePropertyInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), propertyinfo.param().abi()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Compositor(&self) -> windows_core::Result<super::Compositor> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compositor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<super::CompositionPropertySet> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimation<P1>(&self, propertyname: &windows_core::HSTRING, animation: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi()).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ImplicitAnimations(&self) -> windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImplicitAnimations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ImplicitAnimationCollection>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetImplicitAnimations)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ICompositionAnimationBase>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimationGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<super::AnimationController> {
        let this = &windows_core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetAnimationController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAnimationWithController<P1, P2>(&self, propertyname: &windows_core::HSTRING, animation: P1, animationcontroller: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::CompositionAnimation>,
        P2: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn IsPositionXRailsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPositionXRailsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPositionXRailsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPositionYRailsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPositionYRailsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsPositionYRailsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ManipulationRedirectionMode(&self) -> windows_core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationRedirectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetManipulationRedirectionMode(&self, value: VisualInteractionSourceRedirectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationRedirectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXChainingMode(&self) -> windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionXChainingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionXChainingMode(&self, value: InteractionChainingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionXChainingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionXSourceMode(&self) -> windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionXSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionXSourceMode(&self, value: InteractionSourceMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionXSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYChainingMode(&self) -> windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionYChainingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionYChainingMode(&self, value: InteractionChainingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionYChainingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionYSourceMode(&self) -> windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionYSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPositionYSourceMode(&self, value: InteractionSourceMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPositionYSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleChainingMode(&self) -> windows_core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleChainingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaleChainingMode(&self, value: InteractionChainingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaleChainingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaleSourceMode(&self) -> windows_core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleSourceMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScaleSourceMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Source(&self) -> windows_core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation<P0>(&self, pointerpoint: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerPoint>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TryRedirectForManipulation)(windows_core::Interface::as_raw(this), pointerpoint.param().abi()).ok() }
    }
    pub fn DeltaPosition(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeltaScale(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PositionVelocity(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PositionVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Scale(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScaleVelocity(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScaleVelocity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConfigureCenterPointXModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureCenterPointXModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureCenterPointYModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureCenterPointYModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureDeltaPositionXModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureDeltaPositionXModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureDeltaPositionYModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureDeltaPositionYModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn ConfigureDeltaScaleModifiers<P0>(&self, conditionalvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<CompositionConditionalValue>>,
    {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ConfigureDeltaScaleModifiers)(windows_core::Interface::as_raw(this), conditionalvalues.param().abi()).ok() }
    }
    pub fn PointerWheelConfig(&self) -> windows_core::Result<InteractionSourceConfiguration> {
        let this = &windows_core::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerWheelConfig)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(source: P0) -> windows_core::Result<VisualInteractionSource>
    where
        P0: windows_core::Param<super::Visual>,
    {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIVisualElement<P0>(source: P0) -> windows_core::Result<VisualInteractionSource>
    where
        P0: windows_core::Param<super::IVisualElement>,
    {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIVisualElement)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IVisualInteractionSourceStatics<R, F: FnOnce(&IVisualInteractionSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IVisualInteractionSourceStatics2<R, F: FnOnce(&IVisualInteractionSourceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VisualInteractionSource, IVisualInteractionSourceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VisualInteractionSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVisualInteractionSource>();
}
unsafe impl windows_core::Interface for VisualInteractionSource {
    type Vtable = <IVisualInteractionSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVisualInteractionSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.VisualInteractionSource";
}
unsafe impl Send for VisualInteractionSource {}
unsafe impl Sync for VisualInteractionSource {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: Self = Self(0i32);
    pub const CapableTouchpadOnly: Self = Self(1i32);
    pub const PointerWheelOnly: Self = Self(2i32);
    pub const CapableTouchpadAndPointerWheel: Self = Self(3i32);
}
impl windows_core::TypeKind for VisualInteractionSourceRedirectionMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)");
}
