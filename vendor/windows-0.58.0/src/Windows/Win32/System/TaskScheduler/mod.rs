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
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), ptype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_ACTION_TYPE) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<P0>(&self, text: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), text.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, index: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Context(&self, pcontext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn SetContext<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), context.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_ACTION_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IBootTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).ClassId)(windows_core::Interface::as_raw(self), core::mem::transmute(pclsid)).ok()
    }
    pub unsafe fn SetClassId<P0>(&self, clsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClassId)(windows_core::Interface::as_raw(self), clsid.param().abi()).ok()
    }
    pub unsafe fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetData<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IComHandlerAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub ClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClassId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).DaysInterval)(windows_core::Interface::as_raw(self), pdays).ok()
    }
    pub unsafe fn SetDaysInterval(&self, days: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysInterval)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<P0>(&self, randomdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), randomdelay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDailyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetDaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), core::mem::transmute(pserver)).ok()
    }
    pub unsafe fn SetServer<P0>(&self, server: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), server.param().abi()).ok()
    }
    pub unsafe fn Subject(&self, psubject: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), core::mem::transmute(psubject)).ok()
    }
    pub unsafe fn SetSubject<P0>(&self, subject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubject)(windows_core::Interface::as_raw(self), subject.param().abi()).ok()
    }
    pub unsafe fn To(&self, pto: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).To)(windows_core::Interface::as_raw(self), core::mem::transmute(pto)).ok()
    }
    pub unsafe fn SetTo<P0>(&self, to: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTo)(windows_core::Interface::as_raw(self), to.param().abi()).ok()
    }
    pub unsafe fn Cc(&self, pcc: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cc)(windows_core::Interface::as_raw(self), core::mem::transmute(pcc)).ok()
    }
    pub unsafe fn SetCc<P0>(&self, cc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCc)(windows_core::Interface::as_raw(self), cc.param().abi()).ok()
    }
    pub unsafe fn Bcc(&self, pbcc: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Bcc)(windows_core::Interface::as_raw(self), core::mem::transmute(pbcc)).ok()
    }
    pub unsafe fn SetBcc<P0>(&self, bcc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBcc)(windows_core::Interface::as_raw(self), bcc.param().abi()).ok()
    }
    pub unsafe fn ReplyTo(&self, preplyto: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute(preplyto)).ok()
    }
    pub unsafe fn SetReplyTo<P0>(&self, replyto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetReplyTo)(windows_core::Interface::as_raw(self), replyto.param().abi()).ok()
    }
    pub unsafe fn From(&self, pfrom: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).From)(windows_core::Interface::as_raw(self), core::mem::transmute(pfrom)).ok()
    }
    pub unsafe fn SetFrom<P0>(&self, from: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFrom)(windows_core::Interface::as_raw(self), from.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HeaderFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHeaderFields<P0>(&self, pheaderfields: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        (windows_core::Interface::vtable(self).SetHeaderFields)(windows_core::Interface::as_raw(self), pheaderfields.param().abi()).ok()
    }
    pub unsafe fn Body(&self, pbody: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Body)(windows_core::Interface::as_raw(self), core::mem::transmute(pbody)).ok()
    }
    pub unsafe fn SetBody<P0>(&self, body: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), body.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Attachments)(windows_core::Interface::as_raw(self), pattachements).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAttachments)(windows_core::Interface::as_raw(self), pattachements).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEmailAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Cc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCc: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Bcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBcc: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub HeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HeaderFields: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHeaderFields: usize,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Attachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attachments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAttachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAttachments: usize,
}
windows_core::imp::define_interface!(IEnumWorkItems, IEnumWorkItems_Vtbl, 0x148bd528_a2ab_11ce_b11f_00aa00530503);
impl core::ops::Deref for IEnumWorkItems {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumWorkItems, windows_core::IUnknown);
impl IEnumWorkItems {
    pub unsafe fn Next(&self, celt: u32, rgpwsznames: *mut *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgpwsznames, pceltfetched)
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWorkItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumWorkItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Subscription)(windows_core::Interface::as_raw(self), core::mem::transmute(pquery)).ok()
    }
    pub unsafe fn SetSubscription<P0>(&self, query: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscription)(windows_core::Interface::as_raw(self), query.param().abi()).ok()
    }
    pub unsafe fn Delay(&self, pdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValueQueries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueQueries<P0>(&self, pnamedxpaths: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        (windows_core::Interface::vtable(self).SetValueQueries)(windows_core::Interface::as_raw(self), pnamedxpaths.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IEventTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Subscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueQueries: usize,
}
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
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), core::mem::transmute(ppath)).ok()
    }
    pub unsafe fn SetPath<P0>(&self, path: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), path.param().abi()).ok()
    }
    pub unsafe fn Arguments(&self, pargument: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), core::mem::transmute(pargument)).ok()
    }
    pub unsafe fn SetArguments<P0>(&self, argument: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), argument.param().abi()).ok()
    }
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute(pworkingdirectory)).ok()
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, workingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), workingdirectory.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IExecAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).HideAppWindow)(windows_core::Interface::as_raw(self), phideappwindow).ok()
    }
    pub unsafe fn SetHideAppWindow<P0>(&self, hideappwindow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHideAppWindow)(windows_core::Interface::as_raw(self), hideappwindow.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IExecAction2_Vtbl {
    pub base__: IExecAction_Vtbl,
    pub HideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).IdleDuration)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetIdleDuration<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetIdleDuration)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
    pub unsafe fn WaitTimeout(&self, ptimeout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WaitTimeout)(windows_core::Interface::as_raw(self), core::mem::transmute(ptimeout)).ok()
    }
    pub unsafe fn SetWaitTimeout<P0>(&self, timeout: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetWaitTimeout)(windows_core::Interface::as_raw(self), timeout.param().abi()).ok()
    }
    pub unsafe fn StopOnIdleEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopOnIdleEnd)(windows_core::Interface::as_raw(self), pstop).ok()
    }
    pub unsafe fn SetStopOnIdleEnd<P0>(&self, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStopOnIdleEnd)(windows_core::Interface::as_raw(self), stop.param().abi()).ok()
    }
    pub unsafe fn RestartOnIdle(&self, prestart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestartOnIdle)(windows_core::Interface::as_raw(self), prestart).ok()
    }
    pub unsafe fn SetRestartOnIdle<P0>(&self, restart: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRestartOnIdle)(windows_core::Interface::as_raw(self), restart.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIdleSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub IdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetIdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub WaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetWaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
impl IIdleTrigger {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIdleTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
}
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
        (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<P0>(&self, user: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), user.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILogonTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn SetPeriod<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPeriod)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Period(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Period)(windows_core::Interface::as_raw(self), core::mem::transmute(target)).ok()
    }
    pub unsafe fn SetDeadline<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDeadline)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Deadline(&self, target: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), core::mem::transmute(target)).ok()
    }
    pub unsafe fn SetExclusive<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetExclusive)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Exclusive(&self, target: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Exclusive)(windows_core::Interface::as_raw(self), target).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMaintenanceSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Period: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDeadline: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Exclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).DaysOfWeek)(windows_core::Interface::as_raw(self), pdays).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysOfWeek)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn WeeksOfMonth(&self, pweeks: *mut i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WeeksOfMonth)(windows_core::Interface::as_raw(self), pweeks).ok()
    }
    pub unsafe fn SetWeeksOfMonth(&self, weeks: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWeeksOfMonth)(windows_core::Interface::as_raw(self), weeks).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MonthsOfYear)(windows_core::Interface::as_raw(self), pmonths).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMonthsOfYear)(windows_core::Interface::as_raw(self), months).ok()
    }
    pub unsafe fn RunOnLastWeekOfMonth(&self, plastweek: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), plastweek).ok()
    }
    pub unsafe fn SetRunOnLastWeekOfMonth<P0>(&self, lastweek: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), lastweek.param().abi()).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<P0>(&self, randomdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), randomdelay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
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
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).DaysOfMonth)(windows_core::Interface::as_raw(self), pdays).ok()
    }
    pub unsafe fn SetDaysOfMonth(&self, days: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysOfMonth)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MonthsOfYear)(windows_core::Interface::as_raw(self), pmonths).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMonthsOfYear)(windows_core::Interface::as_raw(self), months).ok()
    }
    pub unsafe fn RunOnLastDayOfMonth(&self, plastday: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), plastday).ok()
    }
    pub unsafe fn SetRunOnLastDayOfMonth<P0>(&self, lastday: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), lastday.param().abi()).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<P0>(&self, randomdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), randomdelay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IMonthlyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct INetworkSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self, pname: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetDisplayName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<P0>(&self, user: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), user.param().abi()).ok()
    }
    pub unsafe fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LogonType)(windows_core::Interface::as_raw(self), plogon).ok()
    }
    pub unsafe fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLogonType)(windows_core::Interface::as_raw(self), logon).ok()
    }
    pub unsafe fn GroupId(&self, pgroup: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GroupId)(windows_core::Interface::as_raw(self), core::mem::transmute(pgroup)).ok()
    }
    pub unsafe fn SetGroupId<P0>(&self, group: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGroupId)(windows_core::Interface::as_raw(self), group.param().abi()).ok()
    }
    pub unsafe fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RunLevel)(windows_core::Interface::as_raw(self), prunlevel).ok()
    }
    pub unsafe fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRunLevel)(windows_core::Interface::as_raw(self), runlevel).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrincipal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LogonType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub SetLogonType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub GroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
    pub SetRunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).ProcessTokenSidType)(windows_core::Interface::as_raw(self), pprocesstokensidtype).ok()
    }
    pub unsafe fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProcessTokenSidType)(windows_core::Interface::as_raw(self), processtokensidtype).ok()
    }
    pub unsafe fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequiredPrivilegeCount)(windows_core::Interface::as_raw(self), pcount).ok()
    }
    pub unsafe fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).get_RequiredPrivilege)(windows_core::Interface::as_raw(self), index, core::mem::transmute(pprivilege)).ok()
    }
    pub unsafe fn AddRequiredPrivilege<P0>(&self, privilege: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRequiredPrivilege)(windows_core::Interface::as_raw(self), privilege.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPrincipal2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub SetProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub RequiredPrivilegeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddRequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvideTaskPage, IProvideTaskPage_Vtbl, 0x4086658a_cbbb_11cf_b604_00c04fd8d565);
impl core::ops::Deref for IProvideTaskPage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideTaskPage, windows_core::IUnknown);
impl IProvideTaskPage {
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetPage<P0>(&self, tptype: TASKPAGE, fpersistchanges: P0) -> windows_core::Result<super::super::UI::Controls::HPROPSHEETPAGE>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPage)(windows_core::Interface::as_raw(self), tptype, fpersistchanges.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProvideTaskPage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPage: unsafe extern "system" fn(*mut core::ffi::c_void, TASKPAGE, super::super::Foundation::BOOL, *mut super::super::UI::Controls::HPROPSHEETPAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPage: usize,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<TASK_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Run<P0>(&self, params: P0) -> windows_core::Result<IRunningTask>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), params.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RunEx<P0, P1>(&self, params: P0, flags: i32, sessionid: i32, user: P1) -> windows_core::Result<IRunningTask>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunEx)(windows_core::Interface::as_raw(self), params.param().abi(), flags, sessionid, user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInstances(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstances)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastRunTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastTaskResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastTaskResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumberOfMissedRuns(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumberOfMissedRuns)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NextRunTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NextRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Definition(&self) -> windows_core::Result<ITaskDefinition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Definition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurityDescriptor<P0>(&self, sddl: P0, flags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), sddl.param().abi(), flags).ok()
    }
    pub unsafe fn Stop(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRunTimes)(windows_core::Interface::as_raw(self), pststart, pstend, pcount, pruntimes).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegisteredTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Run: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RunEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RunEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstances: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstances: usize,
    pub LastRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastTaskResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfMissedRuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Definition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Definition: usize,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetRunTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SYSTEMTIME, *const super::super::Foundation::SYSTEMTIME, *mut u32, *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IRegisteredTask>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegisteredTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), core::mem::transmute(pdescription)).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), description.param().abi()).ok()
    }
    pub unsafe fn Author(&self, pauthor: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), core::mem::transmute(pauthor)).ok()
    }
    pub unsafe fn SetAuthor<P0>(&self, author: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAuthor)(windows_core::Interface::as_raw(self), author.param().abi()).ok()
    }
    pub unsafe fn Version(&self, pversion: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), version.param().abi()).ok()
    }
    pub unsafe fn Date(&self, pdate: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Date)(windows_core::Interface::as_raw(self), core::mem::transmute(pdate)).ok()
    }
    pub unsafe fn SetDate<P0>(&self, date: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDate)(windows_core::Interface::as_raw(self), date.param().abi()).ok()
    }
    pub unsafe fn Documentation(&self, pdocumentation: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Documentation)(windows_core::Interface::as_raw(self), core::mem::transmute(pdocumentation)).ok()
    }
    pub unsafe fn SetDocumentation<P0>(&self, documentation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDocumentation)(windows_core::Interface::as_raw(self), documentation.param().abi()).ok()
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<P0>(&self, text: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), text.param().abi()).ok()
    }
    pub unsafe fn URI(&self, puri: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).URI)(windows_core::Interface::as_raw(self), core::mem::transmute(puri)).ok()
    }
    pub unsafe fn SetURI<P0>(&self, uri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetURI)(windows_core::Interface::as_raw(self), uri.param().abi()).ok()
    }
    pub unsafe fn SecurityDescriptor(&self, psddl: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute(psddl)).ok()
    }
    pub unsafe fn SetSecurityDescriptor<P0>(&self, sddl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), sddl.param().abi()).ok()
    }
    pub unsafe fn Source(&self, psource: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), core::mem::transmute(psource)).ok()
    }
    pub unsafe fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), source.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegistrationInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Documentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub URI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetURI: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRegistrationTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Interval)(windows_core::Interface::as_raw(self), core::mem::transmute(pinterval)).ok()
    }
    pub unsafe fn SetInterval<P0>(&self, interval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterval)(windows_core::Interface::as_raw(self), interval.param().abi()).ok()
    }
    pub unsafe fn Duration(&self, pduration: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), core::mem::transmute(pduration)).ok()
    }
    pub unsafe fn SetDuration<P0>(&self, duration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), duration.param().abi()).ok()
    }
    pub unsafe fn StopAtDurationEnd(&self, pstop: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopAtDurationEnd)(windows_core::Interface::as_raw(self), pstop).ok()
    }
    pub unsafe fn SetStopAtDurationEnd<P0>(&self, stop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStopAtDurationEnd)(windows_core::Interface::as_raw(self), stop.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRepetitionPattern_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetStopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstanceGuid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<TASK_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnginePID(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnginePID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRunningTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InstanceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnginePID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<IRunningTask>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRunningTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledWorkItem, IScheduledWorkItem_Vtbl, 0xa6b952f0_a4b1_11d0_997d_00aa006887ec);
impl core::ops::Deref for IScheduledWorkItem {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScheduledWorkItem, windows_core::IUnknown);
impl IScheduledWorkItem {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut Option<ITaskTrigger>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateTrigger)(windows_core::Interface::as_raw(self), pinewtrigger, core::mem::transmute(pptrigger)).ok()
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteTrigger)(windows_core::Interface::as_raw(self), itrigger).ok()
    }
    pub unsafe fn GetTriggerCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTriggerCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> windows_core::Result<ITaskTrigger> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTrigger)(windows_core::Interface::as_raw(self), itrigger, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTriggerString)(windows_core::Interface::as_raw(self), itrigger, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRunTimes)(windows_core::Interface::as_raw(self), pstbegin, pstend, pcount, rgsttasktimes).ok()
    }
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextRunTime)(windows_core::Interface::as_raw(self), pstnextrun).ok()
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetIdleWait)(windows_core::Interface::as_raw(self), widleminutes, wdeadlineminutes).ok()
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdleWait)(windows_core::Interface::as_raw(self), pwidleminutes, pwdeadlineminutes).ok()
    }
    pub unsafe fn Run(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EditWorkItem<P0>(&self, hparent: P0, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).EditWorkItem)(windows_core::Interface::as_raw(self), hparent.param().abi(), dwreserved).ok()
    }
    pub unsafe fn GetMostRecentRunTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMostRecentRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetExitCode(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExitCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetComment<P0>(&self, pwszcomment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetComment)(windows_core::Interface::as_raw(self), pwszcomment.param().abi()).ok()
    }
    pub unsafe fn GetComment(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetComment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreator<P0>(&self, pwszcreator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCreator)(windows_core::Interface::as_raw(self), pwszcreator.param().abi()).ok()
    }
    pub unsafe fn GetCreator(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCreator)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWorkItemData)(windows_core::Interface::as_raw(self), cbdata, rgbdata).ok()
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWorkItemData)(windows_core::Interface::as_raw(self), pcbdata, prgbdata).ok()
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetErrorRetryCount)(windows_core::Interface::as_raw(self), wretrycount).ok()
    }
    pub unsafe fn GetErrorRetryCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorRetryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetErrorRetryInterval)(windows_core::Interface::as_raw(self), wretryinterval).ok()
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetErrorRetryInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccountInformation<P0, P1>(&self, pwszaccountname: P0, pwszpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetAccountInformation)(windows_core::Interface::as_raw(self), pwszaccountname.param().abi(), pwszpassword.param().abi()).ok()
    }
    pub unsafe fn GetAccountInformation(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAccountInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
        (windows_core::Interface::vtable(self).Delay)(windows_core::Interface::as_raw(self), core::mem::transmute(pdelay)).ok()
    }
    pub unsafe fn SetDelay<P0>(&self, delay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDelay)(windows_core::Interface::as_raw(self), delay.param().abi()).ok()
    }
    pub unsafe fn UserId(&self, puser: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UserId)(windows_core::Interface::as_raw(self), core::mem::transmute(puser)).ok()
    }
    pub unsafe fn SetUserId<P0>(&self, user: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserId)(windows_core::Interface::as_raw(self), user.param().abi()).ok()
    }
    pub unsafe fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StateChange)(windows_core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStateChange)(windows_core::Interface::as_raw(self), r#type).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISessionStateChangeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUserId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
    pub SetStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), core::mem::transmute(ptitle)).ok()
    }
    pub unsafe fn SetTitle<P0>(&self, title: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), title.param().abi()).ok()
    }
    pub unsafe fn MessageBody(&self, pmessagebody: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MessageBody)(windows_core::Interface::as_raw(self), core::mem::transmute(pmessagebody)).ok()
    }
    pub unsafe fn SetMessageBody<P0>(&self, messagebody: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMessageBody)(windows_core::Interface::as_raw(self), messagebody.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IShowMessageAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetApplicationName)(windows_core::Interface::as_raw(self), pwszapplicationname.param().abi()).ok()
    }
    pub unsafe fn GetApplicationName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParameters<P0>(&self, pwszparameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), pwszparameters.param().abi()).ok()
    }
    pub unsafe fn GetParameters(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWorkingDirectory<P0>(&self, pwszworkingdirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), pwszworkingdirectory.param().abi()).ok()
    }
    pub unsafe fn GetWorkingDirectory(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, dwpriority: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), dwpriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTaskFlags(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTaskFlags)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetTaskFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTaskFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxRunTime)(windows_core::Interface::as_raw(self), dwmaxruntimems).ok()
    }
    pub unsafe fn GetMaxRunTime(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistrationInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistrationInfo<P0>(&self, pregistrationinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRegistrationInfo>,
    {
        (windows_core::Interface::vtable(self).SetRegistrationInfo)(windows_core::Interface::as_raw(self), pregistrationinfo.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Triggers(&self) -> windows_core::Result<ITriggerCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Triggers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTriggers<P0>(&self, ptriggers: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITriggerCollection>,
    {
        (windows_core::Interface::vtable(self).SetTriggers)(windows_core::Interface::as_raw(self), ptriggers.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> windows_core::Result<ITaskSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSettings<P0>(&self, psettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITaskSettings>,
    {
        (windows_core::Interface::vtable(self).SetSettings)(windows_core::Interface::as_raw(self), psettings.param().abi()).ok()
    }
    pub unsafe fn Data(&self, pdata: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), core::mem::transmute(pdata)).ok()
    }
    pub unsafe fn SetData<P0>(&self, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), data.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Principal(&self) -> windows_core::Result<IPrincipal> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Principal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPrincipal<P0>(&self, pprincipal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrincipal>,
    {
        (windows_core::Interface::vtable(self).SetPrincipal)(windows_core::Interface::as_raw(self), pprincipal.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Actions(&self) -> windows_core::Result<IActionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetActions<P0>(&self, pactions: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActionCollection>,
    {
        (windows_core::Interface::vtable(self).SetActions)(windows_core::Interface::as_raw(self), pactions.param().abi()).ok()
    }
    pub unsafe fn XmlText(&self, pxml: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(pxml)).ok()
    }
    pub unsafe fn SetXmlText<P0>(&self, xml: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), xml.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskDefinition_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Triggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Triggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTriggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTriggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSettings: usize,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Principal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Principal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPrincipal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPrincipal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActions: usize,
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolder<P0>(&self, path: P0) -> windows_core::Result<ITaskFolder>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolders(&self, flags: i32) -> windows_core::Result<ITaskFolderCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFolders)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFolder<P0, P1>(&self, subfoldername: P0, sddl: P1) -> windows_core::Result<ITaskFolder>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFolder)(windows_core::Interface::as_raw(self), subfoldername.param().abi(), sddl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteFolder<P0>(&self, subfoldername: P0, flags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteFolder)(windows_core::Interface::as_raw(self), subfoldername.param().abi(), flags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTask<P0>(&self, path: P0) -> windows_core::Result<IRegisteredTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTask)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTasks(&self, flags: i32) -> windows_core::Result<IRegisteredTaskCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteTask<P0>(&self, name: P0, flags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), name.param().abi(), flags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterTask<P0, P1, P2, P3, P4>(&self, path: P0, xmltext: P1, flags: i32, userid: P2, password: P3, logontype: TASK_LOGON_TYPE, sddl: P4) -> windows_core::Result<IRegisteredTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::VARIANT>,
        P3: windows_core::Param<windows_core::VARIANT>,
        P4: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterTask)(windows_core::Interface::as_raw(self), path.param().abi(), xmltext.param().abi(), flags, userid.param().abi(), password.param().abi(), logontype, sddl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterTaskDefinition<P0, P1, P2, P3, P4>(&self, path: P0, pdefinition: P1, flags: i32, userid: P2, password: P3, logontype: TASK_LOGON_TYPE, sddl: P4) -> windows_core::Result<IRegisteredTask>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<ITaskDefinition>,
        P2: windows_core::Param<windows_core::VARIANT>,
        P3: windows_core::Param<windows_core::VARIANT>,
        P4: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterTaskDefinition)(windows_core::Interface::as_raw(self), path.param().abi(), pdefinition.param().abi(), flags, userid.param().abi(), password.param().abi(), logontype, sddl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurityDescriptor<P0>(&self, sddl: P0, flags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), sddl.param().abi(), flags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskFolder_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolders: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFolder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFolder: usize,
    pub DeleteFolder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, TASK_LOGON_TYPE, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RegisterTaskDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, TASK_LOGON_TYPE, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegisterTaskDefinition: usize,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, index: P0) -> windows_core::Result<ITaskFolder>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskFolderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskHandler, ITaskHandler_Vtbl, 0x839d7762_5121_4009_9234_4f0d19394f04);
impl core::ops::Deref for ITaskHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITaskHandler, windows_core::IUnknown);
impl ITaskHandler {
    pub unsafe fn Start<P0, P1>(&self, phandlerservices: P0, data: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), phandlerservices.param().abi(), data.param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITaskHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskHandlerStatus, ITaskHandlerStatus_Vtbl, 0xeaec7a8f_27a0_4ddc_8675_14726a01a38a);
impl core::ops::Deref for ITaskHandlerStatus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITaskHandlerStatus, windows_core::IUnknown);
impl ITaskHandlerStatus {
    pub unsafe fn UpdateStatus<P0>(&self, percentcomplete: i16, statusmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UpdateStatus)(windows_core::Interface::as_raw(self), percentcomplete, statusmessage.param().abi()).ok()
    }
    pub unsafe fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TaskCompleted)(windows_core::Interface::as_raw(self), taskerrcode).ok()
    }
}
#[repr(C)]
pub struct ITaskHandlerStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UpdateStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i16, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TaskCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ITaskNamedValuePair> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<ITaskNamedValuePair>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskNamedValueCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), core::mem::transmute(pname)).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Value(&self, pvalue: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn SetValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskNamedValuePair_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskScheduler, ITaskScheduler_Vtbl, 0x148bd527_a2ab_11ce_b11f_00aa00530503);
impl core::ops::Deref for ITaskScheduler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITaskScheduler, windows_core::IUnknown);
impl ITaskScheduler {
    pub unsafe fn SetTargetComputer<P0>(&self, pwszcomputer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetComputer)(windows_core::Interface::as_raw(self), pwszcomputer.param().abi()).ok()
    }
    pub unsafe fn GetTargetComputer(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetComputer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Enum(&self) -> windows_core::Result<IEnumWorkItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Activate<P0>(&self, pwszname: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), pwszname.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0>(&self, pwszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), pwszname.param().abi()).ok()
    }
    pub unsafe fn NewWorkItem<P0>(&self, pwsztaskname: P0, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NewWorkItem)(windows_core::Interface::as_raw(self), pwsztaskname.param().abi(), rclsid, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddWorkItem<P0, P1>(&self, pwsztaskname: P0, pworkitem: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IScheduledWorkItem>,
    {
        (windows_core::Interface::vtable(self).AddWorkItem)(windows_core::Interface::as_raw(self), pwsztaskname.param().abi(), pworkitem.param().abi()).ok()
    }
    pub unsafe fn IsOfType<P0>(&self, pwszname: P0, riid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsOfType)(windows_core::Interface::as_raw(self), pwszname.param().abi(), riid).ok()
    }
}
#[repr(C)]
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
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolder<P0>(&self, path: P0) -> windows_core::Result<ITaskFolder>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), path.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRunningTasks(&self, flags: i32) -> windows_core::Result<IRunningTaskCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRunningTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NewTask(&self, flags: u32) -> windows_core::Result<ITaskDefinition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NewTask)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Connect<P0, P1, P2, P3>(&self, servername: P0, user: P1, domain: P2, password: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
        P3: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), servername.param().abi(), user.param().abi(), domain.param().abi(), password.param().abi()).ok()
    }
    pub unsafe fn Connected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Connected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TargetServer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetServer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectedUser)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectedDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HighestVersion(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HighestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRunningTasks: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRunningTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NewTask: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NewTask: usize,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Connected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TargetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ConnectedUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ConnectedDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HighestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).AllowDemandStart)(windows_core::Interface::as_raw(self), pallowdemandstart).ok()
    }
    pub unsafe fn SetAllowDemandStart<P0>(&self, allowdemandstart: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowDemandStart)(windows_core::Interface::as_raw(self), allowdemandstart.param().abi()).ok()
    }
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestartInterval)(windows_core::Interface::as_raw(self), core::mem::transmute(prestartinterval)).ok()
    }
    pub unsafe fn SetRestartInterval<P0>(&self, restartinterval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRestartInterval)(windows_core::Interface::as_raw(self), restartinterval.param().abi()).ok()
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestartCount)(windows_core::Interface::as_raw(self), prestartcount).ok()
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRestartCount)(windows_core::Interface::as_raw(self), restartcount).ok()
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MultipleInstances)(windows_core::Interface::as_raw(self), ppolicy).ok()
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMultipleInstances)(windows_core::Interface::as_raw(self), policy).ok()
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), pstopifonbatteries).ok()
    }
    pub unsafe fn SetStopIfGoingOnBatteries<P0>(&self, stopifonbatteries: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), stopifonbatteries.param().abi()).ok()
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), pdisallowstart).ok()
    }
    pub unsafe fn SetDisallowStartIfOnBatteries<P0>(&self, disallowstart: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), disallowstart.param().abi()).ok()
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllowHardTerminate)(windows_core::Interface::as_raw(self), pallowhardterminate).ok()
    }
    pub unsafe fn SetAllowHardTerminate<P0>(&self, allowhardterminate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowHardTerminate)(windows_core::Interface::as_raw(self), allowhardterminate.param().abi()).ok()
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartWhenAvailable)(windows_core::Interface::as_raw(self), pstartwhenavailable).ok()
    }
    pub unsafe fn SetStartWhenAvailable<P0>(&self, startwhenavailable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStartWhenAvailable)(windows_core::Interface::as_raw(self), startwhenavailable.param().abi()).ok()
    }
    pub unsafe fn XmlText(&self, ptext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute(ptext)).ok()
    }
    pub unsafe fn SetXmlText<P0>(&self, text: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetXmlText)(windows_core::Interface::as_raw(self), text.param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), prunonlyifnetworkavailable).ok()
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable<P0>(&self, runonlyifnetworkavailable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), runonlyifnetworkavailable.param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(pexecutiontimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<P0>(&self, executiontimelimit: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetExecutionTimeLimit)(windows_core::Interface::as_raw(self), executiontimelimit.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), penabled).ok()
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), core::mem::transmute(pexpirationdelay)).ok()
    }
    pub unsafe fn SetDeleteExpiredTaskAfter<P0>(&self, expirationdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), expirationdelay.param().abi()).ok()
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), ppriority).ok()
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Compatibility)(windows_core::Interface::as_raw(self), pcompatlevel).ok()
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCompatibility)(windows_core::Interface::as_raw(self), compatlevel).ok()
    }
    pub unsafe fn Hidden(&self, phidden: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Hidden)(windows_core::Interface::as_raw(self), phidden).ok()
    }
    pub unsafe fn SetHidden<P0>(&self, hidden: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHidden)(windows_core::Interface::as_raw(self), hidden.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdleSettings(&self) -> windows_core::Result<IIdleSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IdleSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIdleSettings<P0>(&self, pidlesettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IIdleSettings>,
    {
        (windows_core::Interface::vtable(self).SetIdleSettings)(windows_core::Interface::as_raw(self), pidlesettings.param().abi()).ok()
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RunOnlyIfIdle)(windows_core::Interface::as_raw(self), prunonlyifidle).ok()
    }
    pub unsafe fn SetRunOnlyIfIdle<P0>(&self, runonlyifidle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRunOnlyIfIdle)(windows_core::Interface::as_raw(self), runonlyifidle.param().abi()).ok()
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WakeToRun)(windows_core::Interface::as_raw(self), pwake).ok()
    }
    pub unsafe fn SetWakeToRun<P0>(&self, wake: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetWakeToRun)(windows_core::Interface::as_raw(self), wake.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetworkSettings(&self) -> windows_core::Result<INetworkSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetworkSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNetworkSettings<P0>(&self, pnetworksettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetworkSettings>,
    {
        (windows_core::Interface::vtable(self).SetNetworkSettings)(windows_core::Interface::as_raw(self), pnetworksettings.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetXmlText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Compatibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_COMPATIBILITY) -> windows_core::HRESULT,
    pub SetCompatibility: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_COMPATIBILITY) -> windows_core::HRESULT,
    pub Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdleSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIdleSettings: usize,
    pub RunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub WakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetWakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetworkSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNetworkSettings: usize,
}
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
        (windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), pdisallowstart).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession<P0>(&self, disallowstart: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart.param().abi()).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), puseunifiedengine).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine<P0>(&self, useunifiedengine: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), pdisallowstart).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession<P0>(&self, disallowstart: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart.param().abi()).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), puseunifiedengine).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine<P0>(&self, useunifiedengine: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaintenanceSettings<P0>(&self, pmaintenancesettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMaintenanceSettings>,
    {
        (windows_core::Interface::vtable(self).SetMaintenanceSettings)(windows_core::Interface::as_raw(self), pmaintenancesettings.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Volatile(&self, pvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Volatile)(windows_core::Interface::as_raw(self), pvolatile).ok()
    }
    pub unsafe fn SetVolatile<P0>(&self, volatile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVolatile)(windows_core::Interface::as_raw(self), volatile.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITaskSettings3_Vtbl {
    pub base__: ITaskSettings_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMaintenanceSettings: usize,
    pub Volatile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetVolatile: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskTrigger, ITaskTrigger_Vtbl, 0x148bd52b_a2ab_11ce_b11f_00aa00530503);
impl core::ops::Deref for ITaskTrigger {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITaskTrigger, windows_core::IUnknown);
impl ITaskTrigger {
    pub unsafe fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTrigger)(windows_core::Interface::as_raw(self), ptrigger).ok()
    }
    pub unsafe fn GetTrigger(&self, ptrigger: *mut TASK_TRIGGER) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTrigger)(windows_core::Interface::as_raw(self), ptrigger).ok()
    }
    pub unsafe fn GetTriggerString(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTriggerString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITaskTrigger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *const TASK_TRIGGER) -> windows_core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_TRIGGER) -> windows_core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskVariables, ITaskVariables_Vtbl, 0x3e4c9351_d966_4b8b_bb87_ceba68bb0107);
impl core::ops::Deref for ITaskVariables {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITaskVariables, windows_core::IUnknown);
impl ITaskVariables {
    pub unsafe fn GetInput(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOutput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), input.param().abi()).ok()
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITaskVariables_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<P0>(&self, randomdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), randomdelay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), ptype).ok()
    }
    pub unsafe fn Id(&self, pid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), core::mem::transmute(pid)).ok()
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> windows_core::Result<IRepetitionPattern> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Repetition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<P0>(&self, prepeat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRepetitionPattern>,
    {
        (windows_core::Interface::vtable(self).SetRepetition)(windows_core::Interface::as_raw(self), prepeat.param().abi()).ok()
    }
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(ptimelimit)).ok()
    }
    pub unsafe fn SetExecutionTimeLimit<P0>(&self, timelimit: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetExecutionTimeLimit)(windows_core::Interface::as_raw(self), timelimit.param().abi()).ok()
    }
    pub unsafe fn StartBoundary(&self, pstart: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StartBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute(pstart)).ok()
    }
    pub unsafe fn SetStartBoundary<P0>(&self, start: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStartBoundary)(windows_core::Interface::as_raw(self), start.param().abi()).ok()
    }
    pub unsafe fn EndBoundary(&self, pend: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetEndBoundary<P0>(&self, end: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEndBoundary)(windows_core::Interface::as_raw(self), end.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), penabled).ok()
    }
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITrigger_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Repetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Repetition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRepetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRepetition: usize,
    pub ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), pcount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Remove<P0>(&self, index: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ITriggerCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_TRIGGER_TYPE2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).DaysOfWeek)(windows_core::Interface::as_raw(self), pdays).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDaysOfWeek)(windows_core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn WeeksInterval(&self, pweeks: *mut i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WeeksInterval)(windows_core::Interface::as_raw(self), pweeks).ok()
    }
    pub unsafe fn SetWeeksInterval(&self, weeks: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWeeksInterval)(windows_core::Interface::as_raw(self), weeks).ok()
    }
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute(prandomdelay)).ok()
    }
    pub unsafe fn SetRandomDelay<P0>(&self, randomdelay: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRandomDelay)(windows_core::Interface::as_raw(self), randomdelay.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWeeklyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetWeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub const CLSID_CTask: windows_core::GUID = windows_core::GUID::from_u128(0x148bd520_a2ab_11ce_b11f_00aa00530503);
pub const CLSID_CTaskScheduler: windows_core::GUID = windows_core::GUID::from_u128(0x148bd52a_a2ab_11ce_b11f_00aa00530503);
pub const TASKPAGE_SCHEDULE: TASKPAGE = TASKPAGE(1i32);
pub const TASKPAGE_SETTINGS: TASKPAGE = TASKPAGE(2i32);
pub const TASKPAGE_TASK: TASKPAGE = TASKPAGE(0i32);
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = TASK_ACTION_TYPE(5i32);
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = TASK_ACTION_TYPE(0i32);
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = TASK_ACTION_TYPE(6i32);
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = TASK_ACTION_TYPE(7i32);
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
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
pub const TASK_DECEMBER: u32 = 2048u32;
pub const TASK_DISABLE: TASK_CREATION = TASK_CREATION(8i32);
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = TASK_CREATION(16i32);
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
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(2i32);
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(0i32);
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(1i32);
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(3i32);
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(4i32);
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(1i32);
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(0i32);
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = TASK_RUN_FLAGS(1i32);
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(2i32);
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(0i32);
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(8i32);
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(4i32);
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(7i32);
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(8i32);
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
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(3i32);
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_UPDATE: TASK_CREATION = TASK_CREATION(4i32);
pub const TASK_VALIDATE_ONLY: TASK_CREATION = TASK_CREATION(1i32);
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASKPAGE(pub i32);
impl windows_core::TypeKind for TASKPAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASKPAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASKPAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_ACTION_TYPE(pub i32);
impl windows_core::TypeKind for TASK_ACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_ACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_ACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_COMPATIBILITY(pub i32);
impl windows_core::TypeKind for TASK_COMPATIBILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_COMPATIBILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_COMPATIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_CREATION(pub i32);
impl windows_core::TypeKind for TASK_CREATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_CREATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_CREATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for TASK_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_ENUM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_INSTANCES_POLICY(pub i32);
impl windows_core::TypeKind for TASK_INSTANCES_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_INSTANCES_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_INSTANCES_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_LOGON_TYPE(pub i32);
impl windows_core::TypeKind for TASK_LOGON_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_LOGON_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_LOGON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_PROCESSTOKENSID_TYPE(pub i32);
impl windows_core::TypeKind for TASK_PROCESSTOKENSID_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_PROCESSTOKENSID_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_PROCESSTOKENSID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_RUNLEVEL_TYPE(pub i32);
impl windows_core::TypeKind for TASK_RUNLEVEL_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_RUNLEVEL_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_RUNLEVEL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_RUN_FLAGS(pub i32);
impl windows_core::TypeKind for TASK_RUN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_RUN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_RUN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_SESSION_STATE_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for TASK_SESSION_STATE_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_SESSION_STATE_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_SESSION_STATE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_STATE(pub i32);
impl windows_core::TypeKind for TASK_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_TRIGGER_TYPE(pub i32);
impl windows_core::TypeKind for TASK_TRIGGER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_TRIGGER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TASK_TRIGGER_TYPE2(pub i32);
impl windows_core::TypeKind for TASK_TRIGGER_TYPE2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TASK_TRIGGER_TYPE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE2").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl windows_core::TypeKind for DAILY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DAILY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl windows_core::TypeKind for MONTHLYDATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONTHLYDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl windows_core::TypeKind for MONTHLYDOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MONTHLYDOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for TASK_TRIGGER {
    type TypeKind = windows_core::CopyType;
}
impl Default for TASK_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl windows_core::TypeKind for TRIGGER_TYPE_UNION {
    type TypeKind = windows_core::CopyType;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl windows_core::TypeKind for WEEKLY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WEEKLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
