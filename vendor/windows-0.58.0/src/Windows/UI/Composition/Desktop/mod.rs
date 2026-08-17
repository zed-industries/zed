windows_core::imp::define_interface!(IDesktopWindowTarget, IDesktopWindowTarget_Vtbl, 0x6329d6ca_3366_490e_9db3_25312929ac51);
impl windows_core::RuntimeType for IDesktopWindowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DesktopWindowTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DesktopWindowTarget, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DesktopWindowTarget, super::IAnimationObject, super::super::super::Foundation::IClosable, super::CompositionTarget, super::CompositionObject);
impl DesktopWindowTarget {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &windows_core::HSTRING, propertyinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::AnimationPropertyInfo>,
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
    pub fn StartAnimation<P0>(&self, propertyname: &windows_core::HSTRING, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
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
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &windows_core::HSTRING, animation: P0, animationcontroller: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::CompositionAnimation>,
        P1: windows_core::Param<super::AnimationController>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAnimationWithController)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), animation.param().abi(), animationcontroller.param().abi()).ok() }
    }
    pub fn Root(&self) -> windows_core::Result<super::Visual> {
        let this = &windows_core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Root)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = &windows_core::Interface::cast::<super::ICompositionTarget>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTopmost(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTopmost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDesktopWindowTarget>();
}
unsafe impl windows_core::Interface for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
    const IID: windows_core::GUID = <IDesktopWindowTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
unsafe impl Send for DesktopWindowTarget {}
unsafe impl Sync for DesktopWindowTarget {}
