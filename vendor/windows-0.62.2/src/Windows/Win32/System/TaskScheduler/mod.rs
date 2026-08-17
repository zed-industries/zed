pub const CLSID_CTask: windows_core::GUID = windows_core::GUID::from_u128(0x148bd520_a2ab_11ce_b11f_00aa00530503);
pub const CLSID_CTaskScheduler: windows_core::GUID = windows_core::GUID::from_u128(0x148bd52a_a2ab_11ce_b11f_00aa00530503);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DAILY {
    pub DaysInterval: u16,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAction, IAction_Vtbl, 0xbae54997_48b1_4cbe_9965_d6be263ebea4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAction {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAction, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAction {
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok() }
    }
    pub unsafe fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)).ok() }
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), ptype as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_ACTION_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAction_Impl: super::Com::IDispatch_Impl {
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAction_Vtbl {
    pub const fn new<Identity: IAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAction_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn SetId<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAction_Impl::SetId(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAction_Impl::Type(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IActionCollection, IActionCollection_Vtbl, 0x02820e19_7b98_4ed2_b2e8_fdccceff619b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IActionCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IActionCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IActionCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount as _).ok() }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok() }
    }
    pub unsafe fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)).ok() }
    }
    pub unsafe fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Context(&self, pcontext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), core::mem::transmute(pcontext)).ok() }
    }
    pub unsafe fn SetContext(&self, context: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(context)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_ACTION_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActionCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IAction>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction>;
    fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Context(&self, pcontext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetContext(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActionCollection_Vtbl {
    pub const fn new<Identity: IActionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn XmlText<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
            }
        }
        unsafe extern "system" fn SetXmlText<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn Create<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn Context<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Context(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn SetContext<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::SetContext(this, core::mem::transmute(&context)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IActionCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IBootTrigger, IBootTrigger_Vtbl, 0x2a9c35da_d357_41f4_bbc1_207ac1b1f3cb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IBootTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IBootTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IBootTrigger {
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootTrigger_Impl: ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: IBootTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delay<Identity: IBootTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetDelay<Identity: IBootTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
            }
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), Delay: Delay::<Identity, OFFSET>, SetDelay: SetDelay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBootTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IComHandlerAction, IComHandlerAction_Vtbl, 0x6d2fd252_75c5_4f66_90ba_2a7d8cc3039f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IComHandlerAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IComHandlerAction, windows_core::IUnknown, super::Com::IDispatch, IAction);
#[cfg(feature = "Win32_System_Com")]
impl IComHandlerAction {
    pub unsafe fn ClassId(&self, pclsid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClassId)(windows_core::Interface::as_raw(self), core::mem::transmute(pclsid)).ok() }
    }
    pub unsafe fn SetClassId(&self, clsid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClassId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clsid)).ok() }
    }
    pub unsafe fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata)).ok() }
    }
    pub unsafe fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IComHandlerAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub ClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComHandlerAction_Impl: IAction_Impl {
    fn ClassId(&self, pclsid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClassId(&self, clsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComHandlerAction_Vtbl {
    pub const fn new<Identity: IComHandlerAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClassId<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::ClassId(this, core::mem::transmute_copy(&pclsid)).into()
            }
        }
        unsafe extern "system" fn SetClassId<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::SetClassId(this, core::mem::transmute(&clsid)).into()
            }
        }
        unsafe extern "system" fn Data<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::Data(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetData<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::SetData(this, core::mem::transmute(&data)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            ClassId: ClassId::<Identity, OFFSET>,
            SetClassId: SetClassId::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComHandlerAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IComHandlerAction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDailyTrigger, IDailyTrigger_Vtbl, 0x126c5cd8_b288_41d5_8dbf_e491446adc5c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDailyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDailyTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IDailyTrigger {
    pub unsafe fn DaysInterval(&self, pdays: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DaysInterval)(windows_core::Interface::as_raw(self), pdays as _).ok() }
    }
    pub unsafe fn SetDaysInterval(&self, days: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysInterval)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok() }
    }
    pub unsafe fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDailyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetDaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDailyTrigger_Impl: ITrigger_Impl {
    fn DaysInterval(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysInterval(&self, days: i16) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDailyTrigger_Vtbl {
    pub const fn new<Identity: IDailyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DaysInterval<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::DaysInterval(this, core::mem::transmute_copy(&pdays)).into()
            }
        }
        unsafe extern "system" fn SetDaysInterval<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::SetDaysInterval(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn RandomDelay<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
            }
        }
        unsafe extern "system" fn SetRandomDelay<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysInterval: DaysInterval::<Identity, OFFSET>,
            SetDaysInterval: SetDaysInterval::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDailyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDailyTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEmailAction, IEmailAction_Vtbl, 0x10f62c64_7e16_4314_a0c2_0c3683f99d40);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEmailAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEmailAction, windows_core::IUnknown, super::Com::IDispatch, IAction);
#[cfg(feature = "Win32_System_Com")]
impl IEmailAction {
    pub unsafe fn Server(&self, pserver: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), core::mem::transmute(pserver)).ok() }
    }
    pub unsafe fn SetServer(&self, server: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server)).ok() }
    }
    pub unsafe fn Subject(&self, psubject: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), core::mem::transmute(psubject)).ok() }
    }
    pub unsafe fn SetSubject(&self, subject: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subject)).ok() }
    }
    pub unsafe fn To(&self, pto: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).To)(windows_core::Interface::as_raw(self), core::mem::transmute(pto)).ok() }
    }
    pub unsafe fn SetTo(&self, to: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(to)).ok() }
    }
    pub unsafe fn Cc(&self, pcc: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cc)(windows_core::Interface::as_raw(self), core::mem::transmute(pcc)).ok() }
    }
    pub unsafe fn SetCc(&self, cc: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cc)).ok() }
    }
    pub unsafe fn Bcc(&self, pbcc: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Bcc)(windows_core::Interface::as_raw(self), core::mem::transmute(pbcc)).ok() }
    }
    pub unsafe fn SetBcc(&self, bcc: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBcc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bcc)).ok() }
    }
    pub unsafe fn ReplyTo(&self, preplyto: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute(preplyto)).ok() }
    }
    pub unsafe fn SetReplyTo(&self, replyto: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(replyto)).ok() }
    }
    pub unsafe fn From(&self, pfrom: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).From)(windows_core::Interface::as_raw(self), core::mem::transmute(pfrom)).ok() }
    }
    pub unsafe fn SetFrom(&self, from: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(from)).ok() }
    }
    pub unsafe fn HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HeaderFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetHeaderFields<P0>(&self, pheaderfields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHeaderFields)(windows_core::Interface::as_raw(self), pheaderfields.param().abi()).ok() }
    }
    pub unsafe fn Body(&self, pbody: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), core::mem::transmute(pbody)).ok() }
    }
    pub unsafe fn SetBody(&self, body: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(body)).ok() }
    }
    pub unsafe fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Attachments)(windows_core::Interface::as_raw(self), pattachements as _).ok() }
    }
    pub unsafe fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAttachments)(windows_core::Interface::as_raw(self), pattachements as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Bcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub SetAttachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEmailAction_Impl: IAction_Impl {
    fn Server(&self, pserver: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetServer(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Subject(&self, psubject: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSubject(&self, subject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn To(&self, pto: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTo(&self, to: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Cc(&self, pcc: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCc(&self, cc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Bcc(&self, pbcc: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBcc(&self, bcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReplyTo(&self, preplyto: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetReplyTo(&self, replyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn From(&self, pfrom: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetFrom(&self, from: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn SetHeaderFields(&self, pheaderfields: windows_core::Ref<ITaskNamedValueCollection>) -> windows_core::Result<()>;
    fn Body(&self, pbody: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBody(&self, body: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEmailAction_Vtbl {
    pub const fn new<Identity: IEmailAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Server<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Server(this, core::mem::transmute_copy(&pserver)).into()
            }
        }
        unsafe extern "system" fn SetServer<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetServer(this, core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn Subject<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Subject(this, core::mem::transmute_copy(&psubject)).into()
            }
        }
        unsafe extern "system" fn SetSubject<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetSubject(this, core::mem::transmute(&subject)).into()
            }
        }
        unsafe extern "system" fn To<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::To(this, core::mem::transmute_copy(&pto)).into()
            }
        }
        unsafe extern "system" fn SetTo<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, to: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetTo(this, core::mem::transmute(&to)).into()
            }
        }
        unsafe extern "system" fn Cc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Cc(this, core::mem::transmute_copy(&pcc)).into()
            }
        }
        unsafe extern "system" fn SetCc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetCc(this, core::mem::transmute(&cc)).into()
            }
        }
        unsafe extern "system" fn Bcc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Bcc(this, core::mem::transmute_copy(&pbcc)).into()
            }
        }
        unsafe extern "system" fn SetBcc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetBcc(this, core::mem::transmute(&bcc)).into()
            }
        }
        unsafe extern "system" fn ReplyTo<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::ReplyTo(this, core::mem::transmute_copy(&preplyto)).into()
            }
        }
        unsafe extern "system" fn SetReplyTo<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetReplyTo(this, core::mem::transmute(&replyto)).into()
            }
        }
        unsafe extern "system" fn From<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::From(this, core::mem::transmute_copy(&pfrom)).into()
            }
        }
        unsafe extern "system" fn SetFrom<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetFrom(this, core::mem::transmute(&from)).into()
            }
        }
        unsafe extern "system" fn HeaderFields<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderfields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::HeaderFields(this) {
                    Ok(ok__) => {
                        ppheaderfields.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHeaderFields<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaderfields: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetHeaderFields(this, core::mem::transmute_copy(&pheaderfields)).into()
            }
        }
        unsafe extern "system" fn Body<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Body(this, core::mem::transmute_copy(&pbody)).into()
            }
        }
        unsafe extern "system" fn SetBody<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetBody(this, core::mem::transmute(&body)).into()
            }
        }
        unsafe extern "system" fn Attachments<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::Attachments(this, core::mem::transmute_copy(&pattachements)).into()
            }
        }
        unsafe extern "system" fn SetAttachments<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::SetAttachments(this, core::mem::transmute_copy(&pattachements)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Server: Server::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            Subject: Subject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            To: To::<Identity, OFFSET>,
            SetTo: SetTo::<Identity, OFFSET>,
            Cc: Cc::<Identity, OFFSET>,
            SetCc: SetCc::<Identity, OFFSET>,
            Bcc: Bcc::<Identity, OFFSET>,
            SetBcc: SetBcc::<Identity, OFFSET>,
            ReplyTo: ReplyTo::<Identity, OFFSET>,
            SetReplyTo: SetReplyTo::<Identity, OFFSET>,
            From: From::<Identity, OFFSET>,
            SetFrom: SetFrom::<Identity, OFFSET>,
            HeaderFields: HeaderFields::<Identity, OFFSET>,
            SetHeaderFields: SetHeaderFields::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            Attachments: Attachments::<Identity, OFFSET>,
            SetAttachments: SetAttachments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmailAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEmailAction {}
windows_core::imp::define_interface!(IEnumWorkItems, IEnumWorkItems_Vtbl, 0x148bd528_a2ab_11ce_b11f_00aa00530503);
windows_core::imp::interface_hierarchy!(IEnumWorkItems, windows_core::IUnknown);
impl IEnumWorkItems {
    pub unsafe fn Next(&self, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgpwsznames as _, pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWorkItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWorkItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumWorkItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT;
    fn Skip(&self, celt: u32) -> windows_core::HRESULT;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWorkItems>;
}
impl IEnumWorkItems_Vtbl {
    pub const fn new<Identity: IEnumWorkItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWorkItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgpwsznames), core::mem::transmute_copy(&pceltfetched))
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWorkItems_Impl::Skip(this, core::mem::transmute_copy(&celt))
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWorkItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumworkitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWorkItems_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenumworkitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWorkItems as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWorkItems {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IEventTrigger, IEventTrigger_Vtbl, 0xd45b0167_9653_4eef_b94f_0732ca7af251);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IEventTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IEventTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IEventTrigger {
    pub unsafe fn Subscription(&self, pquery: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Subscription)(windows_core::Interface::as_raw(self), core::mem::transmute(pquery)).ok() }
    }
    pub unsafe fn SetSubscription(&self, query: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSubscription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(query)).ok() }
    }
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
    pub unsafe fn ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueQueries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetValueQueries<P0>(&self, pnamedxpaths: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValueQueries)(windows_core::Interface::as_raw(self), pnamedxpaths.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Subscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventTrigger_Impl: ITrigger_Impl {
    fn Subscription(&self, pquery: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSubscription(&self, query: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn SetValueQueries(&self, pnamedxpaths: windows_core::Ref<ITaskNamedValueCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: IEventTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Subscription<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::Subscription(this, core::mem::transmute_copy(&pquery)).into()
            }
        }
        unsafe extern "system" fn SetSubscription<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::SetSubscription(this, core::mem::transmute(&query)).into()
            }
        }
        unsafe extern "system" fn Delay<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetDelay<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn ValueQueries<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamedxpaths: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEventTrigger_Impl::ValueQueries(this) {
                    Ok(ok__) => {
                        ppnamedxpaths.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueQueries<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamedxpaths: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::SetValueQueries(this, core::mem::transmute_copy(&pnamedxpaths)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Subscription: Subscription::<Identity, OFFSET>,
            SetSubscription: SetSubscription::<Identity, OFFSET>,
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            ValueQueries: ValueQueries::<Identity, OFFSET>,
            SetValueQueries: SetValueQueries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IExecAction, IExecAction_Vtbl, 0x4c3d624d_fd6b_49a3_b9b7_09cb3cd3f047);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IExecAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IExecAction, windows_core::IUnknown, super::Com::IDispatch, IAction);
#[cfg(feature = "Win32_System_Com")]
impl IExecAction {
    pub unsafe fn Path(&self, ppath: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), core::mem::transmute(ppath)).ok() }
    }
    pub unsafe fn SetPath(&self, path: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)).ok() }
    }
    pub unsafe fn Arguments(&self, pargument: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), core::mem::transmute(pargument)).ok() }
    }
    pub unsafe fn SetArguments(&self, argument: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(argument)).ok() }
    }
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(pworkingdirectory)).ok() }
    }
    pub unsafe fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(workingdirectory)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExecAction_Impl: IAction_Impl {
    fn Path(&self, ppath: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPath(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Arguments(&self, pargument: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetArguments(&self, argument: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WorkingDirectory(&self, pworkingdirectory: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IExecAction_Vtbl {
    pub const fn new<Identity: IExecAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::Path(this, core::mem::transmute_copy(&ppath)).into()
            }
        }
        unsafe extern "system" fn SetPath<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::SetPath(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn Arguments<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pargument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::Arguments(this, core::mem::transmute_copy(&pargument)).into()
            }
        }
        unsafe extern "system" fn SetArguments<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argument: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::SetArguments(this, core::mem::transmute(&argument)).into()
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pworkingdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::WorkingDirectory(this, core::mem::transmute_copy(&pworkingdirectory)).into()
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::SetWorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            Arguments: Arguments::<Identity, OFFSET>,
            SetArguments: SetArguments::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IExecAction {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IExecAction2, IExecAction2_Vtbl, 0xf2a82542_bda5_4e6b_9143_e2bf4f8987b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IExecAction2 {
    type Target = IExecAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IExecAction2, windows_core::IUnknown, super::Com::IDispatch, IAction, IExecAction);
#[cfg(feature = "Win32_System_Com")]
impl IExecAction2 {
    pub unsafe fn HideAppWindow(&self, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HideAppWindow)(windows_core::Interface::as_raw(self), phideappwindow as _).ok() }
    }
    pub unsafe fn SetHideAppWindow(&self, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHideAppWindow)(windows_core::Interface::as_raw(self), hideappwindow).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction2_Vtbl {
    pub base__: IExecAction_Vtbl,
    pub HideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExecAction2_Impl: IExecAction_Impl {
    fn HideAppWindow(&self, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetHideAppWindow(&self, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: IExecAction2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HideAppWindow<Identity: IExecAction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phideappwindow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction2_Impl::HideAppWindow(this, core::mem::transmute_copy(&phideappwindow)).into()
            }
        }
        unsafe extern "system" fn SetHideAppWindow<Identity: IExecAction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hideappwindow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction2_Impl::SetHideAppWindow(this, core::mem::transmute_copy(&hideappwindow)).into()
            }
        }
        Self {
            base__: IExecAction_Vtbl::new::<Identity, OFFSET>(),
            HideAppWindow: HideAppWindow::<Identity, OFFSET>,
            SetHideAppWindow: SetHideAppWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID || iid == &<IExecAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IExecAction2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIdleSettings, IIdleSettings_Vtbl, 0x84594461_0053_4342_a8fd_088fabf11f32);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIdleSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIdleSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIdleSettings {
    pub unsafe fn IdleDuration(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IdleDuration)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetIdleDuration(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIdleDuration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
    pub unsafe fn WaitTimeout(&self, ptimeout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitTimeout)(windows_core::Interface::as_raw(self), core::mem::transmute(ptimeout)).ok() }
    }
    pub unsafe fn SetWaitTimeout(&self, timeout: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWaitTimeout)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timeout)).ok() }
    }
    pub unsafe fn StopOnIdleEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopOnIdleEnd)(windows_core::Interface::as_raw(self), pstop as _).ok() }
    }
    pub unsafe fn SetStopOnIdleEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStopOnIdleEnd)(windows_core::Interface::as_raw(self), stop).ok() }
    }
    pub unsafe fn RestartOnIdle(&self, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestartOnIdle)(windows_core::Interface::as_raw(self), prestart as _).ok() }
    }
    pub unsafe fn SetRestartOnIdle(&self, restart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRestartOnIdle)(windows_core::Interface::as_raw(self), restart).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIdleSettings_Impl: super::Com::IDispatch_Impl {
    fn IdleDuration(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIdleDuration(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WaitTimeout(&self, ptimeout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetWaitTimeout(&self, timeout: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopOnIdleEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopOnIdleEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RestartOnIdle(&self, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRestartOnIdle(&self, restart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIdleSettings_Vtbl {
    pub const fn new<Identity: IIdleSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IdleDuration<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::IdleDuration(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetIdleDuration<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::SetIdleDuration(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn WaitTimeout<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::WaitTimeout(this, core::mem::transmute_copy(&ptimeout)).into()
            }
        }
        unsafe extern "system" fn SetWaitTimeout<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::SetWaitTimeout(this, core::mem::transmute(&timeout)).into()
            }
        }
        unsafe extern "system" fn StopOnIdleEnd<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::StopOnIdleEnd(this, core::mem::transmute_copy(&pstop)).into()
            }
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::SetStopOnIdleEnd(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn RestartOnIdle<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::RestartOnIdle(this, core::mem::transmute_copy(&prestart)).into()
            }
        }
        unsafe extern "system" fn SetRestartOnIdle<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::SetRestartOnIdle(this, core::mem::transmute_copy(&restart)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IdleDuration: IdleDuration::<Identity, OFFSET>,
            SetIdleDuration: SetIdleDuration::<Identity, OFFSET>,
            WaitTimeout: WaitTimeout::<Identity, OFFSET>,
            SetWaitTimeout: SetWaitTimeout::<Identity, OFFSET>,
            StopOnIdleEnd: StopOnIdleEnd::<Identity, OFFSET>,
            SetStopOnIdleEnd: SetStopOnIdleEnd::<Identity, OFFSET>,
            RestartOnIdle: RestartOnIdle::<Identity, OFFSET>,
            SetRestartOnIdle: SetRestartOnIdle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIdleSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIdleTrigger, IIdleTrigger_Vtbl, 0xd537d2b0_9fb3_4d34_9739_1ff5ce7b1ef3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIdleTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIdleTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIdleTrigger_Impl: ITrigger_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: IIdleTrigger_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIdleTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ILogonTrigger, ILogonTrigger_Vtbl, 0x72dade38_fae4_4b3e_baf4_5d009af02b1c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ILogonTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ILogonTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl ILogonTrigger {
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok() }
    }
    pub unsafe fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogonTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILogonTrigger_Impl: ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILogonTrigger_Vtbl {
    pub const fn new<Identity: ILogonTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delay<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetDelay<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn UserId<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
            }
        }
        unsafe extern "system" fn SetUserId<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::SetUserId(this, core::mem::transmute(&user)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogonTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ILogonTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMaintenanceSettings, IMaintenanceSettings_Vtbl, 0xa6024fa8_9652_4adb_a6bf_5cfcd877a7ba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMaintenanceSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMaintenanceSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IMaintenanceSettings {
    pub unsafe fn SetPeriod(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPeriod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn Period(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Period)(windows_core::Interface::as_raw(self), core::mem::transmute(target)).ok() }
    }
    pub unsafe fn SetDeadline(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeadline)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn Deadline(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), core::mem::transmute(target)).ok() }
    }
    pub unsafe fn SetExclusive(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExclusive)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn Exclusive(&self, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Exclusive)(windows_core::Interface::as_raw(self), target as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Period: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDeadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Exclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMaintenanceSettings_Impl: super::Com::IDispatch_Impl {
    fn SetPeriod(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Period(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDeadline(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Deadline(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExclusive(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Exclusive(&self, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMaintenanceSettings_Vtbl {
    pub const fn new<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPeriod<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::SetPeriod(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Period<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::Period(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn SetDeadline<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::SetDeadline(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Deadline<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::Deadline(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn SetExclusive<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::SetExclusive(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Exclusive<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::Exclusive(this, core::mem::transmute_copy(&target)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetPeriod: SetPeriod::<Identity, OFFSET>,
            Period: Period::<Identity, OFFSET>,
            SetDeadline: SetDeadline::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            SetExclusive: SetExclusive::<Identity, OFFSET>,
            Exclusive: Exclusive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMaintenanceSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMaintenanceSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMonthlyDOWTrigger, IMonthlyDOWTrigger_Vtbl, 0x77d025a3_90fa_43aa_b52e_cda5499b946a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMonthlyDOWTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMonthlyDOWTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyDOWTrigger {
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DaysOfWeek)(windows_core::Interface::as_raw(self), pdays as _).ok() }
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysOfWeek)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn WeeksOfMonth(&self, pweeks: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WeeksOfMonth)(windows_core::Interface::as_raw(self), pweeks as _).ok() }
    }
    pub unsafe fn SetWeeksOfMonth(&self, weeks: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWeeksOfMonth)(windows_core::Interface::as_raw(self), weeks).ok() }
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MonthsOfYear)(windows_core::Interface::as_raw(self), pmonths as _).ok() }
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonthsOfYear)(windows_core::Interface::as_raw(self), months).ok() }
    }
    pub unsafe fn RunOnLastWeekOfMonth(&self, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), plastweek as _).ok() }
    }
    pub unsafe fn SetRunOnLastWeekOfMonth(&self, lastweek: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), lastweek).ok() }
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok() }
    }
    pub unsafe fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyDOWTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WeeksOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetWeeksOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RunOnLastWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnLastWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMonthlyDOWTrigger_Impl: ITrigger_Impl {
    fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn WeeksOfMonth(&self, pweeks: *mut i16) -> windows_core::Result<()>;
    fn SetWeeksOfMonth(&self, weeks: i16) -> windows_core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn RunOnLastWeekOfMonth(&self, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnLastWeekOfMonth(&self, lastweek: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMonthlyDOWTrigger_Vtbl {
    pub const fn new<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DaysOfWeek<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::DaysOfWeek(this, core::mem::transmute_copy(&pdays)).into()
            }
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::SetDaysOfWeek(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn WeeksOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::WeeksOfMonth(this, core::mem::transmute_copy(&pweeks)).into()
            }
        }
        unsafe extern "system" fn SetWeeksOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::SetWeeksOfMonth(this, core::mem::transmute_copy(&weeks)).into()
            }
        }
        unsafe extern "system" fn MonthsOfYear<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::MonthsOfYear(this, core::mem::transmute_copy(&pmonths)).into()
            }
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::SetMonthsOfYear(this, core::mem::transmute_copy(&months)).into()
            }
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::RunOnLastWeekOfMonth(this, core::mem::transmute_copy(&plastweek)).into()
            }
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastweek: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::SetRunOnLastWeekOfMonth(this, core::mem::transmute_copy(&lastweek)).into()
            }
        }
        unsafe extern "system" fn RandomDelay<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
            }
        }
        unsafe extern "system" fn SetRandomDelay<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, OFFSET>,
            WeeksOfMonth: WeeksOfMonth::<Identity, OFFSET>,
            SetWeeksOfMonth: SetWeeksOfMonth::<Identity, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, OFFSET>,
            RunOnLastWeekOfMonth: RunOnLastWeekOfMonth::<Identity, OFFSET>,
            SetRunOnLastWeekOfMonth: SetRunOnLastWeekOfMonth::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMonthlyDOWTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IMonthlyTrigger, IMonthlyTrigger_Vtbl, 0x97c45ef1_6b02_4a1a_9c0e_1ebfba1500ac);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IMonthlyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IMonthlyTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyTrigger {
    pub unsafe fn DaysOfMonth(&self, pdays: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DaysOfMonth)(windows_core::Interface::as_raw(self), pdays as _).ok() }
    }
    pub unsafe fn SetDaysOfMonth(&self, days: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysOfMonth)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MonthsOfYear)(windows_core::Interface::as_raw(self), pmonths as _).ok() }
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMonthsOfYear)(windows_core::Interface::as_raw(self), months).ok() }
    }
    pub unsafe fn RunOnLastDayOfMonth(&self, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), plastday as _).ok() }
    }
    pub unsafe fn SetRunOnLastDayOfMonth(&self, lastday: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), lastday).ok() }
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok() }
    }
    pub unsafe fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMonthlyTrigger_Impl: ITrigger_Impl {
    fn DaysOfMonth(&self, pdays: *mut i32) -> windows_core::Result<()>;
    fn SetDaysOfMonth(&self, days: i32) -> windows_core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn RunOnLastDayOfMonth(&self, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnLastDayOfMonth(&self, lastday: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMonthlyTrigger_Vtbl {
    pub const fn new<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DaysOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::DaysOfMonth(this, core::mem::transmute_copy(&pdays)).into()
            }
        }
        unsafe extern "system" fn SetDaysOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::SetDaysOfMonth(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn MonthsOfYear<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::MonthsOfYear(this, core::mem::transmute_copy(&pmonths)).into()
            }
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::SetMonthsOfYear(this, core::mem::transmute_copy(&months)).into()
            }
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::RunOnLastDayOfMonth(this, core::mem::transmute_copy(&plastday)).into()
            }
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastday: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::SetRunOnLastDayOfMonth(this, core::mem::transmute_copy(&lastday)).into()
            }
        }
        unsafe extern "system" fn RandomDelay<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
            }
        }
        unsafe extern "system" fn SetRandomDelay<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfMonth: DaysOfMonth::<Identity, OFFSET>,
            SetDaysOfMonth: SetDaysOfMonth::<Identity, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, OFFSET>,
            RunOnLastDayOfMonth: RunOnLastDayOfMonth::<Identity, OFFSET>,
            SetRunOnLastDayOfMonth: SetRunOnLastDayOfMonth::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMonthlyTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(INetworkSettings, INetworkSettings_Vtbl, 0x9f7dea84_c30b_4245_80b6_00e9f646f1b4);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for INetworkSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(INetworkSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl INetworkSettings {
    pub unsafe fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok() }
    }
    pub unsafe fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkSettings_Impl: super::Com::IDispatch_Impl {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkSettings_Vtbl {
    pub const fn new<Identity: INetworkSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn SetName<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn SetId<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::SetId(this, core::mem::transmute(&id)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for INetworkSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrincipal, IPrincipal_Vtbl, 0xd98d51e5_c9b4_496a_a9c1_18980261cf0f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrincipal {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrincipal, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal {
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok() }
    }
    pub unsafe fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)).ok() }
    }
    pub unsafe fn DisplayName(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok() }
    }
    pub unsafe fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)).ok() }
    }
    pub unsafe fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LogonType)(windows_core::Interface::as_raw(self), plogon as _).ok() }
    }
    pub unsafe fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLogonType)(windows_core::Interface::as_raw(self), logon).ok() }
    }
    pub unsafe fn GroupId(&self, pgroup: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GroupId)(windows_core::Interface::as_raw(self), core::mem::transmute(pgroup)).ok() }
    }
    pub unsafe fn SetGroupId(&self, group: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group)).ok() }
    }
    pub unsafe fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunLevel)(windows_core::Interface::as_raw(self), prunlevel as _).ok() }
    }
    pub unsafe fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunLevel)(windows_core::Interface::as_raw(self), runlevel).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogonType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub SetLogonType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub GroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
    pub SetRunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrincipal_Impl: super::Com::IDispatch_Impl {
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayName(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> windows_core::Result<()>;
    fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::Result<()>;
    fn GroupId(&self, pgroup: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetGroupId(&self, group: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::Result<()>;
    fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrincipal_Vtbl {
    pub const fn new<Identity: IPrincipal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn SetId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetId(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::DisplayName(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn UserId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
            }
        }
        unsafe extern "system" fn SetUserId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetUserId(this, core::mem::transmute(&user)).into()
            }
        }
        unsafe extern "system" fn LogonType<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::LogonType(this, core::mem::transmute_copy(&plogon)).into()
            }
        }
        unsafe extern "system" fn SetLogonType<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logon: TASK_LOGON_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetLogonType(this, core::mem::transmute_copy(&logon)).into()
            }
        }
        unsafe extern "system" fn GroupId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::GroupId(this, core::mem::transmute_copy(&pgroup)).into()
            }
        }
        unsafe extern "system" fn SetGroupId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetGroupId(this, core::mem::transmute(&group)).into()
            }
        }
        unsafe extern "system" fn RunLevel<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::RunLevel(this, core::mem::transmute_copy(&prunlevel)).into()
            }
        }
        unsafe extern "system" fn SetRunLevel<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::SetRunLevel(this, core::mem::transmute_copy(&runlevel)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
            LogonType: LogonType::<Identity, OFFSET>,
            SetLogonType: SetLogonType::<Identity, OFFSET>,
            GroupId: GroupId::<Identity, OFFSET>,
            SetGroupId: SetGroupId::<Identity, OFFSET>,
            RunLevel: RunLevel::<Identity, OFFSET>,
            SetRunLevel: SetRunLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrincipal {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPrincipal2, IPrincipal2_Vtbl, 0x248919ae_e345_4a6d_8aeb_e0d3165c904e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPrincipal2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPrincipal2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal2 {
    pub unsafe fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessTokenSidType)(windows_core::Interface::as_raw(self), pprocesstokensidtype as _).ok() }
    }
    pub unsafe fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProcessTokenSidType)(windows_core::Interface::as_raw(self), processtokensidtype).ok() }
    }
    pub unsafe fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequiredPrivilegeCount)(windows_core::Interface::as_raw(self), pcount as _).ok() }
    }
    pub unsafe fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_RequiredPrivilege)(windows_core::Interface::as_raw(self), index, core::mem::transmute(pprivilege)).ok() }
    }
    pub unsafe fn AddRequiredPrivilege(&self, privilege: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRequiredPrivilege)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(privilege)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub SetProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub RequiredPrivilegeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPrincipal2_Impl: super::Com::IDispatch_Impl {
    fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()>;
    fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()>;
    fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddRequiredPrivilege(&self, privilege: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPrincipal2_Vtbl {
    pub const fn new<Identity: IPrincipal2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessTokenSidType<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::ProcessTokenSidType(this, core::mem::transmute_copy(&pprocesstokensidtype)).into()
            }
        }
        unsafe extern "system" fn SetProcessTokenSidType<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::SetProcessTokenSidType(this, core::mem::transmute_copy(&processtokensidtype)).into()
            }
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::RequiredPrivilegeCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn get_RequiredPrivilege<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::get_RequiredPrivilege(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pprivilege)).into()
            }
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privilege: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::AddRequiredPrivilege(this, core::mem::transmute(&privilege)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProcessTokenSidType: ProcessTokenSidType::<Identity, OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Identity, OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Identity, OFFSET>,
            get_RequiredPrivilege: get_RequiredPrivilege::<Identity, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPrincipal2 {}
windows_core::imp::define_interface!(IProvideTaskPage, IProvideTaskPage_Vtbl, 0x4086658a_cbbb_11cf_b604_00c04fd8d565);
windows_core::imp::interface_hierarchy!(IProvideTaskPage, windows_core::IUnknown);
impl IProvideTaskPage {
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetPage(&self, tptype: TASKPAGE, fpersistchanges: bool) -> windows_core::Result<super::super::UI::Controls::HPROPSHEETPAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPage)(windows_core::Interface::as_raw(self), tptype, fpersistchanges.into(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideTaskPage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPage: unsafe extern "system" fn(*mut core::ffi::c_void, TASKPAGE, windows_core::BOOL, *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPage: usize,
}
#[cfg(feature = "Win32_UI_Controls")]
pub trait IProvideTaskPage_Impl: windows_core::IUnknownImpl {
    fn GetPage(&self, tptype: TASKPAGE, fpersistchanges: windows_core::BOOL) -> windows_core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl IProvideTaskPage_Vtbl {
    pub const fn new<Identity: IProvideTaskPage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPage<Identity: IProvideTaskPage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: windows_core::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideTaskPage_Impl::GetPage(this, core::mem::transmute_copy(&tptype), core::mem::transmute_copy(&fpersistchanges)) {
                    Ok(ok__) => {
                        phpage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPage: GetPage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideTaskPage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Controls")]
impl windows_core::RuntimeName for IProvideTaskPage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRegisteredTask, IRegisteredTask_Vtbl, 0x9c86f320_dee3_4dd1_b972_a303f26b061e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRegisteredTask {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRegisteredTask, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTask {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<TASK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Run(&self, params: &super::Variant::VARIANT) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(params), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RunEx(&self, params: &super::Variant::VARIANT, flags: i32, sessionid: i32, user: &windows_core::BSTR) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(params), flags, sessionid, core::mem::transmute_copy(user), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInstances(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstances)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LastRunTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastTaskResult(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastTaskResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumberOfMissedRuns(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfMissedRuns)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NextRunTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Definition(&self) -> windows_core::Result<ITaskDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Definition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl), flags).ok() }
    }
    pub unsafe fn Stop(&self, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), flags).ok() }
    }
    pub unsafe fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRunTimes)(windows_core::Interface::as_raw(self), pststart, pstend, pcount as _, pruntimes as _).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Run: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RunEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RunEx: usize,
    pub GetInstances: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastTaskResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfMissedRuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Definition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRunTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME, *const super::super::Foundation::SYSTEMTIME, *mut u32, *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegisteredTask_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TASK_STATE>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Run(&self, params: &super::Variant::VARIANT) -> windows_core::Result<IRunningTask>;
    fn RunEx(&self, params: &super::Variant::VARIANT, flags: i32, sessionid: i32, user: &windows_core::BSTR) -> windows_core::Result<IRunningTask>;
    fn GetInstances(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection>;
    fn LastRunTime(&self) -> windows_core::Result<f64>;
    fn LastTaskResult(&self) -> windows_core::Result<i32>;
    fn NumberOfMissedRuns(&self) -> windows_core::Result<i32>;
    fn NextRunTime(&self) -> windows_core::Result<f64>;
    fn Definition(&self) -> windows_core::Result<ITaskDefinition>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn Stop(&self, flags: i32) -> windows_core::Result<()>;
    fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRegisteredTask_Vtbl {
    pub const fn new<Identity: IRegisteredTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Enabled(this) {
                    Ok(ok__) => {
                        penabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: super::Variant::VARIANT, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Run(this, core::mem::transmute(&params)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunEx<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: super::Variant::VARIANT, flags: i32, sessionid: i32, user: *mut core::ffi::c_void, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::RunEx(this, core::mem::transmute(&params), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&sessionid), core::mem::transmute(&user)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstances<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::GetInstances(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pprunningtasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastRunTime<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastruntime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::LastRunTime(this) {
                    Ok(ok__) => {
                        plastruntime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastTaskResult<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasttaskresult: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::LastTaskResult(this) {
                    Ok(ok__) => {
                        plasttaskresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfMissedRuns<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumberofmissedruns: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::NumberOfMissedRuns(this) {
                    Ok(ok__) => {
                        pnumberofmissedruns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextRunTime<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnextruntime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::NextRunTime(this) {
                    Ok(ok__) => {
                        pnextruntime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Definition<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Definition(this) {
                    Ok(ok__) => {
                        ppdefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Xml<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Xml(this) {
                    Ok(ok__) => {
                        pxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: i32, psddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                    Ok(ok__) => {
                        psddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::Stop(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::GetRunTimes(this, core::mem::transmute_copy(&pststart), core::mem::transmute_copy(&pstend), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pruntimes)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            RunEx: RunEx::<Identity, OFFSET>,
            GetInstances: GetInstances::<Identity, OFFSET>,
            LastRunTime: LastRunTime::<Identity, OFFSET>,
            LastTaskResult: LastTaskResult::<Identity, OFFSET>,
            NumberOfMissedRuns: NumberOfMissedRuns::<Identity, OFFSET>,
            NextRunTime: NextRunTime::<Identity, OFFSET>,
            Definition: Definition::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTask as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRegisteredTask {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRegisteredTaskCollection, IRegisteredTaskCollection_Vtbl, 0x86627eb4_42a7_41e4_a4d9_ac33a72f2d52);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRegisteredTaskCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRegisteredTaskCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTaskCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegisteredTaskCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRegisteredTaskCollection_Vtbl {
    pub const fn new<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppregisteredtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppregisteredtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRegisteredTaskCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRegistrationInfo, IRegistrationInfo_Vtbl, 0x416d8b73_cb41_4ea1_805c_9be9a5ac4a74);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRegistrationInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRegistrationInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationInfo {
    pub unsafe fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), core::mem::transmute(pdescription)).ok() }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)).ok() }
    }
    pub unsafe fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), core::mem::transmute(pauthor)).ok() }
    }
    pub unsafe fn SetAuthor(&self, author: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAuthor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(author)).ok() }
    }
    pub unsafe fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), core::mem::transmute(pversion)).ok() }
    }
    pub unsafe fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(version)).ok() }
    }
    pub unsafe fn Date(&self, pdate: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Date)(windows_core::Interface::as_raw(self), core::mem::transmute(pdate)).ok() }
    }
    pub unsafe fn SetDate(&self, date: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(date)).ok() }
    }
    pub unsafe fn Documentation(&self, pdocumentation: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Documentation)(windows_core::Interface::as_raw(self), core::mem::transmute(pdocumentation)).ok() }
    }
    pub unsafe fn SetDocumentation(&self, documentation: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(documentation)).ok() }
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok() }
    }
    pub unsafe fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)).ok() }
    }
    pub unsafe fn URI(&self, puri: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).URI)(windows_core::Interface::as_raw(self), core::mem::transmute(puri)).ok() }
    }
    pub unsafe fn SetURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uri)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SecurityDescriptor(&self, psddl: *mut super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute(psddl)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSecurityDescriptor(&self, sddl: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl)).ok() }
    }
    pub unsafe fn Source(&self, psource: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), core::mem::transmute(psource)).ok() }
    }
    pub unsafe fn SetSource(&self, source: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(source)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Documentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub URI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSecurityDescriptor: usize,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegistrationInfo_Impl: super::Com::IDispatch_Impl {
    fn Description(&self, pdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetAuthor(&self, author: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Date(&self, pdate: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDate(&self, date: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Documentation(&self, pdocumentation: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDocumentation(&self, documentation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn URI(&self, puri: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetURI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SecurityDescriptor(&self, psddl: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetSecurityDescriptor(&self, sddl: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Source(&self, psource: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSource(&self, source: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRegistrationInfo_Vtbl {
    pub const fn new<Identity: IRegistrationInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Description(this, core::mem::transmute_copy(&pdescription)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Author<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Author(this, core::mem::transmute_copy(&pauthor)).into()
            }
        }
        unsafe extern "system" fn SetAuthor<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, author: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetAuthor(this, core::mem::transmute(&author)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Version(this, core::mem::transmute_copy(&pversion)).into()
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetVersion(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn Date<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Date(this, core::mem::transmute_copy(&pdate)).into()
            }
        }
        unsafe extern "system" fn SetDate<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetDate(this, core::mem::transmute(&date)).into()
            }
        }
        unsafe extern "system" fn Documentation<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Documentation(this, core::mem::transmute_copy(&pdocumentation)).into()
            }
        }
        unsafe extern "system" fn SetDocumentation<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetDocumentation(this, core::mem::transmute(&documentation)).into()
            }
        }
        unsafe extern "system" fn XmlText<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
            }
        }
        unsafe extern "system" fn SetXmlText<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn URI<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::URI(this, core::mem::transmute_copy(&puri)).into()
            }
        }
        unsafe extern "system" fn SetURI<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetURI(this, core::mem::transmute(&uri)).into()
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psddl: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SecurityDescriptor(this, core::mem::transmute_copy(&psddl)).into()
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl)).into()
            }
        }
        unsafe extern "system" fn Source<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::Source(this, core::mem::transmute_copy(&psource)).into()
            }
        }
        unsafe extern "system" fn SetSource<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::SetSource(this, core::mem::transmute(&source)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            SetAuthor: SetAuthor::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            Date: Date::<Identity, OFFSET>,
            SetDate: SetDate::<Identity, OFFSET>,
            Documentation: Documentation::<Identity, OFFSET>,
            SetDocumentation: SetDocumentation::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            URI: URI::<Identity, OFFSET>,
            SetURI: SetURI::<Identity, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            Source: Source::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRegistrationInfo {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRegistrationTrigger, IRegistrationTrigger_Vtbl, 0x4c8fec3a_c218_4e0c_b23d_629024db91a2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRegistrationTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRegistrationTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationTrigger {
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRegistrationTrigger_Impl: ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delay<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetDelay<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
            }
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), Delay: Delay::<Identity, OFFSET>, SetDelay: SetDelay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRegistrationTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRepetitionPattern, IRepetitionPattern_Vtbl, 0x7fb9acf1_26be_400e_85b5_294b9c75dfd6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRepetitionPattern {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRepetitionPattern, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRepetitionPattern {
    pub unsafe fn Interval(&self, pinterval: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Interval)(windows_core::Interface::as_raw(self), core::mem::transmute(pinterval)).ok() }
    }
    pub unsafe fn SetInterval(&self, interval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInterval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interval)).ok() }
    }
    pub unsafe fn Duration(&self, pduration: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), core::mem::transmute(pduration)).ok() }
    }
    pub unsafe fn SetDuration(&self, duration: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(duration)).ok() }
    }
    pub unsafe fn StopAtDurationEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopAtDurationEnd)(windows_core::Interface::as_raw(self), pstop as _).ok() }
    }
    pub unsafe fn SetStopAtDurationEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStopAtDurationEnd)(windows_core::Interface::as_raw(self), stop).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRepetitionPattern_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRepetitionPattern_Impl: super::Com::IDispatch_Impl {
    fn Interval(&self, pinterval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInterval(&self, interval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Duration(&self, pduration: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopAtDurationEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopAtDurationEnd(&self, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRepetitionPattern_Vtbl {
    pub const fn new<Identity: IRepetitionPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Interval<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinterval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::Interval(this, core::mem::transmute_copy(&pinterval)).into()
            }
        }
        unsafe extern "system" fn SetInterval<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::SetInterval(this, core::mem::transmute(&interval)).into()
            }
        }
        unsafe extern "system" fn Duration<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::Duration(this, core::mem::transmute_copy(&pduration)).into()
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::SetDuration(this, core::mem::transmute(&duration)).into()
            }
        }
        unsafe extern "system" fn StopAtDurationEnd<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::StopAtDurationEnd(this, core::mem::transmute_copy(&pstop)).into()
            }
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::SetStopAtDurationEnd(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Interval: Interval::<Identity, OFFSET>,
            SetInterval: SetInterval::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            StopAtDurationEnd: StopAtDurationEnd::<Identity, OFFSET>,
            SetStopAtDurationEnd: SetStopAtDurationEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRepetitionPattern as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRepetitionPattern {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRunningTask, IRunningTask_Vtbl, 0x653758fb_7b9a_4f1e_a471_beeb8e9b834e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRunningTask {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRunningTask, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRunningTask {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstanceGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<TASK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnginePID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnginePID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstanceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnginePID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRunningTask_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TASK_STATE>;
    fn CurrentAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn EnginePID(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRunningTask_Vtbl {
    pub const fn new<Identity: IRunningTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstanceGuid<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::InstanceGuid(this) {
                    Ok(ok__) => {
                        pguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stop<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningTask_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningTask_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn EnginePID<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::EnginePID(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            InstanceGuid: InstanceGuid::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            EnginePID: EnginePID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTask as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRunningTask {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRunningTaskCollection, IRunningTaskCollection_Vtbl, 0x6a67614b_6828_4fec_aa54_6d52e8f1f2db);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRunningTaskCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRunningTaskCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRunningTaskCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRunningTaskCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<IRunningTask>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRunningTaskCollection_Vtbl {
    pub const fn new<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTaskCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRunningTaskCollection {}
windows_core::imp::define_interface!(IScheduledWorkItem, IScheduledWorkItem_Vtbl, 0xa6b952f0_a4b1_11d0_997d_00aa006887ec);
windows_core::imp::interface_hierarchy!(IScheduledWorkItem, windows_core::IUnknown);
impl IScheduledWorkItem {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut Option<ITaskTrigger>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateTrigger)(windows_core::Interface::as_raw(self), pinewtrigger as _, core::mem::transmute(pptrigger)).ok() }
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTrigger)(windows_core::Interface::as_raw(self), itrigger).ok() }
    }
    pub unsafe fn GetTriggerCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTriggerCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> windows_core::Result<ITaskTrigger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrigger)(windows_core::Interface::as_raw(self), itrigger, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTriggerString)(windows_core::Interface::as_raw(self), itrigger, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRunTimes)(windows_core::Interface::as_raw(self), pstbegin, pstend, pcount as _, rgsttasktimes as _).ok() }
    }
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNextRunTime)(windows_core::Interface::as_raw(self), pstnextrun as _).ok() }
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIdleWait)(windows_core::Interface::as_raw(self), widleminutes, wdeadlineminutes).ok() }
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIdleWait)(windows_core::Interface::as_raw(self), pwidleminutes as _, pwdeadlineminutes as _).ok() }
    }
    pub unsafe fn Run(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EditWorkItem(&self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EditWorkItem)(windows_core::Interface::as_raw(self), hparent, dwreserved).ok() }
    }
    pub unsafe fn GetMostRecentRunTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMostRecentRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetExitCode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExitCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetComment<P0>(&self, pwszcomment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetComment)(windows_core::Interface::as_raw(self), pwszcomment.param().abi()).ok() }
    }
    pub unsafe fn GetComment(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreator<P0>(&self, pwszcreator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCreator)(windows_core::Interface::as_raw(self), pwszcreator.param().abi()).ok() }
    }
    pub unsafe fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWorkItemData)(windows_core::Interface::as_raw(self), cbdata, rgbdata).ok() }
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetWorkItemData)(windows_core::Interface::as_raw(self), pcbdata as _, prgbdata as _).ok() }
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetErrorRetryCount)(windows_core::Interface::as_raw(self), wretrycount).ok() }
    }
    pub unsafe fn GetErrorRetryCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorRetryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetErrorRetryInterval)(windows_core::Interface::as_raw(self), wretryinterval).ok() }
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorRetryInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccountInformation<P0, P1>(&self, pwszaccountname: P0, pwszpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccountInformation)(windows_core::Interface::as_raw(self), pwszaccountname.param().abi(), pwszpassword.param().abi()).ok() }
    }
    pub unsafe fn GetAccountInformation(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccountInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledWorkItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetTriggerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetRunTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME, *const super::super::Foundation::SYSTEMTIME, *mut u16, *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetNextRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub SetIdleWait: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub GetIdleWait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut u16) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EditWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub GetMostRecentRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetExitCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetComment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetComment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetWorkItemData: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *const u8) -> windows_core::HRESULT,
    pub GetWorkItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut *mut u8) -> windows_core::HRESULT,
    pub SetErrorRetryCount: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetErrorRetryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetErrorRetryInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetErrorRetryInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAccountInformation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAccountInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IScheduledWorkItem_Impl: windows_core::IUnknownImpl {
    fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: windows_core::OutRef<ITaskTrigger>) -> windows_core::Result<()>;
    fn DeleteTrigger(&self, itrigger: u16) -> windows_core::Result<()>;
    fn GetTriggerCount(&self) -> windows_core::Result<u16>;
    fn GetTrigger(&self, itrigger: u16) -> windows_core::Result<ITaskTrigger>;
    fn GetTriggerString(&self, itrigger: u16) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
    fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::Result<()>;
    fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::Result<()>;
    fn Run(&self) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn EditWorkItem(&self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> windows_core::Result<()>;
    fn GetMostRecentRunTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetExitCode(&self) -> windows_core::Result<u32>;
    fn SetComment(&self, pwszcomment: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetComment(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCreator(&self, pwszcreator: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> windows_core::Result<()>;
    fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn SetErrorRetryCount(&self, wretrycount: u16) -> windows_core::Result<()>;
    fn GetErrorRetryCount(&self) -> windows_core::Result<u16>;
    fn SetErrorRetryInterval(&self, wretryinterval: u16) -> windows_core::Result<()>;
    fn GetErrorRetryInterval(&self) -> windows_core::Result<u16>;
    fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u32>;
    fn SetAccountInformation(&self, pwszaccountname: &windows_core::PCWSTR, pwszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAccountInformation(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IScheduledWorkItem_Vtbl {
    pub const fn new<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTrigger<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::CreateTrigger(this, core::mem::transmute_copy(&pinewtrigger), core::mem::transmute_copy(&pptrigger)).into()
            }
        }
        unsafe extern "system" fn DeleteTrigger<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::DeleteTrigger(this, core::mem::transmute_copy(&itrigger)).into()
            }
        }
        unsafe extern "system" fn GetTriggerCount<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetTriggerCount(this) {
                    Ok(ok__) => {
                        pwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTrigger<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetTrigger(this, core::mem::transmute_copy(&itrigger)) {
                    Ok(ok__) => {
                        pptrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetTriggerString(this, core::mem::transmute_copy(&itrigger)) {
                    Ok(ok__) => {
                        ppwsztrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::GetRunTimes(this, core::mem::transmute_copy(&pstbegin), core::mem::transmute_copy(&pstend), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&rgsttasktimes)).into()
            }
        }
        unsafe extern "system" fn GetNextRunTime<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::GetNextRunTime(this, core::mem::transmute_copy(&pstnextrun)).into()
            }
        }
        unsafe extern "system" fn SetIdleWait<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetIdleWait(this, core::mem::transmute_copy(&widleminutes), core::mem::transmute_copy(&wdeadlineminutes)).into()
            }
        }
        unsafe extern "system" fn GetIdleWait<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::GetIdleWait(this, core::mem::transmute_copy(&pwidleminutes), core::mem::transmute_copy(&pwdeadlineminutes)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::Run(this).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::Terminate(this).into()
            }
        }
        unsafe extern "system" fn EditWorkItem<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::EditWorkItem(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetMostRecentRunTime<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetMostRecentRunTime(this) {
                    Ok(ok__) => {
                        pstlastrun.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        phrstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExitCode<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwexitcode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetExitCode(this) {
                    Ok(ok__) => {
                        pdwexitcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetComment<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomment: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetComment(this, core::mem::transmute(&pwszcomment)).into()
            }
        }
        unsafe extern "system" fn GetComment<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcomment: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetComment(this) {
                    Ok(ok__) => {
                        ppwszcomment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreator<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcreator: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetCreator(this, core::mem::transmute(&pwszcreator)).into()
            }
        }
        unsafe extern "system" fn GetCreator<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcreator: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetCreator(this) {
                    Ok(ok__) => {
                        ppwszcreator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkItemData<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetWorkItemData(this, core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&rgbdata)).into()
            }
        }
        unsafe extern "system" fn GetWorkItemData<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::GetWorkItemData(this, core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&prgbdata)).into()
            }
        }
        unsafe extern "system" fn SetErrorRetryCount<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wretrycount: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetErrorRetryCount(this, core::mem::transmute_copy(&wretrycount)).into()
            }
        }
        unsafe extern "system" fn GetErrorRetryCount<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwretrycount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetErrorRetryCount(this) {
                    Ok(ok__) => {
                        pwretrycount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorRetryInterval<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wretryinterval: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetErrorRetryInterval(this, core::mem::transmute_copy(&wretryinterval)).into()
            }
        }
        unsafe extern "system" fn GetErrorRetryInterval<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwretryinterval: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetErrorRetryInterval(this) {
                    Ok(ok__) => {
                        pwretryinterval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccountInformation<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszaccountname: windows_core::PCWSTR, pwszpassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduledWorkItem_Impl::SetAccountInformation(this, core::mem::transmute(&pwszaccountname), core::mem::transmute(&pwszpassword)).into()
            }
        }
        unsafe extern "system" fn GetAccountInformation<Identity: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszaccountname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduledWorkItem_Impl::GetAccountInformation(this) {
                    Ok(ok__) => {
                        ppwszaccountname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTrigger: CreateTrigger::<Identity, OFFSET>,
            DeleteTrigger: DeleteTrigger::<Identity, OFFSET>,
            GetTriggerCount: GetTriggerCount::<Identity, OFFSET>,
            GetTrigger: GetTrigger::<Identity, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, OFFSET>,
            GetNextRunTime: GetNextRunTime::<Identity, OFFSET>,
            SetIdleWait: SetIdleWait::<Identity, OFFSET>,
            GetIdleWait: GetIdleWait::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            EditWorkItem: EditWorkItem::<Identity, OFFSET>,
            GetMostRecentRunTime: GetMostRecentRunTime::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetExitCode: GetExitCode::<Identity, OFFSET>,
            SetComment: SetComment::<Identity, OFFSET>,
            GetComment: GetComment::<Identity, OFFSET>,
            SetCreator: SetCreator::<Identity, OFFSET>,
            GetCreator: GetCreator::<Identity, OFFSET>,
            SetWorkItemData: SetWorkItemData::<Identity, OFFSET>,
            GetWorkItemData: GetWorkItemData::<Identity, OFFSET>,
            SetErrorRetryCount: SetErrorRetryCount::<Identity, OFFSET>,
            GetErrorRetryCount: GetErrorRetryCount::<Identity, OFFSET>,
            SetErrorRetryInterval: SetErrorRetryInterval::<Identity, OFFSET>,
            GetErrorRetryInterval: GetErrorRetryInterval::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, OFFSET>,
            GetAccountInformation: GetAccountInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduledWorkItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScheduledWorkItem {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISessionStateChangeTrigger, ISessionStateChangeTrigger_Vtbl, 0x754da71b_4385_4475_9dd9_598294fa3641);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISessionStateChangeTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISessionStateChangeTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl ISessionStateChangeTrigger {
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok() }
    }
    pub unsafe fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)).ok() }
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok() }
    }
    pub unsafe fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)).ok() }
    }
    pub unsafe fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StateChange)(windows_core::Interface::as_raw(self), ptype as _).ok() }
    }
    pub unsafe fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStateChange)(windows_core::Interface::as_raw(self), r#type).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISessionStateChangeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
    pub SetStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISessionStateChangeTrigger_Impl: ITrigger_Impl {
    fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDelay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetUserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()>;
    fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISessionStateChangeTrigger_Vtbl {
    pub const fn new<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delay<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::Delay(this, core::mem::transmute_copy(&pdelay)).into()
            }
        }
        unsafe extern "system" fn SetDelay<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::SetDelay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn UserId<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::UserId(this, core::mem::transmute_copy(&puser)).into()
            }
        }
        unsafe extern "system" fn SetUserId<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::SetUserId(this, core::mem::transmute(&user)).into()
            }
        }
        unsafe extern "system" fn StateChange<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::StateChange(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn SetStateChange<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::SetStateChange(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            Delay: Delay::<Identity, OFFSET>,
            SetDelay: SetDelay::<Identity, OFFSET>,
            UserId: UserId::<Identity, OFFSET>,
            SetUserId: SetUserId::<Identity, OFFSET>,
            StateChange: StateChange::<Identity, OFFSET>,
            SetStateChange: SetStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISessionStateChangeTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IShowMessageAction, IShowMessageAction_Vtbl, 0x505e9e68_af89_46b8_a30f_56162a83d537);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IShowMessageAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IShowMessageAction, windows_core::IUnknown, super::Com::IDispatch, IAction);
#[cfg(feature = "Win32_System_Com")]
impl IShowMessageAction {
    pub unsafe fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), core::mem::transmute(ptitle)).ok() }
    }
    pub unsafe fn SetTitle(&self, title: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(title)).ok() }
    }
    pub unsafe fn MessageBody(&self, pmessagebody: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MessageBody)(windows_core::Interface::as_raw(self), core::mem::transmute(pmessagebody)).ok() }
    }
    pub unsafe fn SetMessageBody(&self, messagebody: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagebody)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IShowMessageAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IShowMessageAction_Impl: IAction_Impl {
    fn Title(&self, ptitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTitle(&self, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MessageBody(&self, pmessagebody: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetMessageBody(&self, messagebody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IShowMessageAction_Vtbl {
    pub const fn new<Identity: IShowMessageAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Title<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::Title(this, core::mem::transmute_copy(&ptitle)).into()
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::SetTitle(this, core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn MessageBody<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagebody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::MessageBody(this, core::mem::transmute_copy(&pmessagebody)).into()
            }
        }
        unsafe extern "system" fn SetMessageBody<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagebody: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::SetMessageBody(this, core::mem::transmute(&messagebody)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            MessageBody: MessageBody::<Identity, OFFSET>,
            SetMessageBody: SetMessageBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShowMessageAction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IShowMessageAction {}
windows_core::imp::define_interface!(ITask, ITask_Vtbl, 0x148bd524_a2ab_11ce_b11f_00aa00530503);
impl core::ops::Deref for ITask {
    type Target = IScheduledWorkItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITask, windows_core::IUnknown, IScheduledWorkItem);
impl ITask {
    pub unsafe fn SetApplicationName<P0>(&self, pwszapplicationname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationName)(windows_core::Interface::as_raw(self), pwszapplicationname.param().abi()).ok() }
    }
    pub unsafe fn GetApplicationName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters<P0>(&self, pwszparameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), pwszparameters.param().abi()).ok() }
    }
    pub unsafe fn GetParameters(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, pwszworkingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), pwszworkingdirectory.param().abi()).ok() }
    }
    pub unsafe fn GetWorkingDirectory(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPriority(&self, dwpriority: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), dwpriority).ok() }
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTaskFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskFlags)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetTaskFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTaskFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxRunTime)(windows_core::Interface::as_raw(self), dwmaxruntimems).ok() }
    }
    pub unsafe fn GetMaxRunTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITask_Vtbl {
    pub base__: IScheduledWorkItem_Vtbl,
    pub SetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTaskFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTaskFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaxRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ITask_Impl: IScheduledWorkItem_Impl {
    fn SetApplicationName(&self, pwszapplicationname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetApplicationName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetParameters(&self, pwszparameters: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetParameters(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetWorkingDirectory(&self, pwszworkingdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetWorkingDirectory(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPriority(&self, dwpriority: u32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<u32>;
    fn SetTaskFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetTaskFlags(&self) -> windows_core::Result<u32>;
    fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> windows_core::Result<()>;
    fn GetMaxRunTime(&self) -> windows_core::Result<u32>;
}
impl ITask_Vtbl {
    pub const fn new<Identity: ITask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetApplicationName<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszapplicationname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetApplicationName(this, core::mem::transmute(&pwszapplicationname)).into()
            }
        }
        unsafe extern "system" fn GetApplicationName<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszapplicationname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetApplicationName(this) {
                    Ok(ok__) => {
                        ppwszapplicationname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszparameters: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetParameters(this, core::mem::transmute(&pwszparameters)).into()
            }
        }
        unsafe extern "system" fn GetParameters<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetParameters(this) {
                    Ok(ok__) => {
                        ppwszparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszworkingdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetWorkingDirectory(this, core::mem::transmute(&pwszworkingdirectory)).into()
            }
        }
        unsafe extern "system" fn GetWorkingDirectory<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszworkingdirectory: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetWorkingDirectory(this) {
                    Ok(ok__) => {
                        ppwszworkingdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpriority: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetPriority(this, core::mem::transmute_copy(&dwpriority)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpriority: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetPriority(this) {
                    Ok(ok__) => {
                        pdwpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskFlags<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetTaskFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetTaskFlags<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetTaskFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxRunTime<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxruntimems: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITask_Impl::SetMaxRunTime(this, core::mem::transmute_copy(&dwmaxruntimems)).into()
            }
        }
        unsafe extern "system" fn GetMaxRunTime<Identity: ITask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxruntimems: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITask_Impl::GetMaxRunTime(this) {
                    Ok(ok__) => {
                        pdwmaxruntimems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IScheduledWorkItem_Vtbl::new::<Identity, OFFSET>(),
            SetApplicationName: SetApplicationName::<Identity, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            GetWorkingDirectory: GetWorkingDirectory::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetTaskFlags: SetTaskFlags::<Identity, OFFSET>,
            GetTaskFlags: GetTaskFlags::<Identity, OFFSET>,
            SetMaxRunTime: SetMaxRunTime::<Identity, OFFSET>,
            GetMaxRunTime: GetMaxRunTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITask as windows_core::Interface>::IID || iid == &<IScheduledWorkItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITask {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskDefinition, ITaskDefinition_Vtbl, 0xf5bc8fc5_536d_4f77_b852_fbc1356fdeb6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskDefinition {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskDefinition, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskDefinition {
    pub unsafe fn RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistrationInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRegistrationInfo<P0>(&self, pregistrationinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRegistrationInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRegistrationInfo)(windows_core::Interface::as_raw(self), pregistrationinfo.param().abi()).ok() }
    }
    pub unsafe fn Triggers(&self) -> windows_core::Result<ITriggerCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Triggers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTriggers<P0>(&self, ptriggers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITriggerCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTriggers)(windows_core::Interface::as_raw(self), ptriggers.param().abi()).ok() }
    }
    pub unsafe fn Settings(&self) -> windows_core::Result<ITaskSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSettings<P0>(&self, psettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSettings)(windows_core::Interface::as_raw(self), psettings.param().abi()).ok() }
    }
    pub unsafe fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata)).ok() }
    }
    pub unsafe fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)).ok() }
    }
    pub unsafe fn Principal(&self) -> windows_core::Result<IPrincipal> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Principal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrincipal<P0>(&self, pprincipal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrincipal>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrincipal)(windows_core::Interface::as_raw(self), pprincipal.param().abi()).ok() }
    }
    pub unsafe fn Actions(&self) -> windows_core::Result<IActionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetActions<P0>(&self, pactions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActionCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActions)(windows_core::Interface::as_raw(self), pactions.param().abi()).ok() }
    }
    pub unsafe fn XmlText(&self, pxml: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(pxml)).ok() }
    }
    pub unsafe fn SetXmlText(&self, xml: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskDefinition_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Triggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTriggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Principal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrincipal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskDefinition_Impl: super::Com::IDispatch_Impl {
    fn RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo>;
    fn SetRegistrationInfo(&self, pregistrationinfo: windows_core::Ref<IRegistrationInfo>) -> windows_core::Result<()>;
    fn Triggers(&self) -> windows_core::Result<ITriggerCollection>;
    fn SetTriggers(&self, ptriggers: windows_core::Ref<ITriggerCollection>) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<ITaskSettings>;
    fn SetSettings(&self, psettings: windows_core::Ref<ITaskSettings>) -> windows_core::Result<()>;
    fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Principal(&self) -> windows_core::Result<IPrincipal>;
    fn SetPrincipal(&self, pprincipal: windows_core::Ref<IPrincipal>) -> windows_core::Result<()>;
    fn Actions(&self) -> windows_core::Result<IActionCollection>;
    fn SetActions(&self, pactions: windows_core::Ref<IActionCollection>) -> windows_core::Result<()>;
    fn XmlText(&self, pxml: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, xml: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskDefinition_Vtbl {
    pub const fn new<Identity: ITaskDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegistrationInfo<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrationinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::RegistrationInfo(this) {
                    Ok(ok__) => {
                        ppregistrationinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegistrationInfo<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregistrationinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetRegistrationInfo(this, core::mem::transmute_copy(&pregistrationinfo)).into()
            }
        }
        unsafe extern "system" fn Triggers<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptriggers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::Triggers(this) {
                    Ok(ok__) => {
                        pptriggers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTriggers<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptriggers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetTriggers(this, core::mem::transmute_copy(&ptriggers)).into()
            }
        }
        unsafe extern "system" fn Settings<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::Settings(this) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSettings<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetSettings(this, core::mem::transmute_copy(&psettings)).into()
            }
        }
        unsafe extern "system" fn Data<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::Data(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetData<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetData(this, core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn Principal<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprincipal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::Principal(this) {
                    Ok(ok__) => {
                        ppprincipal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrincipal<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprincipal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetPrincipal(this, core::mem::transmute_copy(&pprincipal)).into()
            }
        }
        unsafe extern "system" fn Actions<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppactions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::Actions(this) {
                    Ok(ok__) => {
                        ppactions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActions<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetActions(this, core::mem::transmute_copy(&pactions)).into()
            }
        }
        unsafe extern "system" fn XmlText<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::XmlText(this, core::mem::transmute_copy(&pxml)).into()
            }
        }
        unsafe extern "system" fn SetXmlText<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::SetXmlText(this, core::mem::transmute(&xml)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RegistrationInfo: RegistrationInfo::<Identity, OFFSET>,
            SetRegistrationInfo: SetRegistrationInfo::<Identity, OFFSET>,
            Triggers: Triggers::<Identity, OFFSET>,
            SetTriggers: SetTriggers::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            SetSettings: SetSettings::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            Principal: Principal::<Identity, OFFSET>,
            SetPrincipal: SetPrincipal::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            SetActions: SetActions::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskDefinition as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskDefinition {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskFolder, ITaskFolder_Vtbl, 0x8cfac062_a080_4c15_9a88_aa7c2af80dfc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskFolder {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskFolder, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolder {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFolders(&self, flags: i32) -> windows_core::Result<ITaskFolderCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolders)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreateFolder(&self, subfoldername: &windows_core::BSTR, sddl: &super::Variant::VARIANT) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subfoldername), core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteFolder(&self, subfoldername: &windows_core::BSTR, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subfoldername), flags).ok() }
    }
    pub unsafe fn GetTask(&self, path: &windows_core::BSTR) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTasks(&self, flags: i32) -> windows_core::Result<IRegisteredTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteTask(&self, name: &windows_core::BSTR, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), flags).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterTask(&self, path: &windows_core::BSTR, xmltext: &windows_core::BSTR, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), core::mem::transmute_copy(xmltext), flags, core::mem::transmute_copy(userid), core::mem::transmute_copy(password), logontype, core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RegisterTaskDefinition<P1>(&self, path: &windows_core::BSTR, pdefinition: P1, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask>
    where
        P1: windows_core::Param<ITaskDefinition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterTaskDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), pdefinition.param().abi(), flags, core::mem::transmute_copy(userid), core::mem::transmute_copy(password), logontype, core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl), flags).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolder_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolders: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreateFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreateFolder: usize,
    pub DeleteFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTasks: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::Variant::VARIANT, super::Variant::VARIANT, TASK_LOGON_TYPE, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterTask: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RegisterTaskDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::Variant::VARIANT, super::Variant::VARIANT, TASK_LOGON_TYPE, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RegisterTaskDefinition: usize,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskFolder_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetFolders(&self, flags: i32) -> windows_core::Result<ITaskFolderCollection>;
    fn CreateFolder(&self, subfoldername: &windows_core::BSTR, sddl: &super::Variant::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn DeleteFolder(&self, subfoldername: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn GetTask(&self, path: &windows_core::BSTR) -> windows_core::Result<IRegisteredTask>;
    fn GetTasks(&self, flags: i32) -> windows_core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(&self, name: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
    fn RegisterTask(&self, path: &windows_core::BSTR, xmltext: &windows_core::BSTR, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(&self, path: &windows_core::BSTR, pdefinition: windows_core::Ref<ITaskDefinition>, flags: i32, userid: &super::Variant::VARIANT, password: &super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Variant::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskFolder_Vtbl {
    pub const fn new<Identity: ITaskFolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Path<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetFolder(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolders<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetFolders(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppfolders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: *mut core::ffi::c_void, sddl: super::Variant::VARIANT, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::CreateFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::DeleteFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetTask(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pptasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetTasks(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pptasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::DeleteTask(this, core::mem::transmute(&name), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn RegisterTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, xmltext: *mut core::ffi::c_void, flags: i32, userid: super::Variant::VARIANT, password: super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::RegisterTask(this, core::mem::transmute(&path), core::mem::transmute(&xmltext), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, pdefinition: *mut core::ffi::c_void, flags: i32, userid: super::Variant::VARIANT, password: super::Variant::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::Variant::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::RegisterTaskDefinition(this, core::mem::transmute(&path), core::mem::transmute_copy(&pdefinition), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: i32, psddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                    Ok(ok__) => {
                        psddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetFolders: GetFolders::<Identity, OFFSET>,
            CreateFolder: CreateFolder::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            GetTask: GetTask::<Identity, OFFSET>,
            GetTasks: GetTasks::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            RegisterTask: RegisterTask::<Identity, OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolder as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskFolder {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskFolderCollection, ITaskFolderCollection_Vtbl, 0x79184a66_8664_423f_97f1_637356a5d812);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskFolderCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskFolderCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolderCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskFolderCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: &super::Variant::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskFolderCollection_Vtbl {
    pub const fn new<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolderCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskFolderCollection {}
windows_core::imp::define_interface!(ITaskHandler, ITaskHandler_Vtbl, 0x839d7762_5121_4009_9234_4f0d19394f04);
windows_core::imp::interface_hierarchy!(ITaskHandler, windows_core::IUnknown);
impl ITaskHandler {
    pub unsafe fn Start<P0>(&self, phandlerservices: P0, data: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), phandlerservices.param().abi(), core::mem::transmute_copy(data)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITaskHandler_Impl: windows_core::IUnknownImpl {
    fn Start(&self, phandlerservices: windows_core::Ref<windows_core::IUnknown>, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl ITaskHandler_Vtbl {
    pub const fn new<Identity: ITaskHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandlerservices: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Start(this, core::mem::transmute_copy(&phandlerservices), core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskHandler_Impl::Stop(this) {
                    Ok(ok__) => {
                        pretcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Resume(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskHandler {}
windows_core::imp::define_interface!(ITaskHandlerStatus, ITaskHandlerStatus_Vtbl, 0xeaec7a8f_27a0_4ddc_8675_14726a01a38a);
windows_core::imp::interface_hierarchy!(ITaskHandlerStatus, windows_core::IUnknown);
impl ITaskHandlerStatus {
    pub unsafe fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateStatus)(windows_core::Interface::as_raw(self), percentcomplete, core::mem::transmute_copy(statusmessage)).ok() }
    }
    pub unsafe fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TaskCompleted)(windows_core::Interface::as_raw(self), taskerrcode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandlerStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UpdateStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITaskHandlerStatus_Impl: windows_core::IUnknownImpl {
    fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITaskHandlerStatus_Vtbl {
    pub const fn new<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateStatus<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, percentcomplete: i16, statusmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandlerStatus_Impl::UpdateStatus(this, core::mem::transmute_copy(&percentcomplete), core::mem::transmute(&statusmessage)).into()
            }
        }
        unsafe extern "system" fn TaskCompleted<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskerrcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandlerStatus_Impl::TaskCompleted(this, core::mem::transmute_copy(&taskerrcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UpdateStatus: UpdateStatus::<Identity, OFFSET>,
            TaskCompleted: TaskCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandlerStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskHandlerStatus {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskNamedValueCollection, ITaskNamedValueCollection_Vtbl, 0xb4ef826b_63c3_46e4_a504_ef69e4f7ea4d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskNamedValueCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskNamedValueCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValueCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount as _).ok() }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ITaskNamedValuePair> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<ITaskNamedValuePair> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValueCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskNamedValueCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<ITaskNamedValuePair>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<ITaskNamedValuePair>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskNamedValueCollection_Vtbl {
    pub const fn new<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValueCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pppair.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::Create(this, core::mem::transmute(&name), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        pppair.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValueCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValueCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskNamedValueCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskNamedValuePair, ITaskNamedValuePair_Vtbl, 0x39038068_2b46_4afd_8662_7bb6f868d221);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskNamedValuePair {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskNamedValuePair, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValuePair {
    pub unsafe fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok() }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Value(&self, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalue)).ok() }
    }
    pub unsafe fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValuePair_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskNamedValuePair_Impl: super::Com::IDispatch_Impl {
    fn Name(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskNamedValuePair_Vtbl {
    pub const fn new<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::Name(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn SetName<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::Value(this, core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskNamedValuePair {}
windows_core::imp::define_interface!(ITaskScheduler, ITaskScheduler_Vtbl, 0x148bd527_a2ab_11ce_b11f_00aa00530503);
windows_core::imp::interface_hierarchy!(ITaskScheduler, windows_core::IUnknown);
impl ITaskScheduler {
    pub unsafe fn SetTargetComputer<P0>(&self, pwszcomputer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetComputer)(windows_core::Interface::as_raw(self), pwszcomputer.param().abi()).ok() }
    }
    pub unsafe fn GetTargetComputer(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumWorkItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Activate<P0>(&self, pwszname: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), pwszname.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok() }
    }
    pub unsafe fn NewWorkItem<P0>(&self, pwsztaskname: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewWorkItem)(windows_core::Interface::as_raw(self), pwsztaskname.param().abi(), rclsid, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddWorkItem<P0, P1>(&self, pwsztaskname: P0, pworkitem: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IScheduledWorkItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddWorkItem)(windows_core::Interface::as_raw(self), pwsztaskname.param().abi(), pworkitem.param().abi()).ok() }
    }
    pub unsafe fn IsOfType<P0>(&self, pwszname: P0, riid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsOfType)(windows_core::Interface::as_raw(self), pwszname.param().abi(), riid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskScheduler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTargetComputer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTargetComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub NewWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddWorkItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ITaskScheduler_Impl: windows_core::IUnknownImpl {
    fn SetTargetComputer(&self, pwszcomputer: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTargetComputer(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Enum(&self) -> windows_core::Result<IEnumWorkItems>;
    fn Activate(&self, pwszname: &windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn Delete(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn NewWorkItem(&self, pwsztaskname: &windows_core::PCWSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn AddWorkItem(&self, pwsztaskname: &windows_core::PCWSTR, pworkitem: windows_core::Ref<IScheduledWorkItem>) -> windows_core::Result<()>;
    fn IsOfType(&self, pwszname: &windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl ITaskScheduler_Vtbl {
    pub const fn new<Identity: ITaskScheduler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTargetComputer<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcomputer: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskScheduler_Impl::SetTargetComputer(this, core::mem::transmute(&pwszcomputer)).into()
            }
        }
        unsafe extern "system" fn GetTargetComputer<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszcomputer: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskScheduler_Impl::GetTargetComputer(this) {
                    Ok(ok__) => {
                        ppwszcomputer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enum<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumworkitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskScheduler_Impl::Enum(this) {
                    Ok(ok__) => {
                        ppenumworkitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskScheduler_Impl::Activate(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskScheduler_Impl::Delete(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn NewWorkItem<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztaskname: windows_core::PCWSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskScheduler_Impl::NewWorkItem(this, core::mem::transmute(&pwsztaskname), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddWorkItem<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztaskname: windows_core::PCWSTR, pworkitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskScheduler_Impl::AddWorkItem(this, core::mem::transmute(&pwsztaskname), core::mem::transmute_copy(&pworkitem)).into()
            }
        }
        unsafe extern "system" fn IsOfType<Identity: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskScheduler_Impl::IsOfType(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&riid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTargetComputer: SetTargetComputer::<Identity, OFFSET>,
            GetTargetComputer: GetTargetComputer::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            NewWorkItem: NewWorkItem::<Identity, OFFSET>,
            AddWorkItem: AddWorkItem::<Identity, OFFSET>,
            IsOfType: IsOfType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskScheduler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskScheduler {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskService, ITaskService_Vtbl, 0x2faba4c7_4da9_4013_9697_20cc3fd40f85);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskService {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskService, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskService {
    pub unsafe fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRunningTasks(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NewTask(&self, flags: u32) -> windows_core::Result<ITaskDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewTask)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Connect(&self, servername: &super::Variant::VARIANT, user: &super::Variant::VARIANT, domain: &super::Variant::VARIANT, password: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servername), core::mem::transmute_copy(user), core::mem::transmute_copy(domain), core::mem::transmute_copy(password)).ok() }
    }
    pub unsafe fn Connected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Connected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TargetServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectedUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectedDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HighestVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRunningTasks: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NewTask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT, super::Variant::VARIANT, super::Variant::VARIANT, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Connect: usize,
    pub Connected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TargetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectedUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectedDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HighestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskService_Impl: super::Com::IDispatch_Impl {
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetRunningTasks(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection>;
    fn NewTask(&self, flags: u32) -> windows_core::Result<ITaskDefinition>;
    fn Connect(&self, servername: &super::Variant::VARIANT, user: &super::Variant::VARIANT, domain: &super::Variant::VARIANT, password: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Connected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TargetServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HighestVersion(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskService_Vtbl {
    pub const fn new<Identity: ITaskService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFolder<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::GetFolder(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRunningTasks<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::GetRunningTasks(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pprunningtasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewTask<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::NewTask(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppdefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servername: super::Variant::VARIANT, user: super::Variant::VARIANT, domain: super::Variant::VARIANT, password: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskService_Impl::Connect(this, core::mem::transmute(&servername), core::mem::transmute(&user), core::mem::transmute(&domain), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn Connected<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::Connected(this) {
                    Ok(ok__) => {
                        pconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetServer<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::TargetServer(this) {
                    Ok(ok__) => {
                        pserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectedUser<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::ConnectedUser(this) {
                    Ok(ok__) => {
                        puser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectedDomain<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::ConnectedDomain(this) {
                    Ok(ok__) => {
                        pdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HighestVersion<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::HighestVersion(this) {
                    Ok(ok__) => {
                        pversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetRunningTasks: GetRunningTasks::<Identity, OFFSET>,
            NewTask: NewTask::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Connected: Connected::<Identity, OFFSET>,
            TargetServer: TargetServer::<Identity, OFFSET>,
            ConnectedUser: ConnectedUser::<Identity, OFFSET>,
            ConnectedDomain: ConnectedDomain::<Identity, OFFSET>,
            HighestVersion: HighestVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskService as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskService {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskSettings, ITaskSettings_Vtbl, 0x8fd4711d_2d02_4c8c_87e3_eff699de127e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings {
    pub unsafe fn AllowDemandStart(&self, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllowDemandStart)(windows_core::Interface::as_raw(self), pallowdemandstart as _).ok() }
    }
    pub unsafe fn SetAllowDemandStart(&self, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowDemandStart)(windows_core::Interface::as_raw(self), allowdemandstart).ok() }
    }
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestartInterval)(windows_core::Interface::as_raw(self), core::mem::transmute(prestartinterval)).ok() }
    }
    pub unsafe fn SetRestartInterval(&self, restartinterval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRestartInterval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(restartinterval)).ok() }
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestartCount)(windows_core::Interface::as_raw(self), prestartcount as _).ok() }
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRestartCount)(windows_core::Interface::as_raw(self), restartcount).ok() }
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MultipleInstances)(windows_core::Interface::as_raw(self), ppolicy as _).ok() }
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMultipleInstances)(windows_core::Interface::as_raw(self), policy).ok() }
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), pstopifonbatteries as _).ok() }
    }
    pub unsafe fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), stopifonbatteries).ok() }
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), pdisallowstart as _).ok() }
    }
    pub unsafe fn SetDisallowStartIfOnBatteries(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), disallowstart).ok() }
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AllowHardTerminate)(windows_core::Interface::as_raw(self), pallowhardterminate as _).ok() }
    }
    pub unsafe fn SetAllowHardTerminate(&self, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowHardTerminate)(windows_core::Interface::as_raw(self), allowhardterminate).ok() }
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartWhenAvailable)(windows_core::Interface::as_raw(self), pstartwhenavailable as _).ok() }
    }
    pub unsafe fn SetStartWhenAvailable(&self, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartWhenAvailable)(windows_core::Interface::as_raw(self), startwhenavailable).ok() }
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok() }
    }
    pub unsafe fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)).ok() }
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), prunonlyifnetworkavailable as _).ok() }
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), runonlyifnetworkavailable).ok() }
    }
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(pexecutiontimelimit)).ok() }
    }
    pub unsafe fn SetExecutionTimeLimit(&self, executiontimelimit: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(executiontimelimit)).ok() }
    }
    pub unsafe fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), penabled as _).ok() }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), core::mem::transmute(pexpirationdelay)).ok() }
    }
    pub unsafe fn SetDeleteExpiredTaskAfter(&self, expirationdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(expirationdelay)).ok() }
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), ppriority as _).ok() }
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), priority).ok() }
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Compatibility)(windows_core::Interface::as_raw(self), pcompatlevel as _).ok() }
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompatibility)(windows_core::Interface::as_raw(self), compatlevel).ok() }
    }
    pub unsafe fn Hidden(&self, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Hidden)(windows_core::Interface::as_raw(self), phidden as _).ok() }
    }
    pub unsafe fn SetHidden(&self, hidden: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), hidden).ok() }
    }
    pub unsafe fn IdleSettings(&self) -> windows_core::Result<IIdleSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IdleSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetIdleSettings<P0>(&self, pidlesettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IIdleSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIdleSettings)(windows_core::Interface::as_raw(self), pidlesettings.param().abi()).ok() }
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunOnlyIfIdle)(windows_core::Interface::as_raw(self), prunonlyifidle as _).ok() }
    }
    pub unsafe fn SetRunOnlyIfIdle(&self, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRunOnlyIfIdle)(windows_core::Interface::as_raw(self), runonlyifidle).ok() }
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WakeToRun)(windows_core::Interface::as_raw(self), pwake as _).ok() }
    }
    pub unsafe fn SetWakeToRun(&self, wake: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWakeToRun)(windows_core::Interface::as_raw(self), wake).ok() }
    }
    pub unsafe fn NetworkSettings(&self) -> windows_core::Result<INetworkSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetNetworkSettings<P0>(&self, pnetworksettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetworkSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkSettings)(windows_core::Interface::as_raw(self), pnetworksettings.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRestartCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MultipleInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_INSTANCES_POLICY) -> windows_core::HRESULT,
    pub SetMultipleInstances: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_INSTANCES_POLICY) -> windows_core::HRESULT,
    pub StopIfGoingOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopIfGoingOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DisallowStartIfOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisallowStartIfOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AllowHardTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowHardTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub StartWhenAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStartWhenAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Compatibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_COMPATIBILITY) -> windows_core::HRESULT,
    pub SetCompatibility: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_COMPATIBILITY) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub WakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetWakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub NetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings_Impl: super::Com::IDispatch_Impl {
    fn AllowDemandStart(&self, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetAllowDemandStart(&self, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RestartInterval(&self, prestartinterval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestartInterval(&self, restartinterval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RestartCount(&self, prestartcount: *mut i32) -> windows_core::Result<()>;
    fn SetRestartCount(&self, restartcount: i32) -> windows_core::Result<()>;
    fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::Result<()>;
    fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::Result<()>;
    fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartIfOnBatteries(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowHardTerminate(&self, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetAllowHardTerminate(&self, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn StartWhenAvailable(&self, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetStartWhenAvailable(&self, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetXmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExecutionTimeLimit(&self, executiontimelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetDeleteExpiredTaskAfter(&self, expirationdelay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Priority(&self, ppriority: *mut i32) -> windows_core::Result<()>;
    fn SetPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::Result<()>;
    fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::Result<()>;
    fn Hidden(&self, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetHidden(&self, hidden: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IdleSettings(&self) -> windows_core::Result<IIdleSettings>;
    fn SetIdleSettings(&self, pidlesettings: windows_core::Ref<IIdleSettings>) -> windows_core::Result<()>;
    fn RunOnlyIfIdle(&self, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRunOnlyIfIdle(&self, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn WakeToRun(&self, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetWakeToRun(&self, wake: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NetworkSettings(&self) -> windows_core::Result<INetworkSettings>;
    fn SetNetworkSettings(&self, pnetworksettings: windows_core::Ref<INetworkSettings>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskSettings_Vtbl {
    pub const fn new<Identity: ITaskSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowDemandStart<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowdemandstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::AllowDemandStart(this, core::mem::transmute_copy(&pallowdemandstart)).into()
            }
        }
        unsafe extern "system" fn SetAllowDemandStart<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowdemandstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetAllowDemandStart(this, core::mem::transmute_copy(&allowdemandstart)).into()
            }
        }
        unsafe extern "system" fn RestartInterval<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartinterval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::RestartInterval(this, core::mem::transmute_copy(&prestartinterval)).into()
            }
        }
        unsafe extern "system" fn SetRestartInterval<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartinterval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetRestartInterval(this, core::mem::transmute(&restartinterval)).into()
            }
        }
        unsafe extern "system" fn RestartCount<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::RestartCount(this, core::mem::transmute_copy(&prestartcount)).into()
            }
        }
        unsafe extern "system" fn SetRestartCount<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetRestartCount(this, core::mem::transmute_copy(&restartcount)).into()
            }
        }
        unsafe extern "system" fn MultipleInstances<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::MultipleInstances(this, core::mem::transmute_copy(&ppolicy)).into()
            }
        }
        unsafe extern "system" fn SetMultipleInstances<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetMultipleInstances(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::StopIfGoingOnBatteries(this, core::mem::transmute_copy(&pstopifonbatteries)).into()
            }
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stopifonbatteries: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetStopIfGoingOnBatteries(this, core::mem::transmute_copy(&stopifonbatteries)).into()
            }
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::DisallowStartIfOnBatteries(this, core::mem::transmute_copy(&pdisallowstart)).into()
            }
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetDisallowStartIfOnBatteries(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn AllowHardTerminate<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::AllowHardTerminate(this, core::mem::transmute_copy(&pallowhardterminate)).into()
            }
        }
        unsafe extern "system" fn SetAllowHardTerminate<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowhardterminate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetAllowHardTerminate(this, core::mem::transmute_copy(&allowhardterminate)).into()
            }
        }
        unsafe extern "system" fn StartWhenAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::StartWhenAvailable(this, core::mem::transmute_copy(&pstartwhenavailable)).into()
            }
        }
        unsafe extern "system" fn SetStartWhenAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startwhenavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetStartWhenAvailable(this, core::mem::transmute_copy(&startwhenavailable)).into()
            }
        }
        unsafe extern "system" fn XmlText<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::XmlText(this, core::mem::transmute_copy(&ptext)).into()
            }
        }
        unsafe extern "system" fn SetXmlText<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetXmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::RunOnlyIfNetworkAvailable(this, core::mem::transmute_copy(&prunonlyifnetworkavailable)).into()
            }
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifnetworkavailable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetRunOnlyIfNetworkAvailable(this, core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
            }
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexecutiontimelimit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::ExecutionTimeLimit(this, core::mem::transmute_copy(&pexecutiontimelimit)).into()
            }
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executiontimelimit: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetExecutionTimeLimit(this, core::mem::transmute(&executiontimelimit)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::Enabled(this, core::mem::transmute_copy(&penabled)).into()
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpirationdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::DeleteExpiredTaskAfter(this, core::mem::transmute_copy(&pexpirationdelay)).into()
            }
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expirationdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetDeleteExpiredTaskAfter(this, core::mem::transmute(&expirationdelay)).into()
            }
        }
        unsafe extern "system" fn Priority<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::Priority(this, core::mem::transmute_copy(&ppriority)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn Compatibility<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::Compatibility(this, core::mem::transmute_copy(&pcompatlevel)).into()
            }
        }
        unsafe extern "system" fn SetCompatibility<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetCompatibility(this, core::mem::transmute_copy(&compatlevel)).into()
            }
        }
        unsafe extern "system" fn Hidden<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::Hidden(this, core::mem::transmute_copy(&phidden)).into()
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidden: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetHidden(this, core::mem::transmute_copy(&hidden)).into()
            }
        }
        unsafe extern "system" fn IdleSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidlesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::IdleSettings(this) {
                    Ok(ok__) => {
                        ppidlesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIdleSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidlesettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetIdleSettings(this, core::mem::transmute_copy(&pidlesettings)).into()
            }
        }
        unsafe extern "system" fn RunOnlyIfIdle<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::RunOnlyIfIdle(this, core::mem::transmute_copy(&prunonlyifidle)).into()
            }
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifidle: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetRunOnlyIfIdle(this, core::mem::transmute_copy(&runonlyifidle)).into()
            }
        }
        unsafe extern "system" fn WakeToRun<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::WakeToRun(this, core::mem::transmute_copy(&pwake)).into()
            }
        }
        unsafe extern "system" fn SetWakeToRun<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wake: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetWakeToRun(this, core::mem::transmute_copy(&wake)).into()
            }
        }
        unsafe extern "system" fn NetworkSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::NetworkSettings(this) {
                    Ok(ok__) => {
                        ppnetworksettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::SetNetworkSettings(this, core::mem::transmute_copy(&pnetworksettings)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AllowDemandStart: AllowDemandStart::<Identity, OFFSET>,
            SetAllowDemandStart: SetAllowDemandStart::<Identity, OFFSET>,
            RestartInterval: RestartInterval::<Identity, OFFSET>,
            SetRestartInterval: SetRestartInterval::<Identity, OFFSET>,
            RestartCount: RestartCount::<Identity, OFFSET>,
            SetRestartCount: SetRestartCount::<Identity, OFFSET>,
            MultipleInstances: MultipleInstances::<Identity, OFFSET>,
            SetMultipleInstances: SetMultipleInstances::<Identity, OFFSET>,
            StopIfGoingOnBatteries: StopIfGoingOnBatteries::<Identity, OFFSET>,
            SetStopIfGoingOnBatteries: SetStopIfGoingOnBatteries::<Identity, OFFSET>,
            DisallowStartIfOnBatteries: DisallowStartIfOnBatteries::<Identity, OFFSET>,
            SetDisallowStartIfOnBatteries: SetDisallowStartIfOnBatteries::<Identity, OFFSET>,
            AllowHardTerminate: AllowHardTerminate::<Identity, OFFSET>,
            SetAllowHardTerminate: SetAllowHardTerminate::<Identity, OFFSET>,
            StartWhenAvailable: StartWhenAvailable::<Identity, OFFSET>,
            SetStartWhenAvailable: SetStartWhenAvailable::<Identity, OFFSET>,
            XmlText: XmlText::<Identity, OFFSET>,
            SetXmlText: SetXmlText::<Identity, OFFSET>,
            RunOnlyIfNetworkAvailable: RunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            SetRunOnlyIfNetworkAvailable: SetRunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            DeleteExpiredTaskAfter: DeleteExpiredTaskAfter::<Identity, OFFSET>,
            SetDeleteExpiredTaskAfter: SetDeleteExpiredTaskAfter::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Compatibility: Compatibility::<Identity, OFFSET>,
            SetCompatibility: SetCompatibility::<Identity, OFFSET>,
            Hidden: Hidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            IdleSettings: IdleSettings::<Identity, OFFSET>,
            SetIdleSettings: SetIdleSettings::<Identity, OFFSET>,
            RunOnlyIfIdle: RunOnlyIfIdle::<Identity, OFFSET>,
            SetRunOnlyIfIdle: SetRunOnlyIfIdle::<Identity, OFFSET>,
            WakeToRun: WakeToRun::<Identity, OFFSET>,
            SetWakeToRun: SetWakeToRun::<Identity, OFFSET>,
            NetworkSettings: NetworkSettings::<Identity, OFFSET>,
            SetNetworkSettings: SetNetworkSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskSettings2, ITaskSettings2_Vtbl, 0x2c05c3f0_6eed_4c05_a15f_ed7d7a98a369);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskSettings2 {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskSettings2, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings2 {
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), pdisallowstart as _).ok() }
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart).ok() }
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), puseunifiedengine as _).ok() }
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings2_Impl: super::Com::IDispatch_Impl {
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskSettings2_Vtbl {
    pub const fn new<Identity: ITaskSettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&pdisallowstart)).into()
            }
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::SetDisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&puseunifiedengine)).into()
            }
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::SetUseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskSettings2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITaskSettings3, ITaskSettings3_Vtbl, 0x0ad9d0d7_0c7f_4ebb_9a5f_d1c648dca528);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITaskSettings3 {
    type Target = ITaskSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITaskSettings3, windows_core::IUnknown, super::Com::IDispatch, ITaskSettings);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings3 {
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), pdisallowstart as _).ok() }
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart).ok() }
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), puseunifiedengine as _).ok() }
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine).ok() }
    }
    pub unsafe fn MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMaintenanceSettings<P0>(&self, pmaintenancesettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMaintenanceSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMaintenanceSettings)(windows_core::Interface::as_raw(self), pmaintenancesettings.param().abi()).ok() }
    }
    pub unsafe fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Volatile(&self, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Volatile)(windows_core::Interface::as_raw(self), pvolatile as _).ok() }
    }
    pub unsafe fn SetVolatile(&self, volatile: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolatile)(windows_core::Interface::as_raw(self), volatile).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings3_Vtbl {
    pub base__: ITaskSettings_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Volatile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVolatile: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITaskSettings3_Impl: ITaskSettings_Impl {
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn SetMaintenanceSettings(&self, pmaintenancesettings: windows_core::Ref<IMaintenanceSettings>) -> windows_core::Result<()>;
    fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn Volatile(&self, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetVolatile(&self, volatile: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITaskSettings3_Vtbl {
    pub const fn new<Identity: ITaskSettings3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&pdisallowstart)).into()
            }
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::SetDisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&puseunifiedengine)).into()
            }
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::SetUseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
            }
        }
        unsafe extern "system" fn MaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::MaintenanceSettings(this) {
                    Ok(ok__) => {
                        ppmaintenancesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaintenancesettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::SetMaintenanceSettings(this, core::mem::transmute_copy(&pmaintenancesettings)).into()
            }
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::CreateMaintenanceSettings(this) {
                    Ok(ok__) => {
                        ppmaintenancesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Volatile<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::Volatile(this, core::mem::transmute_copy(&pvolatile)).into()
            }
        }
        unsafe extern "system" fn SetVolatile<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volatile: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::SetVolatile(this, core::mem::transmute_copy(&volatile)).into()
            }
        }
        Self {
            base__: ITaskSettings_Vtbl::new::<Identity, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, OFFSET>,
            MaintenanceSettings: MaintenanceSettings::<Identity, OFFSET>,
            SetMaintenanceSettings: SetMaintenanceSettings::<Identity, OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Identity, OFFSET>,
            Volatile: Volatile::<Identity, OFFSET>,
            SetVolatile: SetVolatile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITaskSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITaskSettings3 {}
windows_core::imp::define_interface!(ITaskTrigger, ITaskTrigger_Vtbl, 0x148bd52b_a2ab_11ce_b11f_00aa00530503);
windows_core::imp::interface_hierarchy!(ITaskTrigger, windows_core::IUnknown);
impl ITaskTrigger {
    pub unsafe fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTrigger)(windows_core::Interface::as_raw(self), ptrigger).ok() }
    }
    pub unsafe fn GetTrigger(&self, ptrigger: *mut TASK_TRIGGER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTrigger)(windows_core::Interface::as_raw(self), ptrigger as _).ok() }
    }
    pub unsafe fn GetTriggerString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTriggerString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskTrigger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *const TASK_TRIGGER) -> windows_core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_TRIGGER) -> windows_core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ITaskTrigger_Impl: windows_core::IUnknownImpl {
    fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> windows_core::Result<()>;
    fn GetTrigger(&self, ptrigger: *mut TASK_TRIGGER) -> windows_core::Result<()>;
    fn GetTriggerString(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ITaskTrigger_Vtbl {
    pub const fn new<Identity: ITaskTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTrigger<Identity: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskTrigger_Impl::SetTrigger(this, core::mem::transmute_copy(&ptrigger)).into()
            }
        }
        unsafe extern "system" fn GetTrigger<Identity: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskTrigger_Impl::GetTrigger(this, core::mem::transmute_copy(&ptrigger)).into()
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsztrigger: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskTrigger_Impl::GetTriggerString(this) {
                    Ok(ok__) => {
                        ppwsztrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTrigger: SetTrigger::<Identity, OFFSET>,
            GetTrigger: GetTrigger::<Identity, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskTrigger as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskTrigger {}
windows_core::imp::define_interface!(ITaskVariables, ITaskVariables_Vtbl, 0x3e4c9351_d966_4b8b_bb87_ceba68bb0107);
windows_core::imp::interface_hierarchy!(ITaskVariables, windows_core::IUnknown);
impl ITaskVariables {
    pub unsafe fn GetInput(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOutput(&self, input: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(input)).ok() }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskVariables_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITaskVariables_Impl: windows_core::IUnknownImpl {
    fn GetInput(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOutput(&self, input: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITaskVariables_Vtbl {
    pub const fn new<Identity: ITaskVariables_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInput<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskVariables_Impl::GetInput(this) {
                    Ok(ok__) => {
                        pinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutput<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, input: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskVariables_Impl::SetOutput(this, core::mem::transmute(&input)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskVariables_Impl::GetContext(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInput: GetInput::<Identity, OFFSET>,
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskVariables as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskVariables {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITimeTrigger, ITimeTrigger_Vtbl, 0xb45747e0_eba7_4276_9f29_85c5bb300006);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITimeTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITimeTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl ITimeTrigger {
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok() }
    }
    pub unsafe fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimeTrigger_Impl: ITrigger_Impl {
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: ITimeTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RandomDelay<Identity: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimeTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
            }
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimeTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITimeTrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITrigger, ITrigger_Vtbl, 0x09941815_ea89_4b5b_89e0_2a773801fac3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITrigger {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITrigger, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), ptype as _).ok() }
    }
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok() }
    }
    pub unsafe fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)).ok() }
    }
    pub unsafe fn Repetition(&self) -> windows_core::Result<IRepetitionPattern> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Repetition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRepetition<P0>(&self, prepeat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRepetitionPattern>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRepetition)(windows_core::Interface::as_raw(self), prepeat.param().abi()).ok() }
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(ptimelimit)).ok() }
    }
    pub unsafe fn SetExecutionTimeLimit(&self, timelimit: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timelimit)).ok() }
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute(pstart)).ok() }
    }
    pub unsafe fn SetStartBoundary(&self, start: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)).ok() }
    }
    pub unsafe fn EndBoundary(&self, pend: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute(pend)).ok() }
    }
    pub unsafe fn SetEndBoundary(&self, end: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(end)).ok() }
    }
    pub unsafe fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), penabled as _).ok() }
    }
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITrigger_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Repetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRepetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITrigger_Impl: super::Com::IDispatch_Impl {
    fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::Result<()>;
    fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetId(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Repetition(&self) -> windows_core::Result<IRepetitionPattern>;
    fn SetRepetition(&self, prepeat: windows_core::Ref<IRepetitionPattern>) -> windows_core::Result<()>;
    fn ExecutionTimeLimit(&self, ptimelimit: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetExecutionTimeLimit(&self, timelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartBoundary(&self, pstart: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStartBoundary(&self, start: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EndBoundary(&self, pend: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetEndBoundary(&self, end: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITrigger_Vtbl {
    pub const fn new<Identity: ITrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::Type(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::Id(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn SetId<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetId(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn Repetition<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepeat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::Repetition(this) {
                    Ok(ok__) => {
                        pprepeat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRepetition<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prepeat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetRepetition(this, core::mem::transmute_copy(&prepeat)).into()
            }
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimelimit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::ExecutionTimeLimit(this, core::mem::transmute_copy(&ptimelimit)).into()
            }
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timelimit: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetExecutionTimeLimit(this, core::mem::transmute(&timelimit)).into()
            }
        }
        unsafe extern "system" fn StartBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::StartBoundary(this, core::mem::transmute_copy(&pstart)).into()
            }
        }
        unsafe extern "system" fn SetStartBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetStartBoundary(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn EndBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::EndBoundary(this, core::mem::transmute_copy(&pend)).into()
            }
        }
        unsafe extern "system" fn SetEndBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetEndBoundary(this, core::mem::transmute(&end)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::Enabled(this, core::mem::transmute_copy(&penabled)).into()
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Repetition: Repetition::<Identity, OFFSET>,
            SetRepetition: SetRepetition::<Identity, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, OFFSET>,
            StartBoundary: StartBoundary::<Identity, OFFSET>,
            SetStartBoundary: SetStartBoundary::<Identity, OFFSET>,
            EndBoundary: EndBoundary::<Identity, OFFSET>,
            SetEndBoundary: SetEndBoundary::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITrigger {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ITriggerCollection, ITriggerCollection_Vtbl, 0x85df5081_1b24_4f32_878a_d9d14df4cb77);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ITriggerCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ITriggerCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ITriggerCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount as _).ok() }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_TRIGGER_TYPE2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITriggerCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self, pcount: *mut i32) -> windows_core::Result<()>;
    fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger>;
    fn Remove(&self, index: &super::Variant::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITriggerCollection_Vtbl {
    pub const fn new<Identity: ITriggerCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITriggerCollection_Impl::Count(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pptrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pptrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITriggerCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITriggerCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITriggerCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITriggerCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWeeklyTrigger, IWeeklyTrigger_Vtbl, 0x5038fc98_82ff_436d_8728_a512a57c9dc1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWeeklyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWeeklyTrigger, windows_core::IUnknown, super::Com::IDispatch, ITrigger);
#[cfg(feature = "Win32_System_Com")]
impl IWeeklyTrigger {
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DaysOfWeek)(windows_core::Interface::as_raw(self), pdays as _).ok() }
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDaysOfWeek)(windows_core::Interface::as_raw(self), days).ok() }
    }
    pub unsafe fn WeeksInterval(&self, pweeks: *mut i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WeeksInterval)(windows_core::Interface::as_raw(self), pweeks as _).ok() }
    }
    pub unsafe fn SetWeeksInterval(&self, weeks: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWeeksInterval)(windows_core::Interface::as_raw(self), weeks).ok() }
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok() }
    }
    pub unsafe fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWeeklyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetWeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWeeklyTrigger_Impl: ITrigger_Impl {
    fn DaysOfWeek(&self, pdays: *mut i16) -> windows_core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn WeeksInterval(&self, pweeks: *mut i16) -> windows_core::Result<()>;
    fn SetWeeksInterval(&self, weeks: i16) -> windows_core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWeeklyTrigger_Vtbl {
    pub const fn new<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DaysOfWeek<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::DaysOfWeek(this, core::mem::transmute_copy(&pdays)).into()
            }
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::SetDaysOfWeek(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn WeeksInterval<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::WeeksInterval(this, core::mem::transmute_copy(&pweeks)).into()
            }
        }
        unsafe extern "system" fn SetWeeksInterval<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::SetWeeksInterval(this, core::mem::transmute_copy(&weeks)).into()
            }
        }
        unsafe extern "system" fn RandomDelay<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::RandomDelay(this, core::mem::transmute_copy(&prandomdelay)).into()
            }
        }
        unsafe extern "system" fn SetRandomDelay<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::SetRandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, OFFSET>,
            WeeksInterval: WeeksInterval::<Identity, OFFSET>,
            SetWeeksInterval: SetWeeksInterval::<Identity, OFFSET>,
            RandomDelay: RandomDelay::<Identity, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeeklyTrigger as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWeeklyTrigger {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASKPAGE(pub i32);
pub const TASKPAGE_SCHEDULE: TASKPAGE = TASKPAGE(1i32);
pub const TASKPAGE_SETTINGS: TASKPAGE = TASKPAGE(2i32);
pub const TASKPAGE_TASK: TASKPAGE = TASKPAGE(0i32);
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = TASK_ACTION_TYPE(5i32);
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = TASK_ACTION_TYPE(0i32);
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = TASK_ACTION_TYPE(6i32);
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = TASK_ACTION_TYPE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_ACTION_TYPE(pub i32);
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_COMPATIBILITY(pub i32);
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = TASK_COMPATIBILITY(0i32);
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(1i32);
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(2i32);
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(3i32);
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(4i32);
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = TASK_COMPATIBILITY(5i32);
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = TASK_COMPATIBILITY(6i32);
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(1i32);
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(2i32);
pub const TASK_CREATE: TASK_CREATION = TASK_CREATION(2i32);
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = TASK_CREATION(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_CREATION(pub i32);
pub const TASK_DECEMBER: u32 = 2048u32;
pub const TASK_DISABLE: TASK_CREATION = TASK_CREATION(8i32);
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = TASK_CREATION(16i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_ENUM_FLAGS(pub i32);
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = TASK_ENUM_FLAGS(1i32);
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(7i32);
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(6i32);
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(5i32);
pub const TASK_FEBRUARY: u32 = 2u32;
pub const TASK_FIRST_WEEK: u32 = 1u32;
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
pub const TASK_FLAG_DISABLED: u32 = 4u32;
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
pub const TASK_FOURTH_WEEK: u32 = 4u32;
pub const TASK_FRIDAY: u32 = 32u32;
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = TASK_CREATION(32i32);
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(2i32);
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_INSTANCES_POLICY(pub i32);
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(1i32);
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(3i32);
pub const TASK_JANUARY: u32 = 1u32;
pub const TASK_JULY: u32 = 64u32;
pub const TASK_JUNE: u32 = 32u32;
pub const TASK_LAST_WEEK: u32 = 5u32;
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = TASK_LOGON_TYPE(4i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = TASK_LOGON_TYPE(3i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(6i32);
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = TASK_LOGON_TYPE(0i32);
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(1i32);
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = TASK_LOGON_TYPE(2i32);
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = TASK_LOGON_TYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_LOGON_TYPE(pub i32);
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(2i32);
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_PROCESSTOKENSID_TYPE(pub i32);
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(1i32);
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(3i32);
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(4i32);
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(1i32);
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_RUNLEVEL_TYPE(pub i32);
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = TASK_RUN_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_RUN_FLAGS(pub i32);
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(2i32);
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(0i32);
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(8i32);
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(4i32);
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_SESSION_STATE_CHANGE_TYPE(pub i32);
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_STATE(pub i32);
pub const TASK_STATE_DISABLED: TASK_STATE = TASK_STATE(1i32);
pub const TASK_STATE_QUEUED: TASK_STATE = TASK_STATE(2i32);
pub const TASK_STATE_READY: TASK_STATE = TASK_STATE(3i32);
pub const TASK_STATE_RUNNING: TASK_STATE = TASK_STATE(4i32);
pub const TASK_STATE_UNKNOWN: TASK_STATE = TASK_STATE(0i32);
pub const TASK_SUNDAY: u32 = 1u32;
pub const TASK_THIRD_WEEK: u32 = 3u32;
pub const TASK_THURSDAY: u32 = 16u32;
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(1i32);
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(3i32);
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(4i32);
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(0i32);
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl Default for TASK_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(8i32);
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(12i32);
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(2i32);
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(0i32);
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(6i32);
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(9i32);
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(4i32);
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(5i32);
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(7i32);
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(11i32);
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_TRIGGER_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TASK_TRIGGER_TYPE2(pub i32);
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(3i32);
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_UPDATE: TASK_CREATION = TASK_CREATION(4i32);
pub const TASK_VALIDATE_ONLY: TASK_CREATION = TASK_CREATION(1i32);
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl Default for TRIGGER_TYPE_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TaskHandlerPS: windows_core::GUID = windows_core::GUID::from_u128(0xf2a69db7_da2c_4352_9066_86fee6dacac9);
pub const TaskHandlerStatusPS: windows_core::GUID = windows_core::GUID::from_u128(0x9f15266d_d7ba_48f0_93c1_e6895f6fe5ac);
pub const TaskScheduler: windows_core::GUID = windows_core::GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
